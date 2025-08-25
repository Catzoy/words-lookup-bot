use crate::{
    inlines::phrase_lookup,
    inlines::word_lookup,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use teloxide::payloads::AnswerInlineQuerySetters;
use teloxide::prelude::{Requester, UserId};
use teloxide::types::InlineQueryId;
use teloxide::{dispatching::{DpHandlerDescription, UpdateFilterExt}, dptree::Handler, prelude::{InlineQuery, Update}, Bot};
use tokio::time::Instant;

#[derive(Debug, Clone)]
pub enum QueryCommands {
    WordLookup(String),
    PhraseLookup(String),
}

pub type InlineHandler = Handler<'static, anyhow::Result<()>, DpHandlerDescription>;

fn extract_command(query: InlineQuery) -> Option<QueryCommands> {
    let words = query.query.split_whitespace()
        .map(|s| s.to_lowercase())
        .collect::<Vec<String>>();
    match &words[..] {
        [] => None,
        [word] => Some(QueryCommands::WordLookup(word.to_owned())),
        _ => Some(QueryCommands::PhraseLookup(words.join(" "))),
    }
}

#[derive(Debug, Clone)]
pub struct Debouncer {
    queries_per_user: Arc<Mutex<HashMap<UserId, (InlineQueryId, Instant)>>>,
    duration: Duration,
}

impl Debouncer {
    pub fn new() -> Self {
        Self {
            queries_per_user: Arc::default(),
            duration: Duration::from_secs(1),
        }
    }

    pub fn get(&self, user_id: UserId) -> Option<InlineQueryId> {
        self.queries_per_user.lock().unwrap()
            .get(&user_id)
            .map(|(query_id, _)| query_id.clone())
    }

    pub fn replace(
        &mut self,
        user_id: UserId,
        query_id: InlineQueryId,
    ) -> Option<InlineQueryId> {
        let query_time = Instant::now();
        let mut existing = self.queries_per_user.lock().unwrap();
        match existing.insert(user_id, (query_id.clone(), query_time)) {
            Some((prev_query_id, prev_time))
            if query_time.duration_since(prev_time).lt(&self.duration)
            => Some(prev_query_id),
            _ => None
        }
    }
}
impl Default for Debouncer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn inlines_tree() -> Handler<'static, anyhow::Result<()>, DpHandlerDescription> {
    Update::filter_inline_query()
        .filter_map(extract_command)
        .filter_async(|bot: Bot, query: InlineQuery, mut debouncer: Debouncer| async move {
            let user_id = query.from.id;
            let query_id = query.id.clone();
            let debouncer_2 = debouncer.clone();
            log::info!("[DEBOUNCING] {:?} {:?} - {}", user_id, query_id, query.query);
            let debounce_query = async {
                let lq = query_id.clone();
                if let Some(prev_query_id) = debouncer.replace(user_id, query_id) {
                    log::info!("[DEBOUNCING] {:?} -> {:?}", prev_query_id, lq);
                    let _ = bot.answer_inline_query(prev_query_id, vec![])
                        .cache_time(0)
                        .await;
                }
            };

            tokio::join!(
                debounce_query,
                tokio::time::sleep(debouncer_2.duration)
            );

            let stored = debouncer_2.get(user_id);
            log::info!("[DEBOUNCING] CHECK {:?} // {:?} = {:?}", user_id, query.id, stored);
            return stored == Some(query.id);
        })
        .branch(word_lookup())
        .branch(phrase_lookup())
}