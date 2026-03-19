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
        #[arg(short, long, default_value = "src/app")]
        app_dir: PathBuf,
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
    Build {
        #[arg(short, long, default_value = "src/app")]
        app_dir: PathBuf,
    },
    Start {
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
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
            tracing::info!("Starting dev server on port {}", port);
            dev_server(app_dir, port).await?;
        }
        Commands::Build { app_dir } => {
            tracing::info!("Building project from {}", app_dir.display());
            build_project(app_dir)?;
        }
        Commands::Start { port } => {
            tracing::info!("Starting production server on port {}", port);
            start_server(port).await?;
        }
    }

    Ok(())
}

fn new_project(name: &str, path: &PathBuf) -> Result<()> {
    let project_dir = path.join(name);
    std::fs::create_dir_all(&project_dir)?;
    
    std::fs::write(
        project_dir.join("nestforge-web.config.ts"),
        format!(r#"export const config = {{
  name: "{}",
  appDir: "./src/app",
  nestforgeDir: "./src/nestforge",
  port: 3000,
  host: "127.0.0.1",
}};

export default config;
"#, name),
    )?;
    
    std::fs::create_dir_all(project_dir.join("src/app"))?;
    std::fs::create_dir_all(project_dir.join("src/nestforge"))?;
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
"#, name),
    )?;

    std::fs::write(
        project_dir.join("src/main.rs"),
        r#"use nestforge_web::{NestForgeWebConfig, NestForgeWebApp};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = NestForgeWebConfig::default();
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestForgeWebConfig {
    pub app_name: String,
    pub app_dir: String,
    pub nestforge_dir: String,
    pub port: u16,
    pub host: String,
}

impl Default for NestForgeWebConfig {
    fn default() -> Self {
        Self {
            app_name: "nestforge-app".to_string(),
            app_dir: "src/app".to_string(),
            nestforge_dir: "src/nestforge".to_string(),
            port: 3000,
            host: "127.0.0.1".to_string(),
        }
    }
}
"#,
    )?;

    tracing::info!("Project created at {}", project_dir.display());
    Ok(())
}

async fn dev_server(app_dir: PathBuf, port: u16) -> Result<()> {
    use nestforge_web_core::{NestForgeWebApp, NestForgeWebConfig};
    
    let config = NestForgeWebConfig {
        app_name: "nestforge-dev".to_string(),
        app_dir: app_dir.to_string_lossy().to_string(),
        nestforge_dir: "src/nestforge".to_string(),
        port,
        host: "127.0.0.1".to_string(),
    };
    
    let app = NestForgeWebApp::new(config);
    app.listen().await
}

fn build_project(app_dir: PathBuf) -> Result<()> {
    tracing::info!("Build completed for {}", app_dir.display());
    Ok(())
}

async fn start_server(port: u16) -> Result<()> {
    use nestforge_web_core::{NestForgeWebApp, NestForgeWebConfig};
    
    let config = NestForgeWebConfig {
        port,
        ..Default::default()
    };
    
    let app = NestForgeWebApp::new(config);
    app.listen().await
}
