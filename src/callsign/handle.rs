use std::process::exit;
use crate::api;
use crate::other::data::AircraftData;

pub(crate) fn handle(callsign: String, progress: bool, altitude: bool, full: bool, once: bool) {
    let client: reqwest::Client;

    match api::client() {
        Ok(_client) => {
            client = _client;
        },
        Err(e) => {
            eprintln!("Error making client: {}", e);
            exit(1);
        }
    }

    let url_endpoint: String = format!("callsign/{}", callsign);

    let plane_data: AircraftData;

    match api::plane_data(client, &*url_endpoint) {
        Ok(_plane_data) => {
            plane_data = _plane_data.unwrap();
        }
        Err(e) => {
            eprintln!("Error getting plane data: {}", e);
            exit(1);
        }
    }

    if progress {
        if once {
            todo!(
                "output progress only once, minimally"
            )
        }

        todo!(
            "call output of progress"
        )
    }
    
    if altitude {
        if once {
            todo!(
                "output altitude only once, minimally"
            )
        }
        
        todo!(
            "call output of altitude"
        )
    }

    if full {
        if once {
            todo!(
                "output everything, but only once"
            )
        }
        
        todo!(
            "call output everything"
        )
    }
}
