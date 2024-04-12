use serde::Deserialize;
use serde_yaml;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    text: String,
}

pub fn parse_config(path: Option<String>) -> Config {
    let file =
        fs::read_to_string(path.unwrap_or("./config.yaml".to_string())).expect("cannot read file");
    let config = serde_yaml::from_str::<Config>(&file).expect("cannot parse yaml");

    config
}
