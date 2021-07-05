extern crate clap;

use clap::Clap;

use commit::*;
use diff::*;
use status::*;

#[path = "../src/commands/status.rs"]
pub mod status;
#[path = "../src/commands/diff.rs"]
pub mod diff;
#[path = "../src/commands/commit.rs"]
pub mod commit;


#[derive(Clap)]
#[clap(name = "hgit", version = "1.0", author = "Kevin K.")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short = 'c', long = "config", default_value = "default.conf")]
    config: String,
    // /// Some input. Because this isn't an Option<T> it's required to be used
    // input: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short = 'v', long = "verbose", parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(name = "status")]
    Status(StatusArgs),

    #[clap(name = "diff")]
    Diff(DiffArgs),

    #[clap(name = "commit")]
    Commit(CommitArgs),
}

/// A subcommand for controlling testing
#[derive(Clap)]
struct Test {
    /// Print debug info
    #[clap(short = 'd')]
    debug: bool,
}

fn main() {
    match Opts::parse().subcmd {
        SubCommand::Status(s) => status(s),
        SubCommand::Diff(s) => diff(s),
        SubCommand::Commit(s) => commit(s)
    };
}
