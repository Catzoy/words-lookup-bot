use crate::bloc::common::{HandlerOwner, LookupError};
use crate::bot::LookupBotX;
use crate::commands::CommandHandler;
use crate::format::LookupFormatter;
use crate::stands4::{AbbreviationDefinition, Stands4Client, VecAbbreviationsExt, WordDefinition};
use futures::TryFutureExt;
use teloxide::dptree::entry;

type Entity = (Vec<WordDefinition>, Vec<AbbreviationDefinition>);

pub struct WordLookup;

impl WordLookup {
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

    fn compose_response<Formatter>(
        word: String,
        mut formatter: Formatter,
        (words, abbrs): Entity,
    ) -> Result<Formatter::Value, LookupError>
    where
        Formatter: LookupFormatter,
    {
        let text = match (words.len(), abbrs.len()) {
            (0, 0) => Ok(Formatter::on_empty()),
            (0, _) => formatter.compose_abbr_defs(&word, &abbrs),
            (_, 0) => formatter.compose_word_defs(&word, &words),
            (_, _) => formatter.compose_words_with_abbrs(&word, &words, &abbrs),
        };
        text.map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}

pub trait WordLookupFormatter<R, E> {
    fn compose_word_defs(self, word: &str, defs: &Vec<WordDefinition>) -> Result<R, E>;
    fn compose_abbr_defs(self, word: &str, defs: &Vec<AbbreviationDefinition>) -> Result<R, E>;

    fn compose_words_with_abbrs(
        self,
        word: &str,
        words: &Vec<WordDefinition>,
        abbrs: &Vec<AbbreviationDefinition>,
    ) -> Result<R, E>;
}

impl<T, R, E> WordLookupFormatter<R, E> for T
where
    T: LookupFormatter<Value = R, Error = E>,
{
    fn compose_word_defs(mut self, word: &str, defs: &Vec<WordDefinition>) -> Result<R, E> {
        self.append_title(format!("Found {} definitions", defs.len()));

        for (i, def) in defs.iter().take(5).enumerate() {
            self.visit_word(i, def);
        }
        if defs.len() > 5 {
            self.append_link(self.link_provider().word_link(word))
        }
        self.build()
    }

    fn compose_abbr_defs(mut self, word: &str, defs: &Vec<AbbreviationDefinition>) -> Result<R, E> {
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
    ) -> Result<R, E> {
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
}

impl HandlerOwner for WordLookup {
    fn handler<Bot>() -> CommandHandler
    where
        Bot: LookupBotX + Clone + Send + Sync + 'static,
    {
        entry()
            .filter_async(|bot: Bot, phrase: String| async move { bot.drop_empty(phrase).await })
            .map_async(Self::get_definitions)
            .filter_map_async(
                |bot: Bot, response: Result<Entity, LookupError>| async move {
                    bot.ensure_request_success(response).await
                },
            )
            .map(|bot: Bot, phrase: String, defs: Entity| async move {
                Self::compose_response(phrase, bot.formatter(), defs)
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
