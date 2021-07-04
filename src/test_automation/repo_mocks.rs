use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use git2::{Commit, ObjectType, Repository, Signature};
use tempdir::TempDir;
use uuid::Uuid;

use path_utils::relative_path;

#[path = "../shared/path_utils.rs"]
pub mod path_utils;

pub fn create_temporary_folder() -> PathBuf {
    let uuid = Uuid::new_v4();
    let tmp_dir = TempDir::new(uuid.to_string().as_str()).unwrap();
    let path = tmp_dir.into_path();
    println!("Created temporary repository: {}", path.display());
    return path;
}

pub fn create_git_repo(path: &Path) -> Repository {
    let repo = match Repository::init(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
    return repo;
}

pub const TEXT_FILE_CONTENT: &str = "A piece of content\n";
pub const TEXT_FILE_CONTENT2: &str = "New piece of content\n";

pub fn add_file(repo: &Repository, name: &str, content: &str, add_to_index: bool) {
    let file_path = repo.path().parent().unwrap().join(name);
    let mut file = File::create(&file_path).expect("Unable to create file");
    file.write_all(content.as_bytes()).expect("Unable to write");
    println!("Created temporary file: {}", file_path.display());
    if add_to_index {
        let mut index = repo.index().expect("Can't fetch index");
        let relative_path = relative_path(&file_path, &PathBuf::from(repo.path().parent().unwrap()));
        println!("Adding file {} ({})", relative_path.display(), file_path.display());
        index.add_path(relative_path.as_path());
        index.write();
    }
}

pub fn change_file_content(repo: &Repository, name: &str, new_content: &str, add_to_index: bool) {
    let file_path = repo.path().parent().unwrap().join(name);
    println!("Changing file file: {}", &file_path.display());
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_path)
        .expect("File doesn't exist");
    file.write_all(new_content.as_bytes()).expect("Unable to write");

    if add_to_index {
        let mut index = repo.index().expect("Can't fetch index");
        let relative_path = relative_path(&file_path, &PathBuf::from(repo.path().parent().unwrap()));
        println!("Adding file {} ({})", relative_path.display(), file_path.display());
        index.add_path(relative_path.as_path());
        index.write();
    }
}

pub fn remove_file(repo: &Repository, name: &str) {
    let file_path = repo.path().parent().unwrap().join(name);
    fs::remove_file(file_path.as_path());
}

pub fn add_all(repo: &Repository) {
    let mut index = repo.index().unwrap();
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None);
    index.write();
}

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

const SIGNATURE_NAME: &str = "Test McTest Face";
const SIGNATURE_EMAIL: &str = "test@mactestface.com";

pub fn commit(repo: &Repository, message: &str) {
    let mut index = repo.index().unwrap();
    let oid = index.write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();

    let signature = Signature::now(SIGNATURE_NAME, SIGNATURE_EMAIL).unwrap();
    let parent = find_last_commit(&repo).ok();
    if let Some(parent) = parent {
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[&parent],
        );
    } else {
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[],
        );
    }
}