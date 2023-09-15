use eyre::Result;
use iroha_2_examples::load_client;
use iroha_client::client::Client;
use iroha_data_model::prelude::*;
use std::str::FromStr;

fn main() -> Result<()> {
    let client: Client = load_client("./config.json")?;

    // Create an Asset Definition Id of the time
    let time_definition = AssetDefinitionId::from_str("time#looking_glass")?;

    // Register the time asset
    let register_time: InstructionBox =
        RegisterBox::new(AssetDefinition::fixed(time_definition.clone()).mintable_once()).into();
    client.submit_blocking(register_time)?;

    // Check if the asset exists
    {
        let definition: AssetDefinition = client.request(
            iroha_data_model::query::asset::model::FindAssetDefinitionById {
                id: time_definition.clone().into(),
            },
        )?;
        println!("Time asset definition from Iroha: {definition:?}");
    }

    // Define the account the asset will belong to
    let account_id = AccountId::from_str("white_rabbit@looking_glass")?;
    let asset_id = AssetId::new(time_definition.clone(), account_id.clone());
    let find_asset_query = iroha_data_model::query::asset::model::FindAssetById {
        id: asset_id.clone().into(),
    };

    // Create a MintBox using a previous asset and account
    let mint: InstructionBox =
        MintBox::new(12.34_f64.try_to_value()?, IdBox::AssetId(asset_id.clone())).into();
    client.submit_blocking(mint)?;

    // TODO: query asset, show result
    {
        let asset: Asset = client.request(find_asset_query.clone())?;
        assert_eq!(asset.value, AssetValue::Fixed(12.34_f64.try_into()?));
        println!("Asset data after mint: {asset:?}");
    }

    // Burn the asset
    let burn: InstructionBox =
        BurnBox::new(2_f64.try_to_value()?, IdBox::AssetId(asset_id.clone())).into();
    client.submit_blocking(burn)?;

    {
        let asset: Asset = client.request(find_asset_query.clone())?;
        assert_eq!(asset.value, AssetValue::Fixed(10.34_f64.try_into()?));
        println!("Asset data after burn: {asset:?}");
    }

    Ok(())
}

// Output of this example:
//
// Time asset definition from Iroha: AssetDefinition { id: time#looking_glass, value_type: Fixed, mintable: Once, logo: None, metadata: Metadata { map: {} }, owned_by: alice@wonderland }
// Asset data after mint: Asset { id: time##white_rabbit@looking_glass, value: Fixed(Fixed(12.34)) }
// Asset data after burn: Asset { id: time##white_rabbit@looking_glass, value: Fixed(Fixed(10.34)) }
