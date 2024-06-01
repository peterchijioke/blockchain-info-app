use dotenv;
use reqwest;
// use serde_json::Result;
use tokio;

use crate::{blockchain_address::BlockchainAddress, blockchain_status::BlockchainStatus};

const HOST_URL: &str = "https://btcbook.nownodes.io/api/";

#[tokio::main]
pub async fn send_request(url: &str) -> String {
    let client = reqwest::Client::new();
    return client
        .get(url)
        .header(
            "api-key",
            dotenv::var("NOW_NODE_API_KEY").expect("Could not find api key"),
        )
        .send()
        .await
        .expect("Failed to get the expected data")
        .text()
        .await
        .expect("Failed to convert the json");
}

pub fn send_status_request() -> BlockchainStatus {
    let response = send_request(&HOST_URL);
    serde_json::from_str(&response).expect("Failed to deserialize from json")
}
pub fn send_address_request(address: &str) -> BlockchainAddress {
    let response = send_request(&[HOST_URL, "/v2/address/", address].join(""));
    serde_json::from_str(&response).expect("Failed to deserialize from json")
}
