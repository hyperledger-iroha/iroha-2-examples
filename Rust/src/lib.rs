//! Iroha examples library.

use iroha::client::Client;
use iroha::config::Config;
use iroha::crypto::{Algorithm, ExposedPrivateKey, KeyPair, PublicKey};
use iroha::data_model::prelude::*;

pub type Result<T> = eyre::Result<T>;

/// An example domain.
pub trait ExampleDomain {
    /// Name of the [`ExampleDomain`].
    const NAME: &'static str;

    /// Constructs a [`DomainId`] for the [`ExampleDomain`].
    fn domain_id() -> DomainId {
        Self::NAME.parse::<DomainId>().unwrap()
    }
}

/// An example signatory (someone who can sign transactions).
///
/// In the examples, each signatory is a character.
pub trait ExampleSignatory {
    /// Alias of the [`ExampleSignatory`].
    const ALIAS: &'static str;

    /// Public key identifying the [`ExampleSignatory`].
    fn public_key() -> PublicKey {
        let (public_key, private_key) =
            KeyPair::from_seed(Self::ALIAS.as_bytes().to_vec(), Algorithm::default()).into_parts();
        println!(
            "---------------\n\
            Generated key pair for `{}`\n\
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
    /// Constructs an [`AccountId`] for the [`ExampleAccount`].
    pub fn account_id() -> AccountId {
        AccountId::new(Domain::domain_id(), Signatory::public_key())
    }

    /// Creates an Iroha client acting on behalf of the [`ExampleAccount`].
    ///
    /// The corresponding config must exist at `../configs/{signatory}_{domain}.toml`.
    pub fn client() -> Client {
        let config = Config::load(format!(
            "configs/{}_{}.toml",
            Signatory::ALIAS,
            Domain::NAME
        ))
        .expect("config is loaded and valid");
        let client = Client::new(config);
        let expected_account = ExampleAccount::<Signatory, Domain>::account_id();
        assert_eq!(
            client.account,
            ExampleAccount::<Signatory, Domain>::account_id(),
            "Client was requested for `{}`, but the actual authority does not match.\n\
            Check the corresponding client configuration file.\n\
            Expected: {}\n\
            Actual: {}",
            Signatory::ALIAS,
            expected_account,
            client.account
        );
        println!(
            "---------------\n\
            Client for `{}` in `{}` created.\n\
            account_id: {}",
            Signatory::ALIAS,
            Domain::NAME,
            client.account,
        );
        client
    }
}

/// An example asset name.
///
/// The same asset name may appear in different domains.
pub trait ExampleAssetName {
    /// Human-readable asset name.
    const NAME: &'static str;

    /// Constructs a [`Name`] from the [`ExampleAssetName`].
    fn asset_name() -> Name {
        Self::NAME.parse::<Name>().unwrap()
    }
}

/// Numeric precision, either [`Unconstrained`]
/// or fixed up to a number of [`DecimalPoints`].
pub trait NumericPrecision {
    /// Returns the [`NumericSpec`] of this [`Precision`].
    fn numeric_spec() -> NumericSpec;
}

/// Unconstrained [`NumericPrecision`].
pub struct Unconstrained;

impl NumericPrecision for Unconstrained {
    fn numeric_spec() -> NumericSpec {
        NumericSpec::unconstrained()
    }
}

/// [`NumericPrecision`] up to `N` decimal points.
pub struct DecimalPoints<const N: u32>;

impl<const N: u32> NumericPrecision for DecimalPoints<N> {
    fn numeric_spec() -> NumericSpec {
        NumericSpec::fractional(N)
    }
}

/// An example asset type.
///
/// This determines whether the asset is [`Numeric`] or [`Store`].
pub trait ExampleAssetType {
    /// Returns the [`AssetValueType`] of the [`ExampleAssetType`].
    fn value_type() -> AssetValueType;

    /// Constructs a [`NewAssetDefinition`] from the [`ExampleAssetType`].
    fn new_asset_definition(asset_definition_id: AssetDefinitionId) -> NewAssetDefinition {
        AssetDefinition::new(asset_definition_id, Self::value_type())
    }
}

/// Non-fungible item.
/// 
/// A non-fungible item is unique, can be minted AND burned exactly once.
pub struct NonFungible;

impl ExampleAssetType for NonFungible {
    fn value_type() -> AssetValueType {
        todo!("Iroha does not support non-fungible assets directly")
    }
}

/// Dictionary-like resource.
///
/// `Store` assets store key-value pairs.
pub struct Store;

impl ExampleAssetType for Store {
    fn value_type() -> AssetValueType {
        AssetValueType::Store
    }
}

/// Numeric resource with the given precision
/// that is `MINTABLE` repeatedly or only once.
///
/// `Numeric` assets are minted and burned.
pub struct Numeric<const MINTABLE: bool, Precision: NumericPrecision>(Precision);

impl<const MINTABLE: bool, Precision: NumericPrecision> ExampleAssetType
    for Numeric<MINTABLE, Precision>
{
    fn value_type() -> AssetValueType {
        AssetValueType::Numeric(Precision::numeric_spec())
    }

    fn new_asset_definition(asset_definition_id: AssetDefinitionId) -> NewAssetDefinition {
        let definition = AssetDefinition::new(asset_definition_id, Self::value_type());
        if MINTABLE {
            definition
        } else {
            definition.mintable_once()
        }
    }
}

/// [`Numeric`] asset type with arbitrary precision
/// that is `MINTABLE` repeatedly or only once.
pub type Arbitrary<const MINTABLE: bool> = Numeric<MINTABLE, Unconstrained>;

/// [`Numeric`] asset type with a fixed precision up to `N` decimal points
/// that is `MINTABLE` repeatedly or only once.
pub type FixedNumeric<const MINTABLE: bool, const N: u32> = Numeric<MINTABLE, DecimalPoints<N>>;

/// An [`ExampleAssetName`] specified to an [`ExampleDomain`].
pub struct ExampleAssetDefinition<Type, Name, Domain>(Type, Name, Domain);

impl<Type, Name, Domain> ExampleAssetDefinition<Type, Name, Domain>
where
    Type: ExampleAssetType,
    Name: ExampleAssetName,
    Domain: ExampleDomain,
{
    /// Constructs an [`AssetDefinitionId`] from the [`ExampleAssetDefinition`].
    pub fn asset_definition_id() -> AssetDefinitionId {
        AssetDefinitionId::new(Domain::domain_id(), Name::asset_name())
    }
}

impl<Type, Name, Domain> ExampleAssetDefinition<Type, Name, Domain>
where
    Type: ExampleAssetType,
    Name: ExampleAssetName,
    Domain: ExampleDomain,
{
    /// Constructs an [`NewAssetDefinition`] from the [`ExampleAssetDefinition`].
    pub fn asset_definition() -> NewAssetDefinition {
        Type::new_asset_definition(Self::asset_definition_id())
    }
}

/// An instance of an [`ExampleAssetDefinition`] belonging to an [`ExampleAccount`].
///
/// **Note**: the asset definition and the account
/// do not have to belong to the same domain.
pub struct ExampleAsset<Definition, Account>(Definition, Account);

impl<AssetType, AssetName, AssetDomain, Signatory, AccountDomain>
    ExampleAsset<
        ExampleAssetDefinition<AssetType, AssetName, AssetDomain>,
        ExampleAccount<Signatory, AccountDomain>,
    >
where
    AssetType: ExampleAssetType,
    AssetName: ExampleAssetName,
    AssetDomain: ExampleDomain,
    Signatory: ExampleSignatory,
    AccountDomain: ExampleDomain,
{
    /// Constructs an [`AssetId`] from the asset.
    pub fn asset_id() -> AssetId {
        AssetId::new(
            ExampleAssetDefinition::<AssetType, AssetName, AssetDomain>::asset_definition_id(),
            ExampleAccount::<Signatory, AccountDomain>::account_id(),
        )
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

/// Numeric asset type that takes integer values
/// and is `MINTABLE` repeatedly or only once.
///
/// - **Example values:** 0, 1, -123.
/// - **Invalid values:** -31.12, 0.1.
pub type Integral<const MINTABLE: bool> = FixedNumeric<MINTABLE, 0>;

/// Numeric asset type that behaves like a currency with a cent
/// and is `MINTABLE` repeatedly or only once.
///
/// - **Example values:** 123, 0.1, 0.01, 999.99.
/// - **Invalid values:** 1.123, 0.001.
pub type Centecimal<const MINTABLE: bool> = FixedNumeric<MINTABLE, 2>;

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

/// The general idea of a book.
pub struct Book;

impl ExampleAssetName for Book {
    const NAME: &'static str = "book";
}

/// The general idea of a title.
pub struct Title;

impl ExampleAssetName for Title {
    const NAME: &'static str = "title";
}

/// `rose#wonderland` is defined in the default genesis block.
///
/// Backed by an [`Arbitrary`] numeric type, and can be minted indefinitely.
pub type WonderlandRoses = ExampleAssetDefinition<Arbitrary<true>, Roses, Wonderland>;
/// `money#wonderland` is defined in the `asset_definition_register` example.
///
/// Backed by a numeric type up to 2 decimal points, and can be minted indefinitely.
pub type WonderlandMoney = ExampleAssetDefinition<Centecimal<true>, Money, Wonderland>;
/// `pawn#chess` is defined in the `asset_definition_register` example.
///
/// Backed by an integral numeric type, and can only be minted once per account.
pub type ChessPawns = ExampleAssetDefinition<Integral<false>, Pawns, Chess>;
/// `book#chess` is defined in the `asset_definition_register` example.
///
/// Backed by a [`Store`] type.
pub type ChessBook = ExampleAssetDefinition<Store, Book, Chess>;
/// `title#chess` is defined in the TODO(`asset_register`) example.
/// 
/// TODO: It is supposed to be an example of a non-fungible token,
///  a unique thing that is neither a numeric asset nor a store asset.
pub type ChessTitle = ExampleAssetDefinition<NonFungible, Title, Chess>;

/// `roses##alice@wonderland` is defined in the default genesis block.
pub type WonderlandRosesOfAliceInWonderland = ExampleAsset<WonderlandRoses, AliceInWonderland>;
/// `roses##alice@wonderland` is defined in the TODO(`asset_register`) example.
pub type WonderlandMoneyOfAliceInWonderland = ExampleAsset<WonderlandRoses, AliceInWonderland>;
/// `book#chess#alice@wonderland` is defined in the TODO(`asset_register`) example.
pub type ChessBookOfAliceInWonderland = ExampleAsset<ChessBook, AliceInWonderland>;
