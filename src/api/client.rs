use reqwest::Client;

pub(crate) fn get() -> reqwest::Result<Client> {
    Client::builder()
        .user_agent("track/0.1.0")
        .build()
}
