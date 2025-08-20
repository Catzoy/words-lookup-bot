use shuttle_runtime::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use teloxide::types::{Me, Message};
use teloxide::Bot;

#[async_trait]
pub trait Command: Sync + Send {
    fn name(&self) -> &'static str;
    async fn handle(&self, me: &Me, bot: &Bot, message: &Message, args: Vec<String>) -> anyhow::Result<()>;
}

pub type CommandRef = (dyn Command);
pub type BoxedCommand = Arc<CommandRef>;
#[derive(Clone)]
pub struct CommandsRegistry {
    unknown_command: BoxedCommand,
    registry: HashMap<&'static str, BoxedCommand>,
}

impl CommandsRegistry {
    pub(crate) fn new<T: Command + 'static>(
        unknown_command: T,
    ) -> CommandsRegistry {
        CommandsRegistry {
            unknown_command: Arc::new(unknown_command),
            registry: HashMap::new(),
        }
    }

    pub(crate) fn insert<T: Command + 'static>(&mut self, command: T) {
        self.registry.insert(command.name(), Arc::new(command));
    }

    pub(crate) fn get(&self, name: String) -> &BoxedCommand {
        self.registry.get(name.as_str()).unwrap_or_else(|| &self.unknown_command)
    }
}