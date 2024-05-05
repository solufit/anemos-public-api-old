mod earthquake;
use std::future::Future;
use std::pin::Pin;


use std::vec;

use tokio::time::{interval, Duration, Instant};

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut interval = interval(Duration::from_secs(10));



    log::info!("Starting Data Collector for Anemos Public API Version: {}", env!("CARGO_PKG_VERSION"));
    loop {    
        log::info!("Starting Data Collection every 10 Seconds");
        let instant = Instant::now();

        let futures: Vec<Pin<Box<dyn Future<Output = ()> + Send>>> = vec![
        Box::pin(crate::earthquake::collect_earthquake::earthquake_operator()),
    ];
        futures::future::join_all(futures).await;

        log::info!("Data Collection Finished in {} milli seconds", instant.elapsed().as_millis());

        interval.tick().await;
    }
}
