use clap::{
    ArgGroup, 
    Parser
};

#[derive(Parser, Debug)]
#[command(
    name = "track",
    about = "A command line tool for tracking flights",
    version,
)]
#[command(
    group(ArgGroup::new("track_command")
        .required(true)
        .args(["callsign","registration"]))
)]
pub struct CliArgs {
    // Type of input
    #[arg(
        short = 'c',
        long = "callsign",
    )]
    pub callsign: Option<String>,

    #[arg(
        short = 'r',
        long = "registration",
    )]
    pub registration: Option<String>,


    // How to output
    #[arg(
        short = '1',
        long = "once",
    )]
    pub once: bool,
}
