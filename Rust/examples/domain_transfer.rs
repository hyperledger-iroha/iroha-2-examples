//! Shows how to transfer ownership of a domain between accounts.
//!
//! Depends on the `domain_register` example.

use iroha::client::{domain, Client};
use iroha::data_model::prelude::{AccountId, DomainId, Transfer};

use iroha_examples::{AliceInWonderland, BobInWonderland, ExampleDomain, Wonderland};

fn main() -> iroha_examples::Result<()> {
    let chess = Wonderland::domain_id();
    let alice_in_wonderland = AliceInWonderland::account_id();
    let bob_in_wonderland = BobInWonderland::account_id();
    // Transfer Chess from Alice in Wonderland to Bob in Wonderland.
    transfer(
        &AliceInWonderland::client(),
        chess.clone(),
        alice_in_wonderland.clone(),
        bob_in_wonderland.clone(),
    )?;
    // Transfer Chess back from Bob in Wonderland to Alice in Wonderland.
    transfer(
        &BobInWonderland::client(),
        chess,
        bob_in_wonderland,
        alice_in_wonderland,
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
        "---------------\n\
        `domain_id`: {}\n\
        Transferred\n\
        \tfrom {from}\n\
        \tto {to}\n\
        by {}",
        domain.id, as_who.account,
    );
    Ok(())
}
