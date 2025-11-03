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

#[derive(Debug, Clone)]
pub struct Commit
{
    pub hash: String,
    pub author: String,
    pub date: String,
    pub message: String,
}

pub fn get_commits(count: usize) -> Vec<Commit>
{
    let output = Command::new("git")
        .args(&[
            "log",
            &format!("-{}", count),
            "--pretty-format:%H|%an|%ad|%s",
            "--date=short"
        ])
        .output()
        .expect("Failed to execute git command");

    let output_str = String::from_utf8_lossy(&output.stdout);

    output_str
        .lines()
        .map(|line|
        {
            let parts: Vec<&str> = line.split('|').collect();
            Commit
            {
                hash: parts[0].to_string(),
                author: parts[1].to_string(),
                date: parts[2].to_string(),
                message: parts[3].to_string(),
            }
        })
        .collect()

    
}