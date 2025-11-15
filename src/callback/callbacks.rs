use crate::inlines::QueryCommands;
use teloxide::dispatching::{DpHandlerDescription, UpdateFilterExt};
use teloxide::dptree;
use teloxide::dptree::Handler;
use teloxide::types::{CallbackQuery, Update};

#[derive(Debug, Clone)]
pub enum CallbackRequest {
    Message(Callbacks),
    Inline(Callbacks),
}

#[derive(Debug, Clone)]
pub enum Callbacks {
    UrbanLookup(String),
    ThesaurusLookup(String),
}

fn extract_callback(CallbackQuery { message, inline_message_id, data, .. }: CallbackQuery) -> Option<CallbackRequest> {
    let text = data?;
    let inline_command = crate::inlines::extract_text_inline_command(text)?;
    let request = match inline_command {
        QueryCommands::UrbanLookup(request) => {
            Some(Callbacks::UrbanLookup(request))
        }
        QueryCommands::ThesaurusLookup(request) => {
            Some(Callbacks::ThesaurusLookup(request))
        }
        _ => None
    }?;
    if let Some(_) = message {
        Some(CallbackRequest::Message(request))
    } else if let Some(_) = inline_message_id {
        Some(CallbackRequest::Inline(request))
    } else {
        None
    }
}

pub fn callbacks_tree() -> Handler<'static, anyhow::Result<()>, DpHandlerDescription> {
    Update::filter_callback_query()
        .filter_map(extract_callback)
        .branch(
            dptree::case![CallbackRequest::Message(request)]
                .endpoint(|callback: Callbacks| async move {
                    log::info!("Received a message callback {:?}", callback);
                    Ok(())
                })
        )
        .branch(
            dptree::case![CallbackRequest::Inline(callback)]
                .endpoint(|callback: Callbacks| async move {
                    log::info!("Received a inline callback {:?}", callback);
                    Ok(())
                })
        )
}
