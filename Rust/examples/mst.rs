use eyre::Result;
use iroha_client::client::{self, ClientQueryError};
use iroha_crypto::KeyPair;
use iroha_data_model::prelude::*;
use iroha_data_model::query::error::{FindError, QueryExecutionFail};
use std::str::FromStr;
use std::time::Duration;

const PIPELINE_DURATION: Duration = Duration::from_millis(1_000);

fn main() -> Result<()> {
    let admin_client = iroha_2_examples::load_client("./config.json")?;

    // 1.

    let key_pair_1 = KeyPair::generate()?;
    let key_pair_2 = KeyPair::generate()?;

    let account_id = AccountId::from_str("mad_hatter@wonderland")?;
    let asset_definition_id = AssetDefinitionId::from_str("camomile#wonderland")?;

    let register_account = RegisterExpr::new(Account::new(
        account_id.clone(),
        [key_pair_1.public_key().clone()],
    ));
    let set_signature_condition = MintExpr::new(
        SignatureCheckCondition::AllAccountSignaturesAnd(
            vec![key_pair_2.public_key().clone()].into(),
        ),
        IdBox::AccountId(account_id.clone()),
    );
    let register_asset_definition =
        RegisterExpr::new(AssetDefinition::quantity(asset_definition_id.clone()));

    let _hash = admin_client.submit_all_blocking({
        let isi: [InstructionExpr; 3] = [
            register_account.into(),
            set_signature_condition.into(),
            register_asset_definition.into(),
        ];
        isi
    })?;

    // 2.

    let mut mad_hatter_client = admin_client.clone();
    mad_hatter_client.key_pair = key_pair_1;
    mad_hatter_client.account_id = account_id.clone();

    let quantity: u32 = 42;
    let asset_id = AssetId::new(asset_definition_id, account_id.clone());
    let mint_asset = MintExpr::new(quantity.to_value(), IdBox::AssetId(asset_id.clone()));

    let transaction_1 = {
        let tx =
            mad_hatter_client.build_transaction([mint_asset.clone()], UnlimitedMetadata::new())?;
        mad_hatter_client.sign_transaction(tx)?
    };
    mad_hatter_client.submit_transaction(&transaction_1)?;

    // ...

    std::thread::sleep(PIPELINE_DURATION);

    // 3.

    let error = mad_hatter_client
        .request(client::asset::by_id(asset_id.clone()))
        .expect_err("Asset should not be found");

    assert!(matches!(
        error,
        ClientQueryError::Validation(ValidationFail::QueryFailed(QueryExecutionFail::Find(
            FindError::Asset(_)
        )))
    ));

    // 4.

    mad_hatter_client.key_pair = key_pair_2;

    // FIXME: not sign tx1, but get original tx from Iroha and sign it
    let transaction_2 = mad_hatter_client.sign_transaction(transaction_1)?;
    mad_hatter_client.submit_transaction(&transaction_2)?;

    // ...

    std::thread::sleep(PIPELINE_DURATION);

    // 5.

    let asset: Asset = mad_hatter_client
        .request(client::asset::by_id(asset_id))
        .expect("Asset should be found")
        .try_into()
        .expect("Value should be Asset");

    assert_eq!(asset.value, quantity.into());

    // ...

    Ok(())
}
