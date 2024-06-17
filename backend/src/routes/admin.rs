use axum::{
    routing::get,
    Router,
};

use crate::controllers;

pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async { controllers::admin::welcome::index().await } ))
}
