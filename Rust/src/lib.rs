//! Iroha examples library. Doubles as a tutorial for Iroha's data model.

use iroha::client::Client;
use iroha::config::Config;
use iroha::crypto::{Algorithm, ExposedPrivateKey, KeyPair, PublicKey};
use iroha::data_model::prelude::*;

pub type Result<T> = eyre::Result<T>;

/// An example domain.
pub trait ExampleDomain {
    /// Name of the example domain.
    ///
    /// Cannot be empty, cannot contain whitespace or reserved characters `@` and `#`.
    const NAME: &'static str;

    /// A domain is identified by a [`DomainId`], which is backed by a [`Name`].
    ///
    /// A [`Name`] cannot be empty, cannot contain whitespace or characters `@` and `#`,
    /// which are reserved for accounts and assets.
    fn id() -> DomainId {
        // You can also parse into a `Name`, then use `DomainId::new`.
        Self::NAME.parse::<DomainId>().unwrap()
    }
}

/// An example signatory (someone who can sign transactions).
///
/// In the examples, each signatory is a character.
pub trait ExampleSignatory {
    /// Alias of the example signatory.
    ///
    /// TODO: Iroha does not support [alias resolution] yet,
    ///  this field is purely for example convenience now,
    ///  since public keys are not as readable. Used to
    ///  generate some of the example key pairs.
    ///
    /// [alias resolution]: https://github.com/hyperledger/iroha/issues/4372
    const ALIAS: &'static str;

    /// A signatory is identified by its public key.
    ///
    /// Some example signatories like [Alice] and [Bob]
    /// override this and provide a previously defined key.
    fn public_key() -> PublicKey {
        let (public_key, private_key) =
            KeyPair::from_seed(Self::ALIAS.as_bytes().to_vec(), Algorithm::default()).into_parts();
        println!(
            "Generated key pair for `{}`\n\
            Public: {}\n\
            Private (save this): {}",
            Self::ALIAS,
            public_key,
            ExposedPrivateKey(private_key),
        );
        public_key
    }
}

/// An example account owned by an [`ExampleSignatory`] in an [`ExampleDomain`].
pub struct ExampleAccount<Signatory, Domain>(Signatory, Domain);

impl<Signatory, Domain> ExampleAccount<Signatory, Domain>
where
    Signatory: ExampleSignatory,
    Domain: ExampleDomain,
{
    /// An account is identified by an [`AccountId`]
    /// composed of a [`PublicKey`] and a [`DomainId`].
    ///
    /// An [`AccountId`] can be parsed from a string of the form `public_key@domain`.
    pub fn id() -> AccountId {
        let signatory = Signatory::public_key();
        let domain = Domain::id();
        // "signatory@domain".parse::<AccountId>().unwrap();
        AccountId::new(domain, signatory)
    }

    /// You need a [`Client`] to be able to submit instructions to Iroha.
    ///
    /// A client acts on behalf of an account, which is sometimes called an *authority*.
    /// Depending on which permissions the *authority* has, and which objects the *authority*
    /// owns, instructions submitted by the client will succeed or fail.
    ///
    /// This function demonstrates how to set up a client, and provides
    /// a way to quickly set up different actors in example scenarios.
    pub fn client() -> Client {
        // The corresponding config must exist at `../configs/{signatory}_{domain}.toml`.
        let config = Config::load(format!(
            "configs/{}_{}.toml",
            Signatory::ALIAS,
            Domain::NAME
        ))
        .expect("config is loaded and valid");
        let client = Client::new(config);
        let expected_account = ExampleAccount::<Signatory, Domain>::id();
        assert_eq!(
            client.account,
            ExampleAccount::<Signatory, Domain>::id(),
            "Client was requested for `{}`, but the actual authority does not match.\n\
            Check the corresponding client configuration file.\n\
            Expected: {}\n\
            Actual: {}",
            Signatory::ALIAS,
            expected_account,
            client.account
        );
        println!(
            "Client for `{}` in `{}` created.\n\
            Authority: {}",
            Signatory::ALIAS,
            Domain::NAME,
            client.account,
        );
        client
    }
}

/// An example asset name.
///
/// **Note:** the same asset name may appear in different domains.
/// It is meaningless unless specified to a [domain](ExampleDomain).
pub trait ExampleAssetName {
    /// Human-readable asset name.
    const NAME: &'static str;

    /// Constructs a [`Name`] from the [`ExampleAssetName`].
    ///
    /// A [`Name`] cannot be empty, cannot contain whitespace or characters `@` and `#`,
    /// which are reserved for accounts and assets.
    ///
    /// **Note:** an asset name is different from an [asset definition] or an [asset].
    ///
    /// [asset definition]: ExampleAssetDefinition
    /// [asset]: ExampleAsset
    fn name() -> Name {
        Self::NAME.parse::<Name>().unwrap()
    }
}

/// An asset definition with a [name](ExampleAssetName)
/// specified to a [domain](ExampleDomain).
///
/// It is essentially a *blueprint* for a resource
/// that can be issued and managed for an account.
pub struct ExampleAssetDefinition<AssetName, Domain>(AssetName, Domain);

impl<AssetName, Domain> ExampleAssetDefinition<AssetName, Domain>
where
    AssetName: ExampleAssetName,
    Domain: ExampleDomain,
{
    /// An asset definition is identified by an [`AssetDefinitionId`]
    /// composed of a [`Name`] and a [`DomainId`].
    ///
    /// An [`AssetDefinitionId`] can be parsed from a string of the form `asset_name#domain`.
    pub fn id() -> AssetDefinitionId {
        let asset_name = AssetName::name();
        let domain = Domain::id();
        // "asset_name#asset_domain".parse::<AssetDefinitionId>().unwrap();
        AssetDefinitionId::new(domain, asset_name)
    }
}

/// An asset of a [certain type](ExampleAssetDefinition)
/// owned by an [account](ExampleAccount).
///
/// **Note:** the asset definition and the account
/// do not have to belong to the same domain.
pub struct ExampleAsset<Definition, Account>(Definition, Account);

impl<AssetName, AssetDomain, AssetOwner, OwnerDomain>
    ExampleAsset<
        ExampleAssetDefinition<AssetName, AssetDomain>,
        ExampleAccount<AssetOwner, OwnerDomain>,
    >
where
    AssetName: ExampleAssetName,
    AssetDomain: ExampleDomain,
    AssetOwner: ExampleSignatory,
    OwnerDomain: ExampleDomain,
{
    /// An asset is identified by an [`AssetId`]
    /// composed of an [`AssetDefinitionId`] and an [`AccountId`].
    ///
    /// An [`AssetId`] has two string representation it can be parsed from:
    /// - `asset_name#asset_domain#asset_owner@owner_domain`:
    ///    when the asset and its owner belong to different domains
    /// - `asset_name##asset_owner@common_domain`:
    ///    when the asset and its owner share the domain
    pub fn id() -> AssetId {
        let asset_definition = ExampleAssetDefinition::<AssetName, AssetDomain>::id();
        let owner = ExampleAccount::<AssetOwner, OwnerDomain>::id();
        // "asset_name#asset_domain#asset_owner@owner_domain".parse::<AssetId>().unwrap();
        // "asset_name##asset_owner@common_domain".parse::<AssetId>().unwrap();
        AssetId::new(asset_definition, owner)
    }
}

////////////////////////////////////////////

////////////////////////////////////////////

/// The `wonderland` domain is defined in the default genesis block.
pub struct Wonderland;

impl ExampleDomain for Wonderland {
    const NAME: &'static str = "wonderland";
}

/// The `chess` domain is registered in the examples.
pub struct Chess;

impl ExampleDomain for Chess {
    const NAME: &'static str = "chess";
}

////////////////////////////////////////////

/// `alice` is one of the signatories
/// with an account defined in the default genesis block.
pub struct Alice;

impl ExampleSignatory for Alice {
    const ALIAS: &'static str = "alice";

    fn public_key() -> PublicKey {
        "ed0120CE7FA46C9DCE7EA4B125E2E36BDB63EA33073E7590AC92816AE1E861B7048B03"
            .parse::<PublicKey>()
            .unwrap()
    }
}

/// `bob` is one of the signatories
/// with an account defined in the default genesis block.
pub struct Bob;

impl ExampleSignatory for Bob {
    const ALIAS: &'static str = "bob";

    fn public_key() -> PublicKey {
        "ed012004FF5B81046DDCCF19E2E451C45DFB6F53759D4EB30FA2EFA807284D1CC33016"
            .parse::<PublicKey>()
            .unwrap()
    }
}

/// `magnus` is an example signatory.
pub struct Magnus;

impl ExampleSignatory for Magnus {
    const ALIAS: &'static str = "magnus";
}

/// `alice@wonderland` is defined in the genesis block.
///
/// This account starts out with:
/// - 13 [`WonderlandRoses`]
/// - 44 [`GardenCabbage`]
/// - ownership of the definition of wonderland roses
/// - ownership of [`Wonderland`]
/// - permission to set parameters
pub type AliceInWonderland = ExampleAccount<Alice, Wonderland>;
/// `bob@wonderland` is defined in the genesis block.
pub type BobInWonderland = ExampleAccount<Bob, Wonderland>;

/// `alice@chess` is defined in the `account_register` example.
pub type AliceInChess = ExampleAccount<Alice, Chess>;
/// `bob@chess` is defined in the `account_register` example.
pub type BobInChess = ExampleAccount<Bob, Chess>;
/// `magnus@chess` is defined in the `account_register` example.
pub type MagnusInChess = ExampleAccount<Magnus, Chess>;

////////////////////////////////////////////

/// The general idea of roses.
pub struct Roses;

impl ExampleAssetName for Roses {
    const NAME: &'static str = "rose";
}

/// The general idea of money.
pub struct Money;

impl ExampleAssetName for Money {
    const NAME: &'static str = "money";
}

/// The general idea of pawns.
pub struct Pawns;

impl ExampleAssetName for Pawns {
    const NAME: &'static str = "pawn";
}

/// The general idea of clothes.
pub struct Clothes;

impl ExampleAssetName for Clothes {
    const NAME: &'static str = "clothes";
}

/// `rose#wonderland` is defined in the default genesis block.
pub type WonderlandRoses = ExampleAssetDefinition<Roses, Wonderland>;
/// `money#wonderland` is defined in the `asset_definition_register` example.
pub type WonderlandMoney = ExampleAssetDefinition<Money, Wonderland>;
/// `pawn#chess` is defined in the `asset_definition_register` example.
pub type ChessPawns = ExampleAssetDefinition<Pawns, Chess>;
/// `book#chess` is defined in the `asset_definition_register` example.
pub type ChessBook = ExampleAssetDefinition<Clothes, Chess>;

/// `roses##alice@wonderland` is defined in the default genesis block.
pub type RosesOfAliceInWonderland = ExampleAsset<WonderlandRoses, AliceInWonderland>;
/// `money##alice@wonderland` is defined in the `asset_numeric` example.
pub type MoneyOfAliceInWonderland = ExampleAsset<WonderlandMoney, AliceInWonderland>;
/// `money##bob@wonderland` is defined in the `asset_numeric` example.
pub type MoneyOfBobInWonderland = ExampleAsset<WonderlandMoney, BobInWonderland>;
/// `money#wonderland#magnus@chess` is defined in the `asset_numeric` example.
pub type WonderlandMoneyOfMagnusInChess = ExampleAsset<WonderlandMoney, MagnusInChess>;
/// `pawn##alice@chess` is defined in the `asset_numeric` example.
pub type PawnsOfAliceInChess = ExampleAsset<ChessPawns, AliceInChess>;
/// `pawn##bob@chess` is defined in the `asset_numeric` example.
pub type PawnsOfBobInChess = ExampleAsset<ChessPawns, BobInChess>;
/// `book##bob@chess` is defined in the `asset_store` example.
pub type BookOfBobInChess = ExampleAsset<ChessBook, BobInChess>;