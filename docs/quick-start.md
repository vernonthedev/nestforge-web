# Quick Start

Get your first NestForge Web application up and running in minutes. This guide covers installation, project creation, and running your first app.

## Prerequisites

Ensure you have the following installed:

- [Rust](https://rustup.rs/) - Rust toolchain
- [Node.js](https://nodejs.org/) - For frontend tooling (optional for frontend-only projects)
- Cargo

## 1. Install NestForge Web CLI

The CLI is the recommended way to scaffold and manage NestForge Web projects.

```bash
# Install from crates.io (when published)
cargo install nestforge-web-cli

# OR install from local development
cargo install --path crates/nestforge-web-cli
```

## 2. Create a New Project

Scaffold a fresh NestForge Web project:

```bash
nestforge-web new my-app
cd my-app
```

This creates:

```
my-app/
├── src/
│   ├── app/                    # Frontend pages & API routes
│   │   ├── page.tsx            # Homepage (/)
│   │   └── layout.tsx          # Root layout
│   ├── components/             # React components
│   ├── backend/                # Rust backend (NestForge)
│   │   └── mod.rs
│   └── lib/                    # Shared utilities
├── Cargo.toml
├── package.json
└── nestforge-web.config.ts     # Framework configuration
```

## 3. Project Variants

### Frontend-Only (No Rust Backend)

```bash
nestforge-web new my-app --frontend-only
```

Creates a pure frontend project with React routing, no Rust backend.

### With TypeScript (Default)

TypeScript is enabled by default:

```bash
nestforge-web new my-ts-app
```

## 4. Development Server

Start the development server with hot module replacement:

```bash
# Using CLI
nestforge-web dev

# Or with custom options
nestforge-web dev --port 3000 --host 127.0.0.1
```

The server will be available at [http://127.0.0.1:3000](http://127.0.0.1:3000).

### Hot Module Replacement

Changes to frontend files (React components, pages) are reflected immediately without full page reload.

For Rust backend changes, the server automatically recompiles and restarts.

## 5. Creating Pages

### Static Page

```tsx
// src/app/about/page.tsx
export default function AboutPage() {
  return (
    <main>
      <h1>About Us</h1>
      <p>Welcome to our website!</p>
    </main>
  );
}
```

Access at: `/about`

### Dynamic Page

```tsx
// src/app/users/[id]/page.tsx
interface PageProps {
  params: { id: string };
}

export default function UserPage({ params }: PageProps) {
  return <h1>User: {params.id}</h1>;
}
```

Access at: `/users/123`

## 6. Creating API Routes

API routes handle backend logic and return JSON:

```typescript
// src/app/api/users/route.ts
export async function GET() {
  const users = [
    { id: "1", name: "Alice" },
    { id: "2", name: "Bob" },
  ];
  return Response.json(users);
}

export async function POST(request: Request) {
  const body = await request.json();
  return Response.json({ created: body }, { status: 201 });
}
```

Access at: `/api/users`

## 7. Building for Production

Build your application:

```bash
nestforge-web build
```

Output is generated to the configured output directory (default: `.next`).

## 8. Running in Production

Start the production server:

```bash
nestforge-web start
nestforge-web start --port 8080 --host 0.0.0.0
```

---

## Next Steps

- **[Routing Guide](routing)**: Learn about file-based routing, dynamic routes, and layouts
- **[CLI Reference](cli)**: Full CLI command documentation
- **[Project Structure](project-structure)**: Understand the project layout
- **[Configuration](configuration)**: Customize your app settings
- **[Backend Integration](backend-integration)**: Connect to NestForge Rust backend
- **[API Routes](api-routes)**: Build REST API endpoints
- **[Deployment](deployment)**: Deploy to production