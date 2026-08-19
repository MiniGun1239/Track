pub mod cli;
pub mod callsign;
pub mod registration;
pub mod api;
pub mod other;
pub mod output;

use crate::cli::cli_args::CliArgs;
use crate::cli::handle;
use crate::cli::parse_flags::parse;

#[tokio::main]
pub async fn main() {
    let args: CliArgs = parse();

    handle(args).await;
}
