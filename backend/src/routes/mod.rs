use axum::{
    Router,
    routing::{ get, post },
};

use sqlx::postgres::PgPool;

use crate::controllers::{
    welcome,
    admin,
    user
};
use crate::midleware::error;

pub fn get_routing(pool: PgPool) -> Router {
    Router::new().route("/", get(welcome::index))
        .route("/admin/session/new", post(admin::authorization))
        .route("/admin/dashboard", get(admin::dashboard))
        .route("/admin/users", get(admin::users))

        .route("/user/registration", post(user::registration))
        .route("/user/session/new", post(user::authorization))
        .route("/user/dashboard", get(user::dashboard))
        .route("/user/point_conditions", post(user::create_point_conditions))
        .route("/user/point_conditions", get(user::point_conditions))

        .with_state(pool)
        .fallback(error::not_found)
}
