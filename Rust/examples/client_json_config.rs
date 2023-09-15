use eyre::{Error, WrapErr};
use iroha_config::client::Configuration;
use std::fs::File;

fn main() {
    let config = load_configuration().expect("Configuration should be loading normally");
    json_config_client_test(&config)
        .expect("JSON config client example is expected to work correctly");

    println!("JSON client configuration test passed successfully!");
}

fn load_configuration() -> Result<Configuration, Error> {
    // #region rust_config_load
    let config_loc = "./config.json";
    let file = File::open(config_loc)
        .wrap_err(format!(
            "Unable to load the configuration file at `{}`",
            config_loc
        ))
        .expect("Config file is loading normally.");
    let config: Configuration = serde_json::from_reader(file)
        .wrap_err(format!("Failed to parse `{}`", config_loc))
        .expect("Verified in tests");
    // #endregion rust_config_load

    // Return the configuration normally
    Ok(config)
}

fn json_config_client_test(config: &Configuration) -> Result<(), Error> {
    use iroha_client::client::Client;

    // Initialise a client with a provided config
    let _current_client: Client = Client::new(&config)?;

    Ok(())
}
