use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct NewRegistration {
    #[validate(length(min = 1, message = "Cannot be empty"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}
