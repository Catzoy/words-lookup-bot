use crate::bloc::common::{CommandHandler, LookupError};
use crate::bot::LookupBot;
use crate::format::LookupFormatter;
use crate::stands4::{AbbreviationDefinition, Stands4Client, VecAbbreviationsExt, WordDefinition};
use futures::TryFutureExt;
use shuttle_runtime::async_trait;
use teloxide::dptree::entry;

type Entity = (Vec<WordDefinition>, Vec<AbbreviationDefinition>);

pub trait WordLookupBot {}

#[async_trait]
pub trait WordLookupHandler {
    async fn get_definitions(client: Stands4Client, word: String) -> Entity {
        futures::future::join(
            client.search_word(&word).unwrap_or_else(|err| {
                log::error!("Failed to retrieve definitions of a word: {:?}", err);
                vec![]
            }),
            client.search_abbreviation(&word).unwrap_or_else(|err| {
                log::error!("Failed to retrieve definitions of an abbr: {:?}", err);
                vec![]
            }),
        )
        .await
    }

    fn word_lookup_handler() -> CommandHandler;
}

pub trait WordLookupFormatter<Value, Error> {
    fn compose_word_defs(self, word: &str, defs: &Vec<WordDefinition>) -> Result<Value, Error>;

    fn compose_abbr_defs(
        self,
        word: &str,
        defs: &Vec<AbbreviationDefinition>,
    ) -> Result<Value, Error>;

    fn compose_words_with_abbrs(
        self,
        word: &str,
        words: &Vec<WordDefinition>,
        abbrs: &Vec<AbbreviationDefinition>,
    ) -> Result<Value, Error>;

    fn compose_word_response(self, word: String, entity: Entity) -> Result<Value, LookupError>;
}

impl<Formatter> WordLookupFormatter<Formatter::Value, Formatter::Error> for Formatter
where
    Formatter: LookupFormatter,
{
    fn compose_word_defs(
        mut self,
        word: &str,
        defs: &Vec<WordDefinition>,
    ) -> Result<Formatter::Value, Formatter::Error> {
        self.append_title(format!("Found {} definitions", defs.len()));

        for (i, def) in defs.iter().take(5).enumerate() {
            self.visit_word(i, def);
        }
        if defs.len() > 5 {
            self.append_link(self.link_provider().word_link(word))
        }
        self.build()
    }

    fn compose_abbr_defs(
        mut self,
        word: &str,
        defs: &Vec<AbbreviationDefinition>,
    ) -> Result<Formatter::Value, Formatter::Error> {
        self.append_title(format!("Found {} definitions", defs.len()));

        let categorized = defs.categorized();
        for (i, (category, defs)) in categorized.iter().take(5).enumerate() {
            self.visit_abbreviations(i, category, defs);
        }
        if categorized.len() > 5 {
            self.append_link(self.link_provider().abbr_link(word))
        }
        self.build()
    }

    fn compose_words_with_abbrs(
        mut self,
        word: &str,
        words: &Vec<WordDefinition>,
        abbrs: &Vec<AbbreviationDefinition>,
    ) -> Result<Formatter::Value, Formatter::Error> {
        self.append_title(format!("Found {} definitions", words.len()));

        for (i, def) in words.iter().take(5).enumerate() {
            self.visit_word(i, def);
        }
        if words.len() > 5 {
            self.append_link(self.link_provider().word_link(word))
        }

        self.append_title(format!("Found {} abbreviations", abbrs.len()));

        let categorized = abbrs.categorized();
        for (i, (category, defs)) in categorized.iter().take(5).enumerate() {
            self.visit_abbreviations(i, category, defs);
        }
        if categorized.len() > 5 {
            self.append_link(self.link_provider().abbr_link(word))
        }

        self.build()
    }

    fn compose_word_response(
        self,
        word: String,
        (words, abbrs): Entity,
    ) -> Result<Formatter::Value, LookupError> {
        let text = match (words.len(), abbrs.len()) {
            (0, 0) => Ok(Self::on_empty()),
            (0, _) => self.compose_abbr_defs(&word, &abbrs),
            (_, 0) => self.compose_word_defs(&word, &words),
            (_, _) => self.compose_words_with_abbrs(&word, &words, &abbrs),
        };
        text.map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}

impl<Bot> WordLookupHandler for Bot
where
    Bot: LookupBot + Send + Sync + 'static,
{
    fn word_lookup_handler() -> CommandHandler {
        entry()
            .filter_async(|bot: Bot, phrase: String| async move { bot.drop_empty(phrase).await })
            .map_async(Self::get_definitions)
            .filter_map_async(
                |bot: Bot, response: Result<Entity, LookupError>| async move {
                    bot.ensure_request_success(response).await
                },
            )
            .map(|bot: Bot, phrase: String, defs: Entity| async move {
                bot.formatter().compose_word_response(phrase, defs)
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
