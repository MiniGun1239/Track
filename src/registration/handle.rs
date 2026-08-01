use crate::{
    api,
    other
};
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

    other::prepare(
        client,
        url_endpoint.as_str(),

        progress,
        altitude,
        full,

        once
    ).await;
}
