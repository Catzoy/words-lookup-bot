use crate::bloc::phrase_lookup::PhraseLookupBot;
use crate::bloc::suggestions::SuggestionsBot;
use crate::bloc::thesaurus_lookup::ThesaurusLookupBot;
use crate::bloc::urban_lookup::UrbanLookupBot;
use crate::bloc::word_finder::WordFinderBot;
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

    /// Sends the stored inline query's answers to Telegram.
    ///
    /// Sends the provided `answers` as the response to the `InlineQuery` contained in this bot.
    /// Returns `Ok(())` on success, or an error containing the underlying API failure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run_example(bot: &crate::InlineBot) -> anyhow::Result<()> {
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

impl WordLookupBot<Vec<InlineQueryResult>> for InlineBot {}
impl PhraseLookupBot<Vec<InlineQueryResult>> for InlineBot {}
impl ThesaurusLookupBot<Vec<InlineQueryResult>> for InlineBot {}
impl UrbanLookupBot<Vec<InlineQueryResult>> for InlineBot {}
impl SuggestionsBot for InlineBot {}

impl WordFinderBot<Vec<InlineQueryResult>> for InlineBot {}