mod loader;
use loader::parse_config;
mod components;
use components::Scp;
fn main() {
    let config = parse_config(None);
    let scp = Scp::new(config.scp.unwrap());
    println!("{:?}", scp)
}
