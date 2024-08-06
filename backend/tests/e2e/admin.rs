use reqwest::Client;
use tokio;

use crate::common;

const AUTH_URL: &str = "http://localhost:3000/admin/session";
const EMAIL: &str = "hoge5@example.com";
const PASSWORD: &str = "passpass";

#[tokio::test]
async fn dashboard() {
    let url = "http://localhost:3000/admin/dashboard";

    let client = Client::new();
    let jwt = common::get_jwt(&client, AUTH_URL, EMAIL, PASSWORD).await;

    common::Curl::new(
        "GET".to_string(), 
        url.to_string(), 
        &None,
        &Some(jwt.clone()),
    ).make();

    let res = client
        .get(url)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await
        .expect("Failed to send request");

    let (status, _body) = common::make_res(res).await;
    assert_eq!(status, 200);
}

#[tokio::test]
async fn users() {
    let url = "http://localhost:3000/admin/users";

    let client = Client::new();
    let jwt = common::get_jwt(&client, AUTH_URL, EMAIL, PASSWORD).await;

    common::Curl::new(
        "GET".to_string(), 
        url.to_string(), 
        &None,
        &Some(jwt.clone()),
    ).make();

    let res = client
        .get(url)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await
        .expect("Failed to send request");

    let (status, body) = common::make_res(res).await;
    assert_eq!(status, 200);

    let users: Vec<serde_json::Value> = serde_json::from_str(&body)
        .expect("Failed to parse JSON");

     assert_eq!(users.len(), 5);
}
