mod bloc;
mod bot;
mod commands;
mod datamuse;
mod format;
mod inlines;
mod server;
mod service;
mod stands4;
mod urban;
mod wordle;

use crate::service::telegram::TelegramService;
use anyhow::Context as _;
use shuttle_runtime::{Error, SecretStore};
use stands4::client::Stands4Client;

#[shuttle_runtime::main]
async fn telegram(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> Result<TelegramService, Error> {
    let token = secret_store
        .get("TELOXIDE_TOKEN")
        .context("TELOXIDE_TOKEN not found")?;
    let stands4_user_id = secret_store
        .get("STANDS4_USER_ID")
        .context("STANDS4_USER_ID not found")?;
    let stands4_token = secret_store
        .get("STANDS4_TOKEN")
        .context("STANDS4_TOKEN not found")?;
    let stands4_client = Stands4Client::new(stands4_user_id, stands4_token);

    let service = TelegramService {
        token,
        stands4_client,
    };

    Ok(service)
}
