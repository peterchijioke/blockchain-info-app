#[macro_use]
extern crate serde_derive;
use dotenv;
mod blockchain_address;
mod blockchain_info;
mod blockchain_status;
mod blockchain_transactions;

use blockchain_address::BlockchainAddress;

use crate::blockchain_status::BlockchainStatus;
pub fn blockchain_info_app(address: &str) {
    let data: BlockchainStatus = blockchain_info::send_status_request();
    println!(
        "\n\nQuerying {} - Chain {} \n\n",
        &data.blockbook.coin, &data.backend.chain
    );

    let address_data: BlockchainAddress = blockchain_info::send_address_request(address);
    println!(r"\n\This is the wallet address {}", &address_data.address);
}
fn main() {
    let dd = dotenv::var("MY_ADDRESS").expect("Error reading environment variable");
    blockchain_info_app(&dd);
}
