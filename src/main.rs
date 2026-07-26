use crate::cli::cli_args::CliArgs;
use crate::cli::handle;
use crate::cli::parse_flags::parse;

pub mod cli;
mod callsign;
pub mod registration;
pub mod api;

pub fn main() {
    let args: CliArgs = parse();

    handle(args)
}
