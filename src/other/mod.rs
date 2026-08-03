use std::process::exit;
use crate::api;
use crate::other::distance as dist;
use crate::other::prepare as prep;

pub mod data;
pub mod distance;
pub mod prepare;


// DISTANCE


pub fn distance(
    (dep_lat, lon_dep): (f64, f64),
    (dest_lat, lon_dest): (f64, f64)
) -> f64 {
    dist::distance_between_airports(
        (dep_lat, lon_dep),
        (dest_lat, lon_dest)
    )
}

pub fn distance_ratio(
    (plane_lat, lon_plane): (f64, f64),
    (dep_lat, lon_dep): (f64, f64),
    (dest_lat, lon_dest): (f64, f64)
) -> f64 {
    dist::distance_ratio(
        (plane_lat, lon_plane),
        (dep_lat, lon_dep),
        (dest_lat, lon_dest)
    )
}


// PREPARE


pub async fn prepare(
    plane_url_endpoint: &str,

    progress: bool,
    altitude: bool,
    full: bool,

    once: bool
) {
    while true {
        let client: reqwest::Client;

        match api::client() {
            Ok(_client) => {
                client = _client;
            },
            Err(e) => {
                eprintln!("Error making client: {}", e);
                exit(1);
            }
        }

        prep::prepare_output(
            client, plane_url_endpoint,
            progress, altitude, full,
            once
        ).await;

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}


pub fn get_all_distances(
    (plane_lat, plane_lon): (f64, f64),
    (dep_lat, dep_lon): (f64, f64),
    (dest_lat, dest_lon): (f64, f64)
) -> (f64, f64) {
    (
        distance(
            (dep_lat, dep_lon), 
            (dest_lat, dest_lon)
        ),
        distance_ratio(
            (plane_lat, plane_lon),
            (dep_lat, dep_lon),
            (dest_lat, dest_lon)
        )
    )
}
