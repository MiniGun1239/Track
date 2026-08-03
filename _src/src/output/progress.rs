use std::io;
use std::io::{Stdout, Write};
use std::sync::atomic;
use crate::other;

static _IS_FIRST: atomic::AtomicBool = atomic::AtomicBool::new(true);

pub(crate) fn output(
    callsign: String, reg: String, plane_type: String, alt_baro: String, gs: f32, plane_lat: f64, plane_lon: f64,
    dep_name: &str, dep_lat: f64, dep_lon: f64, dep_country: &str, dep_icao: &str, dep_iata: &str, dep_airport: &str,
    dest_name: &str, dest_lat: f64, dest_lon: f64, dest_country: &str, dest_icao: &str, dest_iata: &str, dest_airport: &str,
) {
    print_output(
        callsign, reg, plane_type, alt_baro, gs, plane_lat, plane_lon,
        dep_name, dep_lat, dep_lon, dep_country, dep_icao, dep_iata, dep_airport,
        dest_name, dest_lat, dest_lon, dest_country, dest_icao, dest_iata, dest_airport,
    );
}


fn print_output(
    callsign: String, reg: String, plane_type: String, alt_baro: String, gs: f32, plane_lat: f64, plane_lon: f64,
    _dep_name: &str, dep_lat: f64, dep_lon: f64, dep_country: &str, dep_icao: &str, _dep_iata: &str, dep_airport: &str,
    _dest_name: &str, dest_lat: f64, dest_lon: f64, dest_country: &str, dest_icao: &str, _dest_iata: &str, dest_airport: &str,
) {
    let mut stdout: Stdout = io::stdout();

    if !_IS_FIRST.load(atomic::Ordering::Relaxed) {
        // \x1b[2A = move cursor up 2 lines
        // \x1b[J  = clear everything from cursor to end
        print!("\x1b[4A\x1b[J");
    } else {
        _IS_FIRST.store(false, atomic::Ordering::Relaxed);
    }

    let (_distance, travel_remaining) = other::get_all_distances(
        (plane_lat, plane_lon),
        (dep_lat, dep_lon),
        (dest_lat, dest_lon),
    );

    let percentage = (1.0 - travel_remaining) * 100.0;

    let bar_width = 50;
    let filled_blocks = ((percentage / 100.0) * bar_width as f64).round() as usize;
    let empty_blocks = bar_width - filled_blocks;

    let filled_track = "█".repeat(filled_blocks);
    let empty_track = "░".repeat(empty_blocks);

    println!(
        "Callsign: {:8} | Altitude: {:8}ft | Speed {:3.0}kts",
        callsign, alt_baro, gs
    );
    println!(
        "Type: {} | Tail: {}",
        plane_type, reg
    );
    println!(
        "Progress: [{}{}] {:0.2}%",
        filled_track, empty_track, percentage
    );
    println!(
        "{} {} ({}){}{} {} ({})",
        dep_airport, dep_icao, dep_country,
        " ".repeat(
            (bar_width -
                (dep_airport.len() + dep_country.len() + dep_icao.len() +
                    dest_airport.len() + dest_country.len() + dest_icao.len())
            ).clamp(0, bar_width)
        ),
        dest_airport, dest_icao, dest_country
    );

    let _ = stdout.flush();
}
