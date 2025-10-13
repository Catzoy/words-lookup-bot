use teloxide::dispatching::{DpHandlerDescription, UpdateFilterExt};
use teloxide::dptree::Handler;
use teloxide::types::Update;

pub fn callbacks_tree() -> Handler<'static, anyhow::Result<()>, DpHandlerDescription> {
    Update::filter_callback_query()
}
