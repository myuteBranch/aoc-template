use tracing::{debug, info, warn};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt};
use template::part2::solve;

fn main() {
    let log_level_str = "info";
    let log_level = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(log_level_str))
        .unwrap();
    let subscriber = tracing_subscriber::FmtSubscriber::new().with(log_level);
    tracing::subscriber::set_global_default(subscriber).expect("tracing failed");
    warn!("test");
    info!("test");
    debug!("test");
    let input = include_str!("../../part1.txt");
    let out = solve(input);
    info!("{:?}", out);
}