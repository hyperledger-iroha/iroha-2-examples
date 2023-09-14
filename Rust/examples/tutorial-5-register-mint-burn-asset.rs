use eyre::Result;
use iroha_client::client::Client;
use iroha_data_model::prelude::*;
use std::str::FromStr;

fn main() -> Result<()> {
    let client: Client = Client::load_config("./config.json")?;

    // Create an Asset Definition Id of the time
    let time_definition = AssetDefinitionId::from_str("time#looking_glass")?;

    // Register the time asset
    let register_time =
        RegisterBox::new(AssetDefinition::fixed(time_definition.clone()).mintable_once());
    client.submit(register_time.into())?;

    // Define the account the asset will belong to
    let account_id = AccountId::from_str("white_rabbit@looking_glass")?;

    // Create a MintBox using a previous asset and account
    let mint = MintBox::new(
        12.34_f64.try_to_value()?,
        IdBox::AssetId(AssetId::new(time_definition.clone(), account_id.clone())),
    );
    client.submit(mint.into())?;

    // TODO: query asset, show result

    // Burn the asset
    let burn = BurnBox::new(
        42_u32.to_value(),
        IdBox::AssetId(AssetId::new(time_definition, account_id)),
    );
    client.submit(burn.into())?;

    // TODO: query asset, show result of the burn

    Ok(())
}
