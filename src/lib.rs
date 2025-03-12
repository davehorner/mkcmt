//! # mkcmt.lib
//!
//! This library provides functionality to parse conventional commit messages and
//! exposes changelog metadata. The expected commit message format is:
//!
//! ```text
//! <type>(<optional scope>): <description>
//!
//! <optional body>
//!
//! <optional footer>
//! ```
//!
//! Use the modules provided to parse commit messages or retrieve changelog data.

pub mod changelog;
pub mod commit;
