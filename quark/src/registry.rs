//! Command registry for storing and executing commands.

use crate::command::Command;
use crate::error::{CommandError, Result};
use crate::parser::parse_command_string;
use std::collections::HashMap;

/// The main command registry that stores and executes commands.
///
/// `Quark` maintains a collection of registered commands and provides methods
/// to register new commands, execute them, and query available commands.
///
/// # Examples
///
/// ```
/// use quark::{Quark, Command, CommandError};
/// use std::pin::Pin;
/// use std::future::Future;
///
/// // Define a custom command
/// struct HelloCommand;
///
/// impl Command for HelloCommand {
///     fn name(&self) -> &str { "hello" }
///     fn syntax(&self) -> &str { "hello <name>" }
///     fn short(&self) -> &str { "Greet someone" }
///     fn docs(&self) -> &str { "Example: hello Alice" }
///
///     fn execute(&self, args: Vec<String>) -> Result<(), CommandError> {
///         if args.len() != 1 {
///             return Err(CommandError::ArgumentCountMismatch {
///                 expected: 1,
///                 got: args.len(),
///             });
///         }
///         println!("Hello, {}!", args[0]);
///         Ok(())
///     }
/// }
///
/// let mut registry = Quark::new();
/// registry.register_command(HelloCommand);
/// registry.run("hello World").unwrap();
/// ```
pub struct Quark {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Quark {
    /// Creates a new empty command registry.
    ///
    /// # Examples
    ///
    /// ```
    /// use quark::Quark;
    ///
    /// let registry = Quark::new();
    /// ```
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    /// Registers a command in the registry.
    ///
    /// The command's name (returned by `Command::name()`) is used as the key.
    /// If a command with the same name already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to register
    ///
    /// # Examples
    ///
    /// ```
    /// use quark::{Quark, Command, CommandError};
    /// use std::pin::Pin;
    /// use std::future::Future;
    ///
    /// struct MyCommand;
    /// impl Command for MyCommand {
    ///     fn name(&self) -> &str { "my_cmd" }
    ///     fn syntax(&self) -> &str { "my_cmd" }
    ///     fn short(&self) -> &str { "My command" }
    ///     fn docs(&self) -> &str { "Documentation" }
    ///     fn execute(&self, _args: Vec<String>) -> Result<(), CommandError> { Ok(()) }
    /// }
    ///
    /// let mut registry = Quark::new();
    /// registry.register_command(MyCommand);
    /// ```
    pub fn register_command<C: Command + 'static>(&mut self, command: C) {
        let name = command.name().to_string();
        self.commands.insert(name, Box::new(command));
    }

    /// Executes a command synchronously from a command string.
    ///
    /// The command string is parsed to extract the command name and arguments,
    /// then the corresponding command is looked up and executed.
    ///
    /// # Arguments
    ///
    /// * `input` - The command string to execute (e.g., "spawn goblin 5")
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The command string cannot be parsed
    /// - The command is not found in the registry
    /// - The command is async (use `run_async()` instead)
    /// - The command execution fails
    ///
    /// # Examples
    ///
    /// ```
    /// use quark::{Quark, Command, CommandError};
    /// use std::pin::Pin;
    /// use std::future::Future;
    ///
    /// struct EchoCommand;
    /// impl Command for EchoCommand {
    ///     fn name(&self) -> &str { "echo" }
    ///     fn syntax(&self) -> &str { "echo <message>" }
    ///     fn short(&self) -> &str { "Echo a message" }
    ///     fn docs(&self) -> &str { "Prints the message" }
    ///     fn execute(&self, args: Vec<String>) -> Result<(), CommandError> {
    ///         println!("{}", args.join(" "));
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let mut registry = Quark::new();
    /// registry.register_command(EchoCommand);
    /// registry.run("echo hello world").unwrap();
    /// ```
    pub fn run(&self, input: &str) -> Result<()> {
        let (name, args) = parse_command_string(input)?;

        let command = self.commands
            .get(&name)
            .ok_or_else(|| CommandError::NotFound(name.clone()))?;

        if command.is_async() {
            return Err(CommandError::RequiresAsyncRuntime);
        }

        command.execute(args)
    }

    /// Executes a command asynchronously from a command string.
    ///
    /// This method is used for async commands that return futures.
    ///
    /// # Arguments
    ///
    /// * `input` - The command string to execute
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The command string cannot be parsed
    /// - The command is not found in the registry
    /// - The command execution fails
    ///
    /// # Examples
    ///
    /// ```
    /// use quark::{Quark, Command, CommandError};
    /// use std::pin::Pin;
    /// use std::future::Future;
    ///
    /// struct AsyncCommand;
    /// impl Command for AsyncCommand {
    ///     fn name(&self) -> &str { "async_cmd" }
    ///     fn syntax(&self) -> &str { "async_cmd" }
    ///     fn short(&self) -> &str { "Async command" }
    ///     fn docs(&self) -> &str { "An async command" }
    ///     fn is_async(&self) -> bool { true }
    ///     fn execute_async<'a>(&'a self, _args: Vec<String>)
    ///         -> Pin<Box<dyn Future<Output = Result<(), CommandError>> + Send + 'a>>
    ///     {
    ///         Box::pin(async { Ok(()) })
    ///     }
    /// }
    ///
    /// # pollster::block_on(async {
    /// let mut registry = Quark::new();
    /// registry.register_command(AsyncCommand);
    /// registry.run_async("async_cmd").await.unwrap();
    /// # });
    /// ```
    pub async fn run_async(&self, input: &str) -> Result<()> {
        let (name, args) = parse_command_string(input)?;

        let command = self.commands
            .get(&name)
            .ok_or_else(|| CommandError::NotFound(name.clone()))?;

        command.execute_async(args).await
    }

    /// Returns a list of all registered commands.
    ///
    /// The commands are returned as references to trait objects.
    ///
    /// # Examples
    ///
    /// ```
    /// use quark::{Quark, Command, CommandError};
    /// use std::pin::Pin;
    /// use std::future::Future;
    ///
    /// struct Cmd1;
    /// impl Command for Cmd1 {
    ///     fn name(&self) -> &str { "cmd1" }
    ///     fn syntax(&self) -> &str { "cmd1" }
    ///     fn short(&self) -> &str { "Command 1" }
    ///     fn docs(&self) -> &str { "First command" }
    ///     fn execute(&self, _args: Vec<String>) -> Result<(), CommandError> { Ok(()) }
    /// }
    ///
    /// let mut registry = Quark::new();
    /// registry.register_command(Cmd1);
    ///
    /// for cmd in registry.list() {
    ///     println!("{}: {}", cmd.name(), cmd.short());
    /// }
    /// ```
    pub fn list(&self) -> Vec<&dyn Command> {
        self.commands.values().map(|cmd| cmd.as_ref()).collect()
    }

    /// Returns the documentation for a specific command.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the command to get documentation for
    ///
    /// # Returns
    ///
    /// `Some(&str)` with the documentation if the command exists, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use quark::{Quark, Command, CommandError};
    /// use std::pin::Pin;
    /// use std::future::Future;
    ///
    /// struct HelpCmd;
    /// impl Command for HelpCmd {
    ///     fn name(&self) -> &str { "help" }
    ///     fn syntax(&self) -> &str { "help" }
    ///     fn short(&self) -> &str { "Show help" }
    ///     fn docs(&self) -> &str { "Displays help information" }
    ///     fn execute(&self, _args: Vec<String>) -> Result<(), CommandError> { Ok(()) }
    /// }
    ///
    /// let mut registry = Quark::new();
    /// registry.register_command(HelpCmd);
    ///
    /// if let Some(docs) = registry.get_docs("help") {
    ///     println!("Help docs: {}", docs);
    /// }
    /// ```
    pub fn get_docs(&self, name: &str) -> Option<&str> {
        self.commands.get(name).map(|cmd| cmd.docs())
    }

    /// Returns the number of registered commands.
    ///
    /// # Examples
    ///
    /// ```
    /// use quark::Quark;
    ///
    /// let registry = Quark::new();
    /// assert_eq!(registry.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Returns `true` if no commands are registered.
    ///
    /// # Examples
    ///
    /// ```
    /// use quark::Quark;
    ///
    /// let registry = Quark::new();
    /// assert!(registry.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

impl Default for Quark {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCommand {
        name: String,
    }

    impl Command for TestCommand {
        fn name(&self) -> &str {
            &self.name
        }

        fn syntax(&self) -> &str {
            "test <arg>"
        }

        fn short(&self) -> &str {
            "Test command"
        }

        fn docs(&self) -> &str {
            "This is a test command"
        }

        fn execute(&self, args: Vec<String>) -> Result<()> {
            if args.len() != 1 {
                return Err(CommandError::ArgumentCountMismatch {
                    expected: 1,
                    got: args.len(),
                });
            }
            Ok(())
        }
    }

    #[test]
    fn test_new_registry() {
        let registry = Quark::new();
        assert_eq!(registry.len(), 0);
        assert!(registry.is_empty());
    }

    #[test]
    fn test_register_command() {
        let mut registry = Quark::new();
        registry.register_command(TestCommand {
            name: "test".to_string(),
        });
        assert_eq!(registry.len(), 1);
        assert!(!registry.is_empty());
    }

    #[test]
    fn test_run_command() {
        let mut registry = Quark::new();
        registry.register_command(TestCommand {
            name: "test".to_string(),
        });

        assert!(registry.run("test arg1").is_ok());
        assert!(registry.run("test").is_err()); // Wrong arg count
        assert!(registry.run("nonexistent arg").is_err()); // Command not found
    }

    #[test]
    fn test_list_commands() {
        let mut registry = Quark::new();
        registry.register_command(TestCommand {
            name: "cmd1".to_string(),
        });
        registry.register_command(TestCommand {
            name: "cmd2".to_string(),
        });

        let commands = registry.list();
        assert_eq!(commands.len(), 2);
    }

    #[test]
    fn test_get_docs() {
        let mut registry = Quark::new();
        registry.register_command(TestCommand {
            name: "test".to_string(),
        });

        assert_eq!(registry.get_docs("test"), Some("This is a test command"));
        assert_eq!(registry.get_docs("nonexistent"), None);
    }

    #[test]
    fn test_default() {
        let registry = Quark::default();
        assert!(registry.is_empty());
    }
}
