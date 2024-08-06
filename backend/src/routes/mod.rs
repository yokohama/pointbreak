use axum::{
    Router,
    routing::{ get, post },
};

use sqlx::postgres::PgPool;

use crate::controllers::{
    welcome,
    admin,
    user,
};
use crate::middleware::error;

pub fn get_routing(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(welcome::show))

        .route("/admin/session", post(admin::session::create))
        .route("/admin/dashboard", get(admin::dashboard::show))
        .route("/admin/users", get(admin::users::index))

        .route("/user/registration", post(user::registration::create))
        .route("/user/session", post(user::session::create))
        .route("/user/dashboard", get(user::dashboard::show))
        .route("/user/point_conditions", get(user::point_conditions::index))
        .route("/user/point_conditions", post(user::point_conditions::create))

        .with_state(pool)
        .fallback(error::not_found)
}
