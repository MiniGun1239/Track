use regex::Regex;
use std::sync::OnceLock;

#[derive(Debug)]
pub enum ValidationError {
    InvalidLength,
    InvalidFormat,
}

static REG_REGEX: OnceLock<Regex> = OnceLock::new();

pub(crate) fn verify(input: String) -> Result<String, ValidationError> {
    let reg = normalize(input);

    if reg.len() < 3 || reg.len() > 8 {
        return Err(ValidationError::InvalidLength)
    }

    let re = REG_REGEX.get_or_init(|| {
        Regex::new(r"^[A-Z0-9]{1,3}[A-Z0-9]{2,5}$").unwrap()
    });
    
    if re.is_match(&reg) {
        Ok(reg)
    } else {
        Err(ValidationError::InvalidFormat)
    }
}

fn normalize(input: String) -> String {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}
