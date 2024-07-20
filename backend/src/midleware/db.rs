use std::env;
use std::time::Duration;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn get_db_pool() -> PgPool {
    let db_connection_str = env::var("DATABASE_URL")
        .expect("not found database url.");

    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database!")
}
