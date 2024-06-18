use axum::{
    http::StatusCode,
    Router,
    response::IntoResponse,
};

use tracing::info;

use dotenvy::dotenv;

mod config;
mod routes;
mod controllers;
mod services;
mod models;
mod errors;

#[tokio::main]
async fn main() {
    config::logging::app_log_tracing();

    dotenv().ok();

    info!("#### start application ####");

    let pool = services::db::get_db_pool();

    let app = Router::new()
        .nest("/", routes::application::router())
        .nest("/user", routes::user::router())
        .nest("/admin", routes::admin::router())
        .with_state(pool);

    let app = app.fallback(handler_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

