use std::error::Error;
use std::fs::File;
use iroha_client::client::Client;
use std::num::NonZeroU64;

fn main() -> Result<(), Box<dyn Error>> {
    ///Setting configuration of our Iroha_client
    let config_path = "src/resources/config.json";
    let file = File::open(&config_path).expect("Failed to read file at: {config_path:?}");
    let config = serde_json::from_reader(file).unwrap();
    let iroha_client: Client = Client::new(&config).unwrap();

    /// Processing the initial block number to a non zero value
    let block_number = NonZeroU64::new(1).expect("The block number most be > 0");

    /// Initiating the block listener object
    let block_iter = iroha_client.listen_for_blocks(block_number)?;
    // Initiating iteration by blocks. The iterator is infinite
    for block in block_iter {
        match &block {
            Ok(block) => println!("Received block: {block:?}"),
            Err(e) => println!("Error happened: {}", e),
        }
    }

    Ok(())
}