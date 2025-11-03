// src/main.rs
mod git;

use cursive::views::{Dialog, TextView, SelectView, LinearLayout, Panel};
use cursive::{Cursive, CursiveExt};
use git::Commit;

fn main() {
    let mut siv = Cursive::default();

    let current_dir = git::get_current_directory();
    let branch = git::get_current_branch();
    let commits = git::get_commits(10);

    // Create the SelectView for commits
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
                format!("{} - {}", &commit.hash[..7], commit.message),
                commit
            );
        }
    }

    // Set what happens when you select a commit
    select.set_on_submit(show_commit_details);

    let mut layout = LinearLayout::vertical();
    layout.add_child(TextView::new(format!("Directory: {}", current_dir)));
    layout.add_child(TextView::new(format!("Branch: {}", branch)));
    layout.add_child(TextView::new(""));
    layout.add_child(Panel::new(select).title("Commits"));

    siv.add_layer(
        Dialog::around(layout)
            .title("Git Repository Explorer")
            .button("Quit", |s| s.quit()),
    );
    
    siv.run();
}

fn show_commit_details(siv: &mut Cursive, commit: &Commit) {
    if commit.hash.is_empty() {
        return; // Don't show details for empty commits
    }
    
    let details = format!(
        "Hash: {}\nAuthor: {}\nDate: {}\n\nMessage:\n{}",
        commit.hash, commit.author, commit.date, commit.message
    );

    siv.add_layer(
        Dialog::around(TextView::new(details))
            .title("Commit Details")
            .button("Close", |s| {
                s.pop_layer();
            }),
    );
}