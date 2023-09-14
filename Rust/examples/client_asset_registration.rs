use eyre::{Error, WrapErr};
use iroha_config::client::Configuration;
use iroha_data_model::TryToValue;
use std::fs::File;

fn main() {
    let config = load_configuration().expect("Configuration should be loading normally");
    asset_registration_test(&config)
        .expect("Asset registration example is expected to work correctly");

    println!("Asset registration example works!");
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

fn asset_registration_test(config: &Configuration) -> Result<(), Error> {
    // #region register_asset_crates
    use std::str::FromStr as _;

    use iroha_client::client::Client;
    use iroha_data_model::prelude::{
        AccountId, AssetDefinition, AssetDefinitionId, AssetId, IdBox, MintBox, RegisterBox,
    };
    // #endregion register_asset_crates

    // Create an Iroha client
    let iroha_client: Client = Client::new(&config)?;

    // #region register_asset_create_asset
    // Create an asset
    let asset_def_id = AssetDefinitionId::from_str("time#looking_glass")
        .expect("Valid, because the string contains no whitespace, has a single '#' character and is not empty after");
    // #endregion register_asset_create_asset

    // #region register_asset_init_submit
    // Initialise the registration time
    let register_time =
        RegisterBox::new(AssetDefinition::fixed(asset_def_id.clone()).mintable_once());

    // Submit a registration time
    iroha_client.submit(register_time)?;
    // #endregion register_asset_init_submit

    // Create an account using the previously defined asset
    let account_id: AccountId = "white_rabbit@looking_glass"
        .parse()
        .expect("Valid, because the string contains no whitespace, has a single '@' character and is not empty after");

    // #region register_asset_mint_submit
    // Create a MintBox using a previous asset and account
    let mint = MintBox::new(
        12.34_f64.try_to_value()?,
        IdBox::AssetId(AssetId::new(asset_def_id, account_id)),
    );

    // Submit a minting transaction
    iroha_client.submit_all([mint])?;
    // #endregion register_asset_mint_submit

    // Finish the test successfully
    Ok(())
}
