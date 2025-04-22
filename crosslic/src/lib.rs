// --- re-export macro and inventory ---
pub use crosslic_macro::command;
pub use inventory;

// --- modules ---
pub mod api;
pub mod server;
pub mod websocket;

// --- what's in the root namespace ---
pub use api::{COMMANDS, CommandDescriptor, CommandHandler};
pub use server::run_app;
pub use websocket::handle_websocket;
