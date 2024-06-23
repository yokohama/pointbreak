use tracing::info;

mod config;
mod routes;
mod controllers;
mod services;
mod models;
mod errors;

#[tokio::main]
async fn main() {
    config::logging::app_log_tracing();

    info!("#### start application ####");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .unwrap(),
        routes::get()
    ).await.unwrap();
}
