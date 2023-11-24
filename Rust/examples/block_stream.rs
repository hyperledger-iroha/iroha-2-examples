use iroha_client::client::Client;
use std::num::NonZeroU64;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config(get_config_path()?);
    let iroha_client: Client = Client::new(&config)?;

    initiate_block_listener(&iroha_client, 1)?;

    Ok(())
}
/// Auxiliary method for a block listener
/// You shall implement it first
fn non_zero_handler(number: u64) -> NonZeroU64 {
    NonZeroU64::new(number).map_or_else(
        || {
            println!("The number must be > 0, using default value 1");
            NonZeroU64::new(1).unwrap()
        },
        |non_zero| non_zero,
    )
}
/// A block listener configuration
/// iroha_client - Your iroha client implementation
/// initial_block_number - The number of a block listener should start from.
/// To get total quantity of blocks, you may use method iroha_client.get_status().
fn initiate_block_listener(
    iroha_client: &Client,
    initial_block_number: u64,
) -> Result<(), Box<dyn Error>> {
    // Processing the non zero value from the u64
    let block_number = non_zero_handler(initial_block_number);
    // Initiating the block listener object
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
use iroha_config::client::Configuration;
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let exe_path = env::current_exe();
    let binding = exe_path?;
    let ancestors = binding.ancestors();
    for ancestor in ancestors {
        if ancestor.file_name() == Some("target".as_ref()) {
            let source_path = Path::new(ancestor);
            let mut config_path = PathBuf::from(source_path.parent().unwrap());
            config_path.push("src");
            config_path.push("resources");
            config_path.push("config.json");
            return Ok(config_path);
        }
    }
    Err("The source directory was not found in the ancestor path.".into())
}
pub fn get_config(path_buf: PathBuf) -> Configuration {
    let file =
        File::open(&path_buf).unwrap_or_else(|_| panic!("Failed to read file at: {path_buf:?}"));
    serde_json::from_reader(file).unwrap_or_else(|_| panic!("Failed to read config at ?????"))
}