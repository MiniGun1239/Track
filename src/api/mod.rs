pub mod client;

pub fn client() -> reqwest::Result<Client> {
    client::get()
}
