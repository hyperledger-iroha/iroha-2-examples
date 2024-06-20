//! Shows how to unregister an account.
//!
//! Depends on the `account_register` example.

use iroha::client::{account, Client};
use iroha::data_model::account::AccountId;
use iroha::data_model::prelude::Unregister;

use iroha_examples::{AliceInChess, AliceInWonderland, BobInChess, MagnusInChess};

fn main() -> iroha_examples::Result<()> {
    // An account's owner can unregister that account.
    let as_bob_in_chess = BobInChess::client();
    unregister(&as_bob_in_chess, BobInChess::account_id())?;
    // A domain owner can unregister any account in that domain.
    let as_alice_in_wonderland = AliceInWonderland::client();
    unregister(&as_alice_in_wonderland, AliceInChess::account_id())?;
    unregister(&as_alice_in_wonderland, MagnusInChess::account_id())?;
    Ok(())
}

fn unregister(as_who: &Client, account: AccountId) -> iroha_examples::Result<()> {
    let unregister_account = Unregister::account(account.clone());
    as_who.submit_blocking(unregister_account)?;
    // Observe that the account has really been unregistered.
    as_who
        .request(account::by_id(account.clone()))
        .expect_err("account should not be found");
    println!(
        "---------------\n\
        `account_id`: {account}\n\
        Unregistered by {}",
        as_who.account
    );
    Ok(())
}
