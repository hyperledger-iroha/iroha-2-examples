//! Shows how to register asset definitions.
//!
//! Depends on `account_register`.

use iroha::client::{asset, Client};
use iroha::data_model::asset::{AssetDefinition, AssetValueType};
use iroha::data_model::ipfs::IpfsPath;
use iroha::data_model::isi::Grant;
use iroha::data_model::permission::{Permission, PermissionId};
use iroha::data_model::prelude::{Metadata, NewAssetDefinition, NumericSpec, Register, Revoke};
use iroha_examples::{
    AliceInWonderland, BobInChess, Chess, ChessBook, ChessPawns, ExampleDomain,
    WonderlandMoney, WonderlandRoses,
};

fn main() -> iroha_examples::Result<()> {
    let as_alice_in_wland = AliceInWonderland::client();
    // `rose#wonderland` are defined in the default genesis block.
    println!(
        "Wonderland Roses:\n{:#?}",
        as_alice_in_wland.request(asset::definition_by_id(WonderlandRoses::id(),))?
    );
    // Assets can be defined as either numeric or store.
    // Numeric assets can be minted (increased) or burned (decreased).
    // `money#wonderland` is a numeric asset with fractional values up to 2 decimal places.
    register(
        &as_alice_in_wland,
        AssetDefinition::new(
            WonderlandMoney::id(),
            AssetValueType::Numeric(NumericSpec::fractional(2)),
        ),
    )?;
    // Since `bob@chess` is not the owner of `chess`, `alice@wonderland`
    // has to grant `bob@chess` permission to define assets in `chess`.
    let can_define_assets_in_chess = Permission::new(
        "CanRegisterAssetDefinitionInDomain".parse::<PermissionId>()?,
        serde_json::json!({ "domain": Chess::id() }),
    );
    // Grant the permission to `bob@chess`.
    as_alice_in_wland.submit_blocking(Grant::permission(
        can_define_assets_in_chess.clone(),
        BobInChess::id(),
    ))?;
    // `pawn#chess` is a numeric asset with integer values that can only be minted once.
    // It means that a certain amount of an asset can be given to an account one time,
    // and from that point it can only be burnt.
    //
    // Mintability is covered in detail in the `asset_numeric` example.
    //
    // `bob@chess` will be the owner of the definition of `pawn#chess`,
    // meaning he will have the default right to mint/burn `pawn#chess`.
    // Since `alice@wonderland` owns `chess`, she will also have that right.
    register(
        &BobInChess::client(),
        AssetDefinition::new(
            ChessPawns::id(),
            AssetValueType::Numeric(NumericSpec::integer()),
        )
        .mintable_once(),
    )?;
    // Revoke the permission.
    as_alice_in_wland.submit_blocking(Revoke::permission(
        can_define_assets_in_chess.clone(),
        BobInChess::id(),
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
    let asset_definition_id = asset_definition.id.clone();
    let define_asset = Register::asset_definition(asset_definition);
    as_who.submit_blocking(define_asset)?;
    let asset_definition = as_who.request(asset::definition_by_id(asset_definition_id))?;
    println!(
        "Asset definition: {}\nRegistered by: {}",
        asset_definition.id, as_who.account
    );
    // Asset definition: pawn#chess
    // Registered by: ed01...12@wonderland
    Ok(())
}
