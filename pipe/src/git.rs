use std::process::Command;

pub fn get_current_branch() -> String
{
    let output = Command::new("git")
        .args(&["branch", "--show-current"])
        .output()
        .expect("Failed to execute git command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn get_recent_commits(count: usize) -> String
{
    let output = Command::new("git")
        .args(&["log", &format!("-{}", count), "--oneline"])
        .output()
        .expect("Failed to execute git command");

    String::from_utf8_lossy(&output.stdout).to_string()
}