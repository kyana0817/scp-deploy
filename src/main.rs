mod loader;
use loader::parse_config;
fn main() {
    let config = parse_config(None);
    println!("{:?}", config)
}
