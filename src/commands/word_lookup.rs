use crate::{
    commands::{drop_empty, BotExt, CommandHandler, FullMessageFormatter, MessageCommands},
    format::compose_word_with_abbrs_determined,
    stands4::Stands4Client,
};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{Message, ParseMode},
    Bot,
};

async fn word_lookup_handler(
    bot: Bot,
    message: Message,
    stands4_client: Stands4Client,
    word: String,
) -> anyhow::Result<()> {
    log::info!("Looking up word {}", word);

    let results = futures::future::join(
        stands4_client.search_word(&word),
        stands4_client.search_abbreviation(&word),
    ).await;

    let formatter = FullMessageFormatter::default();
    let msg = compose_word_with_abbrs_determined(formatter, &word, &results, || {
        "Found 0 definitions".to_string()
    })?;

    bot.send_message(message.chat.id, msg)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

pub fn word_lookup() -> CommandHandler {
    teloxide::dptree::case![MessageCommands::WordLookup(args)]
        .filter_async(drop_empty)
        .endpoint(
            |word: String, bot: Bot, message: Message, stands4_client: Stands4Client| async move {
                bot.with_err_response(message, move |bot, message| async {
                    word_lookup_handler(bot, message, stands4_client, word).await
                }).await
            },
        )
}
