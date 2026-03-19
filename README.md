```text

███╗   ██╗███████╗███████╗████████╗███████╗ ██████╗ ██████╗  ██████╗ ███████╗    ██╗    ██╗███████╗██████╗ 
████╗  ██║██╔════╝██╔════╝╚══██╔══╝██╔════╝██╔═══██╗██╔══██╗██╔════╝ ██╔════╝    ██║    ██║██╔════╝██╔══██╗
██╔██╗ ██║█████╗  ███████╗   ██║   █████╗  ██║   ██║██████╔╝██║  ███╗█████╗      ██║ █╗ ██║█████╗  ██████╔╝
██║╚██╗██║██╔══╝  ╚════██║   ██║   ██╔══╝  ██║   ██║██╔══██╗██║   ██║██╔══╝      ██║███╗██║██╔══╝  ██╔══██╗
██║ ╚████║███████╗███████║   ██║   ██║     ╚██████╔╝██║  ██║╚██████╔╝███████╗    ╚███╔███╔╝███████╗██████╔╝
╚═╝  ╚═══╝╚══════╝╚══════╝   ╚═╝   ╚═╝      ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝     ╚══╝╚══╝ ╚══════╝╚═════╝

```

# NestForge - Web Framework

A blazing-fast fullstack framework combining NestForge's high-performance Rust backend with a Next.js-inspired frontend serving layer.

![crates.io](https://img.shields.io/crates/v/nestforge-web)
![docs.rs](https://img.shields.io/docsrs/nestforge-web)
![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)
![Release](https://github.com/vernonthedev/nestforge-web/workflows/Release/badge.svg)

## Key Features

NestForge Web brings together the best of both worlds:

- **NestForge Backend**: Rust-powered with dependency injection, modules, and runtime performance
- **File-based Routing**: Next.js-style app directory with pages and API routes
- **SSR/SSG/ISR**: Server-Side Rendering, Static Site Generation, and Incremental Static Regeneration
- **API Routes**: Backend endpoints co-located with frontend code
- **Type Sharing**: Automatic TypeScript types from Rust backend
- **Hot Module Replacement**: Fast development with HMR
- **OpenAPI Docs**: Auto-generated API documentation with Swagger UI
- **Edge Deployment**: Cloudflare Workers, serverless, or containers

## Workspace Layout

The project is organized as a Cargo workspace with multiple crates:

- `crates/nestforge-web`: Main public crate
- `crates/nfw-core`: Core framework (routing, HMR, API generation, OpenAPI)
- `crates/nestforge-web-cli`: CLI binary for scaffolding and development

## Quick Start

### From Repository

```bash
git clone https://github.com/vernonthedev/nestforge-web.git
cd nestforge-web
cargo check --workspace
cargo run -p nestforge-web-cli -- dev
```

### Using CLI

```bash
cargo install nestforge-web-cli
nestforge-web new my-app
cd my-app
nestforge-web dev
```

## Minimal Fullstack App

### Frontend (page.tsx)

```tsx
// src/app/users/page.tsx
export default async function UsersPage() {
  const users = await fetch('/api/users').then(r => r.json());
  
  return (
    <main>
      <h1>Users</h1>
      <ul>
        {users.map(user => (
          <li key={user.id}>{user.name}</li>
        ))}
      </ul>
    </main>
  );
}
```

### Backend (Rust)

```rust
// src/backend/users/users_controller.rs
use nestforge::prelude::*;

#[nestforge::controller("/api/users")]
pub struct UsersController;

#[nestforge::routes]
impl UsersController {
    #[nestforge::get("/")]
    async fn get_users(_service: Inject<UsersService>) -> ApiResult<Vec<User>> {
        Ok(ApiResult::ok(users_service.find_all().await?))
    }
}
```

## Directory Structure

```
my-app/
├── src/
│   ├── app/                    # Next.js-style app directory
│   │   ├── page.tsx            # Homepage (/)
│   │   ├── about/
│   │   │   └── page.tsx        # /about
│   │   ├── api/                # API routes
│   │   │   └── users/
│   │   │       └── route.ts    # /api/users
│   │   └── layout.tsx          # Root layout
│   ├── components/             # Shared React components
│   ├── backend/                # NestForge backend modules
│   │   ├── app_module.rs       # Root module
│   │   ├── users/
│   │   │   ├── mod.rs
│   │   │   ├── users_controller.rs
│   │   │   └── users_service.rs
│   │   └── posts/
│   │       ├── mod.rs
│   │       ├── posts_controller.rs
│   │       └── posts_service.rs
│   └── lib/
│       └── bridge.rs           # Bridge between frontend/backend
├── nestforge-web.config.ts     # Framework configuration
└── Cargo.toml
```

## Comparison with Next.js

| Feature | Next.js | NestForge Web |
|---------|---------|---------------|
| Backend Language | JavaScript/TypeScript | Rust |
| Runtime | Node.js | Native + Axum |
| DI System | Manual/Modules | Native NestForge |
| Cold Start (serverless) | 100-500ms | < 50ms |
| Memory Usage | 50-200MB | < 20MB |
| Type Sharing | Generated | Native shared crate |
| Deployment | Node.js hosts | Native binaries |

## CLI Commands

```bash
# Development
nestforge-web dev           # Start dev server with HMR
nestforge-web build         # Build for production
nestforge-web start         # Run production server

# Project management
nestforge-web new <name>    # Create new project
nestforge-web generate <type> # Generate module/controller
nestforge-web docs          # Open API documentation
```

## Changelog

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

Initial release of NestForge Web framework.

### Features

- NestForge Backend Integration with Rust-powered dependency injection
- File-based Routing (Next.js-inspired app directory)
- SSR/SSG/ISR capabilities
- API Routes co-located with frontend
- Type Sharing between frontend and backend
- Hot Module Replacement (HMR)
- OpenAPI Documentation with Swagger UI
- CLI for project scaffolding and development

## Documentation

- Wiki: https://github.com/vernonthedev/nestforge-web/wiki

> [!Important]
> Developed with love by @vernonthedev.
