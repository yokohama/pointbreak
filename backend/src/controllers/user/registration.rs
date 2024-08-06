use axum::{
    extract::State,
    response::Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    middleware::error,
    models::user,
    requests,
};

pub async fn create(
    State(pool): State<PgPool>,
    validated_form: requests::JsonValidatedForm<requests::user::NewRegistration>,
) -> Result<Json<impl Serialize>, error::AppError> {
    let new_user = user::New {
        email: validated_form.0.email,
        password: validated_form.0.password,
    };
    let create_user = user::create(&pool, new_user).await?;
    Ok(Json(create_user))
}
