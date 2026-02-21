# Contributing to trw-daemon

Thanks for your interest in contributing! Here's how to get started.

## Development Setup

1. Install Rust: https://rustup.rs/
2. Clone the repo:
   ```bash
   git clone https://github.com/therobotworks/trw-daemon.git
   cd trw-daemon
   ```
3. Build and run tests:
   ```bash
   cargo build
   cargo test
   ```

## Making Changes

1. Fork the repo and create a feature branch
2. Write tests for your changes
3. Run the full check suite:
   ```bash
   cargo check --all-features
   cargo clippy --all-features -- -D warnings
   cargo fmt --check
   cargo test --all-features
   ```
4. Open a pull request against `main`

## Finding Work

Look for issues tagged `good-first-issue`. Many collector modules have `todo!()` stubs that need implementation — each one is a self-contained contribution.

## Code Style

- Run `cargo fmt` before committing
- All warnings must be clean (`cargo clippy -- -D warnings`)
- Public items need doc comments
- Tests for all new functionality

## Collector Development

To add a new collector:

1. Create `src/collectors/your_collector.rs`
2. Implement the `Collector` trait
3. Add a feature flag in `Cargo.toml` if it has external dependencies
4. Wire it into `src/collectors/mod.rs`
5. Add config options to `src/config.rs` and `config/default.toml`
6. Write integration tests
