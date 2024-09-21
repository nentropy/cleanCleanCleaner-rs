use tokio::sync::mpsc;
use tokio::sync::oneshot;
use std::collections::VecDeque;
use std::sync::Arc;
use parking_lot::RwLock;
use chrono::{Local, Utc, DateTime};
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs;
use crate::main::Action;

// Ensure chrono DateTime types can be serialized/deserialized with serde
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    FileDeleted(String),
    LogManipulated(String),
    NetworkTraceRemoved(String),
    BashHistoryCleared,
    TimestampUpdated(String),
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ActionRecord {
    // Fix timestamp type here
    timestamp: DateTime<Utc>,
    action: Action,
}

pub struct NetMon {
    sender: mpsc::Sender<Action>,
    receiver: Arc<RwLock<mpsc::Receiver<Action>>>,
    log: Arc<RwLock<VecDeque<ActionRecord>>>,
}

impl NetMon {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(100); // Adjust buffer size as needed
        NetMon {
            sender,
            receiver: Arc::new(RwLock::new(receiver)),
            log: Arc::new(RwLock::new(VecDeque::new())),
        }
    }

    pub fn get_sender(&self) -> mpsc::Sender<Action> {
        self.sender.clone()
    }

    pub async fn start_monitoring(&self) {
        let receiver = self.receiver.clone();
        let log = self.log.clone();

        tokio::spawn(async move {
            loop {
                let mut receiver = receiver.write();
                if let Some(action) = receiver.recv().await {
                    // Use Utc for proper timestamp
                    let timestamp = Utc::now();
                    let record = ActionRecord { timestamp, action: action.clone() };
                    log.write().push_back(record);
                    println!("[{}] Action recorded: {:?}", timestamp, action);
                }
            }
        });
    }

    pub async fn get_actions(&self) -> Vec<ActionRecord> {
        self.log.read().iter().cloned().collect()
    }

    pub async fn clear_actions(&self) {
        self.log.write().clear();
    }

    pub async fn wait_for_action(&self, timeout: std::time::Duration) -> Option<Action> {
        let receiver = self.receiver.clone();
        let mut receiver = receiver.write();      
        match tokio::time::timeout(timeout, receiver.recv()).await {
            Ok(Some(action: Action)) => Some(action), 
            Ok(None) => None,
            Err(_) => None,
        }
    }
        match tokio::time::timeout(timeout, recv).await {
            Ok(Ok(action)) => Some(action),
            _ => {
                receiver_task.abort();
                None
            }
        }
    }

    pub async fn write_to_json(&self) -> Result<(), Box<dyn std::error::Error>> {
        let actions = self.get_actions().await;
        let json = serde_json::to_string_pretty(&actions)?;

        let dir_path = Path::new("./safe/tmp/tool_reports");
        fs::create_dir_all(dir_path)?;

        let file_path = dir_path.join("net_mon_report.json");
        fs::write(file_path, json)?;

        Ok(())
    }

