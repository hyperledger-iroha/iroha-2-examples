//! Shows how to mint, burn and transfer numeric assets.
//!
//! Depends on `asset_definition_register`

use iroha::client::asset;
use iroha::data_model::prelude::{
    numeric, Asset, AssetValue, Burn, Mint, Numeric, Register, Transfer,
};

use iroha_examples::{
    AliceInWonderland, BobInWonderland, MagnusInChess, MoneyOfAliceInWonderland,
    MoneyOfBobInWonderland, WonderlandMoneyOfMagnusInChess,
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
    // TODO: this section is not true, but I'd like it to be;
    //  see https://github.com/hyperledger/iroha/issues/4087#issuecomment-2188067574
    as_alice_in_wland.submit_all_blocking([
        // For `alice@wonderland` to be able to hold `money#wonderland`, we need
        // to register `money##alice@wonderland`. This is sort of like
        // giving her a wallet before she can carry `money#wonderland`.
        Register::asset(Asset::new(money_of_alice_in_wland.clone(), 0_u32)),
        // Register `money#wonderland` for `bob@wonderland` for `alice@wonderland`s later transfer.
        // Since `alice@wonderland` owns the definition of `money#wonderland`, she has to do it.
        Register::asset(Asset::new(money_of_bob_in_wland.clone(), 0_u32)),
        // Register `money#wonderland` for `magnus@chess` for `bob@wonderland`s later transfer.
        Register::asset(Asset::new(wland_money_of_magnus_in_chess.clone(), 0_u32)),
    ])?;
    // FIXME: currently, minting will register the asset if it does not exist.
    // Now `alice@wonderland` can mint `money#wonderland` for herself, since
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
    as_alice_in_wland
        .request(asset::by_id(money_of_alice_in_wland.clone()))?
        .assert_eq(numeric!(5));
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
    as_alice_in_wland
        .request(asset::by_id(money_of_alice_in_wland.clone()))?
        .assert_eq(numeric!(1.97));
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
    as_alice_in_wland
        .request(asset::by_id(money_of_alice_in_wland))?
        .assert_eq(numeric!(0.57));
    // `bob@wonderland` observes that he has 1.4 of `money#wonderland` now.
    // He can do that because he owns `money##bob@wonderland`.
    as_bob_in_wland
        .request(asset::by_id(money_of_bob_in_wland.clone()))?
        .assert_eq(numeric!(1.4));
    as_bob_in_wland.submit_blocking(
        // `bob@wonderland` can transfer some of his `money#wonderland` to `magnus@chess`.
        // Note how `money#wonderland` can be held by accounts in different domains.
        Transfer::asset_numeric(money_of_bob_in_wland, numeric!(0.7), MagnusInChess::id()),
    )?;
    // `alice@wonderland` observes that `magnus@chess` has 0.7 of `money#wonderland`.
    // She can do that because she owns the definition of `money#wonderland`.
    as_alice_in_wland
        .request(asset::by_id(wland_money_of_magnus_in_chess))?
        .assert_eq(numeric!(0.7));
    Ok(())
}

trait NumericAssetExt {
    fn assert_eq(&self, expected: Numeric);
}

impl NumericAssetExt for Asset {
    fn assert_eq(&self, expected: Numeric) {
        let AssetValue::Numeric(actual) = self.value else {
            // FIXME: this API inconvenience should be resolved
            //  when numeric assets are separated from store assets.
            panic!("should be a numeric asset");
        };
        assert_eq!(actual, expected);
    }
}
