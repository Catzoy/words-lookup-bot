use shuttle_runtime::async_trait;
use teloxide::prelude::{InlineQuery, Message, Requester};
use teloxide::Bot;

#[async_trait]
pub trait BotExt<R> {
    async fn respond_generic_err(&self, request: R) -> anyhow::Result<()>;
}

#[async_trait]
impl BotExt<Message> for Bot {
    async fn respond_generic_err(&self, message: Message) -> anyhow::Result<()> {
        let chat_id = message.chat.id;
        let text = "There was an error processing your query, try again later, sorry.";
        if let Err(err) = self.send_message(chat_id, text).await {
            log::error!("Couldn't send error-response: {}", err);
        }
        Ok(())
    }
}

#[async_trait]
impl BotExt<InlineQuery> for Bot {
    async fn respond_generic_err(&self, request: InlineQuery) -> anyhow::Result<()> {
        if let Err(err) = self.answer_inline_query(request.id, vec![]).await {
            log::error!("Couldn't send error-response: {}", err);
        }
        Ok(())
    }
}
