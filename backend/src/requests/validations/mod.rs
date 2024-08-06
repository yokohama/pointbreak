pub mod custom;

use axum::{
    async_trait,
    extract::{
        Request,
        rejection::JsonRejection,
        FromRequest
    },
    response::Json,
};
use validator::Validate;
use serde::de::DeserializeOwned;

use tracing::info;

use crate::middleware::error;

#[derive(Debug, Clone, Default)]
pub struct JsonValidatedForm<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for JsonValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = error::AppError;

    async fn from_request(
        req: Request, 
        state: &S
    ) -> Result<Self, Self::Rejection> {

        let result = Json::<T>::from_request(req, state).await;
        match result {
            Ok(Json(value)) => {
                if let Err(e) = value.validate() {
                    info!("{:#?}", e);
                    //TODO: このfromはどこで定義されているのか？
                    Err(error::AppError::from(e))
                } else {
                    Ok(Self(value))
                }
            },
            Err(e) => {
                info!("{:#?}", e);
                Err(error::AppError::from(e))
            }
        }
    }
}
