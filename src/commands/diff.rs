extern crate colored;

use std::fmt::Debug;
use std::str;

use clap::Clap;
use colored::*;
use git2::{Diff, DiffDelta, DiffFormat, DiffHunk, DiffLine, DiffOptions, Error, Repository, SubmoduleIgnore};

#[derive(Clap)]
pub struct DiffArgs {
    #[clap(short = 'c')]
    pub color: bool,
}

pub fn diff(args: DiffArgs) -> Result<(), Error> {
    let path = ".";
    let repo = Repository::discover(&path)?;
    if repo.is_bare() {
        return Err(Error::from_str("cannot report status on bare repository"));
    }
    let mut opts = DiffOptions::new();
    opts.reverse(false)
        .ignore_whitespace_change(true)
        .patience(true);
    let mut diffs = Vec::new();
    match repo.head() {
        Ok(head) => {
            if let Ok(tree) = head.peel_to_tree() {
                if let Ok(changes) = repo.diff_tree_to_index(Some(&tree), None, None) {
                    diffs.push(changes);
                }
            }
        }
        Err(_) => {
            if let Ok(changes) = repo.diff_tree_to_index(None, None, Some(&mut opts)) {
                diffs.push(changes);
            }
        }
    }
    let diff = repo.diff_index_to_workdir(None, Some(&mut opts)).unwrap();
    if diff.deltas().count() > 0 {
        diffs.push(diff);
    }
    for diff in diffs {
        print_diff(&repo, &diff, &args);
    }
    Ok(())
}

fn print_diff(repo: &Repository, diff: &Diff, args: &DiffArgs) {
    let stats = diff.stats().expect("Can't get diff status");
    let mut format = git2::DiffStatsFormat::NONE;
    let buf = stats.to_buf(format, 80).expect("Failed to get the stat buffer");
    diff.print(DiffFormat::Patch, |d, h, l| print_diff_line(d, h, l, args));
}

fn print_diff_line(
    _delta: DiffDelta,
    _hunk: Option<DiffHunk>,
    line: DiffLine,
    args: &DiffArgs,
) -> bool {
    print!("{}{}",
           match line.origin() {
               '+' | '-' | ' ' => format!("{} ", line.origin()),
               _ => "".to_string()
           },
           match line_color(&line) {
               Some(color) => str::from_utf8(line.content()).unwrap().color(color),
               None => str::from_utf8(line.content()).unwrap().clear()
           });
    true
}

fn line_color(line: &DiffLine) -> Option<Color> {
    match line.origin() {
        '+' => Some(Color::Green),
        '-' => Some(Color::Red),
        '>' => Some(Color::Green),
        '<' => Some(Color::Red),
        'H' => Some(Color::Cyan),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::status::repo_mocks::*;
    use crate::status::test_infra::{hgit, run_test_with_repo};

    #[test]
    fn no_changes_diff() {
        run_test_with_repo(|repo, path| {
            add_file(repo, "a.txt", TEXT_FILE_CONTENT, true);
            add_file(repo, "b.txt", TEXT_FILE_CONTENT, true);
            add_file(repo, "c.txt", TEXT_FILE_CONTENT, true);
            commit(repo, "Initial commit");
            insta::assert_snapshot!(hgit("diff", path));
        })
    }

    #[test]
    fn content_changes_diff() {
        run_test_with_repo(|repo, path| {
            add_file(repo, "a.txt", TEXT_FILE_CONTENT, true);
            add_file(repo, "b.txt", TEXT_FILE_CONTENT, true);
            add_file(repo, "c.txt", TEXT_FILE_CONTENT, true);
            commit(repo, "Initial commit");
            change_file_content(repo, "a.txt", TEXT_FILE_CONTENT2, false);
            change_file_content(repo, "b.txt", TEXT_FILE_CONTENT2, true);
            remove_file(repo, "c.txt");
            insta::assert_snapshot!(hgit("diff", path));
        })
    }
}