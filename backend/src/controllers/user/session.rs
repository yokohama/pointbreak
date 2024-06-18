use axum::Json; 
use jsonwebtoken::{
    encode, 
    Header, 
};
use crate::services::authorization::jwt::{
    KEYS,
    Claims,
};
use crate::services::authorization::auth::{
    AuthPayload,
    AuthBody,
};
use crate::errors::AppError;

pub async fn create(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AppError> {
    // 認証
    /*
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AppError::MissingCredentials("".to_string()));
    }
    */
    if payload.client_id != "foo" || payload.client_secret != "bar" {
        return Err(AppError::WrongCredentials("missing client_id or client_secret".to_string()));
    }

    /*
     * 上記`foo`と`bar`で認証成功したユーザー情報を
     * ここでDBから持ってきてclaimsにセットする。
     */
    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };

    // claimsの内容を使用してトークン生成
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|e| AppError::TokenCreation(e.to_string()))?;

    Ok(Json(AuthBody::new(token)))
}
