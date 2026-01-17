mod bloc;
mod bot;
mod commands;
mod cron;
mod datamuse;
mod format;
mod inlines;
mod server;
mod service;
mod stands4;
mod urban;
mod wordle;

use crate::service::telegram::TelegramService;
use serde::Deserialize;
use stands4::client::Stands4Client;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Deserialize)]
struct Config {
    #[serde(rename = "TELOXIDE_TOKEN")]
    teloxide_token: String,
    #[serde(rename = "STANDS4_USER_ID")]
    stands4_user_id: String,
    #[serde(rename = "STANDS4_TOKEN")]
    stands4_token: String,
}

/// Application entry point: initializes logging, loads configuration from `Secrets.toml`,
/// constructs required clients and services, and binds the Telegram service to 127.0.0.1:8080.
///
/// Attempts to:
/// - initialize the standard logger,
/// - read and parse `Secrets.toml` into `Config`,
/// - create a `Stands4Client` and `TelegramService`,
/// - bind and run the service on `127.0.0.1:8080`.
///
/// # Returns
///
/// `Ok(())` if the service starts and binds successfully, `Err` if configuration loading,
/// client construction, or binding fails.
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    rustls::crypto::ring::default_provider().install_default()?;
    std_logger::Config::logfmt().init();
    let config_str = std::fs::read_to_string("Secrets.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let addr = SocketAddr::new(IpAddr::V4(ip), 8080);
    let stands4_client = Stands4Client::new(config.stands4_user_id, config.stands4_token);
    let service = TelegramService {
        token: config.teloxide_token,
        stands4_client,
    };
    service.bind(addr).await
}
