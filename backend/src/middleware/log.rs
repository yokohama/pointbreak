use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_appender::rolling;
use std::io;

static LOG_DIR: &str = "./logs";
static LOG_FILE_NAME: &str = "app.log";

// BUG: ログファイルに書き込まれない
pub fn app_log_tracing() {
    let file_appender = rolling::daily(LOG_DIR, LOG_FILE_NAME);
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(file_appender);
  
    let stdout_layer = fmt::layer().with_writer(io::stdout);
    let file_layer = fmt::layer().with_writer(non_blocking_writer);
  
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
  
    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer);

    //subscriber.init();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
