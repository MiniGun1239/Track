use crate::cli::cli_args::CliArgs;
use crate::{
    callsign,
    registration
};
use std::process::exit;


pub(crate) async fn handle(args: CliArgs) {
    let mut altitude: bool = false;
    let mut progress: bool = true;

    if args.altitude {
        altitude = args.altitude;
        progress = false;
    }

    if let Some(callsign) = args.callsign {
        match callsign::verify(callsign) {
            Ok(clean_callsign) => {
                callsign::handle(clean_callsign, progress, altitude, args.full, args.once).await;
            },
            Err(e) => {
                eprintln!("{:?}", e);
                exit(1)
            }
        };
    }

    if let Some(registration) = args.registration {
        match registration::verify(registration) {
            Ok(clean_registration) => {
                registration::handle(clean_registration, progress, altitude, args.full, args.once).await;
            },
            Err(e) => {
                eprintln!("{:?}", e);
                exit(1)
            }
        }
    }
}
