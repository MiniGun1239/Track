use clap::{Parser, ArgGroup};

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
#[command(
    group(ArgGroup::new("what_output")
        .required(false)
        .args(["progress", "altitude", "full"]))
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


    // What to output
    #[arg(
        short = 'p',
        long = "progress",
        default_value = "true",
    )]
    pub progress: bool,

    #[arg(
        short = 'a',
        long = "altitude",
        alias = "alt",
    )]
    pub altitude: bool,
    
    #[arg(
        short = 'f',
        long = "full",
    )]
    pub full: bool,


    // How to output
    #[arg(
        short = '1',
        long = "once",
    )]
    pub once: bool,
}
