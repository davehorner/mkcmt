/// The `changelog` module contains metadata about commit types, scopes, and the breaking change marker.

/// Allowed commit types.
pub const TYPES: &[&str] = &[
    "feat",     // new features
    "fix",      // bug fixes
    "docs",     // documentation only changes
    "style",    // formatting and style changes
    "refactor", // code refactoring
    "test",     // tests additions or fixes
    "chore",    // maintenance tasks
];

/// Allowed scopes (e.g., modules or subsystems).
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

/// A structure representing the complete changelog metadata.
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

