use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use crate::server::renderer::PageProps;

#[derive(Debug, Clone)]
pub struct Renderer {
    pages_dir: PathBuf,
    build_dir: PathBuf,
    node_modules: PathBuf,
}

impl Renderer {
    pub fn new(pages_dir: PathBuf) -> Self {
        let build_dir = pages_dir.join(".next");
        let node_modules = pages_dir.parent().unwrap_or(&pages_dir).join("node_modules");
        
        Self {
            pages_dir,
            build_dir,
            node_modules,
        }
    }

    pub async fn render(&self, path: &str, props: PageProps) -> anyhow::Result<String> {
        let script = format!(
            r#"
import React from 'react';
import ReactDOMServer from 'react-dom/server';

async function renderPage() {{
    const {{ default: Page }} = await import('./{}');
    const props = {};
    
    const html = ReactDOMServer.renderToString(
        React.createElement(Page, {{ params: props.params, searchParams: props.searchParams }})
    );
    
    console.log(html);
}}

renderPage().catch(console.error);
"#,
            path.replace("\\", "/"),
            serde_json::to_string(&props)?
        );
        
        let mut child = Command::new("node")
            .args(["--input-type=module", "-e", &script])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(self.pages_dir.parent().unwrap_or(&self.pages_dir))
            .spawn()?;
        
        let output = child.wait_with_output().await?;
        
        if output.status.success() {
            let html = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(html.trim().to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Render failed: {}", stderr);
        }
    }

    pub async fn render_with_layout(&self, path: &str, props: PageProps, layout: &str) -> anyhow::Result<String> {
        let layout_html = self.render(layout, props.clone()).await?;
        let page_html = self.render(path, props).await?;
        
        let html = layout_html.replace("{{children}}", &page_html);
        Ok(html)
    }

    pub fn get_layout_for_path(&self, path: &str) -> Option<String> {
        let mut current = PathBuf::from(&self.pages_dir);
        
        for segment in path.trim_start_matches('/').split('/') {
            if segment.is_empty() {
                continue;
            }
            current.push(segment);
        }
        
        loop {
            let layout_path = current.join("layout.tsx");
            if layout_path.exists() {
                return Some(layout_path.to_string_lossy().to_string());
            }
            
            if !current.pop() {
                break;
            }
        }
        
        let root_layout = self.pages_dir.join("layout.tsx");
        if root_layout.exists() {
            Some(root_layout.to_string_lossy().to_string())
        } else {
            None
        }
    }

    pub fn path_to_page(&self, route_path: &str) -> String {
        let mut page_path = self.pages_dir.clone();
        
        for segment in route_path.trim_start_matches('/').split('/') {
            if segment.is_empty() {
                continue;
            }
            page_path.push(segment);
        }
        
        let tsx_page = page_path.with_file_name(format!("{}.tsx", page_path.file_name().unwrap_or_default()));
        let ts_page = page_path.with_file_name(format!("{}.ts", page_path.file_name().unwrap_or_default()));
        
        if tsx_page.exists() {
            tsx_page.to_string_lossy().to_string()
        } else if ts_page.exists() {
            ts_page.to_string_lossy().to_string()
        } else {
            tsx_page.to_string_lossy().to_string()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RenderOptions {
    pub streaming: bool,
    pub cache_ttl: Option<u64>,
    pub revalidate: bool,
}

pub struct StreamingRenderer {
    renderer: Renderer,
}

impl StreamingRenderer {
    pub fn new(renderer: Renderer) -> Self {
        Self { renderer }
    }

    pub async fn stream(&self, path: &str, props: PageProps) -> anyhow::Result<tokio::io::Lines<tokio::io::BufReader<tokio::process::ChildStdout>>> {
        let page = self.renderer.path_to_page(path);
        let script = format!(
            r#"
import React from 'react';
import {{ renderToPipeableStream }} from 'react-dom/server';
import {{ default: Page }} = await import('./{}');
const props = {};
const stream = renderToPipeableStream(
    React.createElement(Page, {{ params: props.params, searchParams: props.searchParams }}),
    {{ onShellReady() {{ }}, onAllReady() {{ }} }}
);
"#,
            page.replace("\\", "/")
        );
        
        let mut child = Command::new("node")
            .args(["--input-type=module", "-e", &script])
            .stdout(Stdio::piped())
            .current_dir(self.renderer.pages_dir.parent().unwrap_or(&self.renderer.pages_dir))
            .spawn()?;
        
        let stdout = child.stdout.take().unwrap();
        Ok(tokio::io::BufReader::new(stdout).lines())
    }
}
