use reqwest::Client;
use crate::other::data::AircraftData;

pub(crate) fn get_telemetry(
    client: Client,
    endpoint_url: &str, // "callsign/{callsign}" or "registration/{reg}"
) -> Result<Option<AircraftData>, Box<dyn std::error::Error>> {
    let url = format!("https://api.adsb.lol/v2/{}", endpoint_url);

    todo!(
        "remplement api::get_telemetry"
    )
}
