use crate::section_1::params::ConnectParams;
use anyhow::Result;
use postgres::{Client, Config, NoTls};

pub struct PostgressSampleClient;
impl PostgressSampleClient {
    pub fn simple_connect(params: ConnectParams) -> Result<Client> {
        let client = Client::connect(params.connect_string().as_str(), NoTls)?;
        Ok(client)
    }

    pub fn config_connect(params: ConnectParams) -> Result<Client> {
        let client = Config::new()
            .host(params.get_host())
            .port(params.get_port().clone())
            .dbname(params.get_dbname())
            .user(params.get_user())
            .password(params.get_password())
            .connect(NoTls)?;
        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn connect_ok() {
        let params = ConnectParams::new(
            "localhost".to_owned(),
            5432,
            "rust_sample".to_owned(),
            "postgres".to_owned(),
            "postgres".to_owned(),
        );
        match super::PostgressSampleClient::simple_connect(params.clone()) {
            Ok(client) => {
                println!("simple_connect: connection success");
                let _ = client.close();
            }
            Err(error) => {
                println!("{:?}", error.to_string())
            }
        };

        match super::PostgressSampleClient::config_connect(params.clone()) {
            Ok(client) => {
                println!("config_connect : connection success");
                let _ = client.close();
            }
            Err(error) => println!("{:?}", error.to_string()),
        };
    }
}
