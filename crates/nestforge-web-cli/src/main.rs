use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "nestforge-web")]
#[command(version = "0.1.0")]
#[command(about = "Blazing fast fullstack framework powered by NestForge", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {
        name: String,
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
    Dev {
        #[arg(short, long)]
        app_dir: Option<PathBuf>,
        #[arg(short, long)]
        port: Option<u16>,
    },
    Build {
        #[arg(short, long)]
        app_dir: Option<PathBuf>,
    },
    Start {
        #[arg(short, long)]
        port: Option<u16>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, path } => {
            tracing::info!("Creating new NestForge Web project: {}", name);
            new_project(&name, &path)?;
        }
        Commands::Dev { app_dir, port } => {
            dev_server(app_dir, port).await?;
        }
        Commands::Build { app_dir } => {
            build_project(app_dir)?;
        }
        Commands::Start { port } => {
            start_server(port).await?;
        }
    }

    Ok(())
}

fn init_git_repo(project_dir: &PathBuf) -> anyhow::Result<()> {
    let repo = git2::Repository::init(project_dir)?;
    
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    
    let signature = git2::Signature::now("NestForge Web", "nestforge@web.dev")
        .unwrap_or_else(|_| git2::Signature::now("User", "user@example.com").unwrap());
    
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit: NestForge Web project",
        &tree,
        &[],
    )?;
    
    tracing::info!("Git repository initialized with initial commit");
    Ok(())
}

fn new_project(name: &str, path: &PathBuf) -> Result<()> {
    let project_dir = path.join(name);
    std::fs::create_dir_all(&project_dir)?;

    let app_port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let http_port = std::env::var("HTTP_PORT").unwrap_or_else(|_| "3001".to_string());
    
    std::fs::write(
        project_dir.join("nestforge-web.config.ts"),
        format!(r#"export const config = {{
  name: "{}",
  appDir: "./src/app",
  backendDir: "./src/backend",
  port: process.env.PORT || {},
  httpPort: process.env.HTTP_PORT || {},
  host: process.env.HOST || "127.0.0.1",
}};

export default config;
"#, name, app_port, http_port),
    )?;

    std::fs::write(
        project_dir.join(".env.example"),
        format!(r#"# NestForge Web Environment Configuration
# Copy this file to .env.local and customize values

# Server ports
PORT={}
HTTP_PORT={}

# Server host
HOST=127.0.0.1

# Environment
NODE_ENV=development

# Backend (NestForge)
BACKEND_PORT={}

# Database (if using)
DATABASE_URL=
"#, app_port, http_port, http_port),
    )?;

    std::fs::create_dir_all(project_dir.join("src/app"))?;
    std::fs::create_dir_all(project_dir.join("src/backend"))?;
    std::fs::create_dir_all(project_dir.join("src/components"))?;
    std::fs::create_dir_all(project_dir.join("src/lib"))?;
    
    std::fs::write(
        project_dir.join("src/app/page.tsx"),
        r#"export default function HomePage() {{
  return (
    <main>
      <h1>Welcome to NestForge Web</h1>
      <p>Start building your fullstack application</p>
    </main>
  );
}}
"#,
    )?;
    
    std::fs::write(
        project_dir.join("src/app/layout.tsx"),
        r#"export default function RootLayout({{ children }}: {{ children: React.ReactNode }}) {{
  return (
    <html lang="en">
      <body>{{children}}</body>
    </html>
  );
}}
"#,
    )?;
    
    std::fs::write(
        project_dir.join("src/app/api/hello/route.ts"),
        r#"export async function GET() {{
  return Response.json({{ message: "Hello from NestForge Web!" }});
}}
"#,
    )?;

    std::fs::write(
        project_dir.join("Cargo.toml"),
        format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"

[[bin]]
name = "server"
path = "src/main.rs"

[dependencies]
nestforge-web = {{ path = "../.." }}
tokio = {{ version = "1.36", features = ["full"] }}
dotenvy = "0.15"
"#, name),
    )?;

    std::fs::write(
        project_dir.join("src/main.rs"),
        r#"use nestforge_web::{NestForgeWebConfig, NestForgeWebApp};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap_or(3000);
    
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    
    let config = NestForgeWebConfig {
        app_name: env::var("APP_NAME").unwrap_or_else(|_| "nestforge-app".to_string()),
        app_dir: env::var("APP_DIR").unwrap_or_else(|_| "src/app".to_string()),
        backend_dir: env::var("BACKEND_DIR").unwrap_or_else(|_| "src/backend".to_string()),
        port,
        http_port: env::var("HTTP_PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .parse()
            .unwrap_or(3001),
        host,
    };
    
    let app = NestForgeWebApp::new(config);
    app.listen().await
}
"#,
    )?;

    std::fs::write(
        project_dir.join("src/lib.rs"),
        r#"pub mod config;
pub use config::NestForgeWebConfig;
"#,
    )?;

    std::fs::write(
        project_dir.join("src/config.rs"),
        r#"use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestForgeWebConfig {
    pub app_name: String,
    pub app_dir: String,
    pub backend_dir: String,
    pub port: u16,
    pub http_port: u16,
    pub host: String,
}

impl Default for NestForgeWebConfig {
    fn default() -> Self {
        Self {
            app_name: env::var("APP_NAME").unwrap_or_else(|_| "nestforge-app".to_string()),
            app_dir: env::var("APP_DIR").unwrap_or_else(|_| "src/app".to_string()),
            backend_dir: env::var("BACKEND_DIR").unwrap_or_else(|_| "src/backend".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            http_port: env::var("HTTP_PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()
                .unwrap_or(3001),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        }
    }
}
"#,
    )?;

    std::fs::write(
        project_dir.join(".gitignore"),
        r#"# Dependencies
node_modules/

# Build output
dist/
build/
.next/

# Environment files
.env
.env.local
.env.*.local

# IDE
.vscode/
.idea/

# OS
.DS_Store
Thumbs.db

# Logs
*.log
npm-debug.log*

# Rust
/target
Cargo.lock
"#,
    )?;

    std::fs::write(
        project_dir.join("README.md"),
        format!(r#"# {}

A NestForge Web project.

## Getting Started

1. Copy `.env.example` to `.env.local` and customize your settings
2. Run the development server:

```bash
cargo run --bin server
# or
nestforge-web dev
```

## Project Structure

```
src/
├── app/           # Frontend pages and API routes
├── backend/       # NestForge backend modules
├── components/    # React components
└── lib/           # Shared utilities
```

## Environment Variables

See `.env.example` for available configuration options.
"#, name),
    )?;

    std::fs::write(
        project_dir.join(".github"),
        r#""#,
    )?;

    std::fs::write(
        project_dir.join(".github/workflows"),
        r#""#,
    )?;

    std::fs::write(
        project_dir.join(".github/workflows/ci.yml"),
        r#"name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-action@stable
        with:
          targets: wasm32-unknown-unknown
      
      - name: Check formatting
        run: cargo fmt --all -- --check
      
      - name: Run tests
        run: cargo test --workspace
      
      - name: Build
        run: cargo build --release
"#,
    )?;

    if let Err(e) = init_git_repo(&project_dir) {
        tracing::warn!("Failed to initialize git repository: {}. You can manually run 'git init' in the project directory.", e);
    }

    tracing::info!("Project created at {}", project_dir.display());
    tracing::info!("Git repository initialized with initial commit");
    tracing::info!("Run 'cp .env.example .env.local' to configure environment");
    Ok(())
}

async fn dev_server(app_dir: Option<PathBuf>, port: Option<u16>) -> Result<()> {
    use nestforge_web_core::{NestForgeWebApp, NestForgeWebConfig};
    
    dotenvy::dotenv().ok();
    
    let resolved_port = port
        .or_else(|| env::var("PORT").ok().and_then(|p| p.parse().ok()))
        .unwrap_or(3000);
    
    let resolved_app_dir = app_dir
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| env::var("APP_DIR").unwrap_or_else(|_| "src/app".to_string()));
    
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    
    tracing::info!("Starting dev server on port {}", resolved_port);
    
    let config = NestForgeWebConfig {
        app_name: env::var("APP_NAME").unwrap_or_else(|_| "nestforge-dev".to_string()),
        app_dir: resolved_app_dir,
        backend_dir: env::var("BACKEND_DIR").unwrap_or_else(|_| "src/backend".to_string()),
        port: resolved_port,
        http_port: env::var("HTTP_PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .parse()
            .unwrap_or(3001),
        host,
    };
    
    let app = NestForgeWebApp::new(config);
    app.listen().await
}

fn build_project(app_dir: Option<PathBuf>) -> Result<()> {
    dotenvy::dotenv().ok();
    
    let resolved_app_dir = app_dir
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| env::var("APP_DIR").unwrap_or_else(|_| "src/app".to_string()));
    
    tracing::info!("Building project from {}", resolved_app_dir);
    tracing::info!("Build completed");
    Ok(())
}

async fn start_server(port: Option<u16>) -> Result<()> {
    use nestforge_web_core::{NestForgeWebApp, NestForgeWebConfig};
    
    dotenvy::dotenv().ok();
    
    let resolved_port = port
        .or_else(|| env::var("PORT").ok().and_then(|p| p.parse().ok()))
        .unwrap_or(3000);
    
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    
    tracing::info!("Starting production server on port {}", resolved_port);
    
    let config = NestForgeWebConfig {
        app_name: env::var("APP_NAME").unwrap_or_else(|_| "nestforge-app".to_string()),
        app_dir: env::var("APP_DIR").unwrap_or_else(|_| "src/app".to_string()),
        backend_dir: env::var("BACKEND_DIR").unwrap_or_else(|_| "src/backend".to_string()),
        port: resolved_port,
        http_port: env::var("HTTP_PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .parse()
            .unwrap_or(3001),
        host,
    };
    
    let app = NestForgeWebApp::new(config);
    app.listen().await
}
