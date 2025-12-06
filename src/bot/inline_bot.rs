use crate::bloc::phrase_lookup::PhraseLookupBot;
use crate::bloc::suggestions::SuggestionsBot;
use crate::bloc::thesaurus_lookup::ThesaurusLookupBot;
use crate::bloc::urban_lookup::UrbanLookupBot;
use crate::bloc::word_lookup::WordLookupBot;
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

    /// Sends the provided inline query answers to Telegram for the current query.
    ///
    /// Answers the `InlineQuery` stored on this bot by sending the given `answers`.
    /// Returns `Ok(())` when the Telegram API call succeeds, or an `Err` containing the underlying error otherwise.
    ///
    /// # Arguments
    ///
    /// * `answers` - The collection of `InlineQueryResult` to send as the response to the inline query.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example(bot: &crate::InlineBot) -> anyhow::Result<()> {
    /// bot.answer(vec![]).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn answer(&self, answers: Vec<InlineQueryResult>) -> anyhow::Result<()> {
        let query_id = self.query.id.clone();
        let _ = self.bot.answer_inline_query(query_id, answers).await?;
        Ok(())
    }
}

impl WordLookupBot for InlineBot {}
impl PhraseLookupBot for InlineBot {}
impl ThesaurusLookupBot for InlineBot {}
impl UrbanLookupBot for InlineBot {}
impl SuggestionsBot for InlineBot {}