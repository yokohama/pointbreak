use sqlx::{PgPool, query};

use pointbreak::midleware;
use pointbreak::models;

#[tokio::main]
async fn main() {
    let db_pool = midleware::db::get_db_pool().await;
    reset_database(&db_pool).await;
    run_migrations(&db_pool).await;
    run_seeds(&db_pool).await;
}

async fn reset_database(pool: &PgPool) {
    query("DROP SCHEMA public CASCADE")
        .execute(pool)
        .await
        .expect("Error dropping schema");

    query("CREATE SCHEMA public")
        .execute(pool)
        .await
        .expect("Error creating schema");
}

async fn run_migrations(pool: &PgPool) {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Error running migrations");
}

async fn run_seeds(pool: &PgPool) {
    for i in 1..=5 {
        let email = format!("hoge{}@example.com", i);
        let new_user = models::user::New {
            email,
            password: "passpass".to_string(),
        };
    
        let _ = models::user::create(pool, new_user).await;
    }

    // Update admin hoge5@example.com
    let sql = r#"
        UPDATE users set is_admin = TRUE
        WHERE email = 'hoge5@example.com'
    "#;
    query(sql).execute(pool).await.expect("error. update is_admin.");

    let conditions = vec![
        models::point_condition::New {
            user_id: 1,
            lat: 34.052235, 
            lon: -118.243683,
        },
        models::point_condition::New {
            user_id: 1,
            lat: 34.052335, 
            lon: -118.243682,
        },
        models::point_condition::New {
            user_id: 1,
            lat: 34.052435, 
            lon: -118.243680,
        },
    ];
    for condition in conditions {
        let _ = models::point_condition::create(pool, condition).await;
    }
}
