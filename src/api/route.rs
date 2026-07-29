use reqwest::Client;
use crate::other::data::RouteResponse;

pub(crate) fn data(
    client: Client,
    url_endpoint: &str,
) -> Result<Option<RouteResponse>, Box<dyn std::error::Error>> {
    let url = format!("https://vrs-standing-data.adsb.lol/routes/{}.json", url_endpoint);

    todo!(
        "implement this shi"
    )
}
