// https://docs.microsoft.com/en-us/learn/modules/rust-create-command-line-program/

mod cli;
use structopt::StructOpt;

fn main() {
    //  cli::CommandLineArgs::from_args();
    println!("{:#?}", cli::CommandLineArgs::from_args());
}
