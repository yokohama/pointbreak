pub mod application;
pub mod admin;
pub mod user;

use axum::Router;
use tower_http::trace::TraceLayer;
use crate::services;
use crate::errors;

pub fn get() -> Router {
    Router::new()
        .nest("/", application::router())
        .nest("/user", user::router())
        .nest("/admin", admin::router())
        .layer(TraceLayer::new_for_http())
        .with_state(services::db::get_db_pool())
        .fallback(errors::handler_404)
}
