# Quark Quick Start Guide

**Tiny commands, massive impact — fully type-safe and macro-powered for Pulsar.**

## Installation

Add Quark to your `Cargo.toml`:

```toml
[dependencies]
quark = { path = "../quark" }  # Replace with crates.io version when published
```

## Basic Usage

### 1. Define Commands with the Macro

```rust
use quark::{command, Quark};

#[command(
    name = "greet",
    syntax = "greet <name>",
    short = "Greet someone",
    docs = "Example: greet Alice"
)]
fn greet(name: String) {
    println!("Hello, {}!", name);
}

#[command(
    name = "add",
    syntax = "add <a> <b>",
    short = "Add two numbers",
    docs = "Example: add 5 10"
)]
fn add(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}
```

### 2. Register and Execute Commands

```rust
fn main() {
    let mut registry = Quark::new();

    // Register commands using the generated structs
    registry.register_command(GreetCommand);
    registry.register_command(AddCommand);

    // Execute commands from strings
    registry.run("greet World").unwrap();  // Output: Hello, World!
    registry.run("add 5 10").unwrap();      // Output: 5 + 10 = 15
}
```

## Features

### ✅ Type Safety

Arguments are parsed and type-checked automatically:

```rust
#[command(
    name = "teleport",
    syntax = "teleport <x> <y> <z>",
    short = "Teleport to coordinates",
    docs = "Example: teleport 10.5 20.0 5.5"
)]
fn teleport(x: f32, y: f32, z: f32) {
    println!("Teleporting to ({}, {}, {})", x, y, z);
}
```

If you run `teleport abc def ghi`, you'll get a helpful error:
```
Error: Type conversion error: could not convert 'abc' to f32
```

### ✅ Async Support

Async functions work seamlessly:

```rust
#[command(
    name = "save",
    syntax = "save <slot>",
    short = "Save game",
    docs = "Example: save 1"
)]
async fn save_game(slot: usize) {
    println!("Saving to slot {}...", slot);
    // Async operations here
    tokio::time::sleep(Duration::from_millis(100)).await;
    println!("Game saved!");
}

#[tokio::main]
async fn main() {
    let mut registry = Quark::new();
    registry.register_command(Save_gameCommand);

    // Use run_async for async commands
    registry.run_async("save 1").await.unwrap();
}
```

### ✅ Command Discovery

List and query registered commands:

```rust
// List all commands
for cmd in registry.list() {
    println!("{}: {}", cmd.name(), cmd.short());
}

// Get documentation for a specific command
if let Some(docs) = registry.get_docs("greet") {
    println!("Documentation: {}", docs);
}
```

### ✅ Quoted String Arguments

The parser handles quoted strings with spaces:

```rust
#[command(
    name = "say",
    syntax = "say <message>",
    short = "Say something",
    docs = "Example: say \"hello world\""
)]
fn say(message: String) {
    println!("You say: {}", message);
}

// Usage:
registry.run("say \"hello world\"").unwrap();  // Works!
```

## Examples

Run the included examples to see Quark in action:

### Basic Example
```bash
cargo run --example basic
```
Shows command registration, execution, and error handling.

### Async Example
```bash
cargo run --example async_commands
```
Demonstrates mixing sync and async commands.

### Game Console Example
```bash
cargo run --example game_console
```
A complete game debug console simulation.

### Interactive Console (REPL)
```bash
cargo run --example interactive_console
```
**⭐ Recommended!** A fully playable text-based RPG with an interactive command-line interface.

Example session:
```
Enter your character name: Hero

⚔️  > help
📖 ═══ AVAILABLE COMMANDS ═══
  stats              - View your character stats
  move <direction>   - Move (n/s/e/w)
  attack <enemy>     - Attack an enemy
  rest               - Recover health and mana
  ...

⚔️  > stats
╔════════════════════════════════════╗
║        PLAYER STATISTICS           ║
╠════════════════════════════════════╣
║ Name:     Hero                     ║
║ Level:    1                        ║
║ Health:   100/100                  ║
...

⚔️  > move north
🚶 You move north to (0, 1)
⚔️  A wild enemy appears!

⚔️  > attack goblin
⚔️  You attack the goblin!
💥 You deal 15 damage!
💀 The goblin is defeated!
💰 You gain 18 gold!
```

## Advanced Usage

### Custom Command Implementations

You can also implement the `Command` trait manually for full control:

```rust
use quark::{Command, CommandError, Result};

struct CustomCommand;

impl Command for CustomCommand {
    fn name(&self) -> &str { "custom" }
    fn syntax(&self) -> &str { "custom <arg>" }
    fn short(&self) -> &str { "Custom command" }
    fn docs(&self) -> &str { "A manually implemented command" }

    fn execute(&self, args: Vec<String>) -> Result<()> {
        // Custom argument parsing and execution logic
        println!("Custom command with args: {:?}", args);
        Ok(())
    }
}

// Register like any other command
registry.register_command(CustomCommand);
```

### Error Handling

Quark provides detailed error types:

```rust
use quark::CommandError;

match registry.run("unknown command") {
    Ok(_) => println!("Success!"),
    Err(CommandError::NotFound(name)) => {
        println!("Command '{}' not found", name);
    },
    Err(CommandError::ArgumentCountMismatch { expected, got }) => {
        println!("Expected {} args, got {}", expected, got);
    },
    Err(CommandError::TypeConversionError { arg, target_type }) => {
        println!("Cannot convert '{}' to {}", arg, target_type);
    },
    Err(e) => println!("Error: {}", e),
}
```

### Supported Argument Types

Any type implementing `FromStr` can be used as a command argument:

- **Primitives**: `i32`, `u32`, `f32`, `f64`, `bool`, etc.
- **Strings**: `String`, `&str` (note: use `String` for command args)
- **Custom types**: Implement `FromStr` for your own types

```rust
use std::str::FromStr;

struct Vector3 { x: f32, y: f32, z: f32 }

impl FromStr for Vector3 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse format: "x,y,z"
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err("Expected format: x,y,z".to_string());
        }

        Ok(Vector3 {
            x: parts[0].parse().map_err(|_| "Invalid x")?,
            y: parts[1].parse().map_err(|_| "Invalid y")?,
            z: parts[2].parse().map_err(|_| "Invalid z")?,
        })
    }
}

#[command(
    name = "goto",
    syntax = "goto <position>",
    short = "Go to position",
    docs = "Example: goto 10,20,30"
)]
fn goto(position: Vector3) {
    println!("Going to ({}, {}, {})", position.x, position.y, position.z);
}
```

## Command Struct Naming

The `#[command]` macro generates a struct named `{FunctionName}Command` where:
- The function name is converted to PascalCase
- Underscores are preserved
- "Command" is appended

Examples:
```rust
fn spawn(...)          → SpawnCommand
fn move_to(...)        → Move_toCommand
fn save_game(...)      → Save_gameCommand
fn show_stats(...)     → Show_statsCommand
```

## Tips

1. **Keep command names lowercase** - They're case-sensitive when executing
2. **Use descriptive syntax strings** - They help users understand arguments
3. **Write good docs** - Include examples in the `docs` attribute
4. **Handle errors gracefully** - Check your error cases
5. **Test commands** - Write unit tests for your command functions

## Next Steps

- Check out the [examples/](examples/) directory for more examples
- Read the [API documentation](https://docs.rs/quark) (when published)
- See [README.md](README.md) for architecture details

## License

MIT © Tristan James Poland
