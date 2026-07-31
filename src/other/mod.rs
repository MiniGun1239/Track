use crate::other::distance::{
    distance_between_airports,
    distance_ratio
};

pub mod data;
pub mod distance;

pub fn distance(
    (lat_dep, lon_dep): (f64, f64),
    (lat_dest, lon_dest): (f64, f64)
) -> f64 {
    distance_between_airports(
        (lat_dep, lon_dep),
        (lat_dest, lon_dest)
    )
}

pub fn ratio(
    (lat_plane, lon_plane): (f64, f64),
    (lat_dep, lon_dep): (f64, f64),
    (lat_dest, lon_dest): (f64, f64)
) -> f64 {
    distance_ratio(
        (lat_plane, lon_plane),
        (lat_dep, lon_dep),
        (lat_dest, lon_dest)
    )
}
