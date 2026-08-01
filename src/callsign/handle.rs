use crate::{
    api,
    other
};
use std::process::exit;

pub(crate) async fn handle(
    callsign: String, 
    
    progress: bool, 
    altitude: bool, 
    full: bool, 
    
    once: bool
) {
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

    other::prepare(
        client,
        url_endpoint.as_str(),
        
        progress,
        altitude,
        full,
        
        once
    ).await;
}
