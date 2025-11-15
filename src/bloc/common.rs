use crate::bot::LookupBotX;
use crate::commands::CommandHandler;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum LookupError {
    FailedResponseBuilder,
    FailedRequest,
}

pub trait HandlerOwner {
    fn handler<Bot>() -> CommandHandler
    where
        Bot: LookupBotX + Clone + Send + Sync + 'static;
}
