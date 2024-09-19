//! Shows how to unregister an account.
//!
//! Depends on the `account_register` example.

use iroha::client::Client;
use iroha::data_model::account::AccountId;
use iroha::data_model::prelude::{FindAccounts, QueryBuilderExt, Unregister};

use iroha_examples::{AliceInChess, AliceInWonderland, BobInChess, MagnusInChess};

fn main() -> iroha_examples::Result<()> {
    // An account's owner can unregister that account.
    let as_bob_in_chess = BobInChess::client();
    unregister(&as_bob_in_chess, BobInChess::id())?;
    // A domain owner can unregister any account in that domain.
    let as_alice_in_wland = AliceInWonderland::client();
    unregister(&as_alice_in_wland, AliceInChess::id())?;
    unregister(&as_alice_in_wland, MagnusInChess::id())?;
    Ok(())
}

fn unregister(as_who: &Client, account: AccountId) -> iroha_examples::Result<()> {
    let unregister_account = Unregister::account(account.clone());
    as_who.submit_blocking(unregister_account)?;
    // Observe that the account has really been unregistered.
    as_who
        .query(FindAccounts)
        .filter_with(|acc| acc.id.eq(account.clone()))
        .execute_single()
        .expect_err("account should not be found");
    println!("Account: {}\nUnregistered by: {}", account, as_who.account);
    // Account: ed12...41@wonderland
    // Unregistered by: ed01...12@wonderland
    Ok(())
}
