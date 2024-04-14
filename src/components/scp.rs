use crate::loader::ScpConfig;
use russh_config::Config;

#[derive(Debug)]
pub struct Scp {
    pub ssh: Config,
}

impl Scp {
    pub fn new(config: ScpConfig) -> Self {
        let ssh = match config {
            ScpConfig {
                alias: Some(alias), ..
            } => Config::default(&alias),
            ScpConfig {
                host: Some(host), ..
            } => Config::default(&host),
            _ => panic!("Invalid scp config schema"),
        };
        // todo
        Scp { ssh }
    }
}
