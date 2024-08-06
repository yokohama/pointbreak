use serde::Deserialize;
use validator::Validate;

use crate::requests::validations;

#[derive(Deserialize, Validate)]
pub struct NewRegistration {
    #[validate(custom = "validations::custom::email")]
    pub email: String,

    #[validate(custom = "validations::custom::password")]
    pub password: String,
}
