use std::collections::HashMap;
use std::path::Path;

use crate::routing::Route;

#[derive(Debug, Clone)]
pub struct TypeSpec {
    pub name: String,
    pub fields: Vec<FieldSpec>,
    pub is_enum: bool,
    pub enum_variants: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FieldSpec {
    pub name: String,
    pub type_name: String,
    pub is_optional: bool,
    pub is_array: bool,
}

#[derive(Debug, Clone)]
pub struct EndpointSpec {
    pub path: String,
    pub method: String,
    pub request_type: Option<TypeSpec>,
    pub response_type: Option<TypeSpec>,
    pub params: Vec<FieldSpec>,
    pub query_params: Vec<FieldSpec>,
}

pub struct ApiGenerator {
    routes: Vec<Route>,
    types: Vec<TypeSpec>,
}

impl ApiGenerator {
    pub fn new(routes: Vec<Route>) -> Self {
        Self {
            routes,
            types: Vec::new(),
        }
    }

    pub fn add_type(&mut self, spec: TypeSpec) {
        self.types.push(spec);
    }

    pub fn generate_typescript_types(&self) -> String {
        let mut output = String::from("// Auto-generated TypeScript types\n\n");

        for typ in &self.types {
            if typ.is_enum {
                output.push_str(&format!("export enum {} {{\n", typ.name));
                for variant in &typ.enum_variants {
                    output.push_str(&format!("  {} = '{}',\n", variant, variant));
                }
                output.push_str("}\n\n");
            } else {
                output.push_str(&format!("export interface {} {{\n", typ.name));
                for field in &typ.fields {
                    let optional = if field.is_optional { "?" } else { "" };
                    let array = if field.is_array { "[]" } else { "" };
                    output.push_str(&format!(
                        "  {}{}: {}{};\n",
                        field.name, optional, field.type_name, array
                    ));
                }
                output.push_str("}\n\n");
            }
        }

        output
    }

    pub fn generate_typescript_client(&self) -> String {
        let mut output = String::from("// Auto-generated API client\n\n");
        output.push_str(
            r#"
export class ApiClient {
    private baseUrl: string;
    private defaultHeaders: Record<string, string>;

    constructor(baseUrl: string) {
        this.baseUrl = baseUrl;
        this.defaultHeaders = {
            'Content-Type': 'application/json',
        };
    }

    private async request<T>(
        method: string,
        path: string,
        options: {
            params?: Record<string, string>;
            query?: Record<string, string | string[]>;
            body?: unknown;
        } = {}
    ): Promise<T> {
        let url = new URL(path, this.baseUrl);

        if (options.params) {
            for (const [key, value] of Object.entries(options.params)) {
                url.pathname = url.pathname.replace(`{${key}}`, value);
            }
        }

        if (options.query) {
            for (const [key, value] of Object.entries(options.query)) {
                if (Array.isArray(value)) {
                    value.forEach(v => url.searchParams.append(key, v));
                } else {
                    url.searchParams.set(key, value);
                }
            }
        }

        const response = await fetch(url.toString(), {
            method,
            headers: this.defaultHeaders,
            body: options.body ? JSON.stringify(options.body) : undefined,
        });

        if (!response.ok) {
            throw new ApiError(response.status, await response.text());
        }

        return response.json();
    }
"#,
        );

        for endpoint in self.get_endpoint_specs() {
            let method_name = to_camel_case(&endpoint.path.replace("/", "_"));
            let method_lower = endpoint.method.to_lowercase();

            output.push_str(&format!("\n    async {}(options?: {{", method_name));

            if !endpoint.params.is_empty() {
                output.push_str("params: { ");
                let params: Vec<String> = endpoint
                    .params
                    .iter()
                    .map(|p| format!("{}: {}", p.name, p.type_name))
                    .collect();
                output.push_str(&params.join(", "));
                output.push_str(" }, ");
            }

            if !endpoint.query_params.is_empty() {
                output.push_str("query?: { ");
                let query: Vec<String> = endpoint
                    .query_params
                    .iter()
                    .map(|p| format!("{}?: {}", p.name, p.type_name))
                    .collect();
                output.push_str(&query.join(", "));
                output.push_str(" }, ");
            }

            if endpoint.request_type.is_some() {
                output.push_str(&format!(
                    "body: {}, ",
                    endpoint.request_type.as_ref().unwrap().name
                ));
            }

            output.push_str("}): Promise<");
            if let Some(ref resp) = endpoint.response_type {
                output.push_str(&resp.name);
            } else {
                output.push_str("void");
            }
            output.push_str("> {\n");

            output.push_str(&format!(
                "        return this.request<{}>('{}', '{}', {{",
                endpoint
                    .response_type
                    .as_ref()
                    .map(|t| t.name.clone())
                    .unwrap_or_else(|| "void".to_string()),
                method_lower,
                endpoint.path
            ));

            if !endpoint.params.is_empty() {
                output.push_str(" params: options?.params,");
            }
            if !endpoint.query_params.is_empty() {
                output.push_str(" query: options?.query,");
            }
            if endpoint.request_type.is_some() {
                output.push_str(" body: options?.body,");
            }

            output.push_str(" });\n    }\n");
        }

        output.push_str("}\n\n");

        output.push_str(
            r#"
export class ApiError extends Error {
    constructor(public status: number, message: string) {
        super(message);
        this.name = 'ApiError';
    }
}
"#,
        );

        output
    }

    pub fn generate_rust_client(&self) -> String {
        let mut output = String::from("// Auto-generated Rust API client\n\n");

        output.push_str("use serde::{Deserialize, Serialize};\n");
        output.push_str("use std::collections::HashMap;\n\n");

        output.push_str("pub struct ApiClient {\n");
        output.push_str("    base_url: String,\n");
        output.push_str("    client: reqwest::Client,\n");
        output.push_str("    headers: HashMap<String, String>,\n");
        output.push_str("}\n\n");

        output.push_str("impl ApiClient {\n");
        output.push_str("    pub fn new(base_url: &str) -> Self {\n");
        output.push_str("        Self {\n");
        output.push_str("            base_url: base_url.to_string(),\n");
        output.push_str("            client: reqwest::Client::new(),\n");
        output.push_str("            headers: HashMap::new(),\n");
        output.push_str("        }\n");
        output.push_str("    }\n\n");

        output.push_str("    pub fn with_header(mut self, key: &str, value: &str) -> Self {\n");
        output.push_str("        self.headers.insert(key.to_string(), value.to_string());\n");
        output.push_str("        self\n");
        output.push_str("    }\n\n");

        for endpoint in self.get_endpoint_specs() {
            let method_name = to_snake_case(&endpoint.path.replace("/", "_"));
            let http_method = endpoint.method.to_lowercase();
            let return_type = endpoint
                .response_type
                .as_ref()
                .map(|t| "serde_json::Value".to_string())
                .unwrap_or_else(|| "()".to_string());

            output.push_str(&format!(
                "    pub async fn {}(&self, path: &str{} {}) -> anyhow::Result<{}> {{\n",
                method_name,
                if !endpoint.params.is_empty() {
                    ", params: HashMap<String, String>"
                } else {
                    ""
                },
                if endpoint.request_type.is_some() {
                    ", body: serde_json::Value"
                } else {
                    ""
                },
                return_type
            ));

            output.push_str(&format!(
                "        let url = format!(\"{{}}{{}}\", self.base_url, path);\n"
            ));
            output.push_str(&format!(
                "        let mut req = self.client.{}(&url);\n",
                http_method
            ));

            if endpoint.request_type.is_some() {
                output.push_str("        req = req.json(&body);\n");
            }

            output.push_str("        for (k, v) in &self.headers {\n");
            output.push_str("            req = req.header(k, v);\n");
            output.push_str("        }\n");
            output.push_str("        let resp = req.send().await?;\n");
            output.push_str("        let text = resp.text().await?;\n");

            if endpoint.response_type.is_some() {
                output.push_str("        Ok(serde_json::from_str(&text)?)\n");
            } else {
                output.push_str("        Ok(())\n");
            }

            output.push_str("    }\n\n");
        }

        output.push_str("}\n");

        output
    }

    pub fn get_endpoint_specs(&self) -> Vec<EndpointSpec> {
        self.routes
            .iter()
            .map(|r| EndpointSpec {
                path: r.path.clone(),
                method: r.method.as_str().to_string(),
                request_type: None,
                response_type: None,
                params: Vec::new(),
                query_params: Vec::new(),
            })
            .collect()
    }

    pub fn write_typescript(&self, output_dir: &Path) -> anyhow::Result<()> {
        let types_path = output_dir.join("api.types.ts");
        let client_path = output_dir.join("api.client.ts");

        std::fs::write(&types_path, self.generate_typescript_types())?;
        std::fs::write(&client_path, self.generate_typescript_client())?;

        tracing::info!("Generated TypeScript types at {}", types_path.display());
        tracing::info!("Generated TypeScript client at {}", client_path.display());

        Ok(())
    }

    pub fn write_rust(&self, output_dir: &Path) -> anyhow::Result<()> {
        let client_path = output_dir.join("api_client.rs");
        std::fs::write(&client_path, self.generate_rust_client())?;
        tracing::info!("Generated Rust client at {}", client_path.display());
        Ok(())
    }
}

fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize = false;

    for c in s.chars() {
        if c == '_' || c == '-' || c == '/' {
            capitalize = true;
        } else if capitalize {
            result.push(c.to_ascii_uppercase());
            capitalize = false;
        } else {
            result.push(c);
        }
    }

    result
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
    }
    result.replace('-', "_").replace('/', "_")
}
