mod cli;
mod codec;
mod fdetector;
use clap::Parser;
use std::fs::read_to_string;
use std::sync::Arc;
use std::net::{SocketAddrV4, Ipv4Addr, SocketAddr};
use tokio;
use toml;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let args = cli::Args::parse();
    info!("Got peers file: {}", args.peers);
    info!("Got self id of: {}", args.id);
    let contents = match read_to_string(&args.peers) {
        Ok(contents) => contents,
        Err(e) => {
            error!("was unable to open {}", args.peers);
            error!("error: {}", e);
            return;
        }
    };
    let config: fdetector::Config = match toml::from_str(&contents) {
        Ok(peer_data) => peer_data,
        Err(e) => {
            error!("was unable to parse toml");
            error!("error: {}", e);
            return;
        }
    };

    println!("{:#?}", config);

    let rt = match tokio::runtime::Builder::new_current_thread().build() {
        Ok(rt) => rt,
        Err(e) => {
            error!("was not able to construct tokio runtime");
            error!("error: {}", e);
            return;
        }
    };
    let d = fdetector::SwimFailureDetector::new(
        args.id,
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 8080)),
        config.peers,
        config.period,
        config.failure_group_sz,
    );
    let d = Arc::new(d);
    let ft = d.run();
    rt.block_on(ft);
}
