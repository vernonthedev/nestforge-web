```text

███╗   ██╗███████╗███████╗████████╗███████╗ ██████╗ ██████╗  ██████╗ ███████╗    ██╗    ██╗███████╗██████╗ 
████╗  ██║██╔════╝██╔════╝╚══██╔══╝██╔════╝██╔═══██╗██╔══██╗██╔════╝ ██╔════╝    ██║    ██║██╔════╝██╔══██╗
██╔██╗ ██║█████╗  ███████╗   ██║   █████╗  ██║   ██║██████╔╝██║  ███╗█████╗      ██║ █╗ ██║█████╗  ██████╔╝
██║╚██╗██║██╔══╝  ╚════██║   ██║   ██╔══╝  ██║   ██║██╔══██╗██║   ██║██╔══╝      ██║███╗██║██╔══╝  ██╔══██╗
██║ ╚████║███████╗███████║   ██║   ██║     ╚██████╔╝██║  ██║╚██████╔╝███████╗    ╚███╔███╔╝███████╗██████╔╝
╚═╝  ╚═══╝╚══════╝╚══════╝   ╚═╝   ╚═╝      ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝     ╚══╝╚══╝ ╚══════╝╚═════╝

```

# NestForge Web

A blazing-fast fullstack framework combining NestForge's high-performance Rust backend with an Next.js-inspired frontend serving layer.

## Overview

NestForge Web brings together:
- **NestForge** - Rust-powered backend with dependency injection, modules, and runtime performance
- **Web Layer** - SSR/SSG/ISR capabilities with file-based routing, API routes, and edge-ready deployment

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     NestForge Web                           │
├─────────────────────────────────────────────────────────────┤
│  Frontend Layer (React/Components)  │  Backend Layer       │
│  - File-based routing               │  - NestForge modules │
│  - SSR/SSG/ISR                      │  - Controllers        │
│  - API routes                       │  - Services           │
│  - Middleware                       │  - Providers          │
│  - Components                       │  - Guards             │
├─────────────────────────────────────────────────────────────┤
│                    NestForge Core                           │
│  - HTTP Server (Axum)          │  - Module Graph          │
│  - DI Container                │  - Route Builder          │
├─────────────────────────────────────────────────────────────┤
│                    Performance Layer                        │
│  - Rust runtime             │  - Edge deployment          │
│  - WASM compatibility        │  - Streaming SSR            │
└─────────────────────────────────────────────────────────────┘
```

## Core Features

### Backend (NestForge Integration)

- Dependency injection with `#[injectable]` providers
- Module system with `#[module]` for feature isolation
- Controllers with route macros (`#[get]`, `#[post]`, etc.)
- Guards, interceptors, and middleware pipeline
- OpenAPI documentation generation
- Database integration via `nestforge-db` and `nestforge-orm`
- GraphQL and gRPC support

### Frontend (Next.js-inspired)

- **File-based Routing** - Pages and API routes from filesystem
- **Server Components** - React components rendered on server
- **API Routes** - Backend endpoints co-located with frontend
- **Static Generation (SSG)** - Pre-rendered pages at build time
- **Incremental Static Regeneration (ISR)** - On-demand revalidation
- **Server-Side Rendering (SSR)** - Dynamic page rendering
- **Client Components** - Interactive React with `"use client"` directive

### Directory Structure

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
│   ├── components/            # Shared React components
│   ├── backend/              # NestForge backend modules
│   │   ├── app_module.rs      # Root module
│   │   ├── users/
│   │   │   ├── mod.rs
│   │   │   ├── users_controller.rs
│   │   │   └── users_service.rs
│   │   └── posts/
│   │       ├── mod.rs
│   │       ├── posts_controller.rs
│   │       └── posts_service.rs
│   └── lib/
│       └── bridge.rs       # Bridge between frontend/backend
├── nestforge-web.config.ts    # Framework configuration
└── Cargo.toml
```

## Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | React 19+ |
| Rendering | Leptos / Dioxus (WASM) or React with SSR |
| Backend Runtime | NestForge (Rust/Axum) |
| Build Tool | Vite + Cargo |
| CLI | Rust CLI with dev server |
| Deployment | Edge (Cloudflare Workers), Serverless, or Containers |

## Performance Goals

| Metric | Target |
|--------|--------|
| Time to First Byte (TTFB) | < 50ms |
| Serverless Cold Start | < 100ms |
| Memory Footprint | < 10MB baseline |
| Bundle Size | < 50KB (framework) |

## Getting Started

### Prerequisites

- Rust 1.75+
- Node.js 20+
- Cargo workspace support

### Initialize Project

```bash
# Install CLI (once published)
cargo install nestforge-web

# Create new project
nestforge-web new my-app
cd my-app
cargo run
```

### Development

```bash
# Start dev server with HMR
nestforge-web dev

# Build for production
nestforge-web build

# Run production server
nestforge-web start
```

## Example: Fullstack Route

```tsx
// src/app/api/users/route.ts
import { NestForge } from "@nestforge-web/server";

// Automatically typed from backend controller
const api = NestForge.create();

export async function GET(request: Request) {
  return api.users.getAll();
}

export async function POST(request: Request) {
  const body = await request.json();
  return api.users.create(body);
}
```

```rust
// src/nestforge/users/users_controller.rs
#[nestforge::controller("/api/users")]
pub struct UsersController;

#[nestforge::routes]
impl UsersController {
    #[nestforge::get("/")]
    async fn get_all(_users: Inject<UsersService>) -> ApiResult<Vec<User>> {
        Ok(ApiResult::ok(users_service.find_all().await?))
    }

    #[nestforge::post("/")]
    async fn create(
        _users: Inject<UsersService>,
        body: ValidatedBody<CreateUserDto>,
    ) -> ApiResult<User> {
        Ok(ApiResult::ok(users_service.create(body).await?))
    }
}
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




