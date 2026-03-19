# Contributing to NestForge Web

Thanks for contributing to NestForge Web.

This project is a Rust workspace for a fullstack framework combining NestForge's high-performance Rust backend with Next.js-inspired frontend serving. This guide explains how to propose changes, keep quality high, and get PRs merged quickly.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Ways to Contribute](#ways-to-contribute)
3. [Development Setup](#development-setup)
4. [Workspace Layout](#workspace-layout)
5. [Build, Test, and Lint](#build-test-and-lint)
6. [Coding Standards](#coding-standards)
7. [Adding or Changing Features](#adding-or-changing-features)
8. [Documentation Changes](#documentation-changes)
9. [Commit Message Rules](#commit-message-rules)
10. [Pull Request Checklist](#pull-request-checklist)
11. [Getting Help](#getting-help)

## Code of Conduct

Be respectful, constructive, and focused on technical outcomes.

## Ways to Contribute

- Fix bugs
- Add framework features in the appropriate crate
- Improve docs and examples
- Add or improve tests
- Improve CLI ergonomics in `crates/nestforge-web-cli`
- Add frontend rendering capabilities (SSR, SSG, ISR)

For large features or design shifts, open an issue first so we can align on scope.

NestForge Web uses GitHub Issue Forms:

- `Feature Request` for framework API and runtime changes
- `Bug Report` for reproducible defects with `cargo` output
- `Performance Regression` for measurable runtime or allocation slowdowns
- `RFC` for architectural changes

## Development Setup

### Prerequisites

- Rust stable toolchain (Rust 2021 edition)
- `cargo`
- `git`

### Clone and bootstrap

```bash
git clone https://github.com/vernonthedev/nestforge-web.git
cd nestforge-web
cargo check --workspace
```

## Workspace Layout

Keep changes in the most specific crate.

- `crates/nestforge-web/`: Public framework crate (re-exports)
- `crates/nestforge-web-core/`: Core library (routing, SSR, config)
- `crates/nestforge-web-cli/`: CLI binary for scaffolding and dev server
- `docs/`: Documentation content

Do not place framework internals outside of `crates/`.

## Build, Test, and Lint

Run from repository root.

### Fast compile validation

```bash
cargo check --workspace
```

### Build all crates

```bash
cargo build --workspace
```

### Run tests

```bash
cargo test --workspace
```

### Format code

```bash
cargo fmt --all
```

### Lint (CI-quality gate)

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

Before opening a PR, at minimum run:

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Coding Standards

- Rust 2021, 4-space indentation, `rustfmt` output
- Files/modules/functions: `snake_case`
- Types/traits/enums: `PascalCase`
- Constants/statics: `SCREAMING_SNAKE_CASE`
- Prefer small modules and explicit public APIs in each `lib.rs`

General guidance:

- Favor clear ownership/borrowing over unnecessary cloning
- Propagate errors with `Result` and descriptive error types
- Keep public APIs stable and intentional

## Adding or Changing Features

- Add behavior in the crate that owns it
- Keep changes scoped; avoid broad unrelated refactors
- Update tests with every behavior change
- Update docs when public APIs, CLI commands, or generated project structure changes

### Testing expectations

- Unit tests: inline with `#[cfg(test)]` in source files
- Integration tests: `crates/<crate>/tests/*.rs`
- Name tests by behavior, e.g. `scans_dynamic_route_segments`

## Documentation Changes

If behavior changes, update:

- `README.md` for user-facing usage changes
- `docs/` pages for framework concepts and guides
- Example app when it helps clarify usage

## Commit Message Rules

NestForge Web uses Conventional Commits.

Use prefixes such as:

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `refactor:` - Code refactoring
- `chore:` - Maintenance tasks

Scopes are encouraged:

- `feat(routing): add catch-all route support`
- `feat(ssr): implement streaming renderer`
- `fix(cli): correct path generation for Windows`

Keep each commit focused and avoid mixing docs-only edits with functional changes when possible.

## Pull Request Checklist

Before requesting review, ensure:

- The PR has a clear summary and motivation
- Related issue is linked (if applicable)
- `cargo test --workspace` passes locally
- `cargo clippy --workspace --all-targets -- -D warnings` passes locally
- Docs were updated for public/API/CLI changes
- Changes are scoped and include tests for new behavior

### PR description template (recommended)

```md
## Summary

What changed and why.

## Changes

- Item 1
- Item 2

## Validation

- [x] cargo test --workspace
- [x] cargo clippy --workspace --all-targets -- -D warnings

## Docs

- [x] Updated docs/README (or N/A with reason)
```

## Getting Help

- Open a GitHub issue for bugs/features
- Use discussions/issues for design questions
- For security-sensitive issues, avoid posting full exploit details publicly
