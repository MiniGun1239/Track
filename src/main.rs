use crate::cli::cli_args::CliArgs;
use crate::cli::handle;
use crate::cli::parse_flags::parse;

pub fn main() {
    let args: CliArgs = parse();

    handle(args)
}
