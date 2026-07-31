use crate::other::data::{AirportData, RouteResponse};
use reqwest::Client;

pub(crate) async fn data(
    client: Client,
    url_endpoint: &str,
) -> Result<Option<Vec<AirportData>>, Box<dyn std::error::Error>> {
    let url = format!("https://vrs-standing-data.adsb.lol/routes/{}.json", url_endpoint);

    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let payload: RouteResponse = response.json().await?;

        if let Some(route) = payload.airports {
            if !route.is_empty() {
                return Ok(Some(route))
            }
        }
    }

    Ok(None)
}
