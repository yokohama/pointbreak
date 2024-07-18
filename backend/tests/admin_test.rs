mod common;

use reqwest::Client;
use tokio;

const AUTH_URL: &str = "http://localhost:3000/admin/session";
const EMAIL: &str = "hoge5@example.com";
const PASSWORD: &str = "passpass";

#[tokio::test]
async fn dashboard() {
    let url = "http://localhost:3000/admin/dashboard";

    let client = Client::new();
    let jwt = common::get_jwt(&client, AUTH_URL, EMAIL, PASSWORD).await;

    let res = client
        .get(url)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn users() {
    let url = "http://localhost:3000/admin/users";

    let client = Client::new();
    let jwt = common::get_jwt(&client, AUTH_URL, EMAIL, PASSWORD).await;

    let res = client
        .get(url)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await
        .expect("Failed to send request");

    let users: Vec<serde_json::Value> = res
        .json()
        .await
        .expect("Failed to parse JSON");

     assert_eq!(users.len(), 5);
}
