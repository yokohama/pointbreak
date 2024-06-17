use axum::Router;

use tracing::info;

mod config;
mod routes;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    config::logging::app_log_tracing();

    info!("#### start application ####");

    let connection = &mut establish_connection();

    let app = Router::new()
        .nest("/", routes::application::router())
        .nest("/user", routes::user::router())
        .nest("/admin", routes::admin::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
