use crate::bloc::common::{CommandHandler, LookupError};
use crate::bot::LookupBot;
use crate::datamuse::client::DatamuseClient;
use crate::format::{LookupFormatter, ToEscaped};
use teloxide::dptree::entry;

pub trait WordFinderBot {}
pub trait WordFinderHandler {
    async fn get_possible_words(
        client: DatamuseClient,
        mask: String,
    ) -> Result<Vec<String>, LookupError> {
        client.find(mask).await.map_err(|err| {
            log::error!("MW failed request: {}", err);
            LookupError::FailedRequest
        })
    }

    async fn ensure_valid(&self, mask: String) -> bool;

    fn word_finder_handler() -> CommandHandler;
}

trait WordFinderFormater<Value, Error> {
    fn compose_word_finder_response(self, defs: Vec<String>) -> Result<Value, LookupError>;
}

impl<Formatter> WordFinderFormater<Formatter::Value, Formatter::Error> for Formatter
where
    Formatter: LookupFormatter,
{
    fn compose_word_finder_response(
        mut self,
        defs: Vec<String>,
    ) -> Result<Formatter::Value, LookupError> {
        let defs = defs.to_escaped();
        self.append_title(format!("Found {:} definitions", defs.len()));
        for (i, def) in defs.iter().enumerate() {
            self.visit_word_finder_definition(i, def);
        }
        self.build().map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}

impl<Bot, Formatter> WordFinderHandler for Bot
where
    Bot: WordFinderBot + LookupBot<Formatter = Formatter> + Send + Sync + 'static,
    Formatter: LookupFormatter<Value = Bot::Response>,
{
    async fn ensure_valid(&self, mask: String) -> bool {
        if mask.len() < 2 || mask.len() > 15 {
            let _ = self.answer_generic_err().await;
            return false;
        }

        let mut has_blank = false;
        let mut has_filled = false;
        for letter in mask.chars() {
            match letter {
                '_' => {
                    has_blank = true;
                }
                'a'..='z' | 'A'..='Z' => {
                    has_filled = true;
                }
                _ => {
                    break;
                }
            }
        }
        if !has_blank || !has_filled {
            let _ = self.answer_generic_err().await;
            false
        } else {
            true
        }
    }
    fn word_finder_handler() -> CommandHandler {
        entry()
            .filter_async(|bot: Bot, mask: String| async move { bot.drop_empty(mask).await })
            .filter_async(|bot: Bot, mask: String| async move { bot.ensure_valid(mask).await })
            .map_async(Self::get_possible_words)
            .filter_map_async(
                |bot: Bot, response: Result<Vec<String>, LookupError>| async move {
                    bot.ensure_request_success(response).await
                },
            )
            .map(move |bot: Bot, defs: Vec<String>| {
                bot.formatter().compose_word_finder_response(defs)
            })
            .filter_map_async(
                |bot: Bot, response: Result<Bot::Response, LookupError>| async move {
                    bot.retrieve_or_generic_err(response).await
                },
            )
            .endpoint(
                |bot: Bot, response: Bot::Response| async move { bot.respond(response).await },
            )
    }
}
