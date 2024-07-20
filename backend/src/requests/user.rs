use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewRegistration {
    pub email: String,
    pub password: String,
}
