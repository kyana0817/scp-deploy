use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;
use std::sync::Arc;

use crate::loader::ScpConfig;
use russh::client;
use russh_config;
use russh_keys::{key, load_secret_key};

#[derive(Debug)]
pub struct Scp {
    pub ssh_config: russh_config::Config,
    pub client_config: client::Config,
}

struct Client {}

#[async_trait]
impl client::Handler for Client {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_key: &key::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

impl Scp {
    pub fn new(configs: Vec<ScpConfig>) -> Self {
        let ssh_config = match configs.get(0).unwrap() {
            ScpConfig {
                alias: Some(alias), ..
            } => russh_config::Config::default(&alias),
            ScpConfig {
                host: Some(host), ..
            } => russh_config::Config::default(&host),
            _ => panic!("Invalid scp config schema"),
        };

        Self {
            ssh_config,
            client_config: client::Config::default(),
        }
    }

    pub fn run() {
        todo!()
    }

    pub fn with_client_config(mut self, client_config: client::Config) -> Self {
        self.client_config = client_config;
        self
    }

    pub async fn deploy(self) -> Result<()> {
        let config = Arc::new(self.client_config);
        let sh = Client {};
        let key_path = PathBuf::from(
            self.ssh_config
                .identity_file
                .clone()
                .unwrap_or("".to_string()),
        );
        let key_pair = load_secret_key(key_path, None)?;
        let host_name = self.ssh_config.host_name.clone();
        let port = self.ssh_config.port.clone();
        let user = self.ssh_config.user.clone();
        let mut connect = client::connect(config, (host_name, port), sh).await?;
        let auth_res = connect
            .authenticate_publickey(user, Arc::new(key_pair))
            .await?;

        if !auth_res {
            anyhow::bail!("Authentication failed");
        }

        Ok(())
    }
}
