use std::{panic, str};

use clap::Clap;
use git2::{Error, Repository, SubmoduleIgnore};

#[derive(Clap)]
pub struct StatusArgs {
    #[clap(short = 'a')]
    pub all: bool,
}

pub fn status(_args: StatusArgs) -> Result<(), Error> {
    let path = ".";
    let repo = Repository::discover(&path)?;
    if repo.is_bare() {
        return Err(Error::from_str("cannot report status on bare repository"));
    }
    let statuses = repo.statuses(None)?;
    print_short(&repo, &statuses);
    return Ok(());
}

fn print_short(repo: &Repository, statuses: &git2::Statuses) {
    for entry in statuses
        .iter()
        .filter(|e| e.status() != git2::Status::CURRENT)
    {
        let mut istatus = match entry.status() {
            s if s.contains(git2::Status::INDEX_NEW) => 'A',
            s if s.contains(git2::Status::INDEX_MODIFIED) => 'M',
            s if s.contains(git2::Status::INDEX_DELETED) => 'D',
            s if s.contains(git2::Status::INDEX_RENAMED) => 'R',
            s if s.contains(git2::Status::INDEX_TYPECHANGE) => 'T',
            _ => ' ',
        };
        let mut wstatus = match entry.status() {
            s if s.contains(git2::Status::WT_NEW) => {
                if istatus == ' ' {
                    istatus = '?';
                }
                '?'
            }
            s if s.contains(git2::Status::WT_MODIFIED) => 'M',
            s if s.contains(git2::Status::WT_DELETED) => 'D',
            s if s.contains(git2::Status::WT_RENAMED) => 'R',
            s if s.contains(git2::Status::WT_TYPECHANGE) => 'T',
            _ => ' ',
        };

        if entry.status().contains(git2::Status::IGNORED) {
            istatus = '!';
            wstatus = '!';
        }
        if (istatus == '?' && wstatus == '?') || (istatus == '!' && wstatus == '!') {
            continue;
        }
        let mut extra = "";

        // A commit in a tree is how submodules are stored, so let's go take a
        // look at its status.
        //
        // TODO: check for GIT_FILEMODE_COMMIT
        let status = entry.index_to_workdir().and_then(|diff| {
            let ignore = SubmoduleIgnore::Unspecified;
            diff.new_file()
                .path_bytes()
                .and_then(|s| str::from_utf8(s).ok())
                .and_then(|name| repo.submodule_status(name, ignore).ok())
        });
        if let Some(status) = status {
            if status.contains(git2::SubmoduleStatus::WD_MODIFIED) {
                extra = " (new commits)";
            } else if status.contains(git2::SubmoduleStatus::WD_INDEX_MODIFIED)
                || status.contains(git2::SubmoduleStatus::WD_WD_MODIFIED)
            {
                extra = " (modified content)";
            } else if status.contains(git2::SubmoduleStatus::WD_UNTRACKED) {
                extra = " (untracked content)";
            }
        }

        let (mut a, mut b, mut c) = (None, None, None);
        if let Some(diff) = entry.head_to_index() {
            a = diff.old_file().path();
            b = diff.new_file().path();
        }
        if let Some(diff) = entry.index_to_workdir() {
            a = a.or_else(|| diff.old_file().path());
            b = b.or_else(|| diff.old_file().path());
            c = diff.new_file().path();
        }

        match (istatus, wstatus) {
            ('R', 'R') => println!(
                "RR {} {} {}{}",
                a.unwrap().display(),
                b.unwrap().display(),
                c.unwrap().display(),
                extra
            ),
            ('R', w) => println!(
                "R{} {} {}{}",
                w,
                a.unwrap().display(),
                b.unwrap().display(),
                extra
            ),
            (i, 'R') => println!(
                "{}R {} {}{}",
                i,
                a.unwrap().display(),
                c.unwrap().display(),
                extra
            ),
            (i, w) => println!("{}{} {}{}", i, w, a.unwrap().display(), extra),
        }
    }

    for entry in statuses
        .iter()
        .filter(|e| e.status() == git2::Status::WT_NEW)
    {
        println!(
            "?? {}",
            entry.index_to_workdir().unwrap().old_file().path().unwrap().display()
        );
    }
}

#[path = "../test_automation/repo_mocks.rs"]
pub mod test_automation;

#[cfg(test)]
mod tests {
    use std::{env, panic, str};
    use std::borrow::Borrow;
    use std::path::{Path, PathBuf};
    // // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
    use std::process::Command;

    use git2::Repository;

    use crate::status::test_automation::*;
    use crate::status::test_automation::create_git_repo;
    use crate::status::test_automation::create_temporary_folder;

    #[test]
    fn no_changes_status() {
        run_test_with_repo(|repo, path| {
            add_file("a.txt", "A piece of content\n", repo);
            add_file("b.txt", "A piece of content\n", repo);
            add_file("c.txt", "A piece of content\n", repo);
            add_all(repo);
            commit("Initial commit", repo);
            assert_eq!(hgit("status", path), "");
        })
    }

    #[test]
    fn test_status_with_changes() {
        run_test_with_repo(|repo, path| {
            add_file("a.txt", "A piece of content\n", repo);
            add_file("b.txt", "A piece of content\n", repo);
            add_file("c.txt", "A piece of content\n", repo);
            add_all(repo);
            commit("Initial commit", repo);
            change_file_content("a.txt", "New piece of content\n", repo);
            assert_eq!(hgsit("status", path), " M a.txt\n");
        })
    }
}

