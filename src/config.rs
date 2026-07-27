use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const CONFIG_FILE: &str = ".cadence.json";
pub const PUSH_LOG_FILE: &str = ".cadence_log.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub repo_path: String,
    pub remote: String,
    pub branch: String,
    pub timezone: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            repo_path: ".".to_string(),
            remote: "origin".to_string(),
            branch: "main".to_string(),
            timezone: "Asia/Kolkata".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogCommit {
    pub short_hash: String,
    pub subject: String,
    pub release_date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub message: String,
    pub count: usize,
    pub commits: Vec<LogCommit>,
}

pub fn load_config() -> Config {
    if let Ok(content) = fs::read_to_string(CONFIG_FILE) {
        if let Ok(cfg) = serde_json::from_str(&content) {
            return cfg;
        }
    }
    Config::default()
}

pub fn save_config(config: &Config) -> std::io::Result<()> {
    let content = serde_json::to_string_pretty(config)?;
    fs::write(CONFIG_FILE, content)
}

pub fn load_push_log() -> Vec<LogEntry> {
    if let Ok(content) = fs::read_to_string(PUSH_LOG_FILE) {
        if let Ok(logs) = serde_json::from_str(&content) {
            return logs;
        }
    }
    Vec::new()
}

pub fn append_push_log(entry: LogEntry) {
    let mut logs = load_push_log();
    logs.push(entry);
    if let Ok(content) = serde_json::to_string_pretty(&logs) {
        let _ = fs::write(PUSH_LOG_FILE, content);
    }
}
