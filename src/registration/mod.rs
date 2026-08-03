use crate::registration::verify::ValidationError;

pub mod verify;
pub mod handle;

pub async fn handle(callsign: String, once: bool) {
    handle::handle(callsign, once).await;
}

pub fn verify(callsign: String) -> Result<String, ValidationError> {
    verify::verify(callsign)
}
