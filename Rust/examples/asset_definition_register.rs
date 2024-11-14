//! Shows how to register asset definitions.
//!
//! Depends on `account_register`.

use iroha::client::Client;
use iroha::data_model::asset::{AssetDefinition, AssetType};
use iroha::data_model::ipfs::IpfsPath;
use iroha::data_model::isi::Grant;
use iroha::data_model::prelude::{
    FindAssetsDefinitions, Identifiable, Metadata, NewAssetDefinition, NumericSpec,
    QueryBuilderExt, Register, Revoke,
};
use iroha_executor_data_model::permission::asset_definition::CanRegisterAssetDefinition;
use iroha_examples::{
    AliceInWonderland, BobInChess, Chess, ChessBook, ChessPawns, ExampleDomain, WonderlandMoney,
    WonderlandRoses,
};

fn main() -> iroha_examples::Result<()> {
    let as_alice_in_wland = AliceInWonderland::client();
    // `rose#wonderland` are defined in the default genesis block.
    println!(
        "Wonderland Roses:\n{:#?}",
        as_alice_in_wland
            .query(FindAssetsDefinitions)
            .filter_with(|asset_def| asset_def.id.eq(WonderlandRoses::id()))
            .execute_single()?
    );
    // Assets can be defined as either numeric or store.
    // Numeric assets can be minted (increased) or burned (decreased).
    // `money#wonderland` is a numeric asset with fractional values up to 2 decimal places.
    register(
        &as_alice_in_wland,
        AssetDefinition::new(
            WonderlandMoney::id(),
            AssetType::Numeric(NumericSpec::fractional(2)),
        ),
    )?;
    // Since `bob@chess` is not the owner of `chess`, `alice@wonderland`
    // has to grant `bob@chess` permission to define assets in `chess`.
    let bob_in_chess = BobInChess::id();
    let can_define_assets_in_chess = CanRegisterAssetDefinition {
        domain: Chess::id(),
    };
    // Grant the permission to `bob@chess`.
    as_alice_in_wland.submit_blocking(Grant::account_permission(
        can_define_assets_in_chess.clone(),
        bob_in_chess.clone(),
    ))?;
    // `pawn#chess` is a numeric asset with integer values that can only be minted once,
    // meaning that the asset has a globally fixed supply.
    //
    // `bob@chess` will be the owner of the definition of `pawn#chess`,
    // meaning he will have the default right to mint/burn `pawn#chess`.
    // Since `alice@wonderland` owns `chess`, she will also have that right.
    register(
        &BobInChess::client(),
        AssetDefinition::new(ChessPawns::id(), AssetType::Numeric(NumericSpec::integer()))
            .mintable_once(),
    )?;
    // Revoke the permission.
    as_alice_in_wland.submit_blocking(Revoke::account_permission(
        can_define_assets_in_chess,
        bob_in_chess,
    ))?;
    // `book#chess` is a store asset. Store assets are not minted or burned.
    // Instead, key-value pairs are set or removed for them.
    //
    // Here we also provide an optional IPFS path to the asset logo,
    // and some metadata. Metadata is covered in detail in TODO(`metadata`)
    register(
        &as_alice_in_wland,
        AssetDefinition::store(ChessBook::id())
            .with_logo("QmQqzMTavQgT4f4T5v6PWBp7XNKtoPmC9jvn12WPT3gkSE".parse::<IpfsPath>()?)
            .with_metadata(Metadata::default()),
    )?;
    Ok(())
}

fn register(as_who: &Client, asset_definition: NewAssetDefinition) -> iroha_examples::Result<()> {
    let asset_definition_id = asset_definition.id().clone();
    let define_asset = Register::asset_definition(asset_definition);
    as_who.submit_blocking(define_asset)?;
    let asset_definition = as_who
        .query(FindAssetsDefinitions)
        .filter_with(|asset_def| asset_def.id.eq(asset_definition_id))
        .execute_single()?;
    println!(
        "Asset definition: {}\nRegistered by: {}",
        asset_definition.id(), as_who.account
    );
    // Asset definition: pawn#chess
    // Registered by: ed01...12@wonderland
    Ok(())
}
