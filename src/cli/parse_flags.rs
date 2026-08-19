use crate::cli::cli_args::CliArgs;
use clap::Parser;

pub fn parse() -> CliArgs {
    CliArgs::parse()
}
