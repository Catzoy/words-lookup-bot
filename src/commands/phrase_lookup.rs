use crate::commands::{Command, HelpDescriptor, Payload};
use crate::formatting::{FullMessageFormatter, LookupFormatter};
use crate::stands4::client::Stands4Client;
use shuttle_runtime::async_trait;
use teloxide::prelude::Requester;

pub struct PhraseLookup {
    stands4_client: Stands4Client,
}

impl PhraseLookup {
    pub(crate) const NAME: &'static str = "phrase";
    pub(crate) fn new(client: &Stands4Client) -> Self {
        Self { stands4_client: client.clone() }
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
                let mut msg = string_builder::Builder::default();
                msg.append(format!("Found {} definitions\n\n", defs.len()));

                let mut formatter = FullMessageFormatter { builder: msg };
                for (i, def) in defs.iter().take(5).enumerate() {
                    formatter.visit_phrase(i, def);
                }

                let msg = formatter.build()?;
                bot.send_message(message.chat.id, msg).await?;
            }
        };
        Ok(())
    }
}