use axum::{
    routing::get,
    Router,
};

use diesel::pg::PgConnection;
use deadpool_diesel::Manager;
use deadpool_diesel::Pool;

use crate::controllers;

//pub fn router() -> Router {
pub fn router() -> Router<Pool<Manager<PgConnection>>> {
    Router::new()
        .route("/", get(|| async { controllers::application::index().await }))
}
