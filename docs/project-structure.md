# Project Structure

NestForge Web uses a unified project structure combining frontend and backend code.

## Root Structure

```
my-app/
├── src/                        # Source code
│   ├── app/                    # Frontend pages & API routes
│   ├── components/             # Shared React components
│   ├── backend/               # Rust NestForge backend
│   └── lib/                   # Shared utilities and types
├── Cargo.toml                  # Rust dependencies
├── package.json                # Node dependencies
├── nestforge-web.config.ts     # Framework configuration
├── tsconfig.json               # TypeScript configuration
└── SPEC.md                     # Project specification
```

## Frontend Structure (`src/app/`)

```
src/app/
├── page.tsx                   # Homepage (/)
├── layout.tsx                  # Root layout
├── loading.tsx                # Loading state
├── error.tsx                   # Error boundary
├── not-found.tsx               # 404 page
├── about/
│   └── page.tsx               # /about
├── users/
│   ├── page.tsx               # /users
│   └── [id]/
│       └── page.tsx           # /users/:id
├── blog/
│   ├── page.tsx               # /blog
│   └── [slug]/
│       └── page.tsx           # /blog/:slug
└── api/
    ├── users/
    │   └── route.ts           # /api/users
    └── posts/
        └── route.ts           # /api/posts
```

## Component Structure (`src/components/`)

```
src/components/
├── Button/
│   ├── Button.tsx
│   └── index.ts
├── Navbar/
│   ├── Navbar.tsx
│   └── index.ts
└── layout/
    ├── Header.tsx
    ├── Footer.tsx
    └── index.ts
```

## Backend Structure (`src/backend/`)

```
src/backend/
├── mod.rs                     # Module exports
├── app_module.rs              # Main NestForge module
├── app_controller.rs          # Main controller
└── users/
    ├── mod.rs
    ├── users_controller.rs
    └── users_service.rs
```

## Special Files

| File | Purpose |
|------|---------|
| `page.tsx` | Page component for a route |
| `layout.tsx` | Shared UI wrapper for routes |
| `loading.tsx` | Loading state while page loads |
| `error.tsx` | Error boundary for route segment |
| `not-found.tsx` | 404 page for route segment |
| `route.ts` | API endpoint handler |
| `layout.ts` | Route group shared layout |
| `(group)/` | Route group (no URL prefix) |

## Configuration Files

### `nestforge-web.config.ts`

```typescript
export const config = {
  name: "my-app",
  appDir: "./src/app",
  backendDir: "./src/backend",
  port: 3000,
  host: "127.0.0.1",
  outputDir: ".next",
};
```

### `Cargo.toml`

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
nestforge = "0.1"
nestforge-web = { path = "../crates/nestforge-web" }
nfw-core = { path = "../crates/nfw-core" }
tokio = { version = "1", features = ["full"] }
```

---

## Route Groups

Group routes without affecting URLs using parentheses:

```
(app)/
  ├── dashboard/
  │   └── page.tsx        → /dashboard
  └── settings/
      └── page.tsx        → /settings
```

## Private Folders

Prefix folders with underscore to exclude from routing:

```
_components/
  Button.tsx              → NOT routed (utility only)
```

## Dynamic Segments

| Pattern | File | Params |
|---------|------|--------|
| Required | `users/[id]/page.tsx` | `{ id: string }` |
| Catch-all | `docs/[...slug]/page.tsx` | `{ slug: string[] }` |
| Optional | `[[...slug]]/page.tsx` | `{ slug?: string[] }` |