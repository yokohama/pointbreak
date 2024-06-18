use std::fmt::Display;
use std::env;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::{
    headers::{
        authorization::Bearer, 
        Authorization
    },
    TypedHeader,
};
use jsonwebtoken::{
    decode, 
    DecodingKey, 
    EncodingKey, 
    Validation
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use tracing::debug;

use crate::errors::AppError;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = env::var("JWT_SECRET").expect("`JWT_SECRET` must be set");
    Keys::new(secret.as_bytes())
});

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {

        //debug!("{:#?}", parts);

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|e| AppError::InvalidToken(e.to_string()))?;

        // ここでトークンをデコードして検証
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|e| AppError::InvalidToken(e.to_string()))?;

        Ok(token_data.claims)
    }
}

pub struct Keys {
    // jwtの発行のデジタル署名に使用
    pub encoding: EncodingKey,
    // jwtトークンが正しい署名かの検証に使用
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
