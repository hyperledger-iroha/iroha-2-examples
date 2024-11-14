//! Shows how to register an account.
//!
//! Depends on the `domain_register` example.

use iroha::client::Client;
use iroha::data_model::Identifiable;
use iroha::data_model::prelude::{
    Account, AccountId, FindAccounts, Grant, QueryBuilderExt, Register, Revoke,
};
use iroha_executor_data_model::permission::account::CanRegisterAccount;
use iroha_examples::{
    AliceInChess, AliceInWonderland, BobInChess, BobInWonderland, Chess, ExampleDomain,
    MagnusInChess,
};

fn main() -> iroha_examples::Result<()> {
    // An account is created for a signatory in a domain.
    // By default, only the owner of the domain can register accounts in it.
    let as_alice_in_wland = AliceInWonderland::client();
    // The same signatory can have an account in different domains.
    register(&as_alice_in_wland, AliceInChess::id())?;

    // The domain owner can also grant a permission to register accounts in the domain.
    let can_register_accounts_in_chess = CanRegisterAccount {
        domain: Chess::id(),
    };
    // Grant the permission to Bob from Wonderland.
    let bob_in_wland = BobInWonderland::id();
    as_alice_in_wland.submit_blocking(Grant::account_permission(
        can_register_accounts_in_chess.clone(),
        bob_in_wland.clone(),
    ))?;
    // Bob in Wonderland can now register accounts in Chess.
    let as_bob_in_wland = BobInWonderland::client();
    register(&as_bob_in_wland, BobInChess::id())?;
    register(&as_bob_in_wland, MagnusInChess::id())?;
    // Revoke the permission from Bob in Wonderland.
    as_alice_in_wland.submit_blocking(Revoke::account_permission(
        can_register_accounts_in_chess,
        bob_in_wland,
    ))?;
    Ok(())
}

fn register(as_who: &Client, account: AccountId) -> iroha_examples::Result<()> {
    let register_account = Register::account(Account::new(account.clone()));
    as_who.submit_blocking(register_account)?;
    // Observe that the account has really been registered.
    let account = as_who
        .query(FindAccounts)
        .filter_with(|acc| acc.id.eq(account))
        .execute_single()?;
    println!("Account: {}\nRegistered by: {}", account.id(), as_who.account);
    // Account: ed12...41@wonderland
    // Registered by: ed01...12@wonderland
    Ok(())
}
