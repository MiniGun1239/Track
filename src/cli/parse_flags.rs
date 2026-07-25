use clap::Parser;
use crate::cli::cli_args::CliArgs;

pub fn parse() -> CliArgs {
    CliArgs::parse()
}
