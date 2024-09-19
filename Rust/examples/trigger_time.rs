//! Demonstrates how to register a trigger that responds to a time event.

use iroha::data_model::prelude::*;
use iroha::data_model::Level;
use iroha_examples::AliceInWonderland;
use std::time::{Duration, SystemTime};

fn main() -> iroha_examples::Result<()> {
    // Establish a connection as Alice from Wonderland
    let alice_in_wland = AliceInWonderland::id();
    let as_alice_in_wland = AliceInWonderland::client();

    // Construct the trigger schedule start time. We add three seconds
    let start_time =
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)? + Duration::from_secs(3);

    // Define a schedule for the trigger, starting now and repeating every second
    let every_second = TimeSchedule::starting_at(start_time).with_period(Duration::from_secs(1));

    // The instruction to be executed when the trigger fires: minting an asset
    let wland_roses = "rose#wonderland".parse::<AssetDefinitionId>()?;
    let wland_roses_of_alice = AssetId::new(wland_roses, alice_in_wland.clone());
    let mint_wland_roses_of_alice = Mint::asset_numeric(1_u32, wland_roses_of_alice.clone());

    // Create a time-based trigger with the defined schedule
    let trigger_id: TriggerId = "mint_wland_roses_of_alice_every_second".parse()?;
    let trigger = Trigger::new(
        trigger_id.clone(),
        Action::new(
            Some(mint_wland_roses_of_alice),
            Repeats::Indefinitely,
            alice_in_wland,
            TimeEventFilter(ExecutionTime::Schedule(every_second)),
        ),
    );

    // Register the trigger on the blockchain
    as_alice_in_wland.submit_blocking(Register::trigger(trigger))?;
    // Trigger is now registered and will mint a rose every second

    // Query Alice's Wonderland roses before:
    println!(
        "Alice's Wonderland Rose count before trigger execution: {}",
        as_alice_in_wland.query_single(FindAssetQuantityById {
            id: wland_roses_of_alice.clone(),
        })?
    );

    // Sleep for a few seconds to allow the trigger to mint multiple roses
    std::thread::sleep(Duration::from_secs(5));
    as_alice_in_wland.submit_blocking(Log::new(Level::DEBUG, "Dummy".to_owned()))?;

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
