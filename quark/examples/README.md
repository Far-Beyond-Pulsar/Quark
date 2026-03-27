# Quark Examples

This directory contains example programs demonstrating how to use the Quark command system.

## Running Examples

```bash
cargo run --example <example_name>
```

## Available Examples

### 1. Basic (`basic.rs`)

**Complexity**: ⭐ Beginner

The simplest introduction to Quark. Shows:
- Defining commands with the `#[command]` macro
- Registering commands in a registry
- Executing commands from strings
- Error handling
- Listing registered commands
- Direct function calls

**Run:**
```bash
cargo run --example basic
```

**Key concepts:**
- Command registration
- String-based execution
- Type-safe argument parsing

---

### 2. Async Commands (`async_commands.rs`)

**Complexity**: ⭐⭐ Intermediate

Demonstrates async/await support in Quark. Shows:
- Defining async commands
- Mixing sync and async commands
- Using `run_async()` for async execution
- Async runtime integration with Tokio

**Run:**
```bash
cargo run --example async_commands
```

**Key concepts:**
- Async command definitions
- `run_async()` vs `run()`
- Async/sync compatibility

---

### 3. Game Console (`game_console.rs`)

**Complexity**: ⭐⭐ Intermediate

A complete game debug console simulation. Shows:
- Managing game state with commands
- Multiple related commands working together
- Real-world command organization
- Stateful command execution

**Run:**
```bash
cargo run --example game_console
```

**Features:**
- Player stats management
- Entity spawning
- God mode toggle
- Health/damage system
- Game state reset

**Key concepts:**
- Shared state between commands
- Domain-specific command sets
- Complex command interactions

---

### 4. Interactive Console (`interactive_console.rs`) ⭐ **RECOMMENDED**

**Complexity**: ⭐⭐⭐ Advanced

A fully playable text-based RPG with an interactive REPL (Read-Eval-Print Loop). This is the most complete example showing Quark in a real application.

**Run:**
```bash
cargo run --example interactive_console
```

**How to play:**

1. **Start the game:**
   ```bash
   cargo run --example interactive_console
   ```

2. **Enter your character name when prompted**

3. **Type commands at the `⚔️  >` prompt:**
   ```
   ⚔️  > help              # See all commands
   ⚔️  > stats             # View your character
   ⚔️  > move north        # Explore the world
   ⚔️  > attack goblin     # Fight enemies
   ⚔️  > collect           # Find items
   ⚔️  > rest              # Recover health
   ⚔️  > shop              # Visit merchant
   ⚔️  > buy health        # Purchase items
   ⚔️  > map               # View surroundings
   ⚔️  > exit              # Quit game
   ```

**Game Features:**
- ⚔️ Turn-based combat system
- 🎮 Character progression (level up system)
- 💰 Economy (gold and shop)
- 🗺️ Map exploration with random encounters
- 📊 Detailed statistics tracking
- 🎲 Random events (enemies, items)

**Key concepts:**
- Interactive REPL implementation
- Command-driven game loop
- Complex state management
- User input handling
- Real-time command execution

**Sample Session:**
```
⚔️  > move north
🚶 You move north to (0, 1)
⚔️  A wild enemy appears!

⚔️  > attack goblin
⚔️  You attack the goblin!
💥 You deal 15 damage!
💀 The goblin is defeated!
💰 You gain 18 gold!

⚔️  > collect
✨ You found 12 gold!

⚔️  > shop
🏪 ═══ TRAVELING MERCHANT ═══
  Your gold: 30 💰
  ────────────────────────
  [1] Health Potion - 50g (+50 HP)
  [2] Mana Potion   - 30g (+30 Mana)

⚔️  > buy mana
💊 Purchased Mana Potion! (+30 Mana)

⚔️  > stats
╔════════════════════════════════════╗
║        PLAYER STATISTICS           ║
╠════════════════════════════════════╣
║ Name:     Hero                     ║
║ Level:    1                        ║
║ Health:   100/100                  ║
║ Mana:     50/50                    ║
...
```

---

## Comparison Table

| Example | Lines of Code | Features | Best For |
|---------|--------------|----------|----------|
| **basic** | ~120 | Simple commands, error handling | Learning basics |
| **async_commands** | ~90 | Async/await, mixed execution | Async patterns |
| **game_console** | ~260 | Stateful commands, complex logic | Real-world structure |
| **interactive_console** | ~430 | Full REPL, game loop, UI | Complete applications |

## Learning Path

**Recommended order for learning:**

1. **Start with `basic.rs`** to understand core concepts
2. **Try `async_commands.rs`** if you need async support
3. **Explore `game_console.rs`** to see stateful command patterns
4. **Play `interactive_console.rs`** to experience a complete application

## Common Patterns

### Pattern 1: Simple Stateless Commands
```rust
#[command(name = "hello", syntax = "hello", short = "Say hello", docs = "Prints hello")]
fn hello() {
    println!("Hello!");
}
```
**Example:** `basic.rs`

### Pattern 2: Commands with State
```rust
static mut STATE: Option<Arc<Mutex<GameState>>> = None;

#[command(/* ... */)]
fn modify_state(value: i32) {
    let state = get_state();
    let mut state = state.lock().unwrap();
    state.value = value;
}
```
**Examples:** `game_console.rs`, `interactive_console.rs`

### Pattern 3: Async Commands
```rust
#[command(/* ... */)]
async fn async_operation(param: String) {
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Done: {}", param);
}
```
**Example:** `async_commands.rs`

### Pattern 4: Interactive REPL
```rust
loop {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match registry.run(input.trim()) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e),
    }
}
```
**Example:** `interactive_console.rs`

## Tips for Your Own Commands

1. **Start simple** - Begin with stateless commands like in `basic.rs`
2. **Use meaningful names** - Command names should be clear and concise
3. **Write good docs** - Include usage examples in the `docs` attribute
4. **Test error cases** - Try invalid inputs to see how errors are handled
5. **Keep state separate** - Don't mix state management with command logic
6. **Handle user input** - Validate and sanitize when appropriate

## Need Help?

- Check the [QUICKSTART.md](../QUICKSTART.md) guide
- Read the main [README.md](../README.md)
- Look at the inline comments in each example
- Run examples and experiment with modifications

## Contributing

Found a bug or have an idea for a new example? Please open an issue or PR!
