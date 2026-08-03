use crate::callsign::verify::ValidationError;

pub mod verify;
pub mod handle;

pub async fn handle(callsign: String, progress: bool, altitude: bool, full: bool, once: bool) {
    handle::handle(callsign, progress, altitude, full, once).await;
}

pub fn verify(callsign: String) -> Result<String, ValidationError>{
    verify::verify(callsign)
}
