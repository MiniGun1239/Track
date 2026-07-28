use std::error::Error;
use reqwest::Client;
use crate::other::data::{AircraftData, AirportData, RouteResponse};

pub mod client;
pub mod plane;
pub mod route;

pub fn client() -> reqwest::Result<Client> {
    client::get()
}

pub fn plane_data(client: Client, endpoint_url: &str) -> Result<Option<AircraftData>, Box<dyn Error>> {
    plane::data(client, endpoint_url)
}

pub fn route_data(client: Client, endpoint_url: &str) -> Result<Option<RouteResponse>, Box<dyn Error>> {
    route::data(client, endpoint_url)
}
