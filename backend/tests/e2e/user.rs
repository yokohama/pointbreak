use pointbreak::models;
use reqwest::Client;
use reqwest::Response;
use tokio;

use crate::common;

const AUTH_URL: &str = "http://localhost:3000/user/session";
const EMAIL: &str = "hoge1@example.com";
const PASSWORD: &str = "passpass";

const NEW_USER_EMAIL: &str = "hoge100@example.com";

#[tokio::test]
async fn registration_and_dashboard() {
    let url = "http://localhost:3000/user/registration";
    let client = Client::new();

    let data = serde_json::json!({
        "email": NEW_USER_EMAIL,
        "password": PASSWORD,
    });

    let res = client
        .post(url)
        .json(&data)
        .send()
        .await
        .expect("Failed to send request");

    let status = res.status();

    let user: serde_json::Value = res
        .json()
        .await
        .expect("Failed to parse JSON");

    assert_eq!(status, 200);
    assert_eq!(user["email"], NEW_USER_EMAIL);

    let client = Client::new();
    let res = request_dashboard(&client, NEW_USER_EMAIL, PASSWORD).await;

    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn dashboard() {
    let client = Client::new();
    let res = request_dashboard(&client, EMAIL, PASSWORD).await;

    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn point_conditions() {
    let url = "http://localhost:3000/user/point_conditions";
    let client = Client::new();
    let jwt = common::get_jwt(&client, AUTH_URL, EMAIL, PASSWORD).await;

    let new_condition = models::point_condition::New {
        user_id: 1,
        lat: 0.0,
        lon: 0.0,
    };

    let res = client
        .post(url)
        .json(&new_condition)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await
        .expect("Failed to send request");

    let status = res.status();
    assert_eq!(status, 200);

    let res = client
        .get(url)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await
        .expect("Failed to send request");

    let status = res.status();
    assert_eq!(status, 200);

    let conditions: Vec<serde_json::Value> = res
        .json()
        .await
        .expect("Json perse error.");

    assert_eq!(conditions.len(), 4);
}

async fn request_dashboard(
    client: &Client, 
    email: &str, 
    password: &str
) -> Response {
    let url = "http://localhost:3000/user/dashboard";
    let jwt = common::get_jwt(&client, AUTH_URL, email, password).await;

    client
        .get(url)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await
        .expect("Failed to send request")
}
