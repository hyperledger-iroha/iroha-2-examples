//! Shows how to register asset definitions.
//!
//! Depends on `account_register`.

use iroha::client::{asset, Client};
use iroha::data_model::asset::{AssetDefinition, AssetValueType};
use iroha::data_model::ipfs::IpfsPath;
use iroha::data_model::prelude::{Metadata, NewAssetDefinition, NumericSpec, Register};
use iroha_examples::{
    AliceInWonderland, BobInChess, ChessBook, ChessPawns, WonderlandMoney, WonderlandRoses,
};

fn main() -> iroha_examples::Result<()> {
    let as_alice_in_wonderland = AliceInWonderland::client();
    // Roses in Wonderland are defined in the default genesis block.
    println!(
        "Wonderland Roses:\n{:#?}",
        as_alice_in_wonderland.request(asset::definition_by_id(
            WonderlandRoses::asset_definition_id(),
        ))?
    );
    // Assets can be defined as either numeric or store.
    // Numeric assets can be minted (increased) or burned (decreased).
    // Wonderland Money is a numeric asset with fractional values up to 2 decimal places.
    register(
        &as_alice_in_wonderland,
        AssetDefinition::new(
            WonderlandMoney::asset_definition_id(),
            AssetValueType::Numeric(NumericSpec::fractional(2)),
        ),
    )?;
    // Chess Pawns is a numeric asset with integer values that can only be minted once.
    // It means that a certain amount of an asset can be given
    // to an account one time, and from that point it can only be burnt.
    // Mintability is covered in detail in the TODO(`asset_numeric`) example.
    //
    // Bob in Chess will be the owner of the definition of Chess Pawns,
    // meaning he will have the default right to mint/burn Chess Pawns.
    // Since Alice in Wonderland owns Chess, she will also have that right.
    register(
        &BobInChess::client(),
        AssetDefinition::new(
            ChessPawns::asset_definition_id(),
            AssetValueType::Numeric(NumericSpec::integer()),
        ),
    )?;
    // Chess Book is a store asset. Store assets are not minted or burned.
    // Instead, key-value pairs are set or removed for them.
    //
    // Here we also provide an optional IPFS path to the asset logo,
    // and some metadata. Metadata is covered in detail in TODO(`metadata`)
    register(
        &as_alice_in_wonderland,
        AssetDefinition::store(ChessBook::asset_definition_id())
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
