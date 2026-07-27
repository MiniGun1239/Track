use reqwest::Client;

pub mod client;
pub mod plane;

pub fn client() -> reqwest::Result<Client> {
    client::get()
}
