//! Shows how to register asset definitions.
//!
//! Depends on `account_register`.

use iroha::client::{asset, Client};
use iroha::data_model::ipfs::IpfsPath;
use iroha::data_model::prelude::{Metadata, NewAssetDefinition, Register};
use iroha_examples::{
    AliceInWonderland, BobInChess, ChessBook, ChessPawns, WonderlandMoney, WonderlandRoses,
};

fn main() -> iroha_examples::Result<()> {
    let as_alice_in_wonderland = AliceInWonderland::client();
    // Roses in Wonderland are defined in the default genesis block.
    let wonderland_roses = as_alice_in_wonderland.request(asset::definition_by_id(
        WonderlandRoses::asset_definition_id(),
    ))?;
    println!(
        "---------------\n\
        {wonderland_roses:#?}"
    );
    // Assets can be defined as either numeric or store.
    // Numeric assets can be minted (increased) or burned (decreased).
    // Wonderland Money is a numeric asset with fractional values up to 2 decimal places.
    register(&as_alice_in_wonderland, WonderlandMoney::asset_definition())?;
    // Chess Pawns is a numeric asset with integer values that can only be minted once.
    // It means that a certain amount of an asset can be given
    // to an account one time, and from that point it can only be burnt.
    // Mintability is covered in detail in the TODO(`asset_numeric`) example.
    //
    // Bob in Chess will be the owner of the definition of Chess Pawns,
    // meaning he will have the default right to mint/burn Chess Pawns.
    // Since Alice in Wonderland owns Chess, she will also have that right.
    register(&BobInChess::client(), ChessPawns::asset_definition())?;
    // Chess Book is a store asset. Store assets are not minted or burned.
    // Instead, key-value pairs are set or removed for them.
    //
    // Here we also provide the optional IPFS path to the asset logo,
    // and some metadata. Metadata is covered in detail in TODO(`metadata`)
    register(
        &as_alice_in_wonderland,
        ChessBook::asset_definition()
            .with_logo("QmQqzMTavQgT4f4T5v6PWBp7XNKtoPmC9jvn12WPT3gkSE".parse::<IpfsPath>()?)
            .with_metadata(Metadata::default()),
    )?;
    Ok(())
}

fn register(as_who: &Client, asset_definition: NewAssetDefinition) -> iroha_examples::Result<()> {
    let asset_definition_id = asset_definition.id.clone();
    let define_asset = Register::asset_definition(asset_definition);
    as_who.submit_blocking(define_asset)?;
    let chess_pawns = as_who.request(asset::definition_by_id(asset_definition_id))?;
    println!(
        "---------------\n\
        `asset_definition_id`: {}\n\
        Registered by {}",
        chess_pawns.id, as_who.account
    );
    Ok(())
}
