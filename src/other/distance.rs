static EARTH_RADIUS: f64 = 6371.0;

pub(crate) fn distance(
    (lat1, lon1): (f64, f64),
    (lat2, lon2): (f64, f64)
) -> f64 {
    todo!(
        "find distance between 2 points (airport) using haversine formula"
    )
}


pub(crate) fn percent_distance(
    (lat1, lon1): (f64, f64),
    (lat2, lon2): (f64, f64)
) -> f64 {
    todo!(
        "find ratio of 2 points (plane and dest airport) compared to 2 points (airport)"
    )
}


fn haversine_formula(phi_1: f64, lambda_1: f64, phi_2: f64, lambda_2: f64) -> f64 {
    // (Delta Sigma)
    // = 2arcsin(
    //      sqrt(
    //          sin**2((Delta Phi)/2))  +
    //          cos(Phi1)cos(Phi2)sin**2((Delta Lambda)/2)
    //      )
    //  )

    todo!(
        "Implement ts"
    )
}
