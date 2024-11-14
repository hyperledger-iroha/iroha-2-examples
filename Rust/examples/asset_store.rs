//! Shows how to register store assets and set/remove key-value pairs.
//!
//! Depends on the `asset_definition_register` example.
//!
//! See `asset_numeric` for basic explanations on assets.

use iroha::client::Client;
use iroha::data_model::asset::{Asset, AssetValue};
use iroha::data_model::prelude::{AssetId, FindAssets, Json, Metadata, Name, QueryBuilderExt, Register, RemoveKeyValue, SetKeyValue};
use iroha_examples::{AliceInWonderland, BobInChess, BookOfBobInChess};

fn main() -> iroha_examples::Result<()> {
    let as_alice_in_wland = AliceInWonderland::client();
    // `book##bob@chess` is a `clothes#chess` owned by `bob@chess`.
    let book_of_bob_in_chess = BookOfBobInChess::id();
    // TODO: replace with NFT
    as_alice_in_wland.submit_blocking(
        // `book#chess` was defined by `alice@wonderland`, therefore she has to
        // register a `book##bob@chess` for `bob@chess` to be able to use it.
        Register::asset(Asset::new(
            book_of_bob_in_chess.clone(),
            Metadata::default(),
        )),
    )?;
    let as_bob_in_chess = BobInChess::client();
    let title = "title".parse::<Name>()?;
    let created_at = "created_at".parse::<Name>()?;
    as_bob_in_chess.submit_all_blocking([
        // As the owner of a `book#chess`, `bob@chess` can set key-value pairs in it.
        // Only the owner of a store asset and someone with a permission can
        // set and remove key-value pairs – but not the asset definition owner.
        //
        // Keys are Iroha names, while values are arbitrary JSON strings:
        SetKeyValue::asset(
            book_of_bob_in_chess.clone(),
            title.clone(),
            "Bob's book on chess",
        ),
        SetKeyValue::asset(
            book_of_bob_in_chess.clone(),
            created_at.clone(),
            serde_json::json!({"date": "2024-07-04", "time": "16:00:00Z"}),
        ),
    ])?;
    as_bob_in_chess
        .assert_metadata_eq(
            book_of_bob_in_chess.clone(),
            &title,
            Some("Bob's book on chess"),
        )
        .assert_metadata_eq(
            book_of_bob_in_chess.clone(),
            &created_at,
            Some(serde_json::json!({"date": "2024-07-04", "time": "16:00:00Z"})),
        );
    as_bob_in_chess.submit_blocking(
        // A key-value pair can be re-set.
        SetKeyValue::asset(
            book_of_bob_in_chess.clone(),
            title.clone(),
            "Bob's Great Chess Encyclopedia",
        ),
    )?;
    as_bob_in_chess.assert_metadata_eq(
        book_of_bob_in_chess.clone(),
        &title,
        Some("Bob's Great Chess Encyclopedia"),
    );
    as_bob_in_chess.submit_all_blocking([RemoveKeyValue::asset(
        book_of_bob_in_chess.clone(),
        created_at.clone(),
    )])?;
    as_bob_in_chess.assert_metadata_eq(
        book_of_bob_in_chess,
        &created_at,
        None::<serde_json::Value>,
    );
    Ok(())
}

trait StoreAssetExt {
    fn assert_metadata_eq<T: Into<Json>>(
        &self,
        asset_id: AssetId,
        key: &Name,
        expected_value: Option<T>,
    ) -> &Self;
}

impl StoreAssetExt for Client {
    fn assert_metadata_eq<T: Into<Json>>(
        &self,
        asset_id: AssetId,
        key: &Name,
        expected_value: Option<T>,
    ) -> &Self {
        let asset = self
            .query(FindAssets)
            .filter_with(|asset| asset.id.eq(asset_id))
            .execute_single()
            .unwrap();
        let AssetValue::Store(actual) = asset.value() else {
            // FIXME: this API inconvenience should be resolved
            //  when numeric assets are separated from store assets.
            panic!("should be a numeric asset");
        };
        assert_eq!(actual.get(key), expected_value.map(Into::into).as_ref());
        self
    }
}
