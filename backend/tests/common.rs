use reqwest::{ 
    Client,
    StatusCode
};
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

pub struct Curl<'a> {
    method: String,
    url: String,
    data: &'a Option<serde_json::Value>,
    jwt: &'a Option<String>,
}

impl<'a> Curl<'a> {
    pub fn new(
        method: String, 
        url: String, 
        data: &'a Option<serde_json::Value>, 
        jwt: &'a Option<String>,
    ) -> Self {
        Self {
            method,
            url,
            data,
            jwt,
        }
    }

    pub fn make(&self) {
        let mut command = format!(
            "curl -X {} '{}' -H 'Content-Type: application/json'", 
            self.method,
            self.url
        );

        if let Some(ref token) = self.jwt {
            command = format!(
                "{} -H 'Authorization: Bearer {}'", 
                command, 
                token
            );
        }

        if let Some(ref data) = self.data {
            let json_string = serde_json::to_string(data)
                .unwrap_or_else(|_| "{}".to_string());
            command = format!(
                "{} -d '{}'", 
                command, 
                json_string
            );
        }

        println!("");
        println!("#--- Curl");
        println!("{}", command);
        println!("");
    }
}

pub async fn make_res(res: reqwest::Response) -> (StatusCode, String) {
    let status = res.status();
    let body = res.text().await.expect("Failed to read response body");

    println!("#--- Response");
    println!("{:#?}", body);
    println!("");

    (status, body)
}
