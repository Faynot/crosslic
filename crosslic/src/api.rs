use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde_json::Value;
use std::collections::HashMap;

/// Command handler type
pub type CommandHandler = fn(Value) -> Result<Value, String>;

/// Single command descriptor for inventory
pub struct CommandDescriptor {
    pub name: &'static str,
    pub handler: CommandHandler,
}

// We collect all descriptors from different modules
inventory::collect!(CommandDescriptor);

/// Command registry
pub struct CommandRegistry {
    handlers: HashMap<String, CommandHandler>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut reg = CommandRegistry {
            handlers: HashMap::new(),
        };
        // We automatically register everything from inventory
        for descr in inventory::iter::<CommandDescriptor> {
            reg.register(descr.name, descr.handler);
        }
        reg
    }

    pub fn register(&mut self, name: &str, handler: CommandHandler) {
        self.handlers.insert(name.to_string(), handler);
    }

    pub fn handle(&self, command: &str, data: Value) -> Result<Value, String> {
        match self.handlers.get(command) {
            Some(h) => h(data),
            None => Err(format!("Command '{}' not found", command)),
        }
    }
}

lazy_static! {
    /// Global, thread-safe registry
    pub static ref COMMANDS: Mutex<CommandRegistry> =
        Mutex::new(CommandRegistry::new());
}
