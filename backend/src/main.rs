use tracing::info;
use tower_http::trace::TraceLayer;

mod routes;
mod midleware;
mod controllers;
mod models;

#[tokio::main]
async fn main() {
    midleware::log::app_log_tracing();

    info!("#### start application ####");
    midleware::env::check_env();

    let db_pool = midleware::db::get_db_pool().await;
    let router = routes::get_routing(db_pool);
    let app = router.layer(TraceLayer::new_for_http());

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app
    ).await.unwrap();
}
