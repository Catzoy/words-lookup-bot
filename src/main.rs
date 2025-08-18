use anyhow::Context as _;
use shuttle_runtime::{Error, SecretStore};
use std::net::SocketAddr;
use teloxide::prelude::Requester;
use teloxide::types::Message;
use teloxide::{repl, Bot};

pub struct TelegramService {
    token: String,
}

impl TelegramService {
    pub fn new(token: String) -> Self {
        TelegramService { token }
    }
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for TelegramService {
    async fn bind(self, _: SocketAddr) -> Result<(), Error> {
        let client = reqwest::Client::builder()
            .build()
            .expect("Could not build request client");
        let bot = Bot::with_client(self.token, client);

        repl(bot, |bot: Bot, message: Message| async move {
            log::info!("Handling a new message!");
            log::debug!("Message: {:#?}", message);
            let result = bot.send_message(message.chat.id, "Hi there").await;
            match result {
                Ok(_) => {
                    log::info!("Sent successfully!");
                }
                Err(e) => {
                    log::error!("Error while sending message: {}", e);
                }
            }
            Ok(())
        }).await;

        Ok(())
    }
}

#[shuttle_runtime::main]
async fn telegram(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> Result<TelegramService, Error> {
    let token = secret_store
        .get("TELOXIDE_TOKEN")
        .context("TELOXIDE_TOKEN not found")?;
    let service = TelegramService::new(token);

    Ok(service)
}