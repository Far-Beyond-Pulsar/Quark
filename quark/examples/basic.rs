//! Basic example demonstrating the Quark command system.
//!
//! Run with: `cargo run --example basic`

use quark::{command, Quark};

#[command(
    name = "spawn",
    syntax = "spawn <entity> <count>",
    short = "Spawn entities into the game world",
    docs = "Example: `spawn goblin 5` spawns 5 goblins at the current location"
)]
fn spawn(entity: String, count: usize) {
    for i in 0..count {
        println!("[Spawn {}] {}", i + 1, entity);
    }
}

#[command(
    name = "greet",
    syntax = "greet <name>",
    short = "Greet a player",
    docs = "Example: `greet Alice` prints a greeting message"
)]
fn greet(name: String) {
    println!("Hello, {}! Welcome to the game!", name);
}

#[command(
    name = "move",
    syntax = "move <x> <y>",
    short = "Move to a position",
    docs = "Example: `move 10 20` moves to coordinates (10, 20)"
)]
fn move_to(x: i32, y: i32) {
    println!("Moving to position ({}, {})", x, y);
}

#[command(
    name = "help",
    syntax = "help",
    short = "Show available commands",
    docs = "Displays all registered commands with their descriptions"
)]
fn help() {
    println!("Available commands:");
    println!("  spawn <entity> <count> - Spawn entities");
    println!("  greet <name>          - Greet a player");
    println!("  move <x> <y>          - Move to a position");
    println!("  help                   - Show this help message");
}

fn main() {
    // Create the command registry
    let mut registry = Quark::new();

    // Register commands using the generated command structs
    registry.register_command(SpawnCommand);
    registry.register_command(GreetCommand);
    registry.register_command(Move_toCommand);  // Generated from move_to function
    registry.register_command(HelpCommand);

    println!("=== Quark Basic Example ===\n");

    // List all registered commands
    println!("Registered commands:");
    for cmd in registry.list() {
        println!("  {}: {}", cmd.name(), cmd.short());
    }
    println!();

    // Execute some commands
    println!("Executing commands:\n");

    registry.run("help").unwrap();
    println!();

    registry.run("spawn goblin 3").unwrap();
    println!();

    registry.run("greet Alice").unwrap();
    println!();

    registry.run("move 10 20").unwrap();
    println!();

    // You can also call the functions directly
    println!("Direct function call:");
    spawn("dragon".to_string(), 1);
    println!();

    // Show documentation for a specific command
    if let Some(docs) = registry.get_docs("spawn") {
        println!("Documentation for 'spawn': {}", docs);
    }

    // Demonstrate error handling
    println!("\n=== Error Handling ===\n");

    match registry.run("nonexistent command") {
        Ok(_) => println!("Command executed successfully"),
        Err(e) => println!("Error: {}", e),
    }

    match registry.run("spawn invalid_count") {
        Ok(_) => println!("Command executed successfully"),
        Err(e) => println!("Error: {}", e),
    }

    match registry.run("move 10") {
        Ok(_) => println!("Command executed successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
