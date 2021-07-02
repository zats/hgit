#[derive(Clap)]
pub struct DiffArgs {}

pub fn diff(_args: StatusArgs) -> Result<(), Error> {
    let path = ".";
    let repo = Repository::discover(&path)?;
    if repo.is_bare() {
        return Err(Error::from_str("cannot report status on bare repository"));
    }
    let statuses = repo.statuses(None)?;
    print_short(&repo, &statuses);
    return Ok(());
}
