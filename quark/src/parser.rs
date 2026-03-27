//! Command string parsing utilities.

use crate::error::{CommandError, Result};

/// Parses a command string into a command name and arguments.
///
/// This function tokenizes the input string, splitting on whitespace and
/// handling quoted strings properly. Quoted strings (both single and double)
/// are treated as single arguments and can contain spaces.
///
/// # Arguments
///
/// * `input` - The command string to parse (e.g., "spawn goblin 5")
///
/// # Returns
///
/// A tuple of (command_name, arguments) on success, or a `CommandError::ParseError` on failure.
///
/// # Examples
///
/// ```
/// use quark::parse_command_string;
///
/// let (name, args) = parse_command_string("spawn goblin 5").unwrap();
/// assert_eq!(name, "spawn");
/// assert_eq!(args, vec!["goblin", "5"]);
///
/// let (name, args) = parse_command_string("say \"hello world\"").unwrap();
/// assert_eq!(name, "say");
/// assert_eq!(args, vec!["hello world"]);
/// ```
pub fn parse_command_string(input: &str) -> Result<(String, Vec<String>)> {
    let input = input.trim();

    if input.is_empty() {
        return Err(CommandError::ParseError("Empty command string".to_string()));
    }

    let tokens = tokenize(input)?;

    if tokens.is_empty() {
        return Err(CommandError::ParseError("No command name found".to_string()));
    }

    let command_name = tokens[0].clone();
    let args = tokens[1..].to_vec();

    Ok((command_name, args))
}

/// Tokenizes a string into individual arguments, handling quoted strings.
fn tokenize(input: &str) -> Result<Vec<String>> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_quotes = false;
    let mut quote_char = '\0';
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' | '\'' if !in_quotes => {
                // Start of quoted string
                in_quotes = true;
                quote_char = ch;
            }
            '"' | '\'' if in_quotes && ch == quote_char => {
                // End of quoted string
                in_quotes = false;
                quote_char = '\0';
            }
            '\\' if in_quotes => {
                // Handle escape sequences within quotes
                if let Some(next_ch) = chars.next() {
                    match next_ch {
                        'n' => current_token.push('\n'),
                        't' => current_token.push('\t'),
                        'r' => current_token.push('\r'),
                        '\\' => current_token.push('\\'),
                        '"' => current_token.push('"'),
                        '\'' => current_token.push('\''),
                        _ => {
                            current_token.push('\\');
                            current_token.push(next_ch);
                        }
                    }
                }
            }
            _ if ch.is_whitespace() && !in_quotes => {
                // End of token
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => {
                // Regular character
                current_token.push(ch);
            }
        }
    }

    // Check for unclosed quotes
    if in_quotes {
        return Err(CommandError::ParseError(format!(
            "Unclosed quote character: {}",
            quote_char
        )));
    }

    // Add final token if any
    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command() {
        let (name, args) = parse_command_string("spawn goblin 5").unwrap();
        assert_eq!(name, "spawn");
        assert_eq!(args, vec!["goblin", "5"]);
    }

    #[test]
    fn test_command_with_no_args() {
        let (name, args) = parse_command_string("help").unwrap();
        assert_eq!(name, "help");
        assert!(args.is_empty());
    }

    #[test]
    fn test_command_with_double_quotes() {
        let (name, args) = parse_command_string("say \"hello world\"").unwrap();
        assert_eq!(name, "say");
        assert_eq!(args, vec!["hello world"]);
    }

    #[test]
    fn test_command_with_single_quotes() {
        let (name, args) = parse_command_string("say 'hello world'").unwrap();
        assert_eq!(name, "say");
        assert_eq!(args, vec!["hello world"]);
    }

    #[test]
    fn test_command_with_multiple_quoted_args() {
        let (name, args) = parse_command_string("cmd \"arg one\" \"arg two\" normal").unwrap();
        assert_eq!(name, "cmd");
        assert_eq!(args, vec!["arg one", "arg two", "normal"]);
    }

    #[test]
    fn test_command_with_escape_sequences() {
        let (name, args) = parse_command_string(r#"say "hello\nworld""#).unwrap();
        assert_eq!(name, "say");
        assert_eq!(args, vec!["hello\nworld"]);
    }

    #[test]
    fn test_empty_string() {
        let result = parse_command_string("");
        assert!(result.is_err());
    }

    #[test]
    fn test_whitespace_only() {
        let result = parse_command_string("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_unclosed_quote() {
        let result = parse_command_string("say \"hello");
        assert!(result.is_err());
        if let Err(CommandError::ParseError(msg)) = result {
            assert!(msg.contains("Unclosed quote"));
        } else {
            panic!("Expected ParseError");
        }
    }

    #[test]
    fn test_extra_whitespace() {
        let (name, args) = parse_command_string("  spawn   goblin   5  ").unwrap();
        assert_eq!(name, "spawn");
        assert_eq!(args, vec!["goblin", "5"]);
    }

    #[test]
    fn test_mixed_quotes() {
        let (name, args) = parse_command_string(r#"cmd "double" 'single' none"#).unwrap();
        assert_eq!(name, "cmd");
        assert_eq!(args, vec!["double", "single", "none"]);
    }

    #[test]
    fn test_numeric_arguments() {
        let (name, args) = parse_command_string("teleport 10 20 30.5").unwrap();
        assert_eq!(name, "teleport");
        assert_eq!(args, vec!["10", "20", "30.5"]);
    }
}
