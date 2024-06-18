use axum::{
    extract::State,
    response::Json,
};
use diesel::prelude::*;

use tracing::error;

use pointbreak::schema;
use crate::models::User;
use crate::services::authorization::jwt::Claims;
use crate::errors::AppError;

pub async fn index(_claims: Claims,
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<User>>, AppError> {
    let conn = pool.get()
        .await
        .map_err(|e| { 
            error!("{:#?}", e);
            AppError::InternalServerError(e.to_string())
        })?;
    let res = conn
        .interact(|conn|
            schema::users::table
                .select(User::as_select())
                .load(conn)
        )
        .await
        .map_err(|e| {
            error!("{:#?}", e);
            AppError::InternalServerError(e.to_string())
        })?
        .map_err(|e| { 
            error!("{:#?}", e);
            AppError::InternalServerError(e.to_string())
        })?;

    Ok(Json(res))
}
