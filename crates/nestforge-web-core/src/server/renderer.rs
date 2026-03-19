pub mod page;

pub use page::{PageMetadata, PageProps};

mod renderer_mod {
    use std::path::PathBuf;

    #[derive(Clone)]
    pub struct Renderer {
        pages_dir: PathBuf,
    }

    impl Renderer {
        pub fn new(pages_dir: PathBuf) -> Self {
            Self { pages_dir }
        }

        pub async fn render(&self, path: &str, _props: super::PageProps) -> anyhow::Result<String> {
            Ok(format!("<html><body><h1>{}</h1></body></html>", path))
        }

        pub fn get_layout_for_path(&self, _path: &str) -> Option<String> {
            None
        }

        pub async fn render_with_layout(
            &self,
            _path: &str,
            props: super::PageProps,
            _layout: &str,
        ) -> anyhow::Result<String> {
            self.render(_path, props).await
        }

        pub fn path_to_page(&self, _route_path: &str) -> String {
            format!("{}/page.tsx", self.pages_dir.display())
        }
    }
}

pub use renderer_mod::Renderer;
