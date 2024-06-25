//! Shows how to register a domain.

use iroha::client::domain;
use iroha::data_model::prelude::{Domain, Register};

use iroha_examples::{AliceInWonderland, Chess, ExampleDomain};

fn main() -> iroha_examples::Result<()> {
    let as_alice_in_wland = AliceInWonderland::client();
    let chess = Chess::id();
    let register_chess = Register::domain(Domain::new(chess.clone()));
    as_alice_in_wland.submit_blocking(register_chess)?;
    let chess = as_alice_in_wland.request(domain::by_id(chess))?;
    println!(
        "Domain: {}\nRegistered by: {}",
        chess, as_alice_in_wland.account
    );
    // Domain: chess
    // Registered by: ed01...03@wonderland
    Ok(())
}
