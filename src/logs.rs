use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;

#[derive(Serialize)]
pub struct PortLog {
    pub timestamp: String,
    pub action: String,
    pub container_id: u32,
    pub status: String,
    pub balance: f32,
}

pub fn log_event(action: &str, id: u32, status: &str, money: f32) {
    let log = PortLog {
        timestamp: Utc::now().to_rfc3339(),
        action: action.to_string(),
        container_id: id,
        status: status.to_string(),
        balance: money,
    };

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs_puerto.jsonl") 
    {
        if let Ok(json) = serde_json::to_string(&log) {
            let _ = writeln!(file, "{}", json);
        }
    }
}