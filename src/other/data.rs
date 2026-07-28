use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AdsbLOLResponse {
    pub response: Option<Vec<AircraftData>>
}

#[derive(Deserialize, Debug, Clone)]
pub struct RouteResponse {
    #[serde(rename = "_airports")]
    pub airports: Option<Vec<AirportData>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AircraftData {
    pub flight: Option<String>,
    pub t: Option<String>,
    pub r: Option<String>,
    pub alt_baro: Option<String>,

    pub gs: Option<f32>,
    pub track: Option<f32>,

    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AirportData {
    pub name: String,

    pub iata: String,
    pub icao: String,

    #[serde(rename = "countryiso2")]
    pub country: String,
    pub location: String,

    pub lat: f64,
    pub lon: f64,

    pub alt_feet: f32
}
