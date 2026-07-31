static EARTH_RADIUS: f64 = 6371.0;

pub(crate) fn distance(
    (lat1, lon1): (f64, f64),
    (lat2, lon2): (f64, f64)
) -> f64 {

    haversine_formula(lat1, lon1, lat2, lon2)
}


pub(crate) fn percent_distance(
    (lat_plane, lon_plane): (f64, f64),
    (lat_dep, lon_dep): (f64, f64),
    (lat_dest, lon_dest): (f64, f64)
) -> f64 {
    let dep_dest_distance = haversine_formula(lat_dep, lon_dep, lat_dest, lon_dest);
    let plane_airport_distance = haversine_formula(lat_plane, lon_plane, lat_dest, lon_dest);

    (dep_dest_distance / plane_airport_distance) * 100.0
}


fn haversine_formula(mut phi_1: f64, mut lambda_1: f64, mut phi_2: f64, mut lambda_2: f64) -> f64 {
// Formula:
    // Delta Sigma
    // = 2arcsin(
    //      sqrt(
    //          sin**2((Delta Phi)/2))  +
    //          cos(Phi1)cos(Phi2)sin**2((Delta Lambda)/2)
    //      )
    //  )

    phi_1 = phi_1.to_radians();
    lambda_1 = lambda_1.to_radians();

    phi_2 = phi_2.to_radians();
    lambda_2 = lambda_2.to_radians();

    let d_phi: f64 = phi_2 - phi_1;
    let d_lambda: f64 = lambda_1 - lambda_2;

    // a = sin**2((Delta Phi)/2))  +
    //     cos(Phi1)cos(Phi2)sin**2((Delta Lambda)/2)
    let mut a = (d_phi / 2.0).sin().powi(2);
    a = a +          phi_1.cos() * phi_2.cos() * (d_lambda / 2.0).sin().powi(2);

    // c = 2 * atan2(sqrt(a), sqrt(1 - a))
    let c = a.sqrt().atan2((1.0 - a).sqrt()) * 2.0;

    // d = R * c
    EARTH_RADIUS * c
}
