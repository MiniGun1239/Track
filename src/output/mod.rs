use crate::other;

pub(crate) mod progress;
pub(crate) mod altitude;
pub(crate) mod full;

pub(crate) fn progress(
    callsign: String, reg: String, plane_type: String, alt_baro: String, gs: f32, plane_lat: f64, plane_lon: f64,
    dep_name: &str, dep_lat: f64, dep_lon: f64, dep_country: &str, dep_icao: &str, dep_iata: &str, dep_airport: &str,
    dest_name: &str, dest_lat: f64, dest_lon: f64, dest_country: &str, dest_icao: &str, dest_iata: &str, dest_airport: &str,
) {
    progress::output(
        callsign, reg, plane_type, alt_baro, gs, plane_lat, plane_lon,
        dep_name, dep_lat, dep_lon, dep_country, dep_icao, dep_iata, dep_airport,
        dest_name, dest_lat, dest_lon, dest_country, dest_icao, dest_iata, dest_airport,
    )
}


pub(crate) fn altitude(
    callsign: String,
    reg: String,
    plane_type: String,
    alt_baro: String,
    gs: f32,
    plane_lat: f64,
    plane_lon: f64,

    dep_name: &str,
    dep_lat: f64,
    dep_lon: f64,
    dep_country: &str,
    dep_icao: &str,
    dep_iata: &str,
    dep_airport: &str,

    dest_name: &str,
    dest_lat: f64,
    dest_lon: f64,
    dest_country: &str,
    dest_icao: &str,
    dest_iata: &str,
    dest_airport: &str,
) {
    todo!(
        "call altitude::output"
    )
}

pub(crate) fn full(
    callsign: String,
    reg: String,
    plane_type: String,
    alt_baro: String,
    gs: f32,
    plane_lat: f64,
    plane_lon: f64,

    dep_name: &str,
    dep_lat: f64,
    dep_lon: f64,
    dep_country: &str,
    dep_icao: &str,
    dep_iata: &str,
    dep_airport: &str,

    dest_name: &str,
    dest_lat: f64,
    dest_lon: f64,
    dest_country: &str,
    dest_icao: &str,
    dest_iata: &str,
    dest_airport: &str,
) {
    todo!(
        "call full::output"
    )
}
