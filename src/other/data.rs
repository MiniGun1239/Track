use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct AdsbLOLResponse {
    pub(crate) response: Option<Vec<AircraftData>>
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct AircraftData {
    pub(crate) flight: Option<String>,
    pub(crate) t: Option<String>,
    pub(crate) r: Option<String>,
    pub(crate) alt_baro: Option<String>,

    pub(crate) gs: Option<f32>,
    pub(crate) track: Option<f32>,

    pub(crate) lat: Option<f64>,
    pub(crate) lon: Option<f64>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct AirportData {
    pub(crate) name: String,

    pub(crate) iata: String,
    pub(crate) icao: String,

    #[serde(rename = "countryiso2")]
    pub(crate) country: String,
    pub(crate) location: String,

    pub(crate) lat: f64,
    pub(crate) lon: f64,

    pub(crate) alt_feet: f32
}
