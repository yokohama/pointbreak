use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, 
    RequestPartsExt,
    Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{
    decode, 
    encode, 
    DecodingKey, 
    EncodingKey, 
    Header, 
    Validation
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;

use tracing_subscriber::{
    fmt, 
    EnvFilter, 
    layer::SubscriberExt, 
    util::SubscriberInitExt
};
use tracing_appender::rolling;
use tracing::debug;
use std::io;

mod routes;
mod controller;

// 環境変数`JWT_SECRET`から、秘密キーを読み込み
// 署名用キー(encode)のオブジェクトと、
// 検証用キー(decode)のオブジェクトを
// シングルトン(Lazy)で生成。
static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("`JWT_SECRET` must be set");
    Keys::new(secret.as_bytes())
});

#[tokio::main]
async fn main() {
    app_log_tracing();

    debug!("#### start application ####");

    let app = Router::new()
        .route("/authorize", post(authorize))
        /*
         * (疑問)
         * なぜ、`protected`や`authorize`メソッドを読んでいるのに、
         * メソッドに定義されている引数を渡さなくても大丈夫なのか？
         *
         * (解答)
         * axumの仕様。axumは関数のシグネチャから必要な引数を自動的に解析
         * HTTPリクエストから引数に対応するデータを抽出する
         * `protected`メソッドの場合、引数にClaim型が宣言されている。
         * Claim型がFromRequestトレイトを実装しているので、axumがよしなにやっている。
         *
         * (axumよしなの詳細フロー)
         * 1. `protected`メソッドを探す
         * 2. `protected`メソッドのシグネチャを確認 > 引数にClaimsがある
         * 3. Claims構造体には、`FromRequestRarts`トレイトの実装がある
         * 4. 自動的に、Claimsの`from_request_parts`を実行する
         * 5. その際、引数の`Parts`には、リクエストの内容が渡される(axumのよしな)
         */
        .route("/protected", get(protected))
        .nest("/", routes::application::router())
        .nest("/users", routes::user::router())
        .nest("/admins", routes::admin::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn protected(claims: Claims) -> Result<String, AuthError> {
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{claims}",
    ))
}

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    // 認証
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    if payload.client_id != "foo" || payload.client_secret != "bar" {
        return Err(AuthError::WrongCredentials);
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
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}
#[derive(Debug)]
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
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
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {

        debug!("{:#?}", parts);

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        // ここでトークンをデコードして検証
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

struct Keys {
    // jwtの発行のデジタル署名に使用
    encoding: EncodingKey,
    // jwtトークンが正しい署名かの検証に使用
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

#[derive(Debug, Deserialize)]
struct AuthPayload {
    client_id: String,
    client_secret: String,
}

// BUG: ログファイルに書き込まれない
// $ RUST_LOG=debug cargo run
fn app_log_tracing() {
    let file_appender = rolling::daily("./logs", "prefix.log");
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(file_appender);

    // 標準出力用レイヤー
    let stdout_layer = fmt::layer()
        .with_writer(io::stdout);

    // ファイル出力用レイヤー
    let file_layer = fmt::layer()
        .with_writer(non_blocking_writer);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer);

    subscriber.init();
}
