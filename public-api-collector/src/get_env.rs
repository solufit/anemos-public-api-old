use std::env;
use once_cell::sync::Lazy;

pub static API_URL: Lazy<String> = Lazy::new(|| env::var("API_URL").unwrap());
pub static PUBLIC_API_REDIS_HOST: Lazy<String> = Lazy::new(|| env::var("PUBLIC_API_REDIS_HOST").unwrap());
