use crate::section1::params::ConnectParams;
use anyhow::Result;
use postgress::{Client, Config, NoTls};

pub struct PostgressSampleClient;
impl PostgressSampleClient {
    pub fn simple_connect(params: ConnectParams) -> Result<Client> {
        let client = Client::connect(params.connect_string().as_str(), NoTls)?;
        Ok(client)
    }

    pub fn config_connect(params: ConnectParams) -> Result<Client> {
        let client = Config::new()
            .host(params.get_host())
            .port(params.get_port())
            .dbname(params.get_dbname())
            .user(params.get_user())
            .password(params.get_password())
            .connect(NoTls)?;
        Ok(client)
    }
}
