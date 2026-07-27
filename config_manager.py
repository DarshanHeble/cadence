import json
import os
from pathlib import Path
from typing import Dict, Any

CONFIG_FILE = ".cadence.json"
PUSH_LOG_FILE = ".cadence_log.json"


DEFAULT_CONFIG = {
    "repo_path": ".",
    "remote": "origin",
    "branch": "main",
    "timezone": "Asia/Kolkata"
}

def load_config(config_path: str = CONFIG_FILE) -> Dict[str, Any]:
    path = Path(config_path)
    if not path.exists():
        return DEFAULT_CONFIG.copy()
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)

def save_config(config: Dict[str, Any], config_path: str = CONFIG_FILE) -> None:
    with open(config_path, "w", encoding="utf-8") as f:
        json.dump(config, f, indent=2)

def load_push_log(log_path: str = PUSH_LOG_FILE) -> list:
    path = Path(log_path)
    if not path.exists():
        return []
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)

def append_push_log(entry: Dict[str, Any], log_path: str = PUSH_LOG_FILE) -> None:
    logs = load_push_log(log_path)
    logs.append(entry)
    with open(log_path, "w", encoding="utf-8") as f:
        json.dump(logs, f, indent=2)
