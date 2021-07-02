use git2::ErrorClass::Filesystem;

pub enum FileStatus {
    Unknown,
    Modified,
    Added,
    Removed,
    Clean,
    Missing,
    NotTracked,
    Ignored,
}


pub fn file_status_to_string(file_status: &FileStatus) -> &str {
    match file_status {
        FileStatus::Unknown => "",
        FileStatus::Modified => "M",
        FileStatus::Added => "A",
        FileStatus::Removed => "R",
        FileStatus::Clean => "C",
        FileStatus::Missing => "!",
        FileStatus::NotTracked => "?",
        FileStatus::Ignored => "I",
    }
}

pub fn git_status_to_file_status(status: &git2::Status) -> FileStatus {
    match status {
        s if s.contains(git2::Status::INDEX_NEW) => FileStatus::Added,
        s if s.contains(git2::Status::WT_NEW) => FileStatus::NotTracked,
        s if s.contains(git2::Status::WT_MODIFIED) || s.contains(git2::Status::INDEX_MODIFIED) => FileStatus::Modified,
        s if s.contains(git2::Status::WT_DELETED) => FileStatus::Removed,
        s if s.contains(git2::Status::IGNORED) => FileStatus::Ignored,
        _ => FileStatus::Unknown
    }
}