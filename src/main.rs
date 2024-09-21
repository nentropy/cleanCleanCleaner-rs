mod net_mon;
mod utils;

use tokio;
use log::{info, error, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use uuid::Uuid;
use anyhow::{Result, Context};
use futures::future::join_all;
use std::path::Path;
use std::io::{SeekFrom, seek};
use tokio::time::Instant
use tokio::fs;
use tokio::process::Command;
use chrono::{Local, TimeZone, Utc, DateTime};
use rand::Rng;
use filetime::{FileTime, from_unix_time};
use filetime::set_file_mtime;


use crate::net_mon::{ NetMon, get_actions, Action };
use crate::utils::save_to_json;
//use tokio::task;
//use tokio::sync::mpsc;
//use filetime::FileTime;
use anyhow::Result;

#[derive(Debug)]
struct ConfigTime {
    session_id: Uuid,
    uuid: Uuid,
    timestamp: Instant,
    action: Action,
}

impl ConfigTime {
    fn new_action(action: Action) -> Self {
        Self {
            session_id: Uuid::new_v4(),
            uuid: Uuid::new_v4(),
            timestamp: Instant::now(),
            action,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    
    setup_logging()?;
    info!("Starting Concurrent Advanced Cleanup...");

    let session: ConfigTime = ConfigTime::new_action(Action);
    let session_id: Uuid = session.session_id;
    let start_time: Instant = tokio::time::Instant::now();

    let net_mon = NetMon::new();
    net_mon.start_monitoring().await;

    let basic_tasks = vec![
        tokio::spawn(remove_temp_files(&net_mon)),
        tokio::spawn(clear_bash_history(&net_mon)),
        tokio::spawn(update_timestamps(&net_mon)),
    ];

    let advanced_tasks = vec![
        tokio::spawn(secure_delete_file(&net_mon, "./safe/tmp/sensitive_data.txt")),
        tokio::spawn(manipulate_log_file(&net_mon, "./safe/var/log/system.log")),
        tokio::spawn(remove_network_traces(&net_mon)),
    ];

    let all_tasks: Vec<_> = [basic_tasks, advanced_tasks].into_iter().flatten().collect();
    let results: Vec<Result<Result<()>,anyhow::Result<()>> = join_all.iter(all_tasks).await;

    for result in results {
        if let Err(e) = result {
            error!("Task error: {}", e);
        }
    }

    let actions = net_mon.get_actions().await;
    save_to_json(&actions, &format!("cleanup_actions_{}", session_id))?;

    let duration = start_time.elapsed();
    info!("Concurrent Advanced Cleanup completed in {:?}", duration);

    Ok(())
}

async fn remove_temp_files(net_mon: &NetMon) -> Result<()> {
    info!("Removing temporary files...");
    let temp_dir = "/tmp/cleancleanclean";
    match fs::remove_dir_all(temp_dir).await {
        Ok(_) => {
            info!("Removed directory: {}", temp_dir);
            net_mon.get_sender().send(Action::FileDeleted(temp_dir.to_string())).await?;
        },
        Err(e) => {
            error!("Error removing directory {}: {}", temp_dir, e);
            net_mon.get_sender().send(Action::Error(format!("Failed to remove {}: {}", temp_dir, e))).await?;
        }
    }
    Ok(())
}

async fn clear_bash_history(net_mon: &NetMon) -> Result<()> {
    info!("Clearing bash history...");
    let output = Command::new("bash")
        .arg("-c")
        .arg("history -c && history -w")
        .output()
        .await
        .context("Failed to execute bash command")?;

    if output.status.success() {
        info!("Bash history cleared.");
        net_mon.get_sender().send(Action::BashHistoryCleared).await?;
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        error!("Error clearing bash history: {}", error_msg);
        net_mon.get_sender().send(Action::Error(format!("Failed to clear bash history: {}", error_msg))).await?;
    }
    Ok(())
}

async fn secure_delete_file(net_mon: &NetMon, path: &str) -> Result<()> {
    info!("Initiating secure deletion of file: {}", path);

    let file_path = Path::new(path);
    
    // Check if file exists
    if !file_path.exists() {
        let err_msg = format!("File not found: {}", path);
        error!("{}", err_msg);
        net_mon.get_sender().send(Action::Error(err_msg.clone())).await?;
        return Err(anyhow::anyhow!(err_msg));
    }

    let file_size = file_path.metadata()
        .context("Failed to get file metadata")?
        .len();

    let mut file = OpenOptions::new()
        .write(true)
        .open(file_path)
        .await
        .context("Failed to open file for secure deletion")?;

    // Define overwrite patterns
    let patterns: [u8; 3] = [0x00, 0xFF, 0xAA];

    for (i, &pattern) in patterns.iter().enumerate() {
        info!("Starting overwrite pass {} for {}", i + 1, path);
        
        file.seek(SeekFrom::Start(0)).await
            .context("Failed to seek to start of file")?;

        let mut buffer = vec![pattern; 4096]; // 4KB buffer
        let mut written = 0;

        while written < file_size {
            let to_write = std::cmp::min(buffer.len() as u64, file_size - written) as usize;
            file.write_all(&buffer[..to_write]).await
                .context("Failed to write during overwrite")?;
            written += to_write as u64;
        }

        file.flush().await.context("Failed to flush file")?;
        file.sync_all().await.context("Failed to sync file")?;
    }

    // Final pass with random data
    info!("Starting final random overwrite pass for {}", path);
    file.seek(SeekFrom::Start(0)).await
        .context("Failed to seek to start of file for random overwrite")?;

    let mut rng = OsRng;
    let mut buffer = vec![0u8; 4096];
    let mut written = 0;

    while written < file_size {
        rng.fill_bytes(&mut buffer);
        let to_write = std::cmp::min(buffer.len() as u64, file_size - written) as usize;
        file.write_all(&buffer[..to_write]).await
            .context("Failed to write random data")?;
        written += to_write as u64;
    }

    file.flush().await.context("Failed to flush file after random overwrite")?;
    file.sync_all().await.context("Failed to sync file after random overwrite")?;

    // Close the file handle
    drop(file);

    // Delete the file
    tokio::fs::remove_file(file_path).await
        .context("Failed to delete file after secure overwrite")?;

    info!("File securely deleted: {}", path);
    net_mon.get_sender().send(Action::FileDeleted(path.to_string())).await?;

    Ok(())
}

async fn update_timestamps(net_mon: &NetMon) -> Result<()> {
    info!("Updating file timestamps...");
    let files = vec!["/tmp/example1.txt", "/tmp/example2.txt"];
    let now = Local::now();

    for &file_path in &files {
        // Fetch metadata
        let metadata = fs::metadata(&file_path).await?;
        
        // Convert chrono::Local to filetime::FileTime
        let filetime_now = FileTime::from_unix_time(now.timestamp(), now.timestamp_subsec_nanos() as u32);

        // Spawn a blocking task to update the file timestamp
        let file_path_cloned = file_path.to_string();
        let result = tokio::task::spawn_blocking(move || {
            filetime::set_file_mtime(Path::new(&file_path_cloned), filetime_now)
        })
        .await;

        match result {
            Ok(Ok(_)) => {
                net_mon.get_sender()
                    .send(Action::TimestampUpdated(file_path.to_string()))
                    .await?;
            }
            Ok(Err(e)) | Err(e) => {
                net_mon.get_sender()
                    .send(Action::Error(format!("Failed to update timestamp for {}: {}", file_path, e)))
                    .await?;
            }
        }
    }
    
    Ok(())
}
use tokio::fs;
use chrono::Local;
use anyhow::{Result, Context};
use log::{info, warn};

async fn manipulate_log_file(net_mon: &NetMon, log_path: &str) -> Result<()> {
    info!("Manipulating log file: {}", log_path);
    let content = fs::read_to_string(log_path).await.context("Failed to read log file")?;
    let filtered_content: String = content
        .lines()
        .filter(|line| !line.contains("suspicious_activity"))
        .collect::<Vec<&str>>()
        .join("\n");

    fs::write(log_path, filtered_content).await.context("Failed to write filtered content to log file")?;

    let now = Local::now().naive_local();
    let now = Utc::now();
    let filetime = filetime::FileTime::from_unix_time(now.timestamp(), 0);
    tokio::fs::set_file_mtime(log_path, filetime).await.context("Failed to set file modified time")?;

    info!("Log file manipulated: {}", log_path);
    net_mon.get_sender().send(Action::LogManipulated(log_path.to_string())).await?;
    Ok(())
}

async fn remove_network_traces(net_mon: &NetMon) -> Result<()> {
    info!("Removing network traces...");

    // Flush iptables
    let iptables_output = Command::new("iptables")
        .args(&["-F"])
        .output()
        .await
        .context("Failed to flush iptables")?;

    if !iptables_output.status.success() {
        let error_msg = String::from_utf8_lossy(&iptables_output.stderr);
        error!("Error flushing iptables: {}", error_msg);
        net_mon.get_sender().send(Action::Error(format!("Failed to flush iptables: {}", error_msg))).await?;
    } else {
        info!("iptables flushed");
    }

    // Clear DNS cache
    let dns_output = Command::new("systemd-resolve")
        .args(&["--flush-caches"])
        .output()
        .await
        .context("Failed to flush DNS cache")?;

    if !dns_output.status.success() {
        let error_msg = String::from_utf8_lossy(&dns_output.stderr);
        error!("Error flushing DNS cache: {}", error_msg);
        net_mon.get_sender().send(Action::Error(format!("Failed to flush DNS cache: {}", error_msg))).await?;
    } else {
        info!("DNS cache flushed");
    }

    net_mon.get_sender().send(Action::NetworkTraceRemoved("iptables and DNS cache flushed".to_string())).await?;
    Ok(())
}

fn setup_logging() -> Result<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
        .build("./safe/tmp/cleanup.log")
        .context("Failed to create log file")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .context("Failed to configure logger")?;

    log4rs::init_config(config).context("Failed to initialize logger")?;
    Ok(())
}