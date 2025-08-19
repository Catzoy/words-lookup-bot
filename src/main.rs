mod stands4;
mod formatting;

use crate::formatting::{FullMessageFormatter, LookupFormatter};
use anyhow::Context as _;
use shuttle_runtime::{Error, SecretStore};
use stands4::client::Stands4Client;
use std::net::SocketAddr;
use teloxide::dispatching::{Dispatcher, UpdateFilterExt};
use teloxide::error_handlers::LoggingErrorHandler;
use teloxide::prelude::{DependencyMap, Requester, Update};
use teloxide::types::{Me, Message};
use teloxide::utils::command::parse_command;
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

impl TelegramService {
    async fn handle_teapot(&self, bot: &Bot, message: &Message) -> anyhow::Result<()> {
        bot.send_message(message.chat.id, "I'm a teapot").await?;
        Ok(())
    }

    async fn handle_word_lookup(&self, bot: &Bot, message: &Message, args: Vec<&str>) -> anyhow::Result<()> {
        let word = *args.first().unwrap();
        log::info!("Looking up word {}", word);

        let defs = self.stands4_client.search_word(word).await?;
        let mut msg = string_builder::Builder::default();
        msg.append(format!("Found {} definitions\n\n", defs.len()));

        let mut formatter = FullMessageFormatter { builder: msg };
        for (i, def) in defs.iter().take(5).enumerate() {
            formatter.visit_word(i, def);
        }

        let msg = formatter.build()?;
        bot.send_message(message.chat.id, msg).await?;
        Ok(())
    }

    async fn handle_phrase_lookup(&self, bot: &Bot, message: &Message, args: Vec<&str>) -> anyhow::Result<()> {
        let phrase = args.join(" ");
        log::info!("Looking up phrase {}", phrase);

        let defs = self.stands4_client.search_phrase(phrase.as_str()).await?;
        let mut msg = string_builder::Builder::default();
        msg.append(format!("Found {} definitions\n\n", defs.len()));

        let mut formatter = FullMessageFormatter { builder: msg };
        for (i, def) in defs.iter().take(5).enumerate() {
            formatter.visit_phrase(i, def);
        }

        let msg = formatter.build()?;
        bot.send_message(message.chat.id, msg).await?;
        Ok(())
    }

    async fn handle_start_command(&self, bot: &Bot, message: &Message) -> anyhow::Result<()> {
        bot.send_message(
            message.chat.id,
            "Hi!\nI'm a bot that can look up words and phrases.\nSimply send me a message and I'll search for the definition of the text.".to_string(),
        ).await?;
        Ok(())
    }
    async fn handle_unknown_command(&self, bot: &Bot, message: &Message) -> anyhow::Result<()> {
        bot.send_message(
            message.chat.id,
            "I don't know that command, sorry.",
        ).await?;
        Ok(())
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
        let handler = async move |bot: Bot, me: Me, service: TelegramService, message: Message| -> anyhow::Result<()>{
            let text = message.text().unwrap_or_default();
            let (cmd, args) = parse_command(text, me.username.clone().unwrap_or_default())
                .unwrap_or_else(|| {
                    let words = text.split_whitespace().collect::<Vec<&str>>();
                    match words.len() {
                        0 => ("teapot", Vec::default()),
                        1 => ("word", words),
                        _ => ("phrase", words),
                    }
                });

            log::info!("Received message: {:?}", text);
            log::info!("Processing command {} {:?}", cmd, args);

            let result = match cmd {
                "teapot" => service.handle_teapot(&bot, &message).await,
                "word" => service.handle_word_lookup(&bot, &message, args).await,
                "phrase" => service.handle_phrase_lookup(&bot, &message, args).await,
                "start" => service.handle_start_command(&bot, &message).await,
                _ => service.handle_unknown_command(&bot, &message).await,
            };
            match result {
                Ok(_) => {
                    Ok(())
                }
                Err(err) => {
                    let _ = bot.send_message(
                        message.chat.id,
                        "There was an error processing your query, try again later, sorry.",
                    ).await;
                    Err(err)
                }
            }
        };

        let mut deps = DependencyMap::new();
        deps.insert(self);

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