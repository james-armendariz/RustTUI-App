use std::process::Command;
use std::env;

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub author: String,
    pub date: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct FileDiff {
    pub filename: String,
    pub diff: String,
}

pub fn get_current_branch() -> String {
    let output = Command::new("git")
        .args(&["branch", "--show-current"])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        },
        _ => "Not a git repository".to_string()
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

pub fn get_commit_stats(commit_hash: &str) -> String {
    let output = Command::new("git")
        .args(&["show", "--stat", "--pretty=format:", commit_hash])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).to_string()
        },
        _ => "No stats available".to_string()
    }
}

pub fn get_changed_files(commit_hash: &str) -> Vec<String> {
    let output = Command::new("git")
        .args(&["diff-tree", "--no-commit-id", "--name-only", "-r", commit_hash])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            
            output_str
                .lines()
                .filter(|line| !line.is_empty())
                // Filter out common build artifacts and system files
                .filter(|line| {
                    !line.contains("target/") &&
                    !line.ends_with(".o") &&
                    !line.ends_with(".rcgu.o") &&
                    !line.contains(".git/")
                })
                .map(|s| s.to_string())
                .collect()
        },
        _ => Vec::new()
    }
}

pub fn get_file_diff(commit_hash: &str, filename: &str) -> String {
    let output = Command::new("git")
        .args(&["show", &format!("{}:{}", commit_hash, filename)])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            let content = String::from_utf8_lossy(&output.stdout);
            // Limit to first 1000 lines to prevent huge diffs
            content.lines()
                .take(1000)
                .collect::<Vec<_>>()
                .join("\n")
        },
        _ => "Unable to retrieve file content".to_string()
    }
}

pub fn get_all_branches() -> Vec<String> {
    let output = Command::new("git")
        .args(&["branch", "-a", "--format=%(refname:short)"])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            output_str
                .lines()
                .filter(|line| !line.is_empty())
                .map(|s| s.to_string())
                .collect()
        },
        _ => Vec::new()
    }
}

pub fn get_branch_commits(branch: &str, count: usize) -> Vec<Commit> {
    let output = Command::new("git")
        .args(&[
            "log",
            branch,
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

pub fn search_commits(query: &str, max_results:usize) -> Vec<Commit> {
    let output = Command::new("git")
        .args(&[
            "log",
            &format!("-{}", max_results),
            "--pretty=format:%H|%an|%ad|%s",
            "--date=short",
            &format!("--grep={}", query),
            "-i"
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

pub fn search_commits_by_author(author: &str, max_results: usize) -> Vec<Commit> {
    let output = Command::new("git")
        .args(&[
            "log",
            &format!("-{}", max_results),
            "--pretty=format:%H|%an|%ad|%s",
            "--date=short",
            &format!("--author={}", author),
            "-i"
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

pub fn search_commits_by_file(filename: &str, max_results: usize) -> Vec<Commit> {
    let output = Command::new("git")
        .args(&[
            "log",
            &format!("-{}", max_results),
            "--pretty=format:%H|%an|%ad|%s",
            "--date=short",
            "--",
            filename
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
