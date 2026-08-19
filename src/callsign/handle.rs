use crate::other;

pub(crate) async fn handle(callsign: String, once: bool) {
    let url_endpoint: String = format!("callsign/{}", callsign);

    other::prepare(url_endpoint.as_str(), once).await;
}
