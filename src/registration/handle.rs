use crate::other;

pub(crate) async fn handle(registration: String, once: bool) {
    let url_endpoint: String = format!("registration/{}", registration);

    other::prepare(url_endpoint.as_str(), once).await;
}
