use std::process::exit;
use crate::{
    callsign,
    registration
};
use crate::cli::cli_args::CliArgs;


pub(crate) fn handle(args: CliArgs) {
    let mut altitude: bool = false;
    let mut progress: bool = true;

    if args.altitude && args.progress {
        eprintln!("[ERROR] Tags '--altitude' and '--progress' cannot be used together");
        eprintln!("[ERROR] Consider using '--all'");
    } else {
        if args.altitude {
            altitude = args.altitude;
            progress = false;
        }
    }

    if let Some(callsign) = args.callsign {
        match callsign::verify(callsign) {
            Ok(clean_callsign) => {
                callsign::handle(clean_callsign, progress, altitude, args.full, args.once);
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
                registration::handle(clean_registration, progress, altitude, args.full, args.once);
            },
            Err(e) => {
                eprintln!("{:?}", e);
                exit(1)
            }
        }
    }
}
