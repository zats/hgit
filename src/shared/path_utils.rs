use std::path::PathBuf;

pub fn relative_path(full_path: &PathBuf, root_folder: &PathBuf) -> PathBuf {
    let result = match full_path.strip_prefix(root_folder.as_path()) {
        Ok(path) => path,
        Err(error) => full_path.as_path()
    };
    return PathBuf::from(result);
}