/*
 * 絶対に値が必要なキー
 * これがないとpanicにしてappの起動を止める
 */
use std::env;
use tracing::{debug};

pub fn check_env() {
    let env_keys = vec![
        "RUST_BACKTRACE",
        "DATABASE_URL",
        "JWT_SECRET",
    ];

    for key in env_keys {
        match env::var(key) {
            Ok(value) if !value.is_empty() => {
                debug!("Loaded env: {} = {}", key, value);
            },
            Ok(_) => {
                panic!("env {} is required but empty", key);
            },
            Err(_) => {
                panic!("env {} is required but missing", key);
            }
        }
    }
}
