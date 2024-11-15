//! Shows how to unregister asset definitions.
//!
//! Depends on `asset_definition_register`.

use iroha::client::Client;
use iroha::data_model::isi::Grant;
use iroha::data_model::prelude::{
    AssetDefinitionId, FindAssetsDefinitions, QueryBuilderExt, Revoke, Unregister,
};
use iroha_executor_data_model::permission::asset_definition::CanUnregisterAssetDefinition;

use iroha_examples::{AliceInWonderland, BobInChess, ChessBook, ChessPawns, WonderlandMoney};

fn main() -> iroha_examples::Result<()> {
    let as_alice_in_wland = AliceInWonderland::client();
    // `alice@wonderland` owns the definition of `book#chess`
    unregister(&as_alice_in_wland, ChessBook::id())?;
    let as_bob_in_chess = BobInChess::client();
    // `bob@chess` owns the definition of `pawn#chess`
    unregister(&as_bob_in_chess, ChessPawns::id())?;
    // Since `bob@chess` is not the owner of `money#wonderland`, `alice@wonderland`
    // has to grant `bob@chess` permission to unregister its definition.
    let bob_in_chess = BobInChess::id();
    let wonderland_money = WonderlandMoney::id();
    let can_undefine_wonderland_money = CanUnregisterAssetDefinition {
        asset_definition: wonderland_money.clone(),
    };
    // Grant the permission to `bob@chess`.
    as_alice_in_wland.submit_blocking(Grant::account_permission(
        can_undefine_wonderland_money.clone(),
        bob_in_chess.clone(),
    ))?;
    unregister(&as_bob_in_chess, wonderland_money)?;
    // Revoke the permission.
    as_alice_in_wland.submit_blocking(Revoke::account_permission(
        can_undefine_wonderland_money,
        bob_in_chess,
    ))?;
    Ok(())
}

fn unregister(as_who: &Client, asset_definition: AssetDefinitionId) -> iroha_examples::Result<()> {
    let undefine_asset = Unregister::asset_definition(asset_definition.clone());
    as_who.submit_blocking(undefine_asset)?;
    as_who
        .query(FindAssetsDefinitions)
        .filter_with(|asset_def| asset_def.id.eq(asset_definition.clone()))
        .execute_single()
        .expect_err("asset definition should not be found");
    println!(
        "Asset definition: {}\nUnregistered by: {}",
        asset_definition, as_who.account
    );
    // Asset definition: pawn#chess
    // Unregistered by: ed01...12@wonderland
    Ok(())
}
