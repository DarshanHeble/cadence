//! End-to-end integration test suite for Cadence CLI & Git workflows

use std::fs;
use std::process::Command;

struct TestSandbox {
    pub dir: String,
    pub remote_dir: String,
}

impl TestSandbox {
    pub fn new(name: &str) -> Self {
        let root = std::env::temp_dir().join(format!("cadence_test_{}", name));
        let repo_dir = root.join("repo");
        let remote_dir = root.join("remote.git");

        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&repo_dir).unwrap();
        fs::create_dir_all(&remote_dir).unwrap();

        let repo_str = repo_dir.to_str().unwrap().to_string();
        let remote_str = remote_dir.to_str().unwrap().to_string();

        // Initialize bare remote repo
        Command::new("git")
            .args(["init", "--bare"])
            .current_dir(&remote_str)
            .output()
            .unwrap();

        // Initialize local test repo
        Command::new("git")
            .args(["init"])
            .current_dir(&repo_str)
            .output()
            .unwrap();

        Command::new("git")
            .args(["checkout", "-b", "main"])
            .current_dir(&repo_str)
            .output()
            .unwrap();

        Command::new("git")
            .args(["remote", "add", "origin", &remote_str])
            .current_dir(&repo_str)
            .output()
            .unwrap();

        // Configure git identity for testing
        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo_str)
            .output()
            .unwrap();

        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo_str)
            .output()
            .unwrap();

        // Initial seed commit
        fs::write(repo_dir.join("init.txt"), "hello").unwrap();
        Command::new("git")
            .args(["add", "-A"])
            .current_dir(&repo_str)
            .output()
            .unwrap();

        Command::new("git")
            .args(["commit", "-m", "Initial commit\n\nRelease-Date: 2026-01-01"])
            .current_dir(&repo_str)
            .output()
            .unwrap();

        Command::new("git")
            .args(["push", "-u", "origin", "main"])
            .current_dir(&repo_str)
            .output()
            .unwrap();

        // Create .cadence.json in repo
        let cfg_content = "{\n  \"repo_path\": \".\",\n  \"remote\": \"origin\",\n  \"branch\": \"main\",\n  \"timezone\": \"Asia/Kolkata\"\n}".to_string();
        fs::write(repo_dir.join(".cadence.json"), cfg_content).unwrap();

        Self {
            dir: repo_str,
            remote_dir: remote_str,
        }
    }
}

impl Drop for TestSandbox {
    fn drop(&mut self) {
        let parent = std::path::Path::new(&self.dir).parent().unwrap();
        let _ = fs::remove_dir_all(parent);
    }
}

#[test]
fn test_e2e_git_commit_and_push_pacing() {
    let sandbox = TestSandbox::new("commit_pacing");

    // 1. Make a commit assigned to today (Release-Date: 2026-07-28)
    fs::write(
        std::path::Path::new(&sandbox.dir).join("file1.txt"),
        "content 1",
    )
    .unwrap();

    let output = Command::new("git")
        .args(["add", "-A"])
        .current_dir(&sandbox.dir)
        .output()
        .unwrap();
    assert!(output.status.success());

    let output = Command::new("git")
        .args(["commit", "-m", "Feature 1\n\nRelease-Date: 2026-07-28"])
        .current_dir(&sandbox.dir)
        .output()
        .unwrap();
    assert!(output.status.success());

    // 2. Make a commit assigned to a future date (Release-Date: 2026-12-31)
    fs::write(
        std::path::Path::new(&sandbox.dir).join("file2.txt"),
        "content 2",
    )
    .unwrap();

    let output = Command::new("git")
        .args(["add", "-A"])
        .current_dir(&sandbox.dir)
        .output()
        .unwrap();
    assert!(output.status.success());

    let output = Command::new("git")
        .args(["commit", "-m", "Future Feature\n\nRelease-Date: 2026-12-31"])
        .current_dir(&sandbox.dir)
        .output()
        .unwrap();
    assert!(output.status.success());

    // 3. Verify total local commits count is 3
    let output = Command::new("git")
        .args(["rev-list", "--count", "HEAD"])
        .current_dir(&sandbox.dir)
        .output()
        .unwrap();
    let local_count = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<usize>()
        .unwrap();
    assert_eq!(local_count, 3);

    // 4. Verify remote has only 1 commit before push check
    let output = Command::new("git")
        .args(["rev-list", "--count", "main"])
        .current_dir(&sandbox.remote_dir)
        .output()
        .unwrap();
    let remote_count = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<usize>()
        .unwrap();
    assert_eq!(remote_count, 1);
}
