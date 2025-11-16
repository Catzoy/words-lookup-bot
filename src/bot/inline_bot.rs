use crate::bloc::common::LookupError;
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
    async fn answer_generic_err(&self) -> anyhow::Result<()> {
        let query_id = self.query.id.clone();
        if let Err(err) = self.bot.answer_inline_query(query_id, vec![]).await {
            log::error!("Couldn't send error-response: {}", err);
        }
        Ok(())
    }

    async fn answer(&self, answers: Vec<InlineQueryResult>) -> anyhow::Result<()> {
        let query_id = self.query.id.clone();
        &self.bot.answer_inline_query(query_id, answers).await?;
        Ok(())
    }

    async fn drop_empty(&self, phrase: String) -> bool {
        match phrase.as_str() {
            "" => {
                let _ = self.answer(vec![]).await;
                false
            }
            _ => true,
        }
    }

    async fn ensure_request_success<Entity>(
        &self,
        response: Result<Entity, LookupError>,
    ) -> Option<Entity>
    where
        Entity: Send,
    {
        match response {
            Ok(values) => Some(values),
            Err(err) => {
                log::error!("Failed to get request: {:?}", err);
                let result = self.answer(vec![]).await;
                if let Err(e) = result {
                    log::error!("Failed to send no results: {:?}", e);
                    let _ = self.answer_generic_err().await;
                }
                None
            }
        }
    }

    async fn retrieve_or_generic_err(
        &self,
        response: Result<Self::Response, LookupError>,
    ) -> Option<Self::Response> {
        match response {
            Ok(values) => Some(values),
            Err(err) => {
                log::error!("Failed to build response: {:?}", err);
                let result = self.answer(vec![]).await;
                if let Err(e) = result {
                    log::error!("Failed to respond generic err: {:?}", e);
                    let _ = self.answer_generic_err().await;
                }
                None
            }
        }
    }

    async fn respond(&self, response: Self::Response) -> anyhow::Result<()> {
        if let Err(e) = self.answer(response).await {
            log::error!("Failed to respond with query: {:?}", e);
            let _ = self.answer_generic_err().await;
        }
        Ok(())
    }
}
