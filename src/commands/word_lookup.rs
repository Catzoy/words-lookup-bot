use crate::commands::{Command, HelpDescriptor, Payload};
use crate::formatting::{FullMessageFormatter, LookupFormatter};
use crate::stands4::client::Stands4Client;
use crate::stands4::entities::{AbbreviationDefinition, WordDefinition};
use shuttle_runtime::async_trait;
use std::collections::HashMap;
use std::string::FromUtf8Error;
use teloxide::prelude::Requester;
use teloxide::types::ParseMode;

pub struct WordLookup {
    stands4_client: Stands4Client,
}

impl WordLookup {
    pub(crate) const NAME: &'static str = "word";
    pub(crate) fn new(client: &Stands4Client) -> Self {
        Self { stands4_client: client.clone() }
    }
    fn word_link(&self, word: &str) -> String {
        format!("https://www.definitions.net/definition/{}", word)
    }

    fn abbr_link(&self, word: &str) -> String {
        format!("https://www.abbreviations.com/{}", word)
    }

    fn compose_word_defs(&self, word: &str, defs: Vec<WordDefinition>) -> Result<String, FromUtf8Error> {
        let mut formatter = FullMessageFormatter::default();
        formatter.builder.append(format!("Found {} definitions\n\n", defs.len()));

        for (i, def) in defs.iter().take(5).enumerate() {
            formatter.visit_word(i, def);
        }
        if defs.len() > 5 {
            formatter.append_link(self.word_link(word))
        }
        formatter.build()
    }

    fn compose_abbr_defs(&self, word: &str, defs: Vec<AbbreviationDefinition>) -> Result<String, FromUtf8Error> {
        let mut formatter = FullMessageFormatter::default();
        formatter.builder.append(format!("Found {} definitions\n\n", defs.len()));

        let categorized = defs.categorized();
        for (i, (category, defs)) in categorized.iter().take(5).enumerate() {
            formatter.visit_abbreviations(i, category, defs);
        }
        if categorized.len() > 5 {
            formatter.append_link(self.abbr_link(word))
        }
        formatter.build()
    }

    fn compose_words_with_abbrs(
        &self,
        word: &String,
        words: Vec<WordDefinition>,
        abbrs: Vec<AbbreviationDefinition>,
    ) -> Result<String, FromUtf8Error> {
        let mut formatter = FullMessageFormatter::default();
        formatter.builder.append(format!("Found {} definitions\n\n", words.len()));

        for (i, def) in words.iter().take(5).enumerate() {
            formatter.visit_word(i, def);
        }
        if words.len() > 5 {
            formatter.append_link(self.word_link(word))
        }

        formatter.builder.append("And also\n");
        formatter.builder.append(format!("Found {} abbreviations\n\n", abbrs.len()));

        let categorized = abbrs.categorized();
        for (i, (category, defs)) in categorized.iter().take(5).enumerate() {
            formatter.visit_abbreviations(i, category, defs);
        }
        if categorized.len() > 5 {
            formatter.append_link(self.abbr_link(word))
        }

        formatter.build()
    }
}

#[async_trait]
impl Command for WordLookup {
    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn descriptor(&self) -> Option<HelpDescriptor> {
        Some(HelpDescriptor {
            name: Self::NAME,
            description: "Find definition of the specified phrase.\n\
        Any message containing at most 1 word, even with hyphens, will be looked up.",
        })
    }


    async fn handle(&self, &Payload { bot, message, args, .. }: &Payload) -> anyhow::Result<()> {
        match args.first() {
            None => {
                let mut msg = bot.send_message(
                    message.chat.id,
                    "You need to specify a word to look up, like so: `\\word cookies`",
                );
                msg.parse_mode = Some(ParseMode::MarkdownV2);
                msg.await?;
            }
            Some(word) => {
                log::info!("Looking up word {}", word);

                let results = futures::future::join(
                    self.stands4_client.search_word(word),
                    self.stands4_client.search_abbreviation(word),
                ).await;

                let msg = match results {
                    (Ok(words), Ok(abbrs)) =>
                        match (words.len(), abbrs.len()) {
                            (0, 0) => "Found 0 definitions".to_string(),
                            (0, _) => self.compose_abbr_defs(word, abbrs)?,
                            (_, 0) => self.compose_word_defs(word, words)?,
                            (_, _) => self.compose_words_with_abbrs(word, words, abbrs)?
                        }

                    (Ok(words), _) =>
                        self.compose_word_defs(word, words)?,

                    (_, Ok(abbrs)) =>
                        self.compose_abbr_defs(word, abbrs)?,

                    (Err(_), Err(_)) =>
                        "Found 0 definitions".to_string(),
                };

                let mut msg = bot.send_message(message.chat.id, msg);
                msg.parse_mode = Some(ParseMode::MarkdownV2);
                msg.await?;
            }
        }
        Ok(())
    }
}

trait VecAbbreviationsExt {
    fn categorized(&self) -> Vec<(&str, Vec<&AbbreviationDefinition>)>;
}

impl VecAbbreviationsExt for Vec<AbbreviationDefinition> {
    fn categorized(&self) -> Vec<(&str, Vec<&AbbreviationDefinition>)> {
        let categorized = &mut self.iter().fold(
            HashMap::<&str, Vec<&AbbreviationDefinition>>::new(), |mut map, def| {
                let category = def.category.as_str();
                match map.get_mut(category) {
                    Some(v) => { v.push(def); }
                    None => { map.insert(category, vec![def]); }
                };
                map
            },
        );

        let mut common = categorized
            .drain()
            .collect::<Vec<_>>();
        common.sort_by(|(_, v1), (_, v2)| v2.len().cmp(&v1.len()));
        common
    }
}