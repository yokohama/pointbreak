use axum::{
    routing::{get, post},
    Router,
};

use crate::controllers::user;

pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async { user::welcome::index().await }))
        .route("/session", post(user::session::create))
        .route("/dashboard", get(user::dashboard::index))
}
