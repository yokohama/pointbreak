use chrono::Local;

use reqwest::Client;
use reqwest::Response;
use tokio;

use pointbreak::requests;

use crate::common;

const AUTH_URL: &str = "http://localhost:3000/user/session";
const EMAIL: &str = "hoge1@example.com";
const PASSWORD: &str = "passpass";

const NEW_USER_EMAIL: &str = "hoge100@example.com";
const NEW_USER_PASSWORD: &str = "XYz1234&&&";

#[tokio::test]
async fn registration_and_dashboard() {
    let url = "http://localhost:3000/user/registration";
    let client = Client::new();

    let data = serde_json::json!({
        "email": NEW_USER_EMAIL,
        "password": NEW_USER_PASSWORD,
    });

    common::Curl::new(
        "POST".to_string(), 
        url.to_string(), 
        &Some(data.clone()),
        &None,
    ).make();

    let res = client
        .post(url)
        .json(&data)
        .send()
        .await
        .expect("Failed to send request");

    let (status, body) = common::make_res(res).await;
    assert_eq!(status, 200);

    let user: serde_json::Value = serde_json::from_str(&body)
        .expect("Failed to parse JSON");

    assert_eq!(user["email"], NEW_USER_EMAIL);

    let res = request_dashboard(&client, NEW_USER_EMAIL, NEW_USER_PASSWORD)
        .await;

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

    let today = Local::now();
    let today = today.format("%Y-%m-%d");

    let new_condition_req = requests::point_condition::New {
        lat: 35.3741,
        lon: 140.3708,
        start_date: today.to_string(),
        end_date: today.to_string(),
        timezone: "Asia/Tokyo".to_string(),
    };

    common::Curl::new(
        "POST".to_string(), 
        url.to_string(), 
        &None,
        &Some(jwt.clone())
    ).make();

    let res = client
        .post(url)
        .json(&new_condition_req)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await
        .expect("Failed to send request");

    let (status, body) = common::make_res(res).await;
    assert_eq!(status, 200);

    let created: serde_json::Value = serde_json::from_str(&body)
        .expect("Failed to parse JSON");

    assert_eq!("35.3741", created["lat"].to_string());
    assert_eq!("140.3708", created["lon"].to_string());

    common::Curl::new(
        "POST".to_string(), 
        url.to_string(), 
        &None,
        &Some(jwt.clone())
    ).make();

    let res = client
        .get(url)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await
        .expect("Failed to send request");

    let (status, body) = common::make_res(res).await;
    assert_eq!(status, 200);

    let conditions: Vec<serde_json::Value> = serde_json::from_str(&body)
        .expect("Failed to parse JSON");

    assert_eq!(conditions.len(), 4);
}

async fn request_dashboard(
    client: &Client, 
    email: &str, 
    password: &str
) -> Response {
    let url = "http://localhost:3000/user/dashboard";
    let jwt = common::get_jwt(&client, AUTH_URL, email, password)
        .await;

    client
        .get(url)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await
        .expect("Failed to send request")
}
