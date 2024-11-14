//! Shows how to work directly with transactions.

use iroha::data_model::prelude::{Metadata, Mint};
use iroha_examples::{AliceInWonderland, RosesOfAliceInWonderland};

fn main() -> iroha_examples::Result<()> {
    // Prepare the instructions you want to execute
    let roses_of_alice_in_wland = RosesOfAliceInWonderland::id();
    let mint_roses_of_alice_in_wland = Mint::asset_numeric(1_u32, roses_of_alice_in_wland);

    // Combine the instructions
    let instructions = [
        mint_roses_of_alice_in_wland.clone(),
        mint_roses_of_alice_in_wland,
    ];

    // Build a transaction with the prepared instructions and empty metadata
    // on behalf of the account configured with the client
    let as_alice_in_wland = AliceInWonderland::client();
    let signed_tx = as_alice_in_wland.build_transaction(instructions, Metadata::default());

    let _tx_hash_1 = as_alice_in_wland.submit_transaction(&signed_tx)?;

    // Transaction 1 may or may not have been committed or rejected.
    // If you want synchronous behavior, use the _blocking variant:

    let _tx_hash_2 = as_alice_in_wland.submit_transaction_blocking(&signed_tx)?;

    // If this line has been reached, Transaction 2 has been committed.
    Ok(())
}
