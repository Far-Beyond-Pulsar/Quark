//! Interactive game console with live command input.
//!
//! This example creates a fully interactive REPL (Read-Eval-Print Loop)
//! where you can type commands and see the results in real-time.
//!
//! Run with: `cargo run --example interactive_console`

use quark::{command, Quark};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

// Game state
#[derive(Clone)]
struct Player {
    name: String,
    health: i32,
    max_health: i32,
    mana: i32,
    max_mana: i32,
    level: u32,
    position: (i32, i32),
    gold: u32,
}

impl Player {
    fn new(name: String) -> Self {
        Self {
            name,
            health: 100,
            max_health: 100,
            mana: 50,
            max_mana: 50,
            level: 1,
            position: (0, 0),
            gold: 0,
        }
    }
}

struct GameWorld {
    player: Player,
    turn: u32,
    enemies_defeated: u32,
    items_collected: u32,
}

impl GameWorld {
    fn new(player_name: String) -> Self {
        Self {
            player: Player::new(player_name),
            turn: 0,
            enemies_defeated: 0,
            items_collected: 0,
        }
    }
}

// Global game state (for simplicity in this example)
static mut GAME_WORLD: Option<Arc<Mutex<GameWorld>>> = None;

fn get_game() -> Arc<Mutex<GameWorld>> {
    unsafe { GAME_WORLD.as_ref().expect("Game not initialized").clone() }
}

// === Command Definitions ===

#[command(
    name = "stats",
    syntax = "stats",
    short = "Display player statistics",
    docs = "Shows your current health, mana, level, position, and gold"
)]
fn show_stats() {
    let game = get_game();
    let game = game.lock().unwrap();
    let p = &game.player;

    println!("\n╔════════════════════════════════════╗");
    println!("║        PLAYER STATISTICS           ║");
    println!("╠════════════════════════════════════╣");
    println!("║ Name:     {:<24} ║", p.name);
    println!("║ Level:    {:<24} ║", p.level);
    println!("║ Health:   {}/{:<20} ║", p.health, p.max_health);
    println!("║ Mana:     {}/{:<20} ║", p.mana, p.max_mana);
    println!("║ Position: ({}, {})                    ║", p.position.0, p.position.1);
    println!("║ Gold:     {:<24} ║", p.gold);
    println!("╠════════════════════════════════════╣");
    println!("║ Enemies Defeated: {:<16} ║", game.enemies_defeated);
    println!("║ Items Collected:  {:<16} ║", game.items_collected);
    println!("║ Turn:             {:<16} ║", game.turn);
    println!("╚════════════════════════════════════╝\n");
}

#[command(
    name = "move",
    syntax = "move <direction>",
    short = "Move in a direction",
    docs = "Valid directions: north, south, east, west, n, s, e, w"
)]
fn move_player(direction: String) {
    let game = get_game();
    let mut game = game.lock().unwrap();

    let (dx, dy) = match direction.to_lowercase().as_str() {
        "north" | "n" => (0, 1),
        "south" | "s" => (0, -1),
        "east" | "e" => (1, 0),
        "west" | "w" => (-1, 0),
        _ => {
            println!("❌ Invalid direction! Use: north, south, east, west (or n/s/e/w)");
            return;
        }
    };

    game.player.position.0 += dx;
    game.player.position.1 += dy;
    game.turn += 1;

    println!("🚶 You move {} to ({}, {})", direction, game.player.position.0, game.player.position.1);

    // Random encounter chance
    if rand_chance(30) {
        println!("⚔️  A wild enemy appears!");
    } else if rand_chance(20) {
        println!("✨ You found an item!");
    }
}

#[command(
    name = "attack",
    syntax = "attack <enemy>",
    short = "Attack an enemy",
    docs = "Example: attack goblin"
)]
fn attack(enemy: String) {
    let game = get_game();
    let mut game = game.lock().unwrap();

    let damage = 10 + (game.player.level * 5) as i32;
    let mana_cost = 5;

    if game.player.mana < mana_cost {
        println!("❌ Not enough mana! (Need {} mana)", mana_cost);
        return;
    }

    game.player.mana -= mana_cost;
    game.enemies_defeated += 1;
    game.turn += 1;

    let gold_reward = 10 + (rand_num(20) as u32);
    game.player.gold += gold_reward;

    println!("⚔️  You attack the {}!", enemy);
    println!("💥 You deal {} damage!", damage);
    println!("💀 The {} is defeated!", enemy);
    println!("💰 You gain {} gold!", gold_reward);

    if game.enemies_defeated % 5 == 0 {
        game.player.level += 1;
        game.player.max_health += 20;
        game.player.health = game.player.max_health;
        game.player.max_mana += 10;
        game.player.mana = game.player.max_mana;
        println!("🎉 Level up! You are now level {}!", game.player.level);
        println!("✨ Health and mana fully restored!");
    }
}

#[command(
    name = "rest",
    syntax = "rest",
    short = "Rest to recover health and mana",
    docs = "Restores 30 HP and 20 mana"
)]
fn rest() {
    let game = get_game();
    let mut game = game.lock().unwrap();

    let hp_restore = 30.min(game.player.max_health - game.player.health);
    let mana_restore = 20.min(game.player.max_mana - game.player.mana);

    game.player.health = (game.player.health + hp_restore).min(game.player.max_health);
    game.player.mana = (game.player.mana + mana_restore).min(game.player.max_mana);
    game.turn += 1;

    println!("😴 You rest for a moment...");
    println!("❤️  Restored {} health", hp_restore);
    println!("💙 Restored {} mana", mana_restore);
}

#[command(
    name = "collect",
    syntax = "collect",
    short = "Collect items in the area",
    docs = "Search the area for items and gold"
)]
fn collect() {
    let game = get_game();
    let mut game = game.lock().unwrap();

    if rand_chance(60) {
        let gold = 5 + rand_num(15) as u32;
        game.player.gold += gold;
        game.items_collected += 1;
        game.turn += 1;
        println!("✨ You found {} gold!", gold);
    } else {
        game.turn += 1;
        println!("🔍 You search around but find nothing...");
    }
}

#[command(
    name = "shop",
    syntax = "shop",
    short = "Visit the traveling merchant",
    docs = "Buy health potions (50g) or mana potions (30g)"
)]
fn shop() {
    let game = get_game();
    let game = game.lock().unwrap();

    println!("\n🏪 ═══ TRAVELING MERCHANT ═══");
    println!("  Your gold: {} 💰", game.player.gold);
    println!("  ────────────────────────");
    println!("  [1] Health Potion - 50g (+50 HP)");
    println!("  [2] Mana Potion   - 30g (+30 Mana)");
    println!("  ────────────────────────");
    println!("  Use: buy health  or  buy mana");
}

#[command(
    name = "buy",
    syntax = "buy <item>",
    short = "Buy an item from the shop",
    docs = "Example: buy health  or  buy mana"
)]
fn buy(item: String) {
    let game = get_game();
    let mut game = game.lock().unwrap();

    match item.to_lowercase().as_str() {
        "health" | "hp" => {
            if game.player.gold >= 50 {
                game.player.gold -= 50;
                game.player.health = (game.player.health + 50).min(game.player.max_health);
                println!("💊 Purchased Health Potion! (+50 HP)");
            } else {
                println!("❌ Not enough gold! (Need 50g, have {}g)", game.player.gold);
            }
        }
        "mana" | "mp" => {
            if game.player.gold >= 30 {
                game.player.gold -= 30;
                game.player.mana = (game.player.mana + 30).min(game.player.max_mana);
                println!("💊 Purchased Mana Potion! (+30 Mana)");
            } else {
                println!("❌ Not enough gold! (Need 30g, have {}g)", game.player.gold);
            }
        }
        _ => println!("❌ Unknown item! Use: buy health  or  buy mana"),
    }
}

#[command(
    name = "map",
    syntax = "map",
    short = "Show a map of your surroundings",
    docs = "Displays a small map centered on your position"
)]
fn show_map() {
    let game = get_game();
    let game = game.lock().unwrap();
    let (px, py) = game.player.position;

    println!("\n🗺️  ═══ MAP ═══");
    for y in (py - 2)..=(py + 2) {
        print!("  ");
        for x in (px - 2)..=(px + 2) {
            if x == px && y == py {
                print!("[@]");  // Player
            } else if (x + y) % 3 == 0 {
                print!("[🌲]");  // Tree
            } else if (x - y) % 4 == 0 {
                print!("[⛰️ ]");  // Mountain
            } else {
                print!("[ ]");   // Empty
            }
        }
        println!();
    }
    println!();
}

#[command(
    name = "help",
    syntax = "help",
    short = "Show available commands",
    docs = "Lists all available commands and their descriptions"
)]
fn help() {
    println!("\n📖 ═══ AVAILABLE COMMANDS ═══");
    println!("  stats              - View your character stats");
    println!("  move <direction>   - Move (n/s/e/w or north/south/east/west)");
    println!("  attack <enemy>     - Attack an enemy");
    println!("  rest               - Recover health and mana");
    println!("  collect            - Search for items");
    println!("  shop               - Visit merchant");
    println!("  buy <item>         - Buy item (health/mana)");
    println!("  map                - Show map");
    println!("  help               - Show this help");
    println!("  exit/quit          - Exit game");
    println!();
}

// Helper functions
fn rand_num(max: u32) -> u32 {
    // Simple pseudo-random based on time
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (now.as_nanos() % max as u128) as u32
}

fn rand_chance(percent: u32) -> bool {
    rand_num(100) < percent
}

fn main() {
    // Print welcome banner
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║                                                        ║");
    println!("║        ⚔️  QUARK QUEST - Interactive Console  ⚔️        ║");
    println!("║                                                        ║");
    println!("║   A text-based RPG powered by the Quark command       ║");
    println!("║   system. Type commands to explore, fight, and        ║");
    println!("║   collect treasure!                                   ║");
    println!("║                                                        ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    // Get player name
    print!("Enter your character name: ");
    io::stdout().flush().unwrap();
    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name).unwrap();
    let player_name = player_name.trim().to_string();

    if player_name.is_empty() {
        println!("Using default name: Adventurer");
        unsafe {
            GAME_WORLD = Some(Arc::new(Mutex::new(GameWorld::new("Adventurer".to_string()))));
        }
    } else {
        println!("\nWelcome, {}! Your adventure begins...\n", player_name);
        unsafe {
            GAME_WORLD = Some(Arc::new(Mutex::new(GameWorld::new(player_name))));
        }
    }

    // Create command registry
    let mut registry = Quark::new();

    // Register all commands
    registry.register_command(Show_statsCommand);
    registry.register_command(Move_playerCommand);
    registry.register_command(AttackCommand);
    registry.register_command(RestCommand);
    registry.register_command(CollectCommand);
    registry.register_command(ShopCommand);
    registry.register_command(BuyCommand);
    registry.register_command(Show_mapCommand);
    registry.register_command(HelpCommand);

    println!("Type 'help' to see available commands.\n");

    // Main game loop - REPL (Read-Eval-Print Loop)
    loop {
        // Display prompt
        print!("⚔️  > ");
        io::stdout().flush().unwrap();

        // Read input
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }

        let input = input.trim();

        // Check for exit commands
        if input.is_empty() {
            continue;
        }

        if input == "exit" || input == "quit" {
            println!("\n🎮 Thanks for playing Quark Quest!");

            let game = get_game();
            let game = game.lock().unwrap();
            println!("\n╔════════════════════════════════════╗");
            println!("║         FINAL STATISTICS           ║");
            println!("╠════════════════════════════════════╣");
            println!("║ Final Level:      {:<16} ║", game.player.level);
            println!("║ Enemies Defeated: {:<16} ║", game.enemies_defeated);
            println!("║ Items Collected:  {:<16} ║", game.items_collected);
            println!("║ Gold Earned:      {:<16} ║", game.player.gold);
            println!("║ Turns Taken:      {:<16} ║", game.turn);
            println!("╚════════════════════════════════════╝\n");

            break;
        }

        // Execute command
        match registry.run(input) {
            Ok(_) => {
                // Command executed successfully
            }
            Err(e) => {
                println!("❌ Error: {}", e);
                println!("   Type 'help' to see available commands.");
            }
        }

        println!(); // Empty line for readability
    }
}
