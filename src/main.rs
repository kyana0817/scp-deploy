use serde::Deserialize;
use serde_yaml;
use std::fs;

#[derive(Deserialize, Debug)]
struct Config {
    text: String,
}

fn main() {
    let file = fs::read_to_string("./config.yaml").expect("cannot read file");
    let config = serde_yaml::from_str::<Config>(&file).expect("cannot parse yaml");

    println!("{:?}", config)
}

// use russh::client::Config;
// use russh_config::parse_home;

// fn main() {
//     let config = parse_home("oravpn").unwrap();
//     println!("{:?}", config);
// }
