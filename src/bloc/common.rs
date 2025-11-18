use std::fmt::Debug;
use teloxide::dispatching::DpHandlerDescription;
use teloxide::dptree::Endpoint;

pub type CommandHandler = Endpoint<'static, anyhow::Result<()>, DpHandlerDescription>;

#[derive(Debug, Clone)]
pub enum LookupError {
    FailedResponseBuilder,
    FailedRequest,
}
