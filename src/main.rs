mod loader;
use loader::parse_config;
fn main() {
    let config = parse_config(None);
    println!("{:?}", config)
}

// use russh::client::Config;
// use russh_config::parse_home;

// fn main() {
//     let config = parse_home("oravpn").unwrap();
//     println!("{:?}", config);
// }
