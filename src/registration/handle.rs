use crate::{
    api,
    other
};
use std::process::exit;

pub(crate) async fn handle(
    registration: String,

    progress: bool,
    altitude: bool,
    full: bool,

    once: bool
) {
    let url_endpoint: String = format!("registration/{}", registration);

    other::prepare(
        url_endpoint.as_str(),

        progress,
        altitude,
        full,

        once
    ).await;
}
