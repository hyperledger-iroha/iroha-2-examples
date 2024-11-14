//! Shows how to check the status of the blockchain.

use iroha_examples::AliceInWonderland;

fn main() -> iroha_examples::Result<()> {
    let as_alice_in_wonderland = AliceInWonderland::client();
    let status = as_alice_in_wonderland.get_status()?;
    println!("{status:#?}");
    // Status {
    //     peers: 3,
    //     blocks: 15,
    //     txs_accepted: 18,
    //     txs_rejected: 0,
    //     uptime: Uptime(534.26s),
    //     view_changes: 0,
    //     queue_size: 0,
    // }
    Ok(())
}
