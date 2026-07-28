use reqwest::Client;
use crate::other::data::RouteResponse;

pub(crate) fn data(
    client: Client,
    endpoint_url: &str,
) -> Result<Option<RouteResponse>, Box<dyn std::error::Error>> {
    let url = format!("https://vrs-standing-data.adsb.lol/routes/{}.json", endpoint_url);

    todo!(
        "implement this shi"
    )
}
