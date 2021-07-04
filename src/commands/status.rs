use std::str;

use clap::Clap;
use git2::{Error, Repository, SubmoduleIgnore};

use crate::status::FileStatus::{file_status_to_string, git_status_to_file_status};

#[path = "../models/FileStatus.rs"]
pub mod FileStatus;

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
        let file_status = git_status_to_file_status(&entry.status());

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

        println!("{} {}", file_status_to_string(&file_status), a.unwrap().display());
        continue;

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
}

#[path = "../test_automation/repo_mocks.rs"]
pub mod repo_mocks;

#[path = "../test_automation/test_infra.rs"]
pub mod test_infra;

#[cfg(test)]
mod tests {
    use crate::status::repo_mocks::*;
    use crate::status::test_infra::*;

    #[test]
    fn status_no_changes() {
        run_test_with_repo(|repo, path| {
            add_file(repo, "a.txt", TEXT_FILE_CONTENT, true);
            add_file(repo, "b.txt", TEXT_FILE_CONTENT, true);
            add_file(repo, "c.txt", TEXT_FILE_CONTENT, true);
            commit(repo, "Initial commit");
            insta::assert_snapshot!(hgit("status", path));
        })
    }

    #[test]
    fn status_local_mixed_changes() {
        run_test_with_repo(|repo, path| {
            add_file(repo, "a.txt", TEXT_FILE_CONTENT, true);
            add_file(repo, "b.txt", TEXT_FILE_CONTENT, true);
            add_file(repo, "c.txt", TEXT_FILE_CONTENT, true);
            commit(repo, "Initial commit");
            // modified
            change_file_content(repo, "a.txt", TEXT_FILE_CONTENT2, false);
            // modified staged
            change_file_content(repo, "b.txt", TEXT_FILE_CONTENT2, true);
            // tracked
            add_file(repo, "d.txt", TEXT_FILE_CONTENT, true);
            // untracked
            add_file(repo, "e.txt", TEXT_FILE_CONTENT, false);
            // removed
            remove_file(repo, "b.txt");
            insta::assert_snapshot!(hgit("status", path));
        })
    }
}

