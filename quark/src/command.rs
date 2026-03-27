//! Core command trait and types.

use crate::error::{CommandError, Result};
use std::future::Future;
use std::pin::Pin;

/// Trait representing a command that can be registered and executed.
///
/// Commands are typically created via the `#[command]` macro, which generates
/// implementations of this trait automatically. However, you can also implement
/// this trait manually for custom command types.
///
/// # Examples
///
/// ```
/// use quark::{Command, CommandError};
/// use std::pin::Pin;
/// use std::future::Future;
///
/// struct MyCommand;
///
/// impl Command for MyCommand {
///     fn name(&self) -> &str {
///         "my_command"
///     }
///
///     fn syntax(&self) -> &str {
///         "my_command <arg>"
///     }
///
///     fn short(&self) -> &str {
///         "My custom command"
///     }
///
///     fn docs(&self) -> &str {
///         "This is a custom command implementation"
///     }
///
///     fn execute(&self, args: Vec<String>) -> Result<(), CommandError> {
///         println!("Executing with args: {:?}", args);
///         Ok(())
///     }
/// }
/// ```
pub trait Command: Send + Sync {
    /// Returns the name of the command.
    ///
    /// This is the identifier used to invoke the command (e.g., "spawn", "teleport").
    fn name(&self) -> &str;

    /// Returns the syntax string for the command.
    ///
    /// This should show the command name and its parameters in a human-readable format
    /// (e.g., "spawn <entity> <count>").
    fn syntax(&self) -> &str;

    /// Returns a short one-line description of the command.
    ///
    /// This is used for command listings and help menus.
    fn short(&self) -> &str;

    /// Returns detailed documentation for the command.
    ///
    /// This should include usage examples and any important notes.
    fn docs(&self) -> &str;

    /// Returns `true` if this command is asynchronous.
    ///
    /// Async commands should implement `execute_async()` and sync commands
    /// should implement `execute()`. The default is `false` (synchronous).
    fn is_async(&self) -> bool {
        false
    }

    /// Executes the command synchronously with the provided arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - The parsed argument strings to pass to the command
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The argument count is incorrect
    /// - An argument cannot be converted to the expected type
    /// - The command execution fails
    /// - This is an async command (should use `execute_async()` instead)
    fn execute(&self, _args: Vec<String>) -> Result<()> {
        Err(CommandError::RequiresAsyncRuntime)
    }

    /// Executes the command asynchronously with the provided arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - The parsed argument strings to pass to the command
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The argument count is incorrect
    /// - An argument cannot be converted to the expected type
    /// - The command execution fails
    /// - This is a sync command (should use `execute()` instead)
    fn execute_async<'a>(
        &'a self,
        _args: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async { Err(CommandError::SyncCommandInAsyncContext) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCommand;

    impl Command for TestCommand {
        fn name(&self) -> &str {
            "test"
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
    fn test_command_trait() {
        let cmd = TestCommand;
        assert_eq!(cmd.name(), "test");
        assert_eq!(cmd.syntax(), "test <arg>");
        assert_eq!(cmd.short(), "Test command");
        assert_eq!(cmd.docs(), "This is a test command");
        assert!(!cmd.is_async());
    }

    #[test]
    fn test_command_execute() {
        let cmd = TestCommand;
        assert!(cmd.execute(vec!["arg1".to_string()]).is_ok());
        assert!(cmd.execute(vec![]).is_err());
        assert!(cmd.execute(vec!["arg1".to_string(), "arg2".to_string()]).is_err());
    }

    #[test]
    fn test_command_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Box<dyn Command>>();
    }
}
