# NestForge Web Documentation

Welcome to the NestForge Web documentation!

## Overview

NestForge Web is a blazing-fast fullstack framework that combines:
- **NestForge (Rust)** - High-performance backend with dependency injection
- **Next.js-style frontend** - File-based routing, SSR/SSG/ISR

## Quick Start

```bash
# Install CLI
cargo install --path crates/nestforge-web-cli

# Create project
nestforge-web new my-app
cd my-app

# Start dev server
nestforge-web dev
```

## Project Structure

```
my-app/
├── src/
│   ├── app/           # Frontend pages & API routes
│   ├── components/    # React components
│   ├── nestforge/     # Rust backend modules
│   └── lib/           # Shared utilities
├── Cargo.toml
└── nestforge-web.config.ts
```

## File-Based Routing

### Page Routes

```
src/app/
├── page.tsx              → /
├── about/
│   └── page.tsx          → /about
├── blog/
│   ├── page.tsx          → /blog
│   └── [slug]/
│       └── page.tsx      → /blog/:slug
└── [...catchall]/
    └── page.tsx          → /:path*
```

### Dynamic Routes

| Pattern | Example | Params |
|---------|---------|--------|
| `[id]` | `/users/123` | `{ id: "123" }` |
| `[...slug]` | `/a/b/c` | `{ slug: ["a", "b", "c"] }` |
| `[[...slug]]` | `/` or `/a/b` | `{ slug: [] or ["a", "b"] }` |

### API Routes

```
src/app/api/
├── users/
│   └── route.ts          → /api/users
└── posts/
    └── route.ts          → /api/posts
```

## Backend Integration

### NestForge Modules

```rust
// src/nestforge/users/mod.rs
#[nestforge::module]
pub mod users {
    #[nestforge::controller("/api/users")]
    pub struct UsersController;

    #[nestforge::routes]
    impl UsersController {
        #[nestforge::get("/")]
        async fn get_all(_svc: Inject<UsersService>) -> ApiResult<Vec<User>> {
            Ok(ApiResult::ok(vec![]))
        }
    }
}
```

## CLI Commands

| Command | Description |
|---------|-------------|
| `new <name>` | Create new project |
| `dev` | Start dev server with hot reload |
| `build` | Build for production |
| `start` | Run production server |

## Configuration

```typescript
// nestforge-web.config.ts
export const config = {
  name: "my-app",
  appDir: "./src/app",
  nestforgeDir: "./src/nestforge",
  port: 3000,
  host: "127.0.0.1",
};
```

## Architecture

```
┌──────────────────────────────────────┐
│           NestForge Web               │
├─────────────┬────────────────────────┤
│  Frontend   │       Backend          │
│  (React)    │     (NestForge)        │
├─────────────┴────────────────────────┤
│         HTTP Server (Axum)           │
├──────────────────────────────────────┤
│         Rust Runtime                 │
└──────────────────────────────────────┘
```

## Performance

| Metric | Target |
|--------|--------|
| TTFB | < 50ms |
| Cold Start | < 100ms |
| Memory | < 20MB |

## Contributing

See CONTRIBUTING.md in the repository.
