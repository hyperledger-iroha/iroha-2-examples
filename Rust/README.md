# Rust examples for Iroha 2

This directory contains the examples from the [Rust tutorial](https://hyperledger.github.io/iroha-2-docs/guide/rust.html#_2-configuring-iroha-2).

## Running the examples

To run the examples, you need to install [`cargo-nextest`](https://nexte.st/) first.

```bash
cargo install cargo-nextest
```

After it is installed, type:

```bash
cargo nextest run
```

You'll Cargo install the packages that are needed for the tests and the test code will run.

## Extending the example set

Simply add a file with Rust code to the [`examples`](./examples/) directory. It will be launched by `cargo-nextest` on its next run.
