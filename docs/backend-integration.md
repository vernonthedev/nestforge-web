# Backend Integration

NestForge Web allows seamless integration with NestForge Rust backend modules.

## Overview

```
┌─────────────────────────────────────────┐
│           NestForge Web                   │
├─────────────────────────────────────────┤
│  Frontend (React)  │  Backend (Rust)   │
│  src/app/          │  src/backend/      │
├─────────────────────────────────────────┤
│        Axum HTTP Server                  │
└─────────────────────────────────────────┘
```

## Project Setup

### Enable Backend

When creating a project:

```bash
# With Rust backend (default)
nestforge-web new my-app

# Frontend-only (no Rust)
nestforge-web new my-app --frontend-only
```

### Cargo.toml

```toml
[package]
name = "my-app-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
nestforge = "0.1"
nestforge-web = "0.1"
nfw-core = "0.1"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
```

## Creating a Module

### 1. Define the Module

```rust
// src/backend/users/mod.rs
use nestforge::prelude::*;

#[controller("/api/users")]
pub struct UsersController;

#[routes]
impl UsersController {
    #[nestforge::get("/")]
    async fn get_users() -> ApiResult<Vec<User>> {
        let users = vec![
            User { id: 1, name: "Alice".to_string() },
            User { id: 2, name: "Bob".to_string() },
        ];
        Ok(ApiResult::ok(users))
    }

    #[nestforge::get("/{id}")]
    async fn get_user(Path(id): Path<i32>) -> ApiResult<User> {
        Ok(ApiResult::ok(User { id, name: "User".to_string() }))
    }

    #[nestforge::post("/")]
    async fn create_user(Json(body): Json<CreateUserDto>) -> ApiResult<User> {
        let user = User { id: 3, name: body.name };
        Ok(ApiResult::created(user))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub name: String,
}
```

### 2. Register the Module

```rust
// src/backend/app_module.rs
use nestforge::prelude::*;
use crate::backend::users::UsersController;

#[module(
    controllers = [UsersController],
)]
pub struct AppModule;
```

### 3. Export from Backend

```rust
// src/backend/mod.rs
pub mod users;
pub mod app_module;
```

## NestForge Module Structure

```
src/backend/
├── mod.rs              # Module exports
├── app_module.rs       # Main application module
├── app_controller.rs   # Root controller
└── users/
    ├── mod.rs
    ├── users_controller.rs
    └── users_service.rs
```

## Dependency Injection

### Services

```rust
// src/backend/users/users_service.rs
use nestforge::prelude::*;

pub struct UsersService;

#[nestforge::service]
impl UsersService {
    pub async fn find_all(&self) -> Vec<User> {
        // Fetch users from database
        vec![]
    }

    pub async fn find_by_id(&self, id: i32) -> Option<User> {
        Some(User { id, name: "User".to_string() })
    }
}
```

### Using Services in Controllers

```rust
// src/backend/users/users_controller.rs
use nestforge::prelude::*;
use super::users_service::UsersService;

#[controller("/api/users")]
pub struct UsersController;

#[routes]
impl UsersController {
    #[nestforge::get("/")]
    async fn get_users(svc: Inject<UsersService>) -> ApiResult<Vec<User>> {
        let users = svc.find_all().await;
        Ok(ApiResult::ok(users))
    }
}
```

## Request/Response Handling

### Path Parameters

```rust
#[nestforge::get("/{id}")]
async fn get_user(Path(id): Path<i32>) -> ApiResult<User> {
    Ok(ApiResult::ok(User { id, name: "User".to_string() }))
}
```

### Query Parameters

```rust
#[nestforge::get("/")]
async fn get_users(Query(pagination): Query<Pagination>) -> ApiResult<Vec<User>> {
    let users = vec![];
    Ok(ApiResult::ok(users))
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}
```

### Request Body

```rust
#[nestforge::post("/")]
async fn create_user(Json(body): Json<CreateUserDto>) -> ApiResult<User> {
    let user = User { id: 1, name: body.name };
    Ok(ApiResult::created(user))
}
```

### Headers

```rust
#[nestforge::get("/")]
async fn get_users(Header(auth): Header<Authorization>) -> ApiResult<Vec<User>> {
    Ok(ApiResult::ok(vec![]))
}
```

## Connecting Frontend to Backend

### API Client Setup

```typescript
// src/lib/api.ts
const API_BASE = process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:3000";

export async function fetchUsers() {
  const res = await fetch(`${API_BASE}/api/users`);
  return res.json();
}

export async function createUser(data: { name: string }) {
  const res = await fetch(`${API_BASE}/api/users`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(data),
  });
  return res.json();
}
```

### Using in Components

```tsx
// src/app/users/page.tsx
"use client";
import { useState, useEffect } from "react";

interface User {
  id: number;
  name: string;
}

export default function UsersPage() {
  const [users, setUsers] = useState<User[]>([]);

  useEffect(() => {
    fetch(`${API_BASE}/api/users`)
      .then((res) => res.json())
      .then((data) => setUsers(data.data ?? []));
  }, []);

  return (
    <main>
      <h1>Users</h1>
      <ul>
        {users.map((user) => (
          <li key={user.id}>{user.name}</li>
        ))}
      </ul>
    </main>
  );
}
```

## OpenAPI Documentation

Enable OpenAPI for automatic API docs:

```toml
[dependencies]
nestforge = { version = "0.1", features = ["openapi"] }
```

Access docs at `/swagger-ui` in development.

---

## Best Practices

- Keep controllers thin, services thick
- Use DTOs for request/response validation
- Leverage dependency injection for testability
- Enable OpenAPI for API documentation
- Use proper error types with `ApiResult`