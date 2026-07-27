use std::process::exit;
use crate::api;

pub(crate) fn handle(callsign: String, progress: bool, altitude: bool, full: bool, once: bool) {
    match api::client() {
        Ok(_client) => {
            let client: reqwest::Client = _client;
        },
        Err(e) => {
            eprintln!("Error making client: {}", e);
            exit(1);
        }
    }

    todo!(
        "call api::get_telemetry"
    )
}
