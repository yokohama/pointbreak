use chrono::NaiveDateTime;

use serde::Serialize;
use sqlx::{
    PgPool,
    query_as,
    FromRow,
};

use tracing::error;

use crate::midleware::{auth, error};

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct NewUser {
    pub email: String,
    pub password: String,
}

#[derive(FromRow)]
struct UserExists {
    email: String,
}

#[derive(Serialize, FromRow)]
pub struct CreatedUser {
    pub id: i32,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct CurrentUser {
    pub id: i32,
    pub name: Option<String>,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
impl CurrentUser {
    pub async fn new(
        pool: &PgPool, 
        id: i32
    ) -> Result<Self, error::AppError> {
        let user = find_by_id(pool, id).await?;

        Ok(Self {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }
}

pub async fn find_by_id(
    pool: &PgPool, 
    id: i32
) -> Result<User, error::AppError> {
    let sql = r#"
        SELECT id, name, email, created_at, updated_at FROM users
        WHERE id = $1
    "#;
    let user: User = query_as::<_, User>(sql)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            error!("{:#?}", e);
            error::AppError::DatabaseError(e.to_string())
        })?;
    Ok(user)
}

pub async fn all(pool: &PgPool) -> Result<Vec<User>, error::AppError> {
    let sql = r#"
        SELECT id, name, email, created_at, updated_at FROM users
    "#;
    let users: Vec<User> = query_as::<_, User>(sql)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            error!("{:#?}", e);
            error::AppError::DatabaseError(e.to_string())
        })?;

    Ok(users)
}

pub async fn create(
    pool: &PgPool, 
    new_user: NewUser
) -> Result<CreatedUser, error::AppError> {
    let sql = r#"
    SELECT email
    FROM users
    WHERE email = $1
    "#;

    let user_exists = query_as::<_, UserExists>(sql)
    .bind(&new_user.email)
    .fetch_one(pool)
    .await;

    match user_exists {
        Ok(user) => {
            let msg = format!(
                "User already exists. email: {}", 
                user.email
            );
            error!("{}", msg);
            return Err(error::AppError::ConflictError(msg.to_string()));
        },
        Err(sqlx::Error::RowNotFound) => {
            let sql = r#"
            INSERT INTO users (
                email, 
                encrypted_password, 
                created_at, 
                updated_at
            )
            VALUES ($1, $2, NOW(), NOW())
            RETURNING id, email, created_at
            "#;
            
            let encrypted_password = auth::hash_secret(&new_user.password)
                .map_err(|e| {
                    error!("{:#?}", e);
                    error::AppError::InternalServerError(e.to_string())
                })?;

            let created_user = query_as::<_, CreatedUser>(sql)
                .bind(&new_user.email)
                .bind(encrypted_password)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    error!("{:#?}", e);
                    error::AppError::DatabaseError(e.to_string())
                })?;

            Ok(created_user)
        },
        Err(e) => {
            error!("{:#?}", e);
            return Err(error::AppError::DatabaseError(e.to_string()));
        }
    }
}
