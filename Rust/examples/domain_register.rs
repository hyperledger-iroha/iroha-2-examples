//! Shows how to register a domain.

use iroha::client::domain;
use iroha::data_model::prelude::{Domain, Register};

use iroha_examples::{AliceInWonderland, Chess, ExampleDomain};

fn main() -> iroha_examples::Result<()> {
    let as_alice_in_wonderland = AliceInWonderland::client();
    let chess = Chess::domain_id();
    let register_chess = Register::domain(Domain::new(chess.clone()));
    as_alice_in_wonderland.submit_blocking(register_chess)?;
    let chess = as_alice_in_wonderland.request(domain::by_id(chess))?;
    println!(
        "---------------\n\
        `domain_id`: {}\n\
        Registered by {}",
        chess.id, as_alice_in_wonderland.account
    );
    Ok(())
}
