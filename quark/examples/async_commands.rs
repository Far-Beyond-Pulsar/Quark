//! Async command example demonstrating async support in Quark.
//!
//! Run with: `cargo run --example async_commands`

use quark::{command, Quark};
use std::time::Duration;
use tokio::time::sleep;

#[command(
    name = "download",
    syntax = "download <file>",
    short = "Download a file",
    docs = "Example: `download game_data.pak` simulates downloading a file"
)]
async fn download(file: String) {
    println!("Starting download of {}...", file);
    sleep(Duration::from_millis(500)).await;
    println!("Downloaded {} successfully!", file);
}

#[command(
    name = "save",
    syntax = "save <slot>",
    short = "Save the game",
    docs = "Example: `save 1` saves the game to slot 1"
)]
async fn save_game(slot: usize) {
    println!("Saving game to slot {}...", slot);
    sleep(Duration::from_millis(300)).await;
    println!("Game saved to slot {}!", slot);
}

#[command(
    name = "sync_cmd",
    syntax = "sync_cmd",
    short = "A synchronous command",
    docs = "This is a regular sync command for comparison"
)]
fn sync_command() {
    println!("This is a synchronous command - executes immediately");
}

#[tokio::main]
async fn main() {
    let mut registry = Quark::new();

    // Register both async and sync commands
    registry.register_command(DownloadCommand);
    registry.register_command(Save_gameCommand);  // Generated from save_game function
    registry.register_command(Sync_commandCommand);  // Generated from sync_command function

    println!("=== Quark Async Example ===\n");

    // List all commands
    println!("Registered commands:");
    for cmd in registry.list() {
        let cmd_type = if cmd.is_async() { "async" } else { "sync" };
        println!("  {} [{}]: {}", cmd.name(), cmd_type, cmd.short());
    }
    println!();

    // Execute async commands using run_async()
    println!("Executing async commands:\n");

    registry.run_async("download map_pack.zip").await.unwrap();
    println!();

    registry.run_async("save 1").await.unwrap();
    println!();

    // Sync commands also work with run_async()
    registry.run_async("sync_cmd").await.unwrap();
    println!();

    // Demonstrate error: calling async command with sync run()
    println!("=== Error Handling ===\n");

    match registry.run("download should_fail.dat") {
        Ok(_) => println!("Command executed"),
        Err(e) => println!("Expected error: {}", e),
    }

    // You can still call async functions directly
    println!("\nDirect async function call:");
    download("asset_bundle.zip".to_string()).await;
}
