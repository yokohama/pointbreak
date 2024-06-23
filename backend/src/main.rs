use std::env;
use tracing::{debug, info};

mod config;
mod routes;
mod controllers;
mod services;
mod models;
mod errors;

#[tokio::main]
async fn main() {
    config::logging::app_log_tracing();

    info!("#### start application 2 ####");
    print_env();

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .unwrap(),
        routes::get()
    ).await.unwrap();
}

fn print_env() {
    let env_keys = vec![
        "RUST_BACKTRACE",
        "RUST_LOG",
        "DATABASE_URL",
        "JWT_SECRET",
    ];

    for key in env_keys {
        match env::var(key) {
            Ok(value) => debug!("{}: {}", key, value),
            Err(_) => debug!("None! {}", key),
        }
    }
}
