use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use teloxide::payloads::AnswerInlineQuerySetters;
use teloxide::prelude::{Requester, UserId};
use teloxide::types::{InlineQuery, InlineQueryId};
use teloxide::Bot;
use tokio::time::Instant;

#[derive(Debug, Clone)]
pub struct InlineQueryDebouncer {
    queries_per_user: Arc<Mutex<HashMap<UserId, (InlineQueryId, Instant)>>>,
    duration: Duration,
}

impl InlineQueryDebouncer {
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
impl Default for InlineQueryDebouncer {
    fn default() -> Self {
        Self::new()
    }
}

async fn cancel_old_query(
    bot: Bot,
    user_id: UserId,
    query_id: InlineQueryId,
    mut debouncer: InlineQueryDebouncer,
) {
    if let Some(prev_query_id) = debouncer.replace(user_id, query_id) {
        let _ = bot.answer_inline_query(prev_query_id, vec![])
            .cache_time(0)
            .await;
    }
}

pub async fn debounce_inline_queries(
    bot: Bot,
    InlineQuery { from, id, .. }: InlineQuery,
    debouncer: InlineQueryDebouncer,
) -> bool {
    tokio::join!(
        cancel_old_query(bot, from.id, id.clone(), debouncer.clone()),
        tokio::time::sleep(debouncer.duration)
    );

    let stored = debouncer.get(from.id);
    log::info!("[DEBOUNCING] CHECK {:?} // {:?} = {:?}", from.id, id, stored);
    stored == Some(id)
}