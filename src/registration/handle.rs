use std::process::exit;
use crate::api;
use crate::other::data::AircraftData;

pub(crate) async fn handle(registration: String, progress: bool, altitude: bool, full: bool, once: bool) {
    let client: reqwest::Client;

    match api::client() {
        Ok(_client) => {
            client = _client;
        },
        Err(e) => {
            println!("Error making client: {}", e);
            exit(1);
        }
    }

    let url_endpoint: String = format!("registration/{}", registration);

    let plane_data: Option<AircraftData>;

    match api::plane_data(client, &*url_endpoint).await {
        Ok(_plane_data) => {
            plane_data = _plane_data;
        },
        Err(e) => {
            eprintln!("Error getting plane_data: {}", e);
        }
    }

    if progress {
        if once {
            todo!(
                "call output::progress_once()"
            )
        }

        todo!(
            "call output::progress()"
        )
    }

    if altitude {
        if once {
            todo!(
                "call output::altitude_once()"
            )
        }

        todo!(
            "call output::altitude()"
        )
    }

    if full {
        if once {
            todo!(
                "call output::full_once()"
            )
        }

        todo!(
            "call output::full()"
        )
    }
}
