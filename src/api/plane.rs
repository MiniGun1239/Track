use crate::other::data::{
    AircraftData,
    FlightResponse
};
use reqwest::Client;

pub(crate) async fn data(
    client: Client,
    url_endpoint: &str, // "callsign/{callsign}" or "registration/{reg}"
) -> Result<Option<AircraftData>, Box<dyn std::error::Error>> {
    let url = format!("https://api.adsb.lol/v2/{}", url_endpoint);

    let response = client.get(&url).send().await?;

    let status = response.status();

    let _statustext = status.to_string();

    if status.is_success() {
        let payload: FlightResponse = response.json().await?;

        if let Some(mut aircraft_list) = payload.ac {
            if !aircraft_list.is_empty() {
                return Ok(Some(aircraft_list.remove(0)))
            }
        }
    } else {
        return Err(status.to_string().into())
    }

    Ok(None)
}
