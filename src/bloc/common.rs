use crate::bloc::ext::BotExt;
use crate::commands::CommandHandler;
use shuttle_runtime::async_trait;
use std::fmt::Debug;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{InlineQuery, Message, Requester};
use teloxide::types::{InlineQueryResult, ParseMode};
use teloxide::Bot;

#[derive(Debug, Clone)]
pub enum LookupError {
    FailedResponseBuilder,
    FailedRequest,
}

pub trait HandlerOwner {
    fn handler() -> CommandHandler;
}

#[async_trait]
pub trait Lookup: HandlerOwner + Clone {
    type Request: Clone + Send + Sync;
    type Entity: Clone + Send + Sync;
    type Response: Clone + Send + Sync + Default;
}

pub trait CommonLookup<Request, Entity, Response> {
    async fn ensure_request_success(
        bot: Bot,
        request: Request,
        response: Result<Entity, LookupError>,
    ) -> Option<Entity>;

    async fn retrieve_or_generic_err(
        bot: Bot,
        request: Request,
        response: Result<Response, LookupError>,
    ) -> Option<Response>;

    async fn respond(bot: Bot, message: Request, response: Response) -> anyhow::Result<()>;
}

impl<E, T> CommonLookup<Message, E, String> for T
where
    E: Clone + Send + Sync,
    T: Lookup<Request = Message, Entity = E, Response = String>,
{
    async fn ensure_request_success(
        bot: Bot,
        message: Message,
        response: Result<E, LookupError>,
    ) -> Option<E> {
        match response {
            Ok(values) => Some(values),
            Err(_) => {
                let text = "Something went wrong, please try again later";
                let resp = bot.send_message(message.chat.id, text).await;
                if let Err(e) = resp {
                    log::error!("Couldn't send error-response: {:?}", e);
                    let _ = bot.respond_generic_err(message).await;
                }
                None
            }
        }
    }
    async fn retrieve_or_generic_err(
        bot: Bot,
        message: Message,
        response: Result<String, LookupError>,
    ) -> Option<String> {
        match response {
            Ok(value) => Some(value),
            Err(_) => {
                let _ = bot.respond_generic_err(message).await;
                None
            }
        }
    }

    async fn respond(bot: Bot, message: Message, response: String) -> anyhow::Result<()> {
        let res = bot
            .send_message(message.chat.id, response)
            .parse_mode(ParseMode::MarkdownV2)
            .await;
        if let Err(e) = res {
            log::error!("Couldn't send response: {:?}", e);
            let _ = bot.respond_generic_err(message).await;
        }
        Ok(())
    }
}

impl<Entity, T> CommonLookup<InlineQuery, Entity, Vec<InlineQueryResult>> for T
where
    Entity: Clone + Send + Sync,
    T: Lookup<Request = InlineQuery, Entity = Entity, Response = Vec<InlineQueryResult>>,
{
    async fn ensure_request_success(
        bot: Bot,
        request: InlineQuery,
        response: Result<Entity, LookupError>,
    ) -> Option<Entity> {
        match response {
            Ok(values) => Some(values),
            Err(err) => {
                log::error!("Failed to get request: {:?}", err);
                let query = request.clone();
                let result = bot.answer_inline_query(request.id, vec![]).await;
                if let Err(e) = result {
                    log::error!("Failed to send no results: {:?}", e);
                    let _ = bot.respond_generic_err(query).await;
                }
                None
            }
        }
    }
    async fn retrieve_or_generic_err(
        bot: Bot,
        request: InlineQuery,
        response: Result<Vec<InlineQueryResult>, LookupError>,
    ) -> Option<Vec<InlineQueryResult>> {
        match response {
            Ok(values) => Some(values),
            Err(err) => {
                log::error!("Failed to build response: {:?}", err);
                let query = request.clone();
                let result = bot.answer_inline_query(request.id, vec![]).await;
                if let Err(e) = result {
                    log::error!("Failed to respond generic err: {:?}", e);
                    let _ = bot.respond_generic_err(query).await;
                }
                None
            }
        }
    }
    async fn respond(
        bot: Bot,
        request: InlineQuery,
        response: Vec<InlineQueryResult>,
    ) -> anyhow::Result<()> {
        let query = request.clone();
        if let Err(e) = bot.answer_inline_query(request.id, response).await {
            log::error!("Failed to respond with query: {:?}", e);
            let _ = bot.respond_generic_err(query).await;
        }
        Ok(())
    }
}
