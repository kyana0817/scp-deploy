use serde::Deserialize;
use serde_yaml;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub scp: Option<Vec<ScpConfig>>,
}

#[derive(Deserialize, Debug)]
pub struct ScpConfig {
    pub alias: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub remote_path: Option<String>,
    pub local_path: Option<String>,
    pub file_path: Option<String>,
}

pub fn parse_config(path: Option<String>) -> Config {
    let file =
        fs::read_to_string(path.unwrap_or("./config.yaml".to_string())).expect("cannot read file");
    let config = serde_yaml::from_str::<Config>(&file).expect("cannot parse yaml");

    config
}
