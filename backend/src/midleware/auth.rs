use std::fmt::Display;
use chrono::NaiveDateTime;
use bcrypt::{hash, verify, DEFAULT_COST};

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts, RequestPartsExt,
};

use axum_extra::{
    headers::{
        authorization::Bearer, 
        Authorization
    },
    TypedHeader,
};

use jsonwebtoken::{
    encode,
    decode,
    EncodingKey,
    DecodingKey,
    Header,
    Validation,
};

use sqlx::{ PgPool, FromRow };

use tracing::error;
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;

use crate::{
    midleware::error, 
    models::user
};

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap();
    Keys::new(secret.as_bytes())
});

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    created_at: NaiveDateTime,
    exp: usize,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UUID: {}\n", self.sub)
    }
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = error::AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|e| { 
                error!("{:#?}", e.to_string());
                error::AppError::MissingCredentials("Wrong token.".to_string())
            })?;

        let token_data = decode::<Claims>(
            bearer.token(), 
            &KEYS.decoding, 
            &Validation::default()
        )
            .map_err(|e| {
                error!("{:#?}", e.to_string());
                let msg = format!("Token: {} was missing.", bearer.token());
                error::AppError::MissingCredentials(msg.to_string())
            })?;

        Ok(token_data.claims)
    }
}

impl Claims {
    pub async fn get_current_user(
        &self, 
        pool: &PgPool
    ) -> Result<user::CurrentUser, error::AppError> {
        let user_id: i32 = self.sub.parse::<i32>().map_err(|e| {
            error!("Failed to parse sub to i32: {:#?}", e);
            error::AppError::InternalServerError(e.to_string())
        })?;
        let current_user = user::CurrentUser::new(&pool, user_id).await?;
        Ok(current_user)
    }
}

#[derive(FromRow)]
struct AuthUser {
    id: i32,
    encrypted_password: String,
    created_at: NaiveDateTime,
}

pub async fn authorize(
    pool: &PgPool, 
    payload: AuthPayload,
    is_admin: bool,
) -> Result<AuthBody, error::AppError> {

    let sql = if is_admin {
        r#"
            SELECT id, encrypted_password, created_at FROM users
            WHERE email = $1 AND is_admin = TRUE
        "#
    } else {
        r#"
            SELECT id, encrypted_password, created_at FROM users
            WHERE email = $1
        "#
    };

    let auth_user = sqlx::query_as::<_, AuthUser>(sql)
        .bind(&payload.email)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                error!("{:#?}", e);
                let msg = format!("{} was not found.", &payload.email);
                error::AppError::UnAuthorized(msg)
            },
            _ => {
                error!("{:#?}", e);
                error::AppError::InternalServerError(e.to_string())
            }
        })?;

    let is_valid = verify_secret(
        &payload.password, 
        auth_user.encrypted_password.as_str()
    )
    .map_err(|e| {
        error!("{:#?}", e);
        error::AppError::InternalServerError(e.to_string())
    })?;

    if is_valid {
        let claim = Claims {
            sub: auth_user.id.to_string(),
            created_at: auth_user.created_at,
            exp: 2000000000, 
        };
        let token = encode(&Header::default(), &claim, &KEYS.encoding)
            .map_err(|e| {
                error!("{:#?}", e);
                error::AppError::InternalServerError(e.to_string())
            })?;
        Ok(AuthBody::new(token))
    } else {
        let msg = "missing credentials.".to_string();
        error!("{}", msg);
        Err(error::AppError::MissingCredentials(msg))
    }
}

pub fn hash_secret(secret: &str) -> Result<String, bcrypt::BcryptError> {
    hash(secret, DEFAULT_COST)
}

fn verify_secret(
    secret: &str, 
    hashed_secret: &str
) -> Result<bool, bcrypt::BcryptError> {
    verify(secret, hashed_secret)
}
