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
    pub(crate) countryiso2: Option<String>,
    pub(crate) iata: Option<String>,
    pub(crate) icao: Option<String>,

    pub(crate) lat: Option<f32>,
    pub(crate) lon: Option<f32>,
}
