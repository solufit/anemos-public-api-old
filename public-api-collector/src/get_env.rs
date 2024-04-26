use std::env;
use once_cell::sync::Lazy;

pub static API_URL: Lazy<String> = Lazy::new(|| env::var("API_URL").unwrap());
