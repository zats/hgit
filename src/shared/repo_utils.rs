use git2::{Error, Repository};

pub fn current_repo() -> Result<Repository, Error> {
    let path = ".";
    return Repository::discover(&path);
}