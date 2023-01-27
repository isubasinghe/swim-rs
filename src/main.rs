mod cli;
mod fdetector;
use clap::Parser;
use std::fs::read_to_string;
use toml;

fn main() {
    let args = cli::Args::parse();
    println!("{}", args.peers);
    println!("{}", args.id);
    let contents = match read_to_string(&args.peers) {
        Ok(contents) => contents,
        Err(e) => {
            println!("was unable to open {}", args.peers);
            println!("error: {}", e);
            return;
        }
    };
    let config: fdetector::Config = match toml::from_str(&contents) {
        Ok(peer_data) => peer_data,
        Err(e) => {
            println!("was unable to parse toml");
            println!("error: {}", e);
            return;
        }
    };

    let mut d = fdetector::SwimFailureDetector::new(args.id, config.peers, config.period, config.failure_group_sz);
    d.run();
}
