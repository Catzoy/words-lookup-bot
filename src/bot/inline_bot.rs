use crate::bot::LookupBot;
use crate::inlines::formatting::InlineFormatter;
use shuttle_runtime::async_trait;
use teloxide::prelude::{InlineQuery, Requester};
use teloxide::types::InlineQueryResult;
use teloxide::Bot;

#[derive(Debug, Clone)]
pub struct InlineBot {
    pub bot: Bot,
    pub query: InlineQuery,
}

#[async_trait]
impl LookupBot for InlineBot {
    type Request = InlineQuery;
    type Formatter = InlineFormatter;
    type Response = Vec<InlineQueryResult>;

    async fn answer(&self, answers: Vec<InlineQueryResult>) -> anyhow::Result<()> {
        let query_id = self.query.id.clone();
        let _ = &self.bot.answer_inline_query(query_id, answers).await?;
        Ok(())
    }
}
