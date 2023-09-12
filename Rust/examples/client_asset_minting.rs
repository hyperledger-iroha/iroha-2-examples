use std::fs::File;
use eyre::{Error, WrapErr};
use iroha_config::client::Configuration;

fn main() {
    let config = load_configuration()
        .expect("Configuration should be loading normally");
    asset_minting_test(&config)
        .expect("Asset minting example is expected to work correctly");

    println!("Asset minting example works!");
}

fn load_configuration() -> Result<Configuration, Error> {
    // #region rust_config_load
    let config_loc = "./config.json";
    let file = File::open(config_loc)
        .wrap_err(format!("Unable to load the configuration file at `{}`", config_loc))
        .expect("Config file is loading normally.");
    let config: Configuration = serde_json::from_reader(file)
        .wrap_err(format!("Failed to parse `{}`", config_loc))
        .expect("Verified in tests");
    // #endregion rust_config_load

    // Return the configuration normally
    Ok(config)
}

fn asset_minting_test(config: &Configuration) -> Result<(), Error> {
    // #region mint_asset_crates
    use std::str::FromStr;

    use iroha_client::client::Client;
    use iroha_data_model::{
        prelude::{AccountId, AssetDefinitionId, AssetId, MintBox, ToValue},
        IdBox,
    };
    // #endregion mint_asset_crates

    // Create an Iroha client
    let iroha_client: Client = Client::new(&config)?;

    // Define the instances of an Asset and Account
    // #region mint_asset_define_asset_account
    let roses = AssetDefinitionId::from_str("rose#wonderland")
        .expect("Valid, because the string contains no whitespace, has a single '#' character and is not empty after");
    let alice: AccountId = "alice@wonderland".parse()
        .expect("Valid, because the string contains no whitespace, has a single '@' character and is not empty after");
    // #endregion mint_asset_define_asset_account

    // Mint the Asset instance
    // #region mint_asset_mint
    let mint_roses = MintBox::new(
        42_u32.to_value(),
        IdBox::AssetId(AssetId::new(roses, alice)),
    );
    // #endregion mint_asset_mint

    // #region mint_asset_submit_tx
    iroha_client
        .submit(mint_roses)
        .wrap_err("Failed to submit transaction")?;
    // #endregion mint_asset_submit_tx

    // #region mint_asset_mint_alt
    // Mint the Asset instance (alternate syntax).
    // The syntax is `asset_name#asset_domain#account_name@account_domain`,
    // or `roses.to_string() + "#" + alice.to_string()`.
    // The `##` is a short-hand for the rose `which belongs to the same domain as the account
    // to which it belongs to.
    let mint_roses_alt = MintBox::new(
        10_u32.to_value(),
        IdBox::AssetId("rose##alice@wonderland".parse()?),
    );
    // #endregion mint_asset_mint_alt

    // #region mint_asset_submit_tx_alt
    iroha_client
        .submit(mint_roses_alt)
        .wrap_err("Failed to submit transaction")?;
    // #endregion mint_asset_submit_tx_alt

    // Finish the test successfully
    Ok(())
}
