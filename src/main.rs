mod bloc;
mod bot;
mod commands;
mod cron;
mod datamuse;
mod format;
mod inlines;
mod networking;
mod server;
mod service;
mod stands4;
mod urban;
mod wordle;

use crate::service::telegram::TelegramService;
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Deserialize)]
struct Config {
    #[serde(rename = "ADMIN_CHAT_ID")]
    admin_chat: i64,
    #[serde(rename = "TELOXIDE_TOKEN")]
    teloxide_token: String,
    #[serde(rename = "STANDS4_USER_ID")]
    stands4_user_id: String,
    #[serde(rename = "STANDS4_TOKEN")]
    stands4_token: String,
}

/// Program entry point that initializes logging, loads configuration from `Secrets.toml`,
/// constructs required services, and binds the Telegram service to 127.0.0.1:8080.
///
/// On success the Telegram service is bound and the function returns normally; failures from
/// reading the file, parsing the configuration, or binding the service are propagated.
///
/// # Examples
///
/// ```no_run
/// // Run the async `main` from a synchronous context.
/// let rt = tokio::runtime::Runtime::new().unwrap();
/// rt.block_on(crate::main()).unwrap();
/// ```
///
/// # Returns
///
/// `Ok(())` if the service binds successfully, `Err` if configuration loading, parsing, or binding fails.
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let config_str = std::fs::read_to_string("Secrets.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let addr = SocketAddr::new(IpAddr::V4(ip), 8080);
    let service = TelegramService::new(config);
    service.bind(addr).await
}
