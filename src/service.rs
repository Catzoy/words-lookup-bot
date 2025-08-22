use crate::commands::*;
use crate::stands4::client::Stands4Client;
use shuttle_runtime::Error;
use std::net::SocketAddr;
use teloxide::dispatching::{Dispatcher, UpdateFilterExt};
use teloxide::error_handlers::LoggingErrorHandler;
use teloxide::prelude::{DependencyMap, Message, Requester, RequesterExt, Update};
use teloxide::types::{Me, ParseMode};
use teloxide::utils::command::parse_command;
use teloxide::{update_listeners, Bot};

#[derive(Clone)]
pub struct TelegramService {
    token: String,
    pub(crate) registry: CommandsRegistry,
}

impl TelegramService {
    pub fn new<'a>(token: String, stands4_client: Stands4Client) -> Self {
        let mut registry = CommandsRegistry::new(
            UnknownCommand {}
        );
        registry.insert(HelpCommand {});
        registry.insert(StartCommand {});
        registry.insert(TeapotCommand {});
        registry.insert(WordLookup::new(&stands4_client));
        registry.insert(PhraseLookup::new(&stands4_client));


        TelegramService { token, registry }
    }
}

impl TelegramService {
    fn extract_command(&self, me: &Me, message: &Message) -> (&BoxedCommand, Vec<String>) {
        let text = message.text().unwrap_or_default();
        let username = me.username.clone().unwrap_or_default();
        let (cmd, args) = parse_command(text, username).unwrap_or_else(|| {
            let words = text.split_whitespace().collect::<Vec<&str>>();
            match words.len() {
                0 => (TeapotCommand::NAME, Vec::default()),
                1 => (WordLookup::NAME, words),
                _ => (PhraseLookup::NAME, words),
            }
        });

        log::info!("Received message: {:?}", text);
        log::info!("Processing command {} {:?}", cmd, args);

        let cmd = self.registry.get(cmd.to_string());
        (cmd, args.iter().map(|s| s.to_lowercase()).collect())
    }
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for TelegramService {
    async fn bind(self, _: SocketAddr) -> Result<(), Error> {
        let bot = Bot::new(self.token.clone()).parse_mode(ParseMode::MarkdownV2);
        let cloned_bot = bot.clone();

        // Other update types are of no interest to use since this REPL is only for
        // messages. See <https://github.com/teloxide/teloxide/issues/557>.
        let ignore_update = |_upd| Box::pin(async {});
        let handler = async move |bot: Bot, me: Me, service: TelegramService, message: Message| -> anyhow::Result<()>{
            let chat_id = message.chat.id;
            let (command, args) = service.extract_command(&me, &message);
            let payload = Payload {
                bot: &bot,
                me: &me,
                message: &message,
                service: &service,
                args: &args,
            };
            match command.handle(&payload).await {
                Ok(_) => {
                    Ok(())
                }
                Err(err) => {
                    let _ = bot.send_message(
                        chat_id,
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