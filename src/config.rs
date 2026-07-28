//! Configuration and Push Audit Log Manager
//!
//! Handles persistent storage for repository settings (`.cadence.json`)
//! and push history logs (`.cadence_log.json`).

use serde::{Deserialize, Serialize};
use std::fs;

/// Project dotfile paths
pub const CONFIG_FILE: &str = ".cadence.json";
pub const PUSH_LOG_FILE: &str = ".cadence_log.json";

/// Cadence repository configuration settings
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Config {
    /// Target repository path (defaults to ".")
    pub repo_path: String,
    /// Target Git remote (e.g. "origin")
    pub remote: String,
    /// Target release branch (e.g. "main")
    pub branch: String,
    /// Configured timezone for "today" calculation (e.g. "Asia/Kolkata")
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

/// Metadata summary of a committed item in the push log
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct LogCommit {
    pub short_hash: String,
    pub subject: String,
    pub release_date: String,
}

/// Entry representing a single push execution event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub timestamp: String,
    pub message: String,
    pub count: usize,
    pub commits: Vec<LogCommit>,
}

/// Loads repository configuration from `.cadence.json` or returns default settings
pub fn load_config() -> Config {
    fs::read_to_string(CONFIG_FILE)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

/// Saves repository configuration into `.cadence.json`
pub fn save_config(config: &Config) -> std::io::Result<()> {
    let content = serde_json::to_string_pretty(config)?;
    fs::write(CONFIG_FILE, content)
}

/// Loads push history from `.cadence_log.json`
pub fn load_push_log() -> Vec<LogEntry> {
    fs::read_to_string(PUSH_LOG_FILE)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

/// Appends a new push execution record to `.cadence_log.json`
pub fn append_push_log(entry: LogEntry) {
    let mut logs = load_push_log();
    logs.push(entry);
    if let Ok(content) = serde_json::to_string_pretty(&logs) {
        let _ = fs::write(PUSH_LOG_FILE, content);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = Config::default();
        assert_eq!(cfg.repo_path, ".");
        assert_eq!(cfg.remote, "origin");
        assert_eq!(cfg.branch, "main");
        assert_eq!(cfg.timezone, "Asia/Kolkata");
    }

    #[test]
    fn test_config_json_roundtrip() {
        let cfg = Config {
            repo_path: "/path/to/repo".to_string(),
            remote: "upstream".to_string(),
            branch: "master".to_string(),
            timezone: "UTC".to_string(),
        };

        let json = serde_json::to_string(&cfg).expect("Serialization failed");
        let decoded: Config = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(cfg, decoded);
    }

    #[test]
    fn test_log_entry_roundtrip() {
        let entry = LogEntry {
            timestamp: "2026-07-28T12:00:00Z".to_string(),
            message: "Successfully pushed 1 commit".to_string(),
            count: 1,
            commits: vec![LogCommit {
                short_hash: "abc1234".to_string(),
                subject: "Test commit".to_string(),
                release_date: "2026-07-28".to_string(),
            }],
        };

        let json = serde_json::to_string(&entry).expect("Serialization failed");
        let decoded: LogEntry = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(entry, decoded);
    }
}
