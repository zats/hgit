extern crate clap;

use clap::Clap;

use diff::diff;
use diff::DiffArgs;
use status::status;
use status::StatusArgs;

#[path = "../src/commands/status.rs"]
pub mod status;
#[path = "../src/commands/diff.rs"]
pub mod diff;


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
    /// A help message for the Test subcommand
    #[clap(name = "test", version = "1.3", author = "Someone Else")]
    Test(Test),

    #[clap(name = "status")]
    Status(StatusArgs),

    #[clap(name = "diff")]
    Diff(DiffArgs),
}

/// A subcommand for controlling testing
#[derive(Clap)]
struct Test {
    /// Print debug info
    #[clap(short = 'd')]
    debug: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    // println!("Value for config: {}", opts.config);
    // println!("Using input file: {}", opts.input);

    // // Vary the output based on how many times the user used the "verbose" flag
    // // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    // match opts.verbose {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     3 | _ => println!("Don't be crazy"),
    // }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Test(t) => {
            if t.debug {
                println!("Printing debug info...");
            } else {
                println!("Printing normally...");
            }
        }
        SubCommand::Status(s) => {
            status(s);
        }
        SubCommand::Diff(s) => {
            diff(s);
        }
    }
}
