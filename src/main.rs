mod components;
mod loader;
use components::Scp;
use loader::parse_config;
fn main() {
    let config = parse_config(None);
    let scp = Scp::new(config.scp.unwrap());
    println!("{:?}", scp)
}
