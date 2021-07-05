use clap::Clap;
use git2::Error;

use repo_utils::*;

#[path = "../shared/repo_utils.rs"]
pub mod repo_utils;

#[derive(Clap)]
pub struct CommitArgs {
    #[clap(short = 'm')]
    pub message: String,
}


pub fn commit(args: CommitArgs) -> Result<(), Error> {
    let repo = match current_repo() {
        Ok(repo) => repo,
        Err(err) => { return Err(err); }
    };

    Ok(())
}
