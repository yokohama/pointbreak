use axum::{
    routing::{get, post},
    Router,
};

use diesel::pg::PgConnection;
use deadpool_diesel::Manager;
use deadpool_diesel::Pool;

use crate::controllers::user;

//pub fn router() -> Router {
pub fn router() -> Router<Pool<Manager<PgConnection>>> {
    Router::new()
        .route("/", get(|| async { user::welcome::index().await }))
        .route("/session", post(user::session::create))
        .route("/dashboard", get(user::dashboard::index))
}
