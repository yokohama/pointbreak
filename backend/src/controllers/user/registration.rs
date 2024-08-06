use axum::{
    extract::State,
    response::Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    middleware::error,
    models::user,
    requests::{
        user::NewRegistration,
        validations::JsonValidatedForm,
    },
};

pub async fn create(
    State(pool): State<PgPool>,
    validated_form: JsonValidatedForm<NewRegistration>,
) -> Result<Json<impl Serialize>, error::AppError> {
    let new_user = user::New {
        email: validated_form.0.email,
        password: validated_form.0.password,
    };
    let create_user = user::create(&pool, new_user).await?;
    Ok(Json(create_user))
}
