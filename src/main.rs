pub mod config;

use config::Config;

fn main() {
    let config = Config::from_args();
    dbg!(config);
}
