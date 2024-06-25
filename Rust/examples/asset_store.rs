//! Shows how to register store assets and set/remove key-value pairs.
//!
//! Depends on the `asset_definition_register` example.
//!
//! See `asset_numeric` for basic explanations on assets.

use iroha::client::asset;
use iroha::data_model::asset::{Asset, AssetValue};
use iroha::data_model::metadata::{Limits, MetadataValueBox, Path};
use iroha::data_model::prelude::{
    numeric, Metadata, Name, Numeric, Register, RemoveKeyValue, SetKeyValue,
};
use iroha_examples::{
    AliceInWonderland, BobInChess, ClothesOfBobInChess, ExampleDomain, Wonderland,
};

fn main() -> iroha_examples::Result<()> {
    let as_alice_in_wland = AliceInWonderland::client();
    // `clothes##bob@chess` is a `clothes#chess` owned by `bob@chess`.
    let clothes_of_bob_in_chess = ClothesOfBobInChess::id();
    // TODO: this section is not true, but I'd like it to be;
    //  see https://github.com/hyperledger/iroha/issues/4087#issuecomment-2188067574
    as_alice_in_wland.submit_blocking(
        // `clothes#chess` was defined by `alice@wonderland`, therefore she has to
        // register a `clothes##bob@chess` for `bob@chess` to be able to use it.
        Register::asset(Asset::new(clothes_of_bob_in_chess.clone(), Metadata::new())),
    )?;
    let as_bob_in_chess = BobInChess::client();

    let has_been_worn = "has_been_worn".parse::<Name>()?;
    let shoes = "shoes".parse::<Name>()?;
    let size = "size".parse::<Name>()?;
    as_bob_in_chess.submit_all_blocking([
        // As the owner of a `clothes#chess`, `bob@chess` can set key-value pairs in it.
        // Only the owner of a store asset and someone with a permission can
        // set and remove key-value pairs – but not the asset definition owner.
        //
        // Store assets can hold different types of values.
        //
        // Strings:
        SetKeyValue::asset(
            clothes_of_bob_in_chess.clone(),
            "current_date".parse::<Name>()?.clone(),
            "2024-06-26".to_owned(),
        ),
        // Primitive Iroha identifiers:
        SetKeyValue::asset(
            clothes_of_bob_in_chess.clone(),
            "theme_domain".parse::<Name>()?,
            Wonderland::id().name,
        ),
        // Booleans:
        SetKeyValue::asset(
            clothes_of_bob_in_chess.clone(),
            has_been_worn.clone(),
            false,
        ),
        // Arbitrary numerics:
        SetKeyValue::asset(
            clothes_of_bob_in_chess.clone(),
            "weight_grams".parse::<Name>()?,
            numeric!(789.23),
        ),
        // Bytes:
        SetKeyValue::asset(
            clothes_of_bob_in_chess.clone(),
            "photo".parse::<Name>()?,
            "PRETEND_FILE.jpg".as_bytes().to_vec(),
        ),
        // Nested key-value map with a limited capacity and size in bytes:
        SetKeyValue::asset(clothes_of_bob_in_chess.clone(), shoes.clone(), {
            let mut shoes = Metadata::new();
            shoes.insert_with_limits(size.clone(), numeric!(40), Limits::new(2, 1024))?;
            shoes.insert_with_limits(
                "color".parse::<Name>()?,
                "red".to_owned(),
                Limits::new(2, 1024),
            )?;
            shoes
        }),
        // A list of the above (can be heterogeneous):
        SetKeyValue::asset(
            clothes_of_bob_in_chess.clone(),
            "accessories".parse::<Name>()?,
            vec!["bracelet".to_owned(), "watch".to_owned()],
        ),
    ])?;
    as_bob_in_chess
        .request(asset::by_id(clothes_of_bob_in_chess.clone()))?
        .nested_assert_eq(&[has_been_worn.clone()], Some(false))
        .nested_assert_eq(&[shoes.clone(), size], Some(numeric!(40)));
    as_bob_in_chess.submit_blocking(
        // A key-value pair can be re-set.
        SetKeyValue::asset(clothes_of_bob_in_chess.clone(), has_been_worn.clone(), true),
    )?;
    as_bob_in_chess
        .request(asset::by_id(clothes_of_bob_in_chess.clone()))?
        .nested_assert_eq(&[has_been_worn.clone()], Some(true));
    as_bob_in_chess.submit_all_blocking([
        RemoveKeyValue::asset(clothes_of_bob_in_chess.clone(), has_been_worn.clone()),
        RemoveKeyValue::asset(clothes_of_bob_in_chess.clone(), shoes.clone()),
    ])?;
    as_bob_in_chess
        .request(asset::by_id(clothes_of_bob_in_chess))?
        .nested_assert_eq(&[has_been_worn], None::<bool>)
        .nested_assert_eq(&[shoes], None::<Metadata>);
    Ok(())
}

trait NumericAssetExt {
    fn nested_assert_eq<T: Into<MetadataValueBox>>(
        self,
        path: &Path,
        expected_value: Option<T>,
    ) -> Self;
}

impl NumericAssetExt for Asset {
    fn nested_assert_eq<T: Into<MetadataValueBox>>(
        self,
        path: &Path,
        expected_value: Option<T>,
    ) -> Self {
        let AssetValue::Store(actual) = self.value() else {
            // FIXME: this API inconvenience should be resolved
            //  when numeric assets are separated from store assets.
            panic!("should be a numeric asset");
        };
        assert_eq!(
            actual.nested_get(path),
            expected_value.map(Into::into).as_ref()
        );
        self
    }
}
