use std::process::exit;
use crate::{api, output};
use crate::other::data::{AircraftData, AirportData};
use reqwest::Client;

pub(crate) async fn prepare_output(client: Client, plane_url_endpoint: &str, once: bool, ) {
    let (plane_data, route_data): (
        Option<AircraftData>,
        Option<Vec<AirportData>>
    ) = get_plane_and_route(
        client,
        plane_url_endpoint,
    ).await;

    call_output(plane_data, route_data, once)
}

async fn get_plane_and_route(
    client: Client,
    plane_url_endpoint: &str,
) -> (Option<AircraftData>, Option<Vec<AirportData>>) {
    let plane_data: Option<AircraftData>;

    match api::plane_data(client.clone(), plane_url_endpoint).await {
        Ok(_plane_data) => {
            plane_data = _plane_data;
        },
        Err(e) => {
            eprintln!("Error getting plane_data: {}", e);
            plane_data = None
        }
    }

    let callsign: String;
    let route_data: Option<Vec<AirportData>>;

    if let Some(_callsign) = plane_data.clone().unwrap_or_default().flight {
        callsign = _callsign;

        let part_callsign = &callsign[0..2];

        let route_url_endpoint: String = format!("{:?}/{:?}", part_callsign, callsign);

        match api::route_data(client, &*route_url_endpoint).await {
            Ok(_route_data) => {
                route_data = _route_data;
            },
            Err(e) => {
                eprintln!("Error getting route_data: {}", e);
                route_data = None;
            }
        }
    } else {
        route_data = None;
    }

    (plane_data, route_data)
}


fn call_output(
    plane_data: Option<AircraftData>,
    route_data: Option<Vec<AirportData>>,

    once: bool
) {
    if let Some(plane) = plane_data {
        if let Some(airports) = route_data {
            let departure_airport: &AirportData = &airports[0];
            let destination_airport: &AirportData = &airports[1];

            let send = |f: fn(
                String, String, String, String, f32, f64, f64,
                &str, f64, f64, &str, &str, &str, &str,
                &str, f64, f64, &str, &str, &str, &str,
            )| {
                f(
                    plane.flight.unwrap_or_default(),
                    plane.r.unwrap_or_default(),
                    plane.t.unwrap_or_default(),
                    plane.alt_baro.unwrap_or_default(),
                    plane.gs.unwrap_or_default(),
                    plane.lat.unwrap_or_default(),
                    plane.lon.unwrap_or_default(),

                    &departure_airport.name,
                    departure_airport.lat,
                    departure_airport.lon,
                    &departure_airport.country,
                    &departure_airport.icao,
                    &departure_airport.iata,
                    &departure_airport.location,

                    &destination_airport.name,
                    destination_airport.lat,
                    destination_airport.lon,
                    &destination_airport.country,
                    &destination_airport.icao,
                    &destination_airport.iata,
                    &destination_airport.location,
                )
            };

            send(output::progress);

            if once {
                exit(0)
            }
        }
    }
}
