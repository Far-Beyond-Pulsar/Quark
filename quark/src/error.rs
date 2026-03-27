//! Error types for the Quark command system.

use thiserror::Error;

/// Errors that can occur during command parsing, registration, or execution.
#[derive(Error, Debug)]
pub enum CommandError {
    /// The requested command was not found in the registry.
    #[error("Command not found: {0}")]
    NotFound(String),

    /// The command input could not be parsed correctly.
    #[error("Parse error: {0}")]
    ParseError(String),

    /// The number of arguments provided doesn't match what the command expects.
    #[error("Argument count mismatch: expected {expected}, got {got}")]
    ArgumentCountMismatch {
        /// Number of arguments the command expects
        expected: usize,
        /// Number of arguments actually provided
        got: usize,
    },

    /// An argument could not be converted to the expected type.
    #[error("Type conversion error: could not convert '{arg}' to {target_type}")]
    TypeConversionError {
        /// The argument value that failed to convert
        arg: String,
        /// The target type name
        target_type: &'static str,
    },

    /// An error occurred during command execution.
    #[error("Execution error: {0}")]
    ExecutionError(String),

    /// An async command was called with a sync execution method.
    #[error("This command requires an async runtime - use run_async() instead of run()")]
    RequiresAsyncRuntime,

    /// A sync command was called with an async execution method.
    #[error("This command is synchronous - use run() instead of run_async()")]
    SyncCommandInAsyncContext,
}

/// Result type alias for Quark operations.
pub type Result<T> = std::result::Result<T, CommandError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = CommandError::NotFound("spawn".to_string());
        assert_eq!(err.to_string(), "Command not found: spawn");

        let err = CommandError::ArgumentCountMismatch {
            expected: 2,
            got: 3,
        };
        assert_eq!(
            err.to_string(),
            "Argument count mismatch: expected 2, got 3"
        );

        let err = CommandError::TypeConversionError {
            arg: "abc".to_string(),
            target_type: "usize",
        };
        assert_eq!(
            err.to_string(),
            "Type conversion error: could not convert 'abc' to usize"
        );
    }

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<CommandError>();
    }
}
