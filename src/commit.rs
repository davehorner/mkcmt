//! # Conventional Commit Parser and Changelog
//!
//! This library provides a parser for conventional commit messages. The expected commit
//! format is:
//!
//! ```text
//! <type>(<optional scope>): <description>
//! 
//! <optional body>
//! 
//! <optional footer>
//! ```
//!
//! It also exposes a changelog module that models allowed commit types, scopes,
//! and identifies breaking changes using the "!" marker.

/// The commit module contains types and parsing functionality for conventional commits.
    /// Represents a conventional commit message.
    #[derive(Debug, PartialEq, Eq)]
    pub struct CommitMessage {
        /// The commit type (e.g., "feat", "fix").
        pub commit_type: String,
        /// The optional scope (e.g., "ui", "api").
        pub scope: Option<String>,
        /// The commit description.
        pub description: String,
        /// The optional body of the commit message.
        pub body: Option<String>,
        /// The optional footer of the commit message.
        pub footer: Option<String>,
        /// Whether this commit includes breaking changes.
        pub breaking: bool,
    }

    impl CommitMessage {
        /// Parses a conventional commit message from a given input string.
        ///
        /// The parser expects the following structure:
        /// 1. A header line formatted as `<type>(<optional scope>): <description>`
        /// 2. A blank line
        /// 3. An optional body
        /// 4. A blank line
        /// 5. An optional footer (where a "!" in the footer indicates breaking changes)
        ///
        /// # Examples
        ///
        ///
        /// This example is expected to fail because the footer does not include the '!' required
        /// to mark the commit as breaking. In this case, `commit.breaking` remains `false`.
        /// ```rust,should_panic
        /// use mkcmt::commit::CommitMessage;
        ///
        /// let input = "feat(ui): add new button\n\nThis commit adds a new button to the UI.\n\nBREAKING CHANGE: The button API has changed.";
        /// let commit = CommitMessage::parse(input).unwrap();
        /// assert_eq!(commit.commit_type, "feat");
        /// assert_eq!(commit.scope, Some("ui".into()));
        /// // The following assertion fails because the footer is missing an exclamation mark,
        /// // so `commit.breaking` remains false instead of true.
        /// assert!(commit.breaking);
        /// ```
        /// 
        /// 
        /// 
        /// Parses a conventional commit message from a given input string.
///
/// The parser expects the following structure:
/// 1. A header line formatted as `<type>(<optional scope>): <description>`
/// 2. A blank line
/// 3. An optional body
/// 4. A blank line
/// 5. An optional footer (where a "!" in the footer indicates breaking changes)
///
/// # Examples
///
/// This example is expected to fail because the footer does not include the '!' required
/// to mark the commit as breaking. In this case, `commit.breaking` remains `false`.
/// ```rust,should_panic
/// use mkcmt::commit::CommitMessage;
///
/// let input = "feat(ui): add new button\n\nThis commit adds a new button to the UI.\n\nBREAKING CHANGE: The button API has changed.";
/// let commit = CommitMessage::parse(input).unwrap();
/// assert_eq!(commit.commit_type, "feat");
/// assert_eq!(commit.scope, Some("ui".into()));
/// // The following assertion fails because the footer is missing an exclamation mark,
/// // so `commit.breaking` remains false instead of true.
/// assert!(commit.breaking);
/// ```
/// Parses a conventional commit message from a given input string.
///
/// The expected format is:
///
/// ```text
/// <type>(<optional scope>): <description>
///
/// <optional body>
///
/// <optional footer>
/// ```
///
/// A footer containing the breaking change marker (an exclamation mark `!`) indicates a breaking change.
///
/// # Examples
///
/// Proper formatting with blank lines (breaking change detected):
/// ```rust
        /// use mkcmt::commit::CommitMessage;
///
/// let input = r#"feat(ui): add new button
///
/// This commit adds a new button to the UI.
///
/// BREAKING CHANGE!: The button API has changed."#;
///
/// let commit = CommitMessage::parse(input).unwrap();
/// assert!(commit.breaking, "Expected commit to be marked as breaking");
/// ```
///
/// Missing blank lines between sections (no breaking change detected):
/// ```rust
/// use mkcmt::commit::CommitMessage;
///
/// let input = "feat(ui): add new button\nThis commit adds a new button to the UI.\nBREAKING CHANGE!: The button API has changed.";
///
/// let commit = CommitMessage::parse(input).unwrap();
/// // Without proper blank lines, the input is treated as a single header, so no footer is detected.
/// assert!(!commit.breaking, "Expected commit not to be marked as breaking");
/// ```
        pub fn parse(input: &str) -> Result<Self, String> {
            // Split into sections using double newlines to separate header, body, and footer.
            let parts: Vec<&str> = input.split("\n\n").collect();

            if parts.is_empty() {
                return Err("Empty commit message".to_string());
            }

            // Parse header: expecting `<type>(<optional scope>): <description>`
            let header = parts[0].trim();
            let (commit_type, scope, description) = Self::parse_header(header)?;

            // Parse optional body (if present)
            let body = if parts.len() > 1 && !parts[1].trim().is_empty() {
                Some(parts[1].trim().to_string())
            } else {
                None
            };

            // Parse optional footer (if present) and check for breaking changes
            let (footer, breaking) = if parts.len() > 2 && !parts[2].trim().is_empty() {
                let footer_text = parts[2].trim().to_string();
                let is_breaking = footer_text.contains('!');
                (Some(footer_text), is_breaking)
            } else {
                (None, false)
            };

            Ok(CommitMessage {
                commit_type,
                scope,
                description,
                body,
                footer,
                breaking,
            })
        }

        /// Helper function to parse the header line.
        ///
        /// Returns a tuple of (commit_type, scope, description).
        fn parse_header(header: &str) -> Result<(String, Option<String>, String), String> {
            // Find the first colon that separates the header.
            let colon_index = header.find(':').ok_or("Missing ':' in header")?;
            let (meta, description) = header.split_at(colon_index);
            let description = description[1..].trim(); // skip the colon

            // Check if there's an optional scope (enclosed in parentheses)
            if let Some(start) = meta.find('(') {
                let end = meta.find(')').ok_or("Missing closing ')' in header")?;
                let commit_type = meta[..start].trim().to_string();
                let scope = meta[start + 1..end].trim().to_string();
                if scope.is_empty() {
                    return Err("Empty scope in header".into());
                }
                Ok((commit_type, Some(scope), description.to_string()))
            } else {
                // No scope provided.
                Ok((meta.trim().to_string(), None, description.to_string()))
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_with_scope_and_footer_breaking() {
            let input = "\
feat(ui): add new button

This commit adds a new button to the UI.

BREAKING CHANGE!: The button API has changed.";
            let commit = CommitMessage::parse(input).unwrap();
            assert_eq!(commit.commit_type, "feat");
            assert_eq!(commit.scope, Some("ui".into()));
            assert_eq!(commit.description, "add new button");
            assert_eq!(commit.body, Some("This commit adds a new button to the UI.".into()));
            assert!(commit.breaking);
        }

        #[test]
        fn test_parse_without_scope_and_footer() {
            let input = "fix: correct typo";
            let commit = CommitMessage::parse(input).unwrap();
            assert_eq!(commit.commit_type, "fix");
            assert_eq!(commit.scope, None);
            assert_eq!(commit.description, "correct typo");
            assert_eq!(commit.body, None);
            assert_eq!(commit.footer, None);
            assert!(!commit.breaking);
        }

        #[test]
        fn test_parse_with_empty_body_and_footer() {
            let input = "docs(api): update documentation\n\n\n";
            let commit = CommitMessage::parse(input).unwrap();
            assert_eq!(commit.commit_type, "docs");
            assert_eq!(commit.scope, Some("api".into()));
            assert_eq!(commit.description, "update documentation");
            assert_eq!(commit.body, None);
            assert_eq!(commit.footer, None);
            assert!(!commit.breaking);
        }

        #[test]
        fn test_parse_error_missing_colon() {
            let input = "chore update dependencies";
            let err = CommitMessage::parse(input).unwrap_err();
            assert!(err.contains("Missing ':'"));
        }
    }

/// The changelog module contains data about allowed commit types, scopes, and the breaking change marker.
pub mod changelog {
    /// List of allowed commit types.
    pub const TYPES: &[&str] = &[
        "feat",   // new features
        "fix",    // bug fixes
        "docs",   // documentation only changes
        "style",  // code style changes (formatting, etc.)
        "refactor", // code refactoring
        "test",   // adding missing tests or correcting existing tests
        "chore",  // changes to the build process or auxiliary tools and libraries
    ];

    /// List of allowed scopes. These could be modules, subsystems, or components.
    pub const SCOPES: &[&str] = &[
        "core",
        "ui",
        "api",
        "build",
        "docs",
        "tests",
    ];

    /// Marker that indicates a breaking change in the commit message footer.
    pub const BREAKING_CHANGE_MARKER: &str = "!";

    /// Represents the complete changelog metadata.
    #[derive(Debug)]
    pub struct Changelog {
        pub types: &'static [&'static str],
        pub scopes: &'static [&'static str],
        pub breaking_marker: &'static str,
    }

    /// Returns the changelog metadata.
    pub fn get_changelog() -> Changelog {
        Changelog {
            types: TYPES,
            scopes: SCOPES,
            breaking_marker: BREAKING_CHANGE_MARKER,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_changelog_contents() {
            let changelog = get_changelog();
            assert!(changelog.types.contains(&"feat"));
            assert!(changelog.scopes.contains(&"ui"));
            assert_eq!(changelog.breaking_marker, "!");
        }
    }
}

