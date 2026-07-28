use reqwest::Client;

pub mod client;
pub mod plane;
pub mod route;

pub fn client() -> reqwest::Result<Client> {
    client::get()
}
