use crate::commands::{Command, HelpDescriptor, Payload};
use crate::formatting::{FullMessageFormatter, LookupFormatter};
use crate::stands4::client::Stands4Client;
use crate::stands4::entities::{AbbreviationDefinition, WordDefinition};
use shuttle_runtime::async_trait;
use teloxide::prelude::Requester;

pub struct WordLookup {
    stands4_client: Stands4Client,
}

impl WordLookup {
    pub(crate) const NAME: &'static str = "word";
    pub(crate) fn new(client: &Stands4Client) -> Self {
        Self { stands4_client: client.clone() }
    }
}

impl WordLookup {
    fn compose_word_defs(&self, defs: Vec<WordDefinition>) -> Result<String, std::string::FromUtf8Error> {
        let mut msg = string_builder::Builder::default();
        msg.append(format!("Found {} definitions\n\n", defs.len()));

        let mut formatter = FullMessageFormatter { builder: msg };
        for (i, def) in defs.iter().take(5).enumerate() {
            formatter.visit_word(i, def);
        }

        formatter.build()
    }
    fn compose_abbr_defs(&self, defs: Vec<AbbreviationDefinition>) -> Result<String, std::string::FromUtf8Error> {
        let mut msg = string_builder::Builder::default();
        msg.append(format!("Found {} definitions\n\n", defs.len()));

        let mut formatter = FullMessageFormatter { builder: msg };
        for (i, def) in defs.iter().take(5).enumerate() {
            formatter.visit_abbreviation(i, def);
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
                bot.send_message(
                    message.chat.id,
                    "You need to specify a word to look up, like so: `\\word cookies`",
                ).await?;
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
                            (0, _) => self.compose_abbr_defs(abbrs)?,
                            (_, _) => self.compose_word_defs(words)?,
                        }

                    (Ok(words), _) =>
                        self.compose_word_defs(words)?,

                    (_, Ok(abbrs)) =>
                        self.compose_abbr_defs(abbrs)?,

                    (Err(_), Err(_)) =>
                        "Found 0 definitions".to_string(),
                };

                bot.send_message(message.chat.id, msg).await?;
            }
        }
        Ok(())
    }
}