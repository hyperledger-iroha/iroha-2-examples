use std::{thread, time::Duration};

use iroha::samples::{construct_validator, get_config};
use iroha_data_model::prelude::*;
use iroha_genesis::{GenesisNetwork, RawGenesisBlock, RawGenesisBlockBuilder};
use test_network::{
    get_key_pair, wait_for_genesis_committed, Peer as TestPeer, PeerBuilder, TestRuntime,
};
use tokio::runtime::Runtime;

fn generate_genesis(num_domains: u32) -> RawGenesisBlock {
    let mut builder = RawGenesisBlockBuilder::new();

    let key_pair = get_key_pair();
    for i in 0_u32..num_domains {
        builder = builder
            .domain(format!("wonderland-{i}").parse().expect("Valid"))
            .account(
                format!("Alice-{i}").parse().expect("Valid"),
                key_pair.public_key().clone(),
            )
            .asset(
                format!("xor-{i}").parse().expect("Valid"),
                AssetValueType::Quantity,
            )
            .finish_domain();
    }

    builder
        .validator(
            construct_validator("../default_validator").expect("Failed to construct validator"),
        )
        .build()
}

fn main_genesis() {
    let mut peer = <TestPeer>::new().expect("Failed to create peer");
    let configuration = get_config(
        std::iter::once(peer.id.clone()).collect(),
        Some(get_key_pair()),
    );
    let rt = Runtime::test();
    let genesis = GenesisNetwork::from_configuration(
        generate_genesis(1_000_000_u32),
        Some(&configuration.genesis),
    )
    .expect("genesis creation failed");

    let builder = PeerBuilder::new()
        .with_into_genesis(genesis)
        .with_configuration(configuration);

    // This only submits the genesis. It doesn't check if the accounts
    // are created, because that check is 1) not needed for what the
    // test is actually for, 2) incredibly slow, making this sort of
    // test impractical, 3) very likely to overflow memory on systems
    // with less than 16GiB of free memory.
    rt.block_on(builder.start_with_peer(&mut peer));
}

fn create_million_accounts_directly() {
    let (_rt, _peer, test_client) = <PeerBuilder>::new().start_with_runtime();
    wait_for_genesis_committed(&vec![test_client.clone()], 0);
    for i in 0_u32..1_000_000_u32 {
        let domain_id: DomainId = format!("wonderland-{i}").parse().expect("Valid");
        let normal_account_id = AccountId::new(
            format!("bob-{i}").parse().expect("Valid"),
            domain_id.clone(),
        );
        let create_domain = RegisterBox::new(Domain::new(domain_id));
        let create_account = RegisterBox::new(Account::new(normal_account_id.clone(), []));
        if test_client
            .submit_all([create_domain, create_account])
            .is_err()
        {
            thread::sleep(Duration::from_millis(100));
        }
    }
    thread::sleep(Duration::from_secs(1000));
}

fn main() {
    create_million_accounts_directly();
    main_genesis();
}
