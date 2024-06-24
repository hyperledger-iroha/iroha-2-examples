//! Shows how to unregister a domain.
//!
//! Depends on the `domain_register` example.

use iroha::client::domain;
use iroha::data_model::prelude::Unregister;

use iroha_examples::{AliceInWonderland, Chess, ExampleDomain};

fn main() -> iroha_examples::Result<()> {
    let as_alice_in_wonderland = AliceInWonderland::client();
    let chess = Chess::domain_id();
    let unregister_chess = Unregister::domain(chess.clone());
    as_alice_in_wonderland.submit_blocking(unregister_chess)?;
    as_alice_in_wonderland
        .request(domain::by_id(chess.clone()))
        .expect_err("domain should not be found");
    println!(
        "Domain: {}\nUnregistered by: {}",
        chess, as_alice_in_wonderland.account
    );
    // Domain: chess
    // Unregistered by: ed01...03@wonderland
    Ok(())
}
