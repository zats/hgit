pub fn hgit<'a, 'b>(command: &'a str, path: &'a Path) -> String {
    let script_path = PathBuf::from(file!()).parent().unwrap().parent().unwrap().parent().unwrap().canonicalize().unwrap().join("hgit");
    let hgit_output = Command::new(script_path)
        .current_dir(path)
        .arg(command)
        .output()
        .expect("failed to execute process");
    let stdout = hgit_output.stdout;
    return String::from_utf8(stdout).unwrap_or("".to_string());
}

pub fn run_test_with_repo<T>(test: T) -> () where T: FnOnce(&Repository, &Path) -> () + panic::UnwindSafe {
    let result = panic::catch_unwind(|| {
        let path = create_temporary_folder();
        let repo = create_git_repo(path.as_path());
        test(&repo, path.as_path());
    });
    assert!(result.is_ok())
}
