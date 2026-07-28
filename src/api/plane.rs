use reqwest::Client;
use crate::other::data::AircraftData;

pub(crate) fn data(
    client: Client,
    endpoint_url: &str, // "callsign/{callsign}" or "registration/{reg}"
) -> Result<Option<AircraftData>, Box<dyn std::error::Error>> {
    let url = format!("https://api.adsb.lol/v2/{}", endpoint_url);

    todo!(
        "reimplement api::get_telemetry"
    )
}
