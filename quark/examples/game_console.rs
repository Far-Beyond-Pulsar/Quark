//! Complete game console example demonstrating real-world usage of Quark.
//!
//! This example simulates a game debug console with various commands
//! for controlling game state, spawning entities, and managing resources.
//!
//! Run with: `cargo run --example game_console`

use quark::{command, Quark};
use std::sync::{Arc, Mutex};

// Simulated game state
struct GameState {
    player_health: i32,
    player_position: (f32, f32),
    entities_spawned: usize,
    god_mode: bool,
}

impl GameState {
    fn new() -> Self {
        Self {
            player_health: 100,
            player_position: (0.0, 0.0),
            entities_spawned: 0,
            god_mode: false,
        }
    }
}

// We'll use a thread-safe reference to game state
static mut GAME_STATE: Option<Arc<Mutex<GameState>>> = None;

fn get_game_state() -> Arc<Mutex<GameState>> {
    unsafe {
        GAME_STATE
            .as_ref()
            .expect("Game state not initialized")
            .clone()
    }
}

// Game commands
#[command(
    name = "teleport",
    syntax = "teleport <x> <y>",
    short = "Teleport the player to a position",
    docs = "Example: `teleport 100 50` moves the player to coordinates (100, 50)"
)]
fn teleport(x: f32, y: f32) {
    let state = get_game_state();
    let mut state = state.lock().unwrap();
    state.player_position = (x, y);
    println!("✈ Teleported player to ({}, {})", x, y);
}

#[command(
    name = "spawn",
    syntax = "spawn <entity> <count>",
    short = "Spawn entities into the world",
    docs = "Example: `spawn goblin 5` spawns 5 goblins near the player"
)]
fn spawn_entity(entity: String, count: usize) {
    let state = get_game_state();
    let mut state = state.lock().unwrap();
    state.entities_spawned += count;
    println!("⚔ Spawned {} {}(s) (Total entities: {})", count, entity, state.entities_spawned);
}

#[command(
    name = "heal",
    syntax = "heal <amount>",
    short = "Heal the player",
    docs = "Example: `heal 50` restores 50 health points"
)]
fn heal(amount: i32) {
    let state = get_game_state();
    let mut state = state.lock().unwrap();
    state.player_health = (state.player_health + amount).min(100);
    println!("❤ Player healed by {} HP (Current: {} HP)", amount, state.player_health);
}

#[command(
    name = "damage",
    syntax = "damage <amount>",
    short = "Damage the player",
    docs = "Example: `damage 20` deals 20 damage to the player"
)]
fn damage(amount: i32) {
    let state = get_game_state();
    let mut state = state.lock().unwrap();

    if state.god_mode {
        println!("⚠ God mode active - no damage taken");
        return;
    }

    state.player_health = (state.player_health - amount).max(0);
    println!("💔 Player took {} damage (Current: {} HP)", amount, state.player_health);

    if state.player_health == 0 {
        println!("💀 Player has died!");
    }
}

#[command(
    name = "god",
    syntax = "god <on|off>",
    short = "Toggle god mode",
    docs = "Example: `god on` enables invincibility, `god off` disables it"
)]
fn god_mode(state: String) {
    let game_state = get_game_state();
    let mut game_state = game_state.lock().unwrap();

    game_state.god_mode = match state.to_lowercase().as_str() {
        "on" | "true" | "1" => {
            println!("🛡 God mode ENABLED - you are invincible!");
            true
        }
        "off" | "false" | "0" => {
            println!("⚔ God mode DISABLED - you can take damage");
            false
        }
        _ => {
            println!("❌ Invalid state. Use 'on' or 'off'");
            game_state.god_mode
        }
    };
}

#[command(
    name = "status",
    syntax = "status",
    short = "Show player status",
    docs = "Displays current player health, position, and game state"
)]
fn show_status() {
    let state = get_game_state();
    let state = state.lock().unwrap();

    println!("📊 === Player Status ===");
    println!("  Health: {} HP", state.player_health);
    println!("  Position: ({:.1}, {:.1})", state.player_position.0, state.player_position.1);
    println!("  God Mode: {}", if state.god_mode { "ON" } else { "OFF" });
    println!("  Entities Spawned: {}", state.entities_spawned);
}

#[command(
    name = "clear",
    syntax = "clear",
    short = "Clear all spawned entities",
    docs = "Removes all entities from the world"
)]
fn clear_entities() {
    let state = get_game_state();
    let mut state = state.lock().unwrap();
    let count = state.entities_spawned;
    state.entities_spawned = 0;
    println!("🧹 Cleared {} entities from the world", count);
}

#[command(
    name = "reset",
    syntax = "reset",
    short = "Reset game state",
    docs = "Resets player to full health at origin with god mode off"
)]
fn reset_game() {
    let state = get_game_state();
    let mut state = state.lock().unwrap();
    state.player_health = 100;
    state.player_position = (0.0, 0.0);
    state.god_mode = false;
    state.entities_spawned = 0;
    println!("🔄 Game state reset to default");
}

#[command(
    name = "help",
    syntax = "help",
    short = "Show available commands",
    docs = "Displays all available console commands"
)]
fn show_help() {
    println!("📖 === Available Commands ===");
    println!("  teleport <x> <y>     - Teleport to position");
    println!("  spawn <entity> <n>   - Spawn n entities");
    println!("  heal <amount>        - Restore health");
    println!("  damage <amount>      - Take damage");
    println!("  god <on|off>         - Toggle god mode");
    println!("  status               - Show player stats");
    println!("  clear                - Remove all entities");
    println!("  reset                - Reset game state");
    println!("  help                 - Show this message");
}

fn main() {
    // Initialize game state
    unsafe {
        GAME_STATE = Some(Arc::new(Mutex::new(GameState::new())));
    }

    // Create command registry
    let mut registry = Quark::new();

    // Register all commands (using generated struct names based on function names)
    registry.register_command(TeleportCommand);
    registry.register_command(Spawn_entityCommand);
    registry.register_command(HealCommand);
    registry.register_command(DamageCommand);
    registry.register_command(God_modeCommand);
    registry.register_command(Show_statusCommand);
    registry.register_command(Clear_entitiesCommand);
    registry.register_command(Reset_gameCommand);
    registry.register_command(Show_helpCommand);

    println!("🎮 === Quark Game Console Example ===\n");

    // Show help
    registry.run("help").unwrap();
    println!();

    // Demonstrate a gameplay scenario
    println!("🎬 === Gameplay Scenario ===\n");

    registry.run("status").unwrap();
    println!();

    registry.run("spawn goblin 3").unwrap();
    registry.run("spawn dragon 1").unwrap();
    println!();

    registry.run("teleport 50 25").unwrap();
    println!();

    registry.run("damage 30").unwrap();
    println!();

    registry.run("heal 15").unwrap();
    println!();

    registry.run("god on").unwrap();
    registry.run("damage 50").unwrap();
    println!();

    registry.run("god off").unwrap();
    registry.run("damage 50").unwrap();
    println!();

    registry.run("status").unwrap();
    println!();

    registry.run("reset").unwrap();
    registry.run("status").unwrap();

    println!("\n✅ Example completed!");
}
