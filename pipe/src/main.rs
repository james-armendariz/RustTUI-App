// src/main.rs
mod git;

use cursive::views::{Dialog, TextView, SelectView, LinearLayout, Panel, ScrollView};
use cursive::view::{Resizable, Scrollable};
use cursive::{Cursive, CursiveExt};
use git::Commit;

fn main() {
    let mut siv = Cursive::default();

    // Build the main view
    build_main_view(&mut siv);
    
    // Add global keyboard shortcuts
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('b', show_branch_selector);
    
    siv.run();
}

fn build_main_view(siv: &mut Cursive) {
    let current_dir = git::get_current_directory();
    let branch = git::get_current_branch();
    let commits = git::get_commits(20);  // Show more commits

    let mut select = SelectView::new();
    
    if commits.is_empty() {
        select.add_item("No commits found", Commit {
            hash: "".to_string(),
            author: "".to_string(),
            date: "".to_string(),
            message: "".to_string(),
        });
    } else {
        for commit in commits {
            select.add_item(
                format!("{} - {} - {}", 
                    &commit.hash[..7], 
                    commit.date,
                    commit.message
                ),
                commit
            );
        }
    }

    select.set_on_submit(show_commit_details);

    let mut layout = LinearLayout::vertical();
    layout.add_child(TextView::new(format!("📁 Directory: {}", current_dir)));
    layout.add_child(TextView::new(format!("🌿 Branch: {}", branch)));
    layout.add_child(TextView::new("\nPress 'b' to switch branches, 'q' to quit\n"));
    layout.add_child(Panel::new(select.scrollable()).title("Commits"));

    siv.add_layer(
        Dialog::around(layout)
            .title("Git Repository Explorer"),
    );
}

fn show_branch_selector(siv: &mut Cursive) {
    let branches = git::get_all_branches();
    let current_branch = git::get_current_branch();
    
    let mut select = SelectView::new();
    
    if branches.is_empty() {
        select.add_item("No branches found", "".to_string());
    } else {
        for branch in branches {
            let display = if branch == current_branch {
                format!("* {} (current)", branch)
            } else {
                branch.clone()
            };
            select.add_item(display, branch);
        }
    }
    
    select.set_on_submit(|s, branch: &String| {
        if !branch.is_empty() {
            switch_to_branch(s, branch);
        }
    });
    
    siv.add_layer(
        Dialog::around(select.scrollable())
            .title("Select Branch")
            .button("Cancel", |s| {
                s.pop_layer();
            })
            .max_height(20),
    );
}

fn switch_to_branch(siv: &mut Cursive, branch: &str) {
    // Close the branch selector dialog
    siv.pop_layer();
    
    // Clear the main view
    siv.pop_layer();
    
    // Show commits from the selected branch
    let current_dir = git::get_current_directory();
    let commits = git::get_branch_commits(branch, 20);

    let mut select = SelectView::new();
    
    if commits.is_empty() {
        select.add_item("No commits found", Commit {
            hash: "".to_string(),
            author: "".to_string(),
            date: "".to_string(),
            message: "".to_string(),
        });
    } else {
        for commit in commits {
            select.add_item(
                format!("{} - {} - {}", 
                    &commit.hash[..7], 
                    commit.date,
                    commit.message
                ),
                commit
            );
        }
    }

    select.set_on_submit(show_commit_details);

    let mut layout = LinearLayout::vertical();
    layout.add_child(TextView::new(format!("📁 Directory: {}", current_dir)));
    layout.add_child(TextView::new(format!("🌿 Branch: {}", branch)));
    layout.add_child(TextView::new("\nPress 'b' to switch branches, 'q' to quit\n"));
    layout.add_child(Panel::new(select.scrollable()).title("Commits"));

    siv.add_layer(
        Dialog::around(layout)
            .title("Git Repository Explorer"),
    );
}

fn show_commit_details(siv: &mut Cursive, commit: &Commit) {
    if commit.hash.is_empty() {
        return;
    }
    
    let stats = git::get_commit_stats(&commit.hash);
    let files = git::get_changed_files(&commit.hash);
    
    let mut file_select = SelectView::new();
    
    if files.is_empty() {
        file_select.add_item("No files changed", "".to_string());
    } else {
        for file in &files {
            file_select.add_item(file.clone(), file.clone());
        }
    }
    
    let commit_clone = commit.clone();
    
    file_select.set_on_submit(move |s, filename: &String| {
        if !filename.is_empty() {
            show_file_content(s, &commit_clone, filename);
        }
    });
    
    let details = format!(
        "Hash: {}\nAuthor: {}\nDate: {}\n\nMessage:\n{}\n\n{}",
        commit.hash, commit.author, commit.date, commit.message, stats
    );
    
    let scrollable_details = ScrollView::new(TextView::new(details));
    
    let mut layout = LinearLayout::vertical();
    layout.add_child(scrollable_details);
    layout.add_child(TextView::new(""));
    layout.add_child(Panel::new(file_select).title("Changed Files (Click to View)"));
    
    siv.add_layer(
        Dialog::around(layout)
            .title("Commit Details")
            .button("Close", |s| {
                s.pop_layer();
            })
            .min_height(30),
    );
}

fn show_file_content(siv: &mut Cursive, commit: &Commit, filename: &str) {
    let content = git::get_file_diff(&commit.hash, filename);
    
    let header = format!("File: {}\nCommit: {}\n{}\n", 
        filename, 
        &commit.hash[..7],
        "-".repeat(60)
    );
    
    let full_content = format!("{}\n{}", header, content);
    
    let scrollable = ScrollView::new(TextView::new(full_content));
    
    siv.add_layer(
        Dialog::around(scrollable)
            .title("File Content")
            .button("Close", |s| {
                s.pop_layer();
            })
            .max_height(30),
    );
}