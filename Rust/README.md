# Rust examples for Hyperledger Iroha

This project contains code examples to help you get started using the Rust SDK for Hyperledger Iroha, as well as understand Iroha conceptually.

## Running the examples

To run the examples, you should have a running Iroha network with the default genesis block configuration. Some examples depend on other examples being executed. Examples without dependencies are good starting points:

- [`domain_register`](examples/domain_register.rs)

## Helper library

A small helper library is included to facilitate the creation of code examples that demonstrate Iroha usage patterns while focusing on high-level concepts.

Users can learn to use the lower-level APIs by checking out the implementations of the building blocks.

### Usage

* Define primitives like domains (`Wonderland`, `Chess`), signatories (`Alice`, `Bob`, `Magnus`), assets (`Roses`, `Pawns`, `Book`), and more using traits like `ExampleDomain`, `ExampleSignatory` `ExampleAssetName` and others.
* Combine primitives into compound types for accounts (`AliceInWonderland`, `BobInChess`), asset definitions (`WonderlandRoses`, `GardenCabbages`) and more using `ExampleAccount`, `ExampleAssetDefinition`, etc.
* Easily construct identifiers (`BobInWonderland::account_id()`, `ChessBookOfAliceInWonderland::asset_id()`) as needed.
* Construct clients acting on behalf of various accounts using `AliceInWonderland::client()`, `BobInChess::client()`.