//! # Quark
//!
//! **Tiny commands, massive impact — fully type-safe and macro-powered for Pulsar.**
//!
//! Quark is a **high-performance, type-safe command system** for the **Pulsar game engine**.
//! It allows developers to register and execute commands with **compile-time argument checking**,
//! rich documentation, and **async support**, all powered by **ergonomic Rust macros**.
//!
//! ## Features
//!
//! * ⚡ **Type-Safe Arguments** — Commands define their argument types at compile time.
//! * ✨ **Macro-Powered Registration** — Register commands in one line, with zero boilerplate.
//! * 📚 **Integrated Documentation** — Short descriptions and detailed docs for every command.
//! * 🔄 **Async & Sync Support** — Works with both synchronous and asynchronous functions.
//! * 🛠 **Engine-Agnostic** — Can be used anywhere in Pulsar: scripts, core modules, debug consoles.
//! * 🧩 **Composable & Modular** — Commands can be organized per module, globally queryable.
//!
//! ## Quick Start
//!
//! ```rust
//! use quark::{Quark, Command, CommandError};
//! use std::pin::Pin;
//! use std::future::Future;
//!
//! // Define a command by implementing the Command trait
//! struct SpawnCommand;
//!
//! impl Command for SpawnCommand {
//!     fn name(&self) -> &str {
//!         "spawn"
//!     }
//!
//!     fn syntax(&self) -> &str {
//!         "spawn <entity> <count>"
//!     }
//!
//!     fn short(&self) -> &str {
//!         "Spawn entities into the game world"
//!     }
//!
//!     fn docs(&self) -> &str {
//!         "Example: `spawn goblin 5` spawns 5 goblins at the current location"
//!     }
//!
//!     fn execute(&self, args: Vec<String>) -> Result<(), CommandError> {
//!         if args.len() != 2 {
//!             return Err(CommandError::ArgumentCountMismatch {
//!                 expected: 2,
//!                 got: args.len(),
//!             });
//!         }
//!
//!         let entity = &args[0];
//!         let count: usize = args[1].parse()
//!             .map_err(|_| CommandError::TypeConversionError {
//!                 arg: args[1].clone(),
//!                 target_type: "usize",
//!             })?;
//!
//!         for _ in 0..count {
//!             println!("Spawning {}", entity);
//!         }
//!         Ok(())
//!     }
//! }
//!
//! fn main() {
//!     let mut registry = Quark::new();
//!     registry.register_command(SpawnCommand);
//!
//!     // Execute commands
//!     registry.run("spawn goblin 3").unwrap();
//! }
//! ```
//!
//! ## Using the Macro
//!
//! With the `#[command]` macro, you can write:
//!
//! ```rust
//! use quark::{Quark, command};
//!
//! #[command(
//!     name = "spawn",
//!     syntax = "spawn <entity> <count>",
//!     short = "Spawn entities into the game world",
//!     docs = "Example: `spawn goblin 5` spawns 5 goblins at the current location"
//! )]
//! fn spawn(entity: String, count: usize) {
//!     for _ in 0..count {
//!         println!("Spawning {}", entity);
//!     }
//! }
//!
//! fn main() {
//!     let mut registry = Quark::new();
//!     // Register using the generated SpawnCommand struct
//!     registry.register_command(SpawnCommand);
//!
//!     // Execute the command
//!     registry.run("spawn goblin 3").unwrap();
//!
//!     // You can still call the function directly
//!     spawn("orc".to_string(), 2);
//! }
//! ```
//!
//! ## Async Support
//!
//! Quark supports both synchronous and asynchronous commands:
//!
//! ```rust
//! use quark::{Quark, Command, CommandError};
//! use std::pin::Pin;
//! use std::future::Future;
//!
//! struct AsyncCommand;
//!
//! impl Command for AsyncCommand {
//!     fn name(&self) -> &str { "async_example" }
//!     fn syntax(&self) -> &str { "async_example" }
//!     fn short(&self) -> &str { "An async command" }
//!     fn docs(&self) -> &str { "This command runs asynchronously" }
//!     fn is_async(&self) -> bool { true }
//!
//!     fn execute_async<'a>(&'a self, _args: Vec<String>)
//!         -> Pin<Box<dyn Future<Output = Result<(), CommandError>> + Send + 'a>>
//!     {
//!         Box::pin(async {
//!             println!("Running async command");
//!             Ok(())
//!         })
//!     }
//! }
//!
//! # pollster::block_on(async {
//! let mut registry = Quark::new();
//! registry.register_command(AsyncCommand);
//! registry.run_async("async_example").await.unwrap();
//! # });
//! ```

// Re-export the procedural macro
pub use quark_macros::command;

mod command;
mod error;
mod metadata;
mod parser;
mod registry;

// Public exports
pub use command::Command;
pub use error::{CommandError, Result};
pub use metadata::CommandMetadata;
pub use parser::parse_command_string;
pub use registry::Quark;
