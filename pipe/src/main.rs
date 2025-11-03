mod git;

// Importing Dependencies
use cursive::views::{Dialog, TextView, LinearLayout, Panel};
use cursive::{Cursive, CursiveExt};

fn main()
{
    // Create a new Cursive root
    let mut siv = Cursive::default();

    // Get current directory's git info
    let branch = git::get_current_branch();
    let recent_commits = git::get_recent_commits(5);

    let mut layout = LinearLayout::vertical();

    layout.add_child(TextView::new(format!("Branch: {}", branch)));
    layout.add_child(TextView::new("\nRecent Commits:"));
    layout.add_child(TextView::new(recent_commits));

    siv.add_layer(
        Dialog::around(Panel::new(layout))
            .title("Git Repository Explorer")
            .button("Quit", |s| s.quit()),
    );

    siv.run();
}