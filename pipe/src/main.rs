mod git;

// Importing Dependencies
use cursive::views::{Dialog, TextView, SelectView, LinearLayout, Panel};
use cursive::{Cursive, CursiveExt};
use git::Commit;

fn main()
{
    // Create a new Cursive root
    let mut siv = Cursive::default();

    // Get current directory's git info
    let branch = git::get_current_branch();
    let commits = git::get_commits(10);

    let mut select = SelectView::new();
    for commit in commits
    {
        select.add_item(
            format!("{} - {}", &commit.hash[..7], commit.message),
            commit
        );
    }

    select.set_on_submit(show_commit_details);

    let mut layout = LinearLayout::vertical();

    layout.add_child(TextView::new(format!("Branch: {}", branch)));
    layout.add_child(TextView::new(""));
    layout.add_child(Panel::new(select).title("Commits"));

    siv.add_layer(
        Dialog::around(Panel::new(layout))
            .title("Git Repository Explorer")
            .button("Quit", |s| s.quit()),
    );

    siv.run();
}

fn show_commit_details(siv: &mut Cursive, commit: &Commit)
{
    let details = format!(
        "Hash: {}\nAtuhor: {}\nDate: {}\nMessage: \n{}",
        commit.hash, commit.author, commit.date, commit.message
    );

    siv.add_layer(
        Dialog::around(TextView::new(details))
            .title("Commit Details")
            .button("Close", |s|
            {
                s.pop_layer();
            }),
    );
}