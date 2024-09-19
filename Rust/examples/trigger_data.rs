//! Demonstrates how to register a trigger that responds to a data event.

use iroha::data_model::prelude::*;
use iroha_examples::{AliceInWonderland, ExampleDomain, Wonderland};

fn main() -> iroha_examples::Result<()> {
    // Establish a connection as Alice from Wonderland
    let alice_in_wland = AliceInWonderland::id();
    let as_alice_in_wland = AliceInWonderland::client();

    // The instruction to be executed when the trigger fires: minting an asset
    let wland_roses = "rose#wonderland".parse::<AssetDefinitionId>()?;
    let wland_roses_of_alice = AssetId::new(wland_roses, alice_in_wland.clone());
    let mint_wland_roses_of_alice = Mint::asset_numeric(1_u32, wland_roses_of_alice.clone());

    // Create a trigger reacting to domain metadata edits:
    let trigger_id: TriggerId = "mint_wland_roses_of_alice_on_asset_mint".parse()?;
    let trigger = Trigger::new(
        trigger_id.clone(),
        Action::new(
            Some(mint_wland_roses_of_alice),
            Repeats::Indefinitely,
            alice_in_wland,
            DataEventFilter::Domain(
                DomainEventFilter::new().for_events(DomainEventSet::MetadataInserted),
            ),
        ),
    );

    // Register the trigger on the blockchain
    as_alice_in_wland.submit_blocking(Register::trigger(trigger))?;

    // Query Alice's Wonderland roses before:
    println!(
        "Alice's Wonderland Rose count before trigger execution: {}",
        as_alice_in_wland.query_single(FindAssetQuantityById {
            id: wland_roses_of_alice.clone(),
        })?
    );

    // Trigger is now registered and will mint a rose
    // every time any domain gets a new key-value pair in its metadata.
    as_alice_in_wland.submit_blocking(SetKeyValue::domain(
        // any domain
        Wonderland::id(),
        "any_key".parse()?,
        "any_value",
    ))?;

    // Query Alice's Wonderland roses after:
    println!(
        "Alice's Wonderland Rose count after trigger execution: {}",
        as_alice_in_wland.query_single(FindAssetQuantityById {
            id: wland_roses_of_alice.clone(),
        })?
    );

    // Unregister the trigger.
    as_alice_in_wland.submit_blocking(Unregister::trigger(trigger_id))?;
    Ok(())
}
