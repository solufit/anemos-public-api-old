mod earthquake;
use std::future::Future;
use std::pin::Pin;


use std::vec;

use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut interval = interval(Duration::from_secs(10));



    log::info!("Starting Data Collector for Anemos Public API Version: {}", env!("CARGO_PKG_VERSION"));
    loop {    
        let futures: Vec<Pin<Box<dyn Future<Output = ()> + Send>>> = vec![
        Box::pin(crate::earthquake::collect_earthquake::earthquake_operator()),
    ];
        interval.tick().await;
        futures::future::join_all(futures).await;
    }
}
