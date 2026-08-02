pub(crate) mod progress;
pub(crate) mod altitude;
pub(crate) mod full;

pub(crate) fn progress(
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
        "call progress::regular"
    )
}

pub(crate) fn altitude(plane_data: Option<AircraftData>) {
    todo!(
        "call altitude::regular"
    )
}

pub(crate) fn full(plane_data: Option<AircraftData>) {
    todo!(
        "call full::regular"
    )
}
