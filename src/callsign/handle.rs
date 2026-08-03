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
    let url_endpoint: String = format!("callsign/{}", callsign);

    other::prepare(
        url_endpoint.as_str(),
        
        progress,
        altitude,
        full,
        
        once
    ).await;
}
