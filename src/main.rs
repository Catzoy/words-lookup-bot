mod stands4;

use anyhow::Context as _;
use shuttle_runtime::{Error, SecretStore};
use stands4::client::Stands4Client;
use std::net::SocketAddr;
use teloxide::dispatching::{Dispatcher, UpdateFilterExt};
use teloxide::error_handlers::LoggingErrorHandler;
use teloxide::prelude::{DependencyMap, Requester, Update};
use teloxide::types::Message;
use teloxide::{update_listeners, Bot};

#[derive(Clone)]
pub struct TelegramService {
    token: String,
    stands4_client: Stands4Client,
}

impl TelegramService {
    pub fn new(token: String, stands4_client: Stands4Client) -> Self {
        TelegramService { token, stands4_client }
    }
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for TelegramService {
    async fn bind(self, _: SocketAddr) -> Result<(), Error> {
        let client = reqwest::Client::builder()
            .build()
            .expect("Could not build request client");
        let bot = Bot::with_client(self.token.clone(), client);
        let cloned_bot = bot.clone();

        // Other update types are of no interest to use since this REPL is only for
        // messages. See <https://github.com/teloxide/teloxide/issues/557>.
        let ignore_update = |_upd| Box::pin(async {});
        let handler = async move |bot: Bot, client: Stands4Client, message: Message| -> anyhow::Result<()>{
            let text = message.text().unwrap_or_default();
            let words = text.split_whitespace()
                .map(|s| s.to_lowercase())
                .collect::<Vec<String>>();

            log::info!("Received message: {:?}", text);
            log::info!("Received words: {:?}", words.len());

            match words.len() {
                0 => {
                    bot.send_message(message.chat.id, "I'm a teapot").await?;
                }
                1 => {
                    let word = words.first().unwrap();
                    log::info!("Looking up word {}", word);

                    let defs = client.search_word(word).await?;
                    let msg = format!("Found {} defs", defs.len());
                    bot.send_message(message.chat.id, msg).await?;
                }
                _ => {
                    let phrase = words.join(" ");
                    log::info!("Looking up phrase {}", phrase);

                    let phrases = client.search_phrase(phrase.as_str()).await?;
                    let msg = format!("Found {} phrases", phrases.len());
                    bot.send_message(message.chat.id, msg).await?;
                }
            }
            Ok(())
        };

        let mut deps = DependencyMap::new();
        deps.insert(self.stands4_client);

        Dispatcher::builder(bot, Update::filter_message().endpoint(handler))
            .default_handler(ignore_update)
            .dependencies(deps)
            .enable_ctrlc_handler()
            .build()
            .dispatch_with_listener(
                update_listeners::polling_default(cloned_bot).await,
                LoggingErrorHandler::with_custom_text("An error from the update listener"),
            )
            .await;

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
    let stands4_user_id = secret_store
        .get("STANDS4_USER_ID")
        .context("STANDS4_USER_ID not found")?;
    let stands4_token = secret_store
        .get("STANDS4_TOKEN")
        .context("STANDS4_TOKEN not found")?;
    let stands4_client = Stands4Client::new(
        stands4_user_id,
        stands4_token,
    );

    let service = TelegramService::new(token, stands4_client);

    Ok(service)
}