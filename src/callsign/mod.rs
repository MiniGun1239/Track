use crate::callsign::verify::ValidationError;

pub mod verify;
pub mod handle;

pub fn handle(callsign: String, progress: bool, altitude: bool, full: bool, once: bool) {
    handle::handle(callsign, progress, altitude, full, once);
}

pub fn verify(callsign: String) -> Result<String, ValidationError>{
    verify::verify(callsign)
}
