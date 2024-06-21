use axum::Router;

use tracing::info;
use tower_http::trace::TraceLayer;

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

    let app = Router::new()
        .nest("/", routes::application::router())
        .nest("/user", routes::user::router())
        .nest("/admin", routes::admin::router())
        .layer(TraceLayer::new_for_http())
        .with_state(services::db::get_db_pool())
        .fallback(errors::handler_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
