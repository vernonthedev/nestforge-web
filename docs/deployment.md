# Deployment

Deploy your NestForge Web application to production.

## Build for Production

```bash
# Build the application
nestforge-web build

# Output is generated to .next/ (or configured outputDir)
```

## Environment Configuration

### Production `.env`

```bash
# Server
PORT=8080
HOST=0.0.0.0

# Environment
NODE_ENV=production
RUST_BACKTRACE=0

# Backend
DATABASE_URL=postgres://user:pass@host:5432/db
JWT_SECRET=your-production-secret
```

## Running in Production

### Using the CLI

```bash
nestforge-web start --port 8080 --host 0.0.0.0
```

### Using a Process Manager

#### Systemd Service

```ini
# /etc/systemd/system/nestforge-web.service
[Unit]
Description=NestForge Web Application
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/my-app
ExecStart=/usr/local/bin/nestforge-web start --port 8080
Restart=on-failure
RestartSec=5
Environment=NODE_ENV=production
Environment=PORT=8080

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable nestforge-web
sudo systemctl start nestforge-web
```

#### PM2 (Node.js ecosystem)

```bash
npm install -g pm2

# Create ecosystem file
cat > ecosystem.config.js << EOF
module.exports = {
  apps: [{
    name: "my-app",
    script: "nestforge-web",
    args: "start --port 8080",
    instances: 1,
    autorestart: true,
    watch: false,
    max_memory_restart: "1G",
    env: {
      NODE_ENV: "production"
    }
  }]
};
EOF

pm2 start ecosystem.config.js
pm2 save
pm2 startup
```

## Docker Deployment

### Dockerfile

```dockerfile
# Build stage
FROM rust:1.75 as builder
WORKDIR /app

# Install dependencies
RUN cargo install nestforge-web-cli

# Clone and build app
WORKDIR /build
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app

# Copy binaries
COPY --from=builder /root/.cargo/bin/nestforge-web /usr/local/bin/
COPY --from=builder /build/target/release/my-app /app/

# Copy app files
COPY . .

EXPOSE 8080

CMD ["nestforge-web", "start", "--port", "8080"]
```

### Docker Compose

```yaml
version: "3.8"

services:
  app:
    build: .
    ports:
      - "8080:8080"
    environment:
      - NODE_ENV=production
      - DATABASE_URL=postgres://user:pass@db:5432/myapp
    depends_on:
      - db
    restart: unless-stopped

  db:
    image: postgres:15
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: pass
      POSTGRES_DB: myapp
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
```

## Reverse Proxy

### Nginx

```nginx
server {
    listen 80;
    server_name example.com;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }
}
```

### Caddy

```Caddyfile
example.com {
    reverse_proxy localhost:8080
}
```

## Performance Tuning

### Rust Backend

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### Server Settings

```bash
# Increase file descriptors
ulimit -n 65535

# TCP optimizations
sysctl -w net.core.somaxconn=65535
sysctl -w net.ipv4.tcp_max_syn_backlog=65535
```

## Health Checks

Add a health endpoint:

```typescript
// src/app/api/health/route.ts
export async function GET() {
  return Response.json({
    status: "healthy",
    timestamp: new Date().toISOString(),
  });
}
```

### Nginx Health Check

```nginx
location /api/health {
    proxy_pass http://127.0.0.1:8080;
    access_log off;
}
```

## Monitoring

### Logging

Configure structured logging:

```typescript
// In your app
export async function GET() {
  console.log(JSON.stringify({
    level: "info",
    message: "Health check",
    timestamp: new Date().toISOString(),
  }));
}
```

### Metrics

Use Prometheus for metrics collection:

```rust
// Rust backend
use prometheus::{IntCounter, IntGauge};

static REQUEST_COUNT: IntCounter = IntCounter::new(
    "http_requests_total",
    "Total HTTP requests"
).expect("metric creation");

REQUEST_COUNT.inc();
```

## Security

### HTTPS

Always use HTTPS in production. Configure with Let's Encrypt:

```bash
# Certbot
sudo certbot --nginx -d example.com
```

### Security Headers

```nginx
add_header X-Frame-Options "SAMEORIGIN" always;
add_header X-Content-Type-Options "nosniff" always;
add_header X-XSS-Protection "1; mode=block" always;
add_header Referrer-Policy "strict-origin-when-cross-origin" always;
```

### Rate Limiting

Configure at reverse proxy level or application level.

---

## CI/CD

### GitHub Actions

```yaml
# .github/workflows/deploy.yml
name: Deploy

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build
        run: |
          cargo build --release
          npm run build

      - name: Deploy
        run: |
          # Your deployment logic
```

## Troubleshooting

### High Memory Usage

- Check for memory leaks in Rust code
- Enable release optimizations
- Reduce concurrent connections

### Slow Response Times

- Enable LTO and optimizations in Cargo.toml
- Use connection pooling for databases
- Enable caching where appropriate

### Connection Refused

- Verify PORT and HOST settings
- Check firewall rules
- Ensure no other process is using the port