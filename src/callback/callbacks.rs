use teloxide::dispatching::{DpHandlerDescription, UpdateFilterExt};
use teloxide::dptree::Handler;
use teloxide::types::{CallbackQuery, Update};

pub fn callbacks_tree() -> Handler<'static, anyhow::Result<()>, DpHandlerDescription> {
    Update::filter_callback_query()
        .inspect(|query: CallbackQuery| {
            log::debug!("Received callback query: {:?}", query);
        })
        .endpoint(|| async { Ok(()) })
}
