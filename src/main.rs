use anyhow::Context as _;
use shuttle_runtime::{Error, SecretStore};
use std::net::SocketAddr;
use teloxide::prelude::Requester;
use teloxide::types::{Me, Message};
use teloxide::utils::command::parse_command;
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
        repl(bot, |bot: Bot, me: Me, message: Message| async move {
            log::info!("Handling a new message!");
            log::debug!("Message: {:#?}", message);
            let text = message.text().unwrap_or_default();
            let bot_name = me.username.clone().unwrap_or_default();
            let (cmd, args) = parse_command(text, bot_name).unwrap_or_default();
            match cmd {
                _ => {}
            };
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