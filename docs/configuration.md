# Configuration

NestForge Web provides flexible configuration through code and environment variables.

## Configuration File

Create `nestforge-web.config.ts` in your project root:

```typescript
// nestforge-web.config.ts
export const config = {
  name: "my-app",
  appDir: "./src/app",
  backendDir: "./src/backend",
  port: 3000,
  host: "127.0.0.1",
  outputDir: ".next",
};
```

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `name` | `string` | `"app\"` | Application name |
| `appDir` | `string` | `"./src/app\"` | Frontend source directory |
| `backendDir` | `string` | `"./src/backend\"` | Rust backend directory |
| `port` | `number` | `3000` | Server port |
| `host` | `string` | `"127.0.0.1"` | Server host |
| `outputDir` | `string` | `".next\"` | Production output directory |

## Environment Variables

### Development

Create a `.env` file in your project root:

```bash
# Server
PORT=3000
HOST=127.0.0.1

# Directories
APP_DIR=./src/app
BACKEND_DIR=./src/backend

# Environment
NODE_ENV=development
RUST_BACKTRACE=1
```

### Production

```bash
# Server
PORT=8080
HOST=0.0.0.0

# Environment
NODE_ENV=production
```

## CLI Overrides

Override config via CLI flags:

```bash
# Custom port
nestforge-web dev --port 3001

# Custom host
nestforge-web start --host 0.0.0.0

# Custom app directory
nestforge-web dev --app-dir ./frontend
```

## Rust Backend Config

Configure the NestForge backend via `Cargo.toml`:

```toml
[dependencies]
nestforge = { version = "0.1", features = ["openapi", "graphql"] }
```

### Environment Loading

NestForge automatically loads `.env` files:

```bash
# .env
DATABASE_URL=postgres://localhost:5432/myapp
JWT_SECRET=your-secret-key
REDIS_URL=redis://localhost:6379
```

## TypeScript Configuration

### `tsconfig.json`

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "jsx": "react-jsx",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "allowJs": true,
    "strict": true,
    "noEmit": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "target"]
}
```

## Path Aliases

Configure path aliases for cleaner imports:

```json
{
  "compilerOptions": {
    "paths": {
      "@/*": ["./src/*"],
      "@components/*": ["./src/components/*"],
      "@lib/*": ["./src/lib/*"]
    }
  }
}
```

Usage:

```tsx
import Button from "@components/Button";
import { api } from "@lib/api";
```

---

## Runtime Config

Access configuration at runtime:

```typescript
// In pages
import { useRuntimeConfig } from "nestforge-web/client";

export default function Page() {
  const config = useRuntimeConfig();
  return <div>App: {config.name}</div>;
}
```

## Feature Flags

Enable/disable features in `nestforge-web.config.ts`:

```typescript
export const config = {
  // ... other options
  features: {
    serverSideRendering: true,
    staticExport: false,
    imageOptimization: true,
    apiRoutes: true,
  },
};
```

## Next Steps

- **[Quick Start](quick-start)**: Get up and running
- **[CLI Reference](cli)**: CLI configuration options
- **[Deployment](deployment)**: Production configuration