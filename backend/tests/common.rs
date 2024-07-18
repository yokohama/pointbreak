use reqwest::Client;
use serde_json::json;

pub async fn get_jwt(
    client: &Client, 
    auth_url: &str,
    email: &str,
    password: &str,
) -> String {
    let auth_data = json!({
        "email": email,
        "password": password,
    });

    let res = client
        .post(auth_url)
        .json(&auth_data)
        .send()
        .await
        .expect("Failed to send request")
        .json::<serde_json::Value>()
        .await
        .expect("Failed to parse JSON");

    res["access_token"]
        .as_str()
        .expect("Failed to get JWT")
        .to_string()
}
