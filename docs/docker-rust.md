# Docker and Rust: Working Together

Docker and Rust are a powerful combination for building reliable, portable applications. This guide explains how they work together and best practices for containerizing Rust applications.

## Why Use Docker with Rust?

Docker provides several benefits for Rust development:
- **Consistent build environments** - Same compiler version across all machines
- **Easy deployment** - Package your Rust binary with all dependencies
- **Isolation** - Run multiple Rust projects with different toolchain versions
- **Cross-compilation** - Build for different platforms without complex setup

## Basic Dockerfile for Rust

Here's a simple Dockerfile for a Rust application:

```dockerfile
FROM rust:1.75

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/your-app-name"]
```

### What This Does:
1. Uses the official Rust Docker image (version 1.75)
2. Sets `/app` as the working directory
3. Copies your project files into the container
4. Builds your project in release mode
5. Runs the compiled binary

## Multi-Stage Builds (Recommended)

The basic approach creates large images. Multi-stage builds solve this by separating the build and runtime environments:

```dockerfile
# Build stage
FROM rust:1.75 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Copy only the compiled binary from builder
COPY --from=builder /app/target/release/your-app-name .

CMD ["./your-app-name"]
```

### Benefits:
- **Smaller image size** - Only includes the binary, not the entire Rust toolchain
- **Faster deployments** - Less data to transfer
- **More secure** - Fewer attack surfaces without build tools in production

### Size Comparison:
- Basic Dockerfile: ~1.5 GB
- Multi-stage build: ~80 MB

## Optimizing Build Times with Layer Caching

Docker caches layers. We can leverage this to avoid rebuilding dependencies every time:

```dockerfile
FROM rust:1.75 as builder

WORKDIR /app

# Copy only dependency files first
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer gets cached)
RUN cargo build --release

# Remove dummy files
RUN rm -rf src

# Now copy actual source code
COPY src ./src

# Build with real code (only this runs when you change code)
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/your-app-name .
CMD ["./your-app-name"]
```

### How It Works:
1. First build: Compiles all dependencies + your code
2. Code-only changes: Only recompiles your code (dependencies cached)
3. Dependency changes: Rebuilds everything (as needed)

## Using Alpine for Even Smaller Images

Alpine Linux creates the smallest possible images:

```dockerfile
FROM rust:1.75-alpine as builder

# Install build dependencies
RUN apk add --no-cache musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build with musl for static linking
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/your-app-name .

CMD ["./your-app-name"]
```

### Result:
- Image size: ~10-20 MB (depending on your binary)
- Fully static binary (no runtime dependencies)

## Common Patterns

### Web Server Example

```dockerfile
FROM rust:1.75 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim

# Install CA certificates for HTTPS
RUN apt-update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/web-server .

EXPOSE 8080

CMD ["./web-server"]
```

### With Environment Variables

```dockerfile
FROM rust:1.75 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/your-app .

ENV RUST_LOG=info
ENV APP_PORT=8080

CMD ["./your-app"]
```

## Docker Compose for Development

Create a `docker-compose.yml` for easier development:

```yaml
version: '3.8'

services:
  app:
    build: .
    ports:
      - "8080:8080"
    volumes:
      - ./src:/app/src
      - cargo-cache:/usr/local/cargo
    environment:
      - RUST_LOG=debug
    command: cargo run

volumes:
  cargo-cache:
```

### Benefits:
- Hot reload during development
- Cached dependencies in named volume
- Easy port mapping and environment configuration

## Best Practices

### 1. Use .dockerignore

Create a `.dockerignore` file to exclude unnecessary files:

```
target/
.git/
.gitignore
*.md
Dockerfile
.dockerignore
```

### 2. Pin Rust Versions

Instead of `rust:latest`, use specific versions:
```dockerfile
FROM rust:1.75.0
```

### 3. Use cargo-chef for Better Caching

Install `cargo-chef` for optimal dependency caching:

```dockerfile
FROM rust:1.75 as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.75 as builder
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/your-app .
CMD ["./your-app"]
```

### 4. Security Scanning

Run security scans on your images:
```bash
docker scan your-image-name
```

## Common Commands

### Build an image:
```bash
docker build -t my-rust-app .
```

### Run a container:
```bash
docker run -p 8080:8080 my-rust-app
```

### Run with environment variables:
```bash
docker run -e RUST_LOG=debug -p 8080:8080 my-rust-app
```

### Interactive shell in container:
```bash
docker run -it my-rust-app /bin/bash
```

### Build for development:
```bash
docker-compose up
```

## Troubleshooting

### Issue: "Cannot find binary"
**Solution:** Make sure the binary name in `CMD` matches your `Cargo.toml` package name.

### Issue: "Permission denied"
**Solution:** The binary might not be executable. Add:
```dockerfile
RUN chmod +x /app/your-app-name
```

### Issue: Large image sizes
**Solution:** Use multi-stage builds and Alpine Linux as shown above.

### Issue: Slow builds
**Solution:** Implement dependency caching or use `cargo-chef`.

## Summary

Docker and Rust work excellently together:
- **Multi-stage builds** keep images small
- **Layer caching** speeds up development
- **Alpine Linux** creates minimal production images
- **Docker Compose** simplifies local development

Start with a basic Dockerfile, then optimize with multi-stage builds and caching as your project grows.
