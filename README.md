# NestForge Web

A blazing-fast fullstack framework combining NestForge (Rust) backend with Next.js-style frontend.

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Release](https://img.shields.io/github/actions/workflow/status/vernonthedev/nestforge-web/release.yml?branch=main&label=release)](https://github.com/vernonthedev/nestforge-web/actions/workflows/release.yml)

## What You Get

- **File-based routing** - Next.js-style routing for frontend pages
- **API routes** - Built-in REST API endpoints
- **Rust backend** - NestForge integration for high-performance APIs
- **Hot module replacement** - Instant feedback during development
- **TypeScript** - Full type safety across frontend and backend

## Quick Start

```bash
# Install CLI
cargo install nestforge-web-cli

# Create project
nestforge-web new my-app
cd my-app

# Start dev server
nestforge-web dev
```

## Documentation

- **[Quick Start](docs/quick-start.md)** - Get up and running
- **[Project Structure](docs/project-structure.md)** - Understand the layout
- **[Routing](docs/routing.md)** - File-based routing guide
- **[API Routes](docs/api-routes.md)** - Build REST endpoints
- **[Configuration](docs/configuration.md)** - Customize your app
- **[Backend Integration](docs/backend-integration.md)** - NestForge Rust backend
- **[Deployment](docs/deployment.md)** - Production deployment
- **[CLI Reference](docs/cli.md)** - Full CLI documentation

## Project Structure

```
my-app/
├── src/
│   ├── app/           # Pages & API routes
│   ├── components/   # React components
│   └── backend/      # Rust NestForge modules
├── Cargo.toml
└── nestforge-web.config.ts
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Apache-2.0 ([LICENSE](LICENSE)).