use sqlx::{PgPool, query};

use pointbreak::middleware;
use pointbreak::models;

#[tokio::main]
async fn main() {
    let db_pool = middleware::db::get_db_pool().await;
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
            time: "2024-07-20".to_string(),
            swell_wave_direction: 0,
            swell_wave_direction_unit: "°".to_string(),
            swell_wave_height: 0.0,
            swell_wave_height_unit: "m".to_string(),
            rain: 0.0,
            rain_unit: "mm".to_string(),
            temperature: 32.0,
            temperature_unit: "℃".to_string(),
            weather_code: 1,
            weather_code_unit: "wmocode".to_string(),
            wind_speed: 3.1,
            wind_speed_unit: "km/h".to_string(),
        },
        models::point_condition::New {
            user_id: 1,
            lat: 34.052335, 
            lon: -118.243682,
            time: "2024-07-20".to_string(),
            swell_wave_direction: 0,
            swell_wave_direction_unit: "°".to_string(),
            swell_wave_height: 0.0,
            swell_wave_height_unit: "m".to_string(),
            rain: 80.0,
            rain_unit: "mm".to_string(),
            temperature: 29.0,
            temperature_unit: "℃".to_string(),
            weather_code: 2,
            weather_code_unit: "wmocode".to_string(),
            wind_speed: 0.1,
            wind_speed_unit: "km/h".to_string(),
        },
        models::point_condition::New {
            user_id: 1,
            lat: 34.052435, 
            lon: -118.243680,
            time: "2024-07-20".to_string(),
            swell_wave_direction: 0,
            swell_wave_direction_unit: "°".to_string(),
            swell_wave_height: 0.0,
            swell_wave_height_unit: "m".to_string(),
            rain: 10.0,
            rain_unit: "mm".to_string(),
            temperature: 30.0,
            temperature_unit: "℃".to_string(),
            weather_code: 3,
            weather_code_unit: "wmocode".to_string(),
            wind_speed: 7.0,
            wind_speed_unit: "km/h".to_string(),
        },
    ];
    for condition in conditions {
        let _ = models::point_condition::create(pool, condition).await;
    }
}
