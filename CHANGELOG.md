# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1](https://github.com/davehorner/mkcmt/compare/v0.2.0...v0.2.1) - 2025-03-12

### Added

feat(mode_current, mode_recovery, mode_softreset, tests): implement comprehensive branch management enhancements with soft reset (-s), current commit retrieval (-c), and advanced recovery mode integration

- Introduced the -s flag to perform a soft reset on the current branch, preserving uncommitted changes.
- Added the -c flag to retrieve and display the current last commit message for quick reference.
- Enhanced recovery mode to seamlessly restore previous commit messages from HEAD@{1}.
- Developed extensive integration tests to ensure robust functionality across all branch management modes.
- Begin code for parsing commits and CHANGELOGs


## [0.2.0](https://github.com/davehorner/mkcmt/compare/v0.1.1...v0.2.0) - 2025-03-11

### Other

- *(clipboard)* [**breaking**] remove unmaintained dependency

## [0.1.2](https://github.com/davehorner/mkcmt/compare/v0.1.1...v0.1.2) - 2025-03-11

### Other

- update dependencies in Cargo.lock and Cargo.toml, removed old clipboard package, and improved README documentation

## [0.1.1](https://github.com/davehorner/mkcmt/compare/v0.1.0...v0.1.1) - 2025-03-11

### Other

- release v0.1.0

## [0.1.0](https://github.com/davehorner/mkcmt/releases/tag/v0.1.0) - 2025-03-11

### Added

- [**breaking**] add configuration & support scripts, update dependencies, and enhance main logic
- add README with project overview, installation instructions, and usage details
- add initial, including Git diff handling/progressive prompting for commit message generation.
