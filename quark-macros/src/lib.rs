//! Procedural macros for the Quark command system.
//!
//! This crate provides the `#[command]` attribute macro that automatically
//! generates Command trait implementations from annotated functions.

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

mod command;

/// Attribute macro for defining commands with automatic Command trait implementation.
///
/// This macro takes a function and generates a Command implementation that:
/// - Stores the provided metadata (name, syntax, short description, documentation)
/// - Parses string arguments and converts them to the expected types
/// - Calls the original function with type-safe arguments
/// - Handles both sync and async functions automatically
///
/// # Attributes
///
/// - `name`: The command name (required)
/// - `syntax`: The command syntax string (required)
/// - `short`: Short description (required)
/// - `docs`: Detailed documentation (required)
///
/// # Examples
///
/// ```ignore
/// use quark::command;
///
/// #[command(
///     name = "spawn",
///     syntax = "spawn <entity> <count>",
///     short = "Spawn entities into the game world",
///     docs = "Example: `spawn goblin 5` spawns 5 goblins at the current location"
/// )]
/// fn spawn(entity: String, count: usize) {
///     for _ in 0..count {
///         println!("Spawning {}", entity);
///     }
/// }
///
/// // Async example
/// #[command(
///     name = "teleport",
///     syntax = "teleport <x> <y> <z>",
///     short = "Teleport the player",
///     docs = "Example: `teleport 10 20 5` moves the player to coordinates (10, 20, 5)"
/// )]
/// async fn teleport(x: f32, y: f32, z: f32) {
///     println!("Teleporting to ({}, {}, {})", x, y, z);
/// }
/// ```
#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Generate the command implementation
    match command::generate_command_impl(attr.into(), input_fn) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
