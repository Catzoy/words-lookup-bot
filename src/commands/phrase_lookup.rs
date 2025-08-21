use crate::commands::{Command, HelpDescriptor, Payload};
use crate::formatting::{FullMessageFormatter, LookupFormatter};
use crate::stands4::client::Stands4Client;
use crate::stands4::entities::PhraseDefinition;
use shuttle_runtime::async_trait;
use std::string::FromUtf8Error;
use teloxide::prelude::Requester;

pub struct PhraseLookup {
    stands4_client: Stands4Client,
}

impl PhraseLookup {
    pub(crate) const NAME: &'static str = "phrase";
    pub(crate) fn new(client: &Stands4Client) -> Self {
        Self { stands4_client: client.clone() }
    }

    fn phrase_link(&self, components: &Vec<String>) -> String {
        format!("https://www.phrases.com/psearch/{}", components.join("+"))
    }
    fn compose_phrase_defs(&self, components: &Vec<String>, defs: Vec<PhraseDefinition>) -> Result<String, FromUtf8Error> {
        let mut formatter = FullMessageFormatter::default();
        formatter.builder.append(format!("Found {} definitions\n\n", defs.len()));
        
        for (i, def) in defs.iter().take(5).enumerate() {
            formatter.visit_phrase(i, def);
        }
        if defs.len() > 5 {
            formatter.append_link(self.phrase_link(components));
        }

        formatter.build()
    }
}

#[async_trait]
impl Command for PhraseLookup {
    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn descriptor(&self) -> Option<HelpDescriptor> {
        Some(HelpDescriptor {
            name: PhraseLookup::NAME,
            description:
            "Find definition of the specified phrase.\n\
            Any message with more than 1 word is considered to be a phrase",
        })
    }

    async fn handle(&self, &Payload { bot, message, args, .. }: &Payload) -> anyhow::Result<()> {
        match args.join(" ").as_str() {
            "" => {
                bot.send_message(
                    message.chat.id,
                    "You meed to specify a phrase to look up, like so: `\\phrase buckle up`",
                ).await?;
            }
            phrase => {
                log::info!("Looking up phrase {}", phrase);

                let defs = self.stands4_client.search_phrase(phrase).await?;
                let msg = self.compose_phrase_defs(args, defs)?;
                bot.send_message(message.chat.id, msg).await?;
            }
        };
        Ok(())
    }
}