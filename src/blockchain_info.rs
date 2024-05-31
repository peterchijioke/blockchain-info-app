use dotenv;

use crate::blockchain_address::BlockchainAddress;
use crate::blockchain_status::BlockchainStatus;
use crate::blockchain_transactions::BlockchainTransaction;

use reqwest;
use serde::Result;
use tokio;

const HOST_URL: &str = "https://btcbook.nownodes.io/api";

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
