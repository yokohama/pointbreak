use axum::{
    routing::get,
    Router,
};

use crate::controller;

pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async { controller::admin::index().await } ))
}
