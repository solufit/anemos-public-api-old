mod get_env;
mod earthquake;


use tokio::time::{interval, Duration};

async fn abc() {
    // Your code here
    log::info!("Run every 10s");
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut interval = interval(Duration::from_secs(10));
    log::info!("Starting Data Collector for Anemos Public API Version: {}", env!("CARGO_PKG_VERSION"));
    loop {
        interval.tick().await;
        abc().await;
    }
}
