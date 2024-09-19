//! Shows how to mint, burn and transfer numeric assets.
//!
//! Depends on `asset_definition_register`

use iroha::client::Client;
use iroha::data_model::prelude::{
    numeric, Asset, AssetId, AssetValue, Burn, FindAssets, Mint, Numeric, QueryBuilderExt,
    Register, Transfer,
};

use iroha_examples::{
    AliceInChess, AliceInWonderland, BobInChess, BobInWonderland, MagnusInChess,
    MoneyOfAliceInWonderland, MoneyOfBobInWonderland, PawnsOfAliceInChess, PawnsOfBobInChess,
    WonderlandMoneyOfMagnusInChess,
};

fn main() -> iroha_examples::Result<()> {
    let as_alice_in_wland = AliceInWonderland::client();
    let as_bob_in_wland = BobInWonderland::client();
    // When specific `money#wonderland` belongs to `alice@wonderland`,
    // we call that her asset, `money##alice@wonderland`.
    // Thus, an asset is an instance of an asset definition owned by an account.
    let money_of_alice_in_wland = MoneyOfAliceInWonderland::id();
    let money_of_bob_in_wland = MoneyOfBobInWonderland::id();
    // `money#wonderland` can be held by accounts outside `wonderland`.
    let wland_money_of_magnus_in_chess = WonderlandMoneyOfMagnusInChess::id();

    // `alice@wonderland` can mint `money#wonderland` for herself, since
    // she was the one who defined it. Someone holding a relevant permission
    // can also mint an asset.
    //
    // Minting increases the asset's amount.
    as_alice_in_wland.submit_all_blocking([
        Mint::asset_numeric(numeric!(1.25), money_of_alice_in_wland.clone()),
        // `money#wonderland` is defined to be mintable repeatedly,
        // therefore we can repeat the Mint instruction as much as we want.
        Mint::asset_numeric(numeric!(1.25), money_of_alice_in_wland.clone()),
        Mint::asset_numeric(numeric!(1.25), money_of_alice_in_wland.clone()),
        Mint::asset_numeric(numeric!(1.25), money_of_alice_in_wland.clone()),
    ])?;
    // Observe that `alice@wonderland` has 5 of `money#wonderland`.
    as_alice_in_wland.assert_asset_eq(money_of_alice_in_wland.clone(), numeric!(5));
    // Now that `alice@wonderland` has some of `money#wonderland`,
    // she can burn it. An asset can be burned by its owner,
    // the owner of its definition, and a holder of a relevant permission.
    //
    // Burning decreases the asset's amount.
    // You cannot burn more of the asset that is actually owned.
    as_alice_in_wland.submit_all_blocking([
        Burn::asset_numeric(numeric!(0.01), money_of_alice_in_wland.clone()),
        Burn::asset_numeric(numeric!(1.01), money_of_alice_in_wland.clone()),
        Burn::asset_numeric(numeric!(2.01), money_of_alice_in_wland.clone()),
    ])?;
    // Observe that `alice@wonderland` has 1.97 of `money#wonderland` left.
    as_alice_in_wland.assert_asset_eq(money_of_alice_in_wland.clone(), numeric!(1.97));
    as_alice_in_wland.submit_blocking(
        // `alice@wonderland` can transfer some of her `money#wonderland` to another account.
        // Like with minting, an asset can be transferred by its owner, the owner of
        // its definition, or someone with a relevant permission.
        Transfer::asset_numeric(
            money_of_alice_in_wland.clone(),
            numeric!(1.4),
            BobInWonderland::id(),
        ),
    )?;
    // `alice@wonderland` observes that she has 0.57 of `money#wonderland` left.
    as_alice_in_wland.assert_asset_eq(money_of_alice_in_wland.clone(), numeric!(0.57));
    // `bob@wonderland` observes that he has 1.4 of `money#wonderland` now.
    // He can do that because he owns `money##bob@wonderland`.
    as_bob_in_wland.assert_asset_eq(money_of_bob_in_wland.clone(), numeric!(1.4));
    as_bob_in_wland.submit_blocking(
        // `bob@wonderland` can transfer some of his `money#wonderland` to `magnus@chess`.
        // Note how `money#wonderland` can be held by accounts in different domains.
        Transfer::asset_numeric(money_of_bob_in_wland, numeric!(0.7), MagnusInChess::id()),
    )?;
    // `alice@wonderland` observes that `magnus@chess` has 0.7 of `money#wonderland`.
    // She can do that because she owns the definition of `money#wonderland`.
    as_alice_in_wland.assert_asset_eq(wland_money_of_magnus_in_chess, numeric!(0.7));

    // `pawn#chess` is a mintable-once asset. It has a fixed global supply.
    let as_bob_in_chess = BobInChess::client();
    let pawns_of_alice_in_chess = PawnsOfAliceInChess::id();
    let pawns_of_bob_in_chess = PawnsOfBobInChess::id();
    as_bob_in_chess.submit_all_blocking([
        Register::asset(Asset::new(pawns_of_alice_in_chess.clone(), Numeric::ZERO)),
        Register::asset(Asset::new(pawns_of_bob_in_chess.clone(), Numeric::ZERO)),
    ])?;
    as_bob_in_chess.submit_blocking(
        // By minting `pawn##chess@alice`, we fix the global supply.
        Mint::asset_numeric(numeric!(16), pawns_of_alice_in_chess.clone()),
    )?;
    // No more `pawn#chess` can be minted.
    [
        as_bob_in_chess.submit_blocking(Mint::asset_numeric(
            numeric!(1),
            pawns_of_alice_in_chess.clone(),
        )),
        as_bob_in_chess.submit_blocking(Mint::asset_numeric(
            numeric!(1),
            pawns_of_bob_in_chess.clone(),
        )),
    ]
    .map(|r| assert!(r.is_err()));
    // `alice@chess` can still burn and transfer her `pawn#chess`:
    let as_alice_in_chess = AliceInChess::client();
    as_alice_in_chess.submit_blocking(Burn::asset_numeric(
        numeric!(8),
        pawns_of_alice_in_chess.clone(),
    ))?;
    as_alice_in_chess.submit_blocking(Transfer::asset_numeric(
        pawns_of_alice_in_chess.clone(),
        numeric!(8),
        BobInChess::id(),
    ))?;
    as_alice_in_chess.assert_asset_eq(pawns_of_alice_in_chess, Numeric::ZERO);
    as_bob_in_chess.assert_asset_eq(pawns_of_bob_in_chess, numeric!(8));
    Ok(())
}

trait NumericAssetExt {
    fn assert_asset_eq(&self, asset_id: AssetId, expected: Numeric);
}

impl NumericAssetExt for Client {
    fn assert_asset_eq(&self, asset_id: AssetId, expected: Numeric) {
        let asset = self
            .query(FindAssets)
            .filter_with(|asset| asset.id.eq(asset_id))
            .execute_single()
            .unwrap();
        let AssetValue::Numeric(actual) = asset.value else {
            // FIXME: this API inconvenience should be resolved
            //  when numeric assets are separated from store assets.
            panic!("should be a numeric asset");
        };
        assert_eq!(actual, expected);
    }
}
