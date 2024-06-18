use diesel::pg::PgConnection;
use deadpool_diesel::Manager;
use deadpool_diesel::Pool;

use std::env;

pub fn get_db_pool() -> Pool<Manager<PgConnection>> {
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = deadpool_diesel::postgres::Manager::new(
        db_url.clone(), 
        deadpool_diesel::Runtime::Tokio1
    );

    println!("{}", db_url);

    deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
