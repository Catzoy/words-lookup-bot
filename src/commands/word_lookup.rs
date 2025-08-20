use crate::commands::{Command, HelpDescriptor, Payload};
use crate::formatting::{FullMessageFormatter, LookupFormatter};
use crate::stands4::client::Stands4Client;
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

                let defs = self.stands4_client.search_word(word).await?;
                let mut msg = string_builder::Builder::default();
                msg.append(format!("Found {} definitions\n\n", defs.len()));

                let mut formatter = FullMessageFormatter { builder: msg };
                for (i, def) in defs.iter().take(5).enumerate() {
                    formatter.visit_word(i, def);
                }

                let msg = formatter.build()?;
                bot.send_message(message.chat.id, msg).await?;
            }
        }
        Ok(())
    }
}