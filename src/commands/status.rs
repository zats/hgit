use clap::Clap;
use git2::{Error, Repository};

use repo_utils::*;

use crate::status::FileStatus::*;

#[path = "../shared/repo_utils.rs"]
pub mod repo_utils;

#[path = "../models/FileStatus.rs"]
pub mod FileStatus;

#[derive(Clap)]
pub struct StatusArgs {
    #[clap(short = 'a')]
    pub all: bool,
}

pub fn status(_args: StatusArgs) -> Result<(), Error> {
    let repo = match current_repo() {
        Ok(repo) => repo,
        Err(err) => { return Err(err); }
    };
    if repo.is_bare() {
        return Err(Error::from_str("cannot report status on bare repository"));
    }
    let statuses = repo.statuses(None)?;
    print_short(&repo, &statuses);
    return Ok(());
}

fn print_short(_repo: &Repository, statuses: &git2::Statuses) {
    for entry in statuses
        .iter()
        .filter(|e| e.status() != git2::Status::CURRENT)
    {
        let file_status = git_status_to_file_status(&entry.status());

        let mut a = None;
        if let Some(diff) = entry.head_to_index() {
            a = diff.old_file().path();
        }
        if let Some(diff) = entry.index_to_workdir() {
            a = a.or_else(|| diff.old_file().path());
        }
        println!("{} {}", file_status_to_string(&file_status), a.unwrap().display());
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

