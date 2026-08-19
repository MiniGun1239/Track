use regex::Regex;
use std::sync::OnceLock;

#[derive(Debug)]
pub enum ValidationError {
    InvalidLength,
    InvalidFormat,
}


static CALLSIGN_REGEX: OnceLock<Regex> = OnceLock::new();


pub(crate) fn verify(input: String) -> Result<String, ValidationError> {
    let callsign: String = normalize(input);

    if callsign.len() < 4 || callsign.len() > 8 {
        return Err(ValidationError::InvalidLength);
    }

    let re = CALLSIGN_REGEX.get_or_init(|| {
        Regex::new(r"^[A-Z]{3}[0-9]{1,4}[A-Z]{0,2}$").unwrap()
    });

    if re.is_match(&callsign) {
        Ok(callsign)
    } else {
        Err(ValidationError::InvalidFormat)
    }
}


fn normalize(input: String) -> String {
    input
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .map(|c| c.to_ascii_uppercase())
        .collect()
}
