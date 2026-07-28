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

    let endpoint_url: String = format!("callsign/{}", callsign);

    let plane_data: Option<AircraftData>;

    match api::plane_data(client, &*endpoint_url) {
        Ok(_plane_data) => {
            plane_data = _plane_data;
        }
        Err(e) => {
            eprintln!("Error getting plane data: {}", e);
            exit(1);
        }
    }

    todo!(
        "call output, comparing prog, alt, full and once"
    )
}
