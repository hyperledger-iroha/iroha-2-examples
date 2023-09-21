use eyre::Result;
use iroha_client::client::Client;
use iroha_data_model::prelude::*;

use iroha_2_examples::load_client;

// TODO: move to prelude?
use iroha_data_model::query::domain::model::FindAllDomains;

fn main() -> Result<()> {
    let client: Client = load_client("./config.json")?;

    // Create a Domain Id
    let looking_glass: DomainId = "looking_glass".parse()?;

    // Register the domain
    let register: InstructionBox = RegisterBox::new(Domain::new(looking_glass.clone())).into();
    client.submit_blocking(register)?;

    // Check what domains there are now
    let domains = client
        .request(FindAllDomains)?
        .collect::<Result<Vec<_>, _>>()?;
    assert!(domains.iter().find(|x| x.id == looking_glass).is_some());

    Ok(())
}
