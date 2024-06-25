//! Shows how to transfer ownership of a domain between accounts.
//!
//! Depends on the `domain_register` example.

use iroha::client::{domain, Client};
use iroha::data_model::prelude::{AccountId, DomainId, Transfer};

use iroha_examples::{AliceInWonderland, BobInWonderland, ExampleDomain, Wonderland};

fn main() -> iroha_examples::Result<()> {
    let chess = Wonderland::id();
    let alice_in_wland = AliceInWonderland::id();
    let bob_in_wland = BobInWonderland::id();
    // Transfer Chess from Alice in Wonderland to Bob in Wonderland.
    transfer(
        &AliceInWonderland::client(),
        chess.clone(),
        alice_in_wland.clone(),
        bob_in_wland.clone(),
    )?;
    // Transfer Chess back from Bob in Wonderland to Alice in Wonderland.
    transfer(
        &BobInWonderland::client(),
        chess,
        bob_in_wland,
        alice_in_wland,
    )?;
    Ok(())
}

fn transfer(
    as_who: &Client,
    domain: DomainId,
    from: AccountId,
    to: AccountId,
) -> iroha_examples::Result<()> {
    {
        // Observe that the old owner owns the domain.
        let domain = as_who.request(domain::by_id(domain.clone()))?;
        assert_eq!(domain.owned_by, from);
    }
    let transfer_domain = Transfer::domain(from.clone(), domain.clone(), to.clone());
    as_who.submit_blocking(transfer_domain)?;
    // Observe that now the new owner owns the domain.
    let domain = as_who.request(domain::by_id(domain.clone()))?;
    assert_eq!(domain.owned_by, to);
    println!(
        "Domain: {}\nTransferred\n\tfrom: {}\n\tto: {}\nby: {}",
        domain.id, from, to, as_who.account
    );
    // Domain: chess
    // Transferred
    //     from: ed01...03@wonderland
    //     to: ed01...16@wonderland
    // by: ed01...03@wonderland
    Ok(())
}
