use std::process::Command;
use std::env;

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub author: String,
    pub date: String,
    pub message: String,
}

pub fn get_current_branch() -> String {
    let output = Command::new("git")
        .args(&["branch", "--show-current"])
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                "Not a git repository".to_string()
            }
        },
        Err(_) => "Git not found".to_string()
    }
}

pub fn get_commits(count: usize) -> Vec<Commit> {
    let output = Command::new("git")
        .args(&[
            "log",
            &format!("-{}", count),
            "--pretty=format:%H|%an|%ad|%s",
            "--date=short"
        ])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            
            output_str
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| {
                    let parts: Vec<&str> = line.split('|').collect();
                    Commit {
                        hash: parts.get(0).unwrap_or(&"").to_string(),
                        author: parts.get(1).unwrap_or(&"").to_string(),
                        date: parts.get(2).unwrap_or(&"").to_string(),
                        message: parts.get(3).unwrap_or(&"").to_string(),
                    }
                })
                .collect()
        },
        _ => Vec::new()
    }
}

pub fn get_current_directory() -> String {
    env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "Unknown".to_string())
}