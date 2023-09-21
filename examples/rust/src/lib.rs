use eyre::Result;
use iroha_client::client::Client;
use iroha_config::client::ConfigurationProxy;
use iroha_config_base::proxy::LoadFromDisk;
use std::path::Path;

// FIXME: this should be a part of the client out of the box
pub fn load_client(path: impl AsRef<Path>) -> Result<Client> {
    let mut config = ConfigurationProxy::from_path(path.as_ref().clone());
    config.finish()?;
    let config = config.build()?;
    Client::new(&config)
}
