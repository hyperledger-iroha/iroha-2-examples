//! Shows how to work with metadata.

use iroha::data_model::prelude::{FindAccountMetadata, Name, RemoveKeyValue, SetKeyValue};
use iroha_examples::AliceInWonderland;

fn main() -> iroha_examples::Result<()> {
    let alice_in_wland = AliceInWonderland::id();
    let as_alice_in_wland = AliceInWonderland::client();
    let key = "key".parse::<Name>()?;
    let value = as_alice_in_wland
        .query_single(FindAccountMetadata::new(
            alice_in_wland.clone(),
            key.clone(),
        ))
        .ok();
    println!("metadata[{key}] of alice@wonderland: {value:?}");

    as_alice_in_wland.submit_blocking(SetKeyValue::account(
        alice_in_wland.clone(),
        key.clone(),
        "new_value",
    ))?;

    let new_value = as_alice_in_wland.query_single(FindAccountMetadata::new(
        alice_in_wland.clone(),
        key.clone(),
    ))?;
    println!("metadata[{key}] of alice@wonderland: {new_value:?}");

    as_alice_in_wland
        .submit_blocking(RemoveKeyValue::account(alice_in_wland.clone(), key.clone()))?;

    as_alice_in_wland
        .query_single(FindAccountMetadata::new(
            alice_in_wland.clone(),
            key.clone(),
        ))
        .expect_err("key-value should be removed");
    Ok(())
}
