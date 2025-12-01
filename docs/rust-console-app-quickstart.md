# Rust Console App Quick Start

A minimal guide to creating, running, and testing a Rust console application.

## Create a New Project

```bash
cargo new my-app
cd my-app
```

This creates:
```
my-app/
├── Cargo.toml       # Project configuration
└── src/
    └── main.rs      # Your code
```

## Run Your App

```bash
cargo run
```

This compiles and runs your program in one command.

### Run in Release Mode (Optimized)

```bash
cargo run --release
```

## Build Without Running

```bash
cargo build          # Debug build
cargo build --release # Optimized build
```

The binary will be in:
- Debug: `target/debug/my-app`
- Release: `target/release/my-app`

## Test Your App

### Write a Test

Add tests to `src/main.rs` or create `src/lib.rs`:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
```

### Run Tests

```bash
cargo test
```

### Run Specific Test

```bash
cargo test test_add
```

### Run Tests with Output

```bash
cargo test -- --nocapture
```

## Quick Command Reference

| Command | Description |
|---------|-------------|
| `cargo new my-app` | Create new project |
| `cargo run` | Build and run |
| `cargo build` | Build only |
| `cargo test` | Run all tests |
| `cargo check` | Check code without building |
| `cargo clean` | Remove build artifacts |

## Example: Simple Console App

```rust
// src/main.rs
use std::io;

fn main() {
    println!("What's your name?");
    
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    println!("Hello, {}!", name.trim());
}
```

Run it:
```bash
cargo run
```

That's it! You now know how to create, run, and test Rust console applications.
