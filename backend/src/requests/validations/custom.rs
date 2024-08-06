use validator::ValidationError;

/*
 * パスワードのバリデーション
 */
pub fn password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("Password must be at least 8 characters long"));
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_number = password.chars().any(|c| c.is_numeric());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

    if has_uppercase && has_lowercase && has_number && has_symbol {
        Ok(())
    } else {
        Err(ValidationError::new("Password must include at least one uppercase letter, one lowercase letter, one number, and one symbol"))
    }
}

/*
 * メアドのバリデーション
 */
pub fn email(email: &str) -> Result<(), ValidationError> {
    let email_regex = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    if email_regex.is_match(email) {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid email format"))
    }
}
