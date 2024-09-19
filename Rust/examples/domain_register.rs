//! Shows how to register a domain.

use iroha::data_model::prelude::{Domain, FindDomains, QueryBuilderExt, Register};

use iroha_examples::{AliceInWonderland, Chess, ExampleDomain};

fn main() -> iroha_examples::Result<()> {
    let as_alice_in_wland = AliceInWonderland::client();
    let chess = Chess::id();
    let register_chess = Register::domain(Domain::new(chess.clone()));
    as_alice_in_wland.submit_blocking(register_chess)?;
    let chess = as_alice_in_wland
        .query(FindDomains)
        .filter_with(|domain| domain.id.eq(chess))
        .execute_single()?;
    println!(
        "Domain: {}\nRegistered by: {}",
        chess, as_alice_in_wland.account
    );
    // Domain: chess
    // Registered by: ed01...03@wonderland
    Ok(())
}
