//! # Efficient and effective basic cleanup tool 
//! #### For following an Offensive Op
//! - Priority is on being quiet
//! 
//! Features:
//! - Remove TMP Files
//! - Clear bash history (or zsh)
//! - Update Timestamps
//! 
use tokio::fs;
use tokio::process::Command;
use chrono::Local;
use futures::future::join_all;
use std::fs::File;
use syn_crabs::setup_logging;
use anyhow::Result;

let mut DEBUG: &bool = true;

if DEBUG: &bool = true {
    BASE_FILEPATH="./safe/tmp/"
} else &bool {
    BASE_FILEPATH="./"
}

#[tokio::main]
async fn main() -> Result<()>{
    println!("Starting Concurrent Cleanup...");

    let tasks = vec![
        tokio::spawn(step_remove_temp_files()),
        tokio::spawn(step_clear_bash_history()),
        tokio::spawn(step_update_timestamps()),
    ];

    join_all(tasks).await;

    anyhow::Ok(log::info!("Concurrent Cleanup completed."));

}

async fn step_remove_temp_files() {
    let temp_dir = "./safe/tmp/cleancleanclean";
    match fs::remove_dir_all(temp_dir).await {
        Ok(_) => println!("Removed directory: {}", temp_dir),
        Err(e) => println!("Error removing directory {}: {}", temp_dir, e),
    }
}

async fn step_clear_bash_history() {
   
    let output = Command::new("bash")
        .arg("-c")
        .arg("history -c && history -w")
        .output()
        .await;
    match output {
        Ok(_) => log::info!("Bash history cleared."),
        Err(e) => log::info!("Error clearing bash history: {}", e),
    }
}

async fn step_update_timestamps() {
    let files = vec!["./safe/tmp/example1.txt", "./safe/tmp/example2.txt"];
    let now = Local::now().naive_local();

    let timestamp_tasks = files.into_iter().map(|file| {
        tokio::spawn(async move {
            match fs::metadata(file).await {
                Ok(_) => {
                    let result = tokio::fs::File::open(file).await.and_then(|_| {
                        tokio::task::spawn_blocking(move || {
                            filetime::set_file_mtime(file, filetime::FileTime::from_unix_time(now.timestamp(), 0))
                        }).await.unwrap()
                    });
                    match result {
                        Ok(_) => println!("Updated timestamp for: {}", file),
                        Err(e) => println!("Error updating timestamp for {}: {}", file, e),
                    }
                },
                Err(_) => println!("File not found: {}", file),
            }
        })
    });

    join_all(timestamp_tasks).await;
}
