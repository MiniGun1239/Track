use crate::cli::cli_args::CliArgs;

pub(crate) mod cli_args;
pub(crate) mod parse_flags;
pub(crate) mod handle;

pub async fn handle(args: CliArgs) {
    handle::handle(args).await;
}
