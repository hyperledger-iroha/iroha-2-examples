use eyre::Result;
use iroha_2_examples::load_client;
use iroha_client::client::Client;
use iroha_data_model::prelude::*;
use std::str::FromStr;

fn get_key_from_user() -> Result<PublicKey> {
    let (public_key, _) = iroha_crypto::KeyPair::generate()?.into();
    Ok(public_key)
}

fn register_account(client: &Client, account_name: &str, domain_name: &str) -> Result<AccountId> {
    let account_id: AccountId = format!("{}@{}", account_name, domain_name).parse()?;
    let public_key = get_key_from_user()?;
    let register = RegisterBox::new(Account::new(account_id.clone(), [public_key]));
    client.submit_blocking(register)?;
    Ok(account_id)
}

fn mint_to_account(client: &Client, asset_definition_id: &AssetDefinitionId, account_id: &AccountId, value: f64) -> Result<()> {
    let asset_id = AssetId::new(asset_definition_id.clone(), account_id.clone());
    let mint: InstructionBox =
        MintBox::new(value.try_to_value()?, IdBox::AssetId(asset_id.clone())).into();
    client.submit_blocking(mint)?;
    Ok(())
}

fn print_asset(client: &Client, asset_id: &AssetId) -> Result<()> {
    let asset: Asset = client.request(iroha_data_model::query::asset::model::FindAssetById {
        id: asset_id.clone().into(),
    })?;
    println!("Asset data: {:?}", asset);
    Ok(())
}

fn main() -> Result<()> {
    let client: Client = load_client("configs/demo/config.json")?;

    // Create and register a Domain
    let domain_name = "unreasonable";
    let domain_id: DomainId = domain_name.parse()?;
    let register: InstructionBox = RegisterBox::new(Domain::new(domain_id)).into();
    client.submit_blocking(register)?;

    // Register accounts
    let user_account_id = register_account(&client, "user", domain_name)?;
    let user2_account_id = register_account(&client, "user2", domain_name)?;

    // Register the time asset
    let time_definition_id = AssetDefinitionId::from_str("time#unreasonable")?;
    let register_time: InstructionBox = RegisterBox::new(AssetDefinition::fixed(time_definition_id.clone())).into();
    client.submit_blocking(register_time)?;

    // Mint to accounts and print asset
    mint_to_account(&client, &time_definition_id, &user_account_id, 12.34)?;
    print_asset(&client, &AssetId::new(time_definition_id.clone(), user_account_id.clone()))?;

    mint_to_account(&client, &time_definition_id, &user2_account_id, 12.34)?;
    print_asset(&client, &AssetId::new(time_definition_id.clone(), user2_account_id.clone()))?;

    // Transfer asset between accounts
    let asset_id_from = AssetId::new(time_definition_id.clone(), user_account_id);
    let asset_id_to = AssetId::new(time_definition_id, user2_account_id);
    let transfer = TransferBox::new(
        IdBox::AssetId(asset_id_from),
        Value::Numeric(1_f64.try_into()?),
        IdBox::AssetId(asset_id_to)
    );
    println!("Preparing to transfer 1 time from user to user2...");
    client.submit_blocking(transfer)?;

    Ok(())
}
