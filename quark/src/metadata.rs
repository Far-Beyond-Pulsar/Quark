//! Command metadata structures for storing documentation and command information.

/// Metadata associated with a command.
///
/// This structure stores all the documentation and syntax information
/// that was provided via the `#[command]` macro attributes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandMetadata {
    /// The name of the command (e.g., "spawn", "teleport")
    name: String,
    /// The syntax string showing how to use the command (e.g., "spawn <entity> <count>")
    syntax: String,
    /// A short one-line description of what the command does
    short: String,
    /// Detailed documentation with usage examples
    docs: String,
}

impl CommandMetadata {
    /// Creates a new `CommandMetadata` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use quark::CommandMetadata;
    ///
    /// let metadata = CommandMetadata::new(
    ///     "spawn",
    ///     "spawn <entity> <count>",
    ///     "Spawn entities into the game world",
    ///     "Example: `spawn goblin 5` spawns 5 goblins at the current location"
    /// );
    ///
    /// assert_eq!(metadata.name(), "spawn");
    /// ```
    pub fn new(name: impl Into<String>, syntax: impl Into<String>, short: impl Into<String>, docs: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            syntax: syntax.into(),
            short: short.into(),
            docs: docs.into(),
        }
    }

    /// Returns the command name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the command syntax.
    pub fn syntax(&self) -> &str {
        &self.syntax
    }

    /// Returns the short description.
    pub fn short(&self) -> &str {
        &self.short
    }

    /// Returns the detailed documentation.
    pub fn docs(&self) -> &str {
        &self.docs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_creation() {
        let metadata = CommandMetadata::new(
            "test_cmd",
            "test_cmd <arg>",
            "Test command",
            "This is a test",
        );

        assert_eq!(metadata.name(), "test_cmd");
        assert_eq!(metadata.syntax(), "test_cmd <arg>");
        assert_eq!(metadata.short(), "Test command");
        assert_eq!(metadata.docs(), "This is a test");
    }

    #[test]
    fn test_metadata_clone() {
        let metadata = CommandMetadata::new("cmd", "syntax", "short", "docs");
        let cloned = metadata.clone();
        assert_eq!(metadata, cloned);
    }
}
