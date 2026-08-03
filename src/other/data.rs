use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct FlightResponse {
    pub ac: Option<Vec<AircraftData>>
}

#[derive(Deserialize, Debug, Clone)]
pub struct RouteResponse {
    #[serde(rename = "_airports")]
    pub airports: Option<Vec<AirportData>>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct AircraftData {
    pub flight: Option<String>,
    //  type
    pub t: Option<String>,
    // registration
    pub r: Option<String>,
    pub alt_baro: Option<String>,

    // ground speed
    pub gs: Option<f32>,
    // heading
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
