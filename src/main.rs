use crate::cli::cli_args::CliArgs;
use crate::cli::parse_flags::parse;

pub mod cli;
mod callsign;
pub mod registration;

pub fn main() {
    let args: CliArgs = parse();

    todo!(
        "call the cli::handle::handle"
    )
}
