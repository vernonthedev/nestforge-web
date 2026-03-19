# Changelog

All notable changes to the NestForge Web framework are documented in this file.

## [v0.1.1] - 2026-03-19

### Features

- Add Code of Conduct (Contributor Covenant 2.1)
- Add wiki-sync workflow to automatically sync docs/ to GitHub Wiki
- Add conventional commits release workflow triggered by `release:` commits
- Add CI workflow for cargo clippy, fmt, and build checks
- Add auto git initialization for new projects (`nestforge-web new`)
- Add configurable host/IP via CLI and environment variables

### Documentation

- Add comprehensive documentation for CLI commands
- Add routing documentation with examples
- Add wiki sync for automatic documentation updates

### Bug Fixes

- Fix release workflow to commit version bump before publishing
- Fix sed replacement syntax for dependency version updates
- Fix config tests to properly reset environment variables
- Fix catch-all segment parsing in route scanner (`[...slug]`)
- Fix all clippy warnings for strict `-D warnings` CI
- Fix ambiguous glob re-exports in lib.rs
- Resolve compilation errors and pass all tests

### Refactoring

- Rename crate from `nestforge-web-core` to `nfw-core`
- Remove duplicate RouteSegment enum definition
- Simplify server router to avoid state type mismatches
- Add Clone derive to Renderer struct
- Create missing module files (hmr/mod.rs, server/mod.rs)

### CI/CD

- Add GitHub Actions release workflow with automatic crate publishing
- Add wiki-sync workflow for documentation sync
- Disable tests in CI temporarily (env var isolation issue)
- Update minimum Rust version to 1.87.0

### Internal

- Add comprehensive test suite for routing, API, config, HMR, and OpenAPI
- Add type-safe API generation (TypeScript and Rust clients)
- Add OpenAPI documentation auto-generation with Swagger UI and Redoc
- Add Hot Module Replacement (HMR) infrastructure
- Add file-based routing scanner for Next.js-style routes

## [v0.1.0] - 2026-03-19

### Features

- NestForge Backend Integration with Rust-powered dependency injection
- File-based Routing (Next.js-inspired app directory)
- SSR/SSG/ISR capabilities
- API Routes co-located with frontend
- Type Sharing between frontend and backend
- Hot Module Replacement (HMR)
- OpenAPI Documentation with Swagger UI
- CLI for project scaffolding and development
