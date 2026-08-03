use reqwest::Client;

pub(crate) fn get() -> reqwest::Result<Client> {
    Client::builder()
        .user_agent("track/1.0.0")
        .build()
}
