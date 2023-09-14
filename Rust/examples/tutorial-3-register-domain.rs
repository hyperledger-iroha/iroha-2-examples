use eyre::Result;
use iroha_client::client::Client;
use iroha_data_model::prelude::*;

// TODO: move to prelude?
use iroha_data_model::query::domain::model::FindAllDomains;

fn main() -> Result<()> {
    let client: Client = Client::load_config("./config.json")?;

    // Create a Domain Id
    let looking_glass: DomainId = "looking_glass".parse()?;

    // Register the domain
    let register = RegisterBox::new(Domain::new(looking_glass));
    client.submit(register.into())?;

    // Check what domains there are now
    let domains = client.request(FindAllDomains);
    // TODO: print or assert_eq?
    println!("{domains:?}");

    Ok(())
}
