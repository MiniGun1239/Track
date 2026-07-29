use std::error::Error;
use reqwest::Client;
use crate::other::data::{AircraftData, AirportData, RouteResponse};

pub mod client;
pub mod plane;
pub mod route;

pub fn client() -> reqwest::Result<Client> {
    client::get()
}

pub async fn plane_data(client: Client, url_endpoint: &str) -> Result<Option<AircraftData>, Box<dyn Error>> {
    plane::data(client, url_endpoint).await
}

pub fn route_data(client: Client, url_endpoint: &str) -> Result<Option<RouteResponse>, Box<dyn Error>> {
    route::data(client, url_endpoint)
}
