use crate::{
    commands::{drop_empty, BotExt, CommandHandler, FullMessageFormatter, MessageCommands},
    format::compose_syn_ant_defs,
    stands4::Stands4Client,
};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{Message, ParseMode},
    Bot,
};

async fn thesaurus_lookup_handler(
    bot: Bot,
    message: Message,
    stands4_client: Stands4Client,
    term: String,
) -> anyhow::Result<()> {
    log::info!("Looking up word {}", term);

    let results = stands4_client.search_syn_ant(&term).await?;

    let formatter = FullMessageFormatter::default();
    let msg = compose_syn_ant_defs(formatter, &term, &results)?;

    bot.send_message(message.chat.id, msg)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

pub fn thesaurus_lookup() -> CommandHandler {
    teloxide::dptree::case![MessageCommands::Thesaurus(args)]
        .filter_async(drop_empty)
        .endpoint(
            |term: String, bot: Bot, message: Message, stands4_client: Stands4Client| async move {
                bot.with_err_response(message, move |bot, message| async {
                    thesaurus_lookup_handler(bot, message, stands4_client, term).await
                })
                .await
            },
        )
}
