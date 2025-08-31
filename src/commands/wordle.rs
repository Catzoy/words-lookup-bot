use crate::commands::{BotExt, CommandHandler, FullMessageFormatter, MessageCommands};
use crate::format::formatter::compose_word_defs;
use crate::stands4::Stands4LinksProvider;
use crate::wordle::cache::WordleCache;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester};
use teloxide::types::ParseMode;
use teloxide::Bot;

async fn wordle_lookup_handler(bot: Bot, message: Message, cache: WordleCache) -> anyhow::Result<()> {
    let msg = cache.with_answer(|answer| {
        let formatter = FullMessageFormatter::new(Stands4LinksProvider {});
        compose_word_defs(formatter, &answer.answer.solution, &answer.definitions)
    }).await??;

    bot.send_message(message.chat.id, msg)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

async fn ensure_wordle_answer(bot: Bot, message: Message, mut cache: WordleCache) -> anyhow::Result<()> {
    if let Err(err) = cache.require_fresh_answer().await {
        log::error!("Failed to get today's wordle, err: {}", err);
        bot.send_message(
            message.chat.id,
            "Could not get today's wordle, sorry, try again in an hour or so.",
        ).await?;
        Err(err)
    } else {
        Ok(())
    }
}
pub fn wordle_lookup() -> CommandHandler {
    teloxide::dptree::case![MessageCommands::Wordle]
        .map_async(ensure_wordle_answer)
        .endpoint(|bot: Bot, message: Message, cache: WordleCache| async move {
            bot.with_err_response(message, move |bot, message| async {
                wordle_lookup_handler(bot, message, cache).await
            }).await
        })
}