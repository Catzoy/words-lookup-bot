use crate::bloc::common::{CommandHandler, LookupError};
use crate::bot::LookupBot;
use crate::format::{LookupFormatter, ToEscaped};
use crate::mw::client::MerriamWebsterClient;
use crate::mw::entities::FoundWords;
use teloxide::dptree::entry;

pub trait WordFinderBot {}
pub trait WordFinderHandler {
    async fn get_possible_words(
        client: MerriamWebsterClient,
        mask: String,
    ) -> Result<FoundWords, LookupError> {
        client.find(&mask).await.map_err(|err| {
            log::error!("MW failed request: {}", err);
            LookupError::FailedRequest
        })
    }

    async fn ensure_valid(&self, mask: String) -> bool;

    fn word_finder_handler() -> CommandHandler;
}

trait WordFinderFormater<Value, Error> {
    fn compose_commons_only(self, common: Vec<String>) -> Result<Value, Error>;
    fn compose_possible_only(self, possible: Vec<String>) -> Result<Value, Error>;
    fn compose_full(self, common: Vec<String>, possible: Vec<String>) -> Result<Value, Error>;
    fn compose_word_finder_response(self, defs: FoundWords) -> Result<Value, LookupError>;
}

impl<Formatter> WordFinderFormater<Formatter::Value, Formatter::Error> for Formatter
where
    Formatter: LookupFormatter,
{
    fn compose_commons_only(
        mut self,
        common: Vec<String>,
    ) -> Result<Formatter::Value, Formatter::Error> {
        self.append_title(format!("Found {:} common definitions", common.len()));
        for (i, def) in common.iter().enumerate() {
            self.visit_word_finder_definition(i, def);
        }
        self.build()
    }

    fn compose_possible_only(
        mut self,
        possible: Vec<String>,
    ) -> Result<Formatter::Value, Formatter::Error> {
        self.append_title(format!("Found {:} possible definitions", possible.len()));
        for (i, def) in possible.iter().enumerate() {
            self.visit_word_finder_definition(i, def);
        }
        self.build()
    }

    fn compose_full(
        mut self,
        common: Vec<String>,
        possible: Vec<String>,
    ) -> Result<Formatter::Value, Formatter::Error> {
        self.append_title(format!("Found {:} common definitions", common.len()));
        for (i, def) in common.iter().enumerate() {
            self.visit_word_finder_definition(i, def);
        }
        self.append_title(format!("And {:} possible definitions", possible.len()));
        for (i, def) in possible.iter().enumerate() {
            self.visit_word_finder_definition(i, def);
        }
        self.build()
    }

    fn compose_word_finder_response(
        self,
        defs: FoundWords,
    ) -> Result<Formatter::Value, LookupError> {
        let defs = defs.to_escaped();
        let text = match (defs.common.len(), defs.possible.len()) {
            (0, 0) => Ok(Self::on_empty()),
            (0, _) => self.compose_commons_only(defs.common),
            (_, 0) => self.compose_possible_only(defs.possible),
            (_, _) => self.compose_full(defs.common, defs.possible),
        };
        text.map_err(|err| {
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
                |bot: Bot, response: Result<FoundWords, LookupError>| async move {
                    bot.ensure_request_success(response).await
                },
            )
            .map(move |bot: Bot, defs: FoundWords| {
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
