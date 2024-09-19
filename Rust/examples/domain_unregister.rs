//! Shows how to unregister a domain.
//!
//! Depends on the `domain_register` example.

use iroha::data_model::prelude::{FindDomains, QueryBuilderExt, Unregister};

use iroha_examples::{AliceInWonderland, Chess, ExampleDomain};

fn main() -> iroha_examples::Result<()> {
    let as_alice_in_wland = AliceInWonderland::client();
    let chess = Chess::id();
    let unregister_chess = Unregister::domain(chess.clone());
    as_alice_in_wland.submit_blocking(unregister_chess)?;
    as_alice_in_wland
        .query(FindDomains)
        .filter_with(|dom| dom.id.eq(chess.clone()))
        .execute_single()
        .expect_err("domain should not be found");
    println!(
        "Domain: {}\nUnregistered by: {}",
        chess, as_alice_in_wland.account
    );
    // Domain: chess
    // Unregistered by: ed01...03@wonderland
    Ok(())
}
