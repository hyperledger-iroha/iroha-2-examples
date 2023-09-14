use eyre::{Error, WrapErr};
use iroha_config::client::Configuration;
use std::fs::File;

fn main() {
    let config = load_configuration().expect("Configuration should be loading normally");
    domain_registration_test(&config)
        .expect("Domain registration example is expected to work correctly");

    println!("Domain registration example works!");
}

fn load_configuration() -> Result<Configuration, Error> {
    // #region rust_config_load
    let config_loc = "./config.json";
    let file = File::open(config_loc)
        .wrap_err(format!(
            "Unable to load the configuration file at `{}`",
            config_loc
        ))
        .expect("Config file is loading normally.");
    let config: Configuration = serde_json::from_reader(file)
        .wrap_err(format!("Failed to parse `{}`", config_loc))
        .expect("Verified in tests");
    // #endregion rust_config_load

    // Return the configuration normally
    Ok(config)
}

fn domain_registration_test(config: &Configuration) -> Result<(), Error> {
    // #region domain_register_example_crates
    use iroha_client::client::Client;
    use iroha_data_model::{
        metadata::UnlimitedMetadata,
        prelude::{Domain, DomainId, InstructionBox, RegisterBox},
    };
    // #endregion domain_register_example_crates

    // #region domain_register_example_create_domain
    // Create a domain Id
    let looking_glass: DomainId = "looking_glass".parse()?;
    // #endregion domain_register_example_create_domain

    // #region domain_register_example_create_isi
    // Create an ISI
    let create_looking_glass = RegisterBox::new(Domain::new(looking_glass));
    // #endregion domain_register_example_create_isi

    // #region rust_client_create
    // Create an Iroha client
    let iroha_client: Client = Client::new(&config)?;
    // #endregion rust_client_create

    // #region domain_register_example_prepare_tx
    // Prepare a transaction
    let metadata = UnlimitedMetadata::default();
    let instructions: Vec<InstructionBox> = vec![create_looking_glass.into()];
    let tx = iroha_client
        .build_transaction(instructions, metadata)
        .wrap_err("Error building a domain registration transaction")?;
    // #endregion domain_register_example_prepare_tx

    // #region domain_register_example_submit_tx
    // Submit a prepared domain registration transaction
    iroha_client
        .submit_transaction(&tx)
        .wrap_err("Failed to submit transaction")?;
    // #endregion domain_register_example_submit_tx

    // Finish the test successfully
    Ok(())
}
