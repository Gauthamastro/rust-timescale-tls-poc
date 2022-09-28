#![feature(async_closure)]
use std::sync::Arc;
use governor::{Quota, RateLimiter};
use nonzero_ext::*;

#[tokio::main]
async fn main() {
    env_logger::init();
    let bucket = Arc::new(RateLimiter::direct(Quota::per_second(nonzero!(200u32))));
    log::info!(target:"generator","Starting generator...");


    loop {
        let bucket = Arc::clone(&bucket);
        tokio::spawn(async move {
            match bucket.check() {
                Ok(()) => {
                    // Create new order
                    log::info!(target:"generator","Creating new order...");
                }
                Err(err) => {
                    // Don't create new order here.
                    // log::error!(target:"generator", "Not creating new order.");
                }
            }
        });
    }

}

