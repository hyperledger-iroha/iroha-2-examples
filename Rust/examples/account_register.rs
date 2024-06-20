//! Shows how to register an account.
//!
//! Depends on the `domain_register` example.

use iroha::client::{account, Client};
use iroha::data_model::prelude::{
    Account, AccountId, Grant, Permission, PermissionId, Register, Revoke,
};

use iroha_examples::{
    AliceInChess, AliceInWonderland, BobInChess, BobInWonderland, Chess, ExampleDomain,
    MagnusInChess,
};

fn main() -> iroha_examples::Result<()> {
    // An account is created for a signatory in a domain.
    // By default, only the owner of the domain can register accounts in it.
    let as_alice_in_wonderland = AliceInWonderland::client();
    // The same signatory can have an account in different domains.
    register(&as_alice_in_wonderland, AliceInChess::account_id())?;
    
    // The domain owner can also grant a permission to register accounts in the domain.
    let can_register_accounts_in_chess = Permission::new(
        "CanRegisterAccountInDomain".parse::<PermissionId>()?,
        serde_json::json!({ "domain": Chess::domain_id() }),
    );
    // Grant the permission to Bob from Wonderland.
    let bob_in_wonderland = BobInWonderland::account_id();
    as_alice_in_wonderland.submit_blocking(Grant::permission(
        can_register_accounts_in_chess.clone(),
        bob_in_wonderland.clone(),
    ))?;
    // Bob in Wonderland can now register accounts in Chess.
    let as_bob_in_wonderland = BobInWonderland::client();
    register(&as_bob_in_wonderland, BobInChess::account_id())?;
    register(&as_bob_in_wonderland, MagnusInChess::account_id())?;
    // Revoke the permission from Bob in Wonderland.
    as_alice_in_wonderland.submit_blocking(Revoke::permission(
        can_register_accounts_in_chess,
        bob_in_wonderland,
    ))?;
    Ok(())
}

fn register(as_who: &Client, account: AccountId) -> iroha_examples::Result<()> {
    let register_account = Register::account(Account::new(account.clone()));
    as_who.submit_blocking(register_account)?;
    // Observe that the account has really been registered.
    let account = as_who.request(account::by_id(account))?;
    println!(
        "---------------\n\
        `account_id`: {}\n\
        Registered by {}",
        account.id, as_who.account,
    );
    Ok(())
}
