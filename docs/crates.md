# Rust Crates and Libraries

## What Are Crates?

**Crates and libraries are the same thing in Rust.** A "crate" is Rust's term for a package or library of reusable code. Think of it like npm packages in JavaScript, gems in Ruby, or pip packages in Python.

### Why "Cargo" and "Crates"?

Rust has a fun shipping/logistics theme for its package management:

- **Crates** 📦 - Packages of code (like shipping crates)
- **Cargo** 🚢 - The build tool and package manager (ships crates around)
- **crates.io** 🏭 - The package registry (the warehouse)

**Why `Cargo.toml`?**
- **Cargo** - The name of Rust's build tool and package manager
- **TOML** - "Tom's Obvious, Minimal Language" - a simple configuration file format
  - Created by Tom Preston-Werner (GitHub co-founder)
  - Easy to read and write
  - Similar to INI files but more powerful
  - Alternative to JSON/YAML for config files

So `Cargo.toml` = "Cargo's configuration file in TOML format"

Other languages use different formats:
- JavaScript: `package.json` (JSON format)
- Python: `requirements.txt` or `pyproject.toml` (text or TOML)
- Ruby: `Gemfile` (Ruby DSL)
- Go: `go.mod` (custom format)

### Types of Crates

1. **Binary Crates** - Executable programs with a `main()` function
   - Example: Your Hangman game
   - Compiled into a runnable binary/executable

2. **Library Crates** - Reusable code that other projects can import
   - Example: `rand`, `serde`, `tauri`
   - Compiled into libraries that get linked into your project

## How Crates Work

### 1. Declaring Dependencies in `Cargo.toml`

All Rust projects use a `Cargo.toml` file to declare their dependencies. This is similar to `package.json` in JavaScript or `requirements.txt` in Python.

```toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"                                    # Simple version
serde = { version = "1.0", features = ["derive"] }  # With features
tauri = { version = "1.5", features = ["shell-open"] }
```

**Version Syntax:**
- `"0.8"` - Any version compatible with 0.8.x (uses semantic versioning)
- `"=0.8.5"` - Exact version 0.8.5
- `">=0.8"` - Version 0.8 or higher
- `"^0.8"` - Compatible with 0.8 (same as just "0.8")

**Features:**
- Many crates have optional features you can enable
- Example: `features = ["derive"]` enables the derive macros in serde
- This keeps crates lightweight by only including what you need

### 2. Where Crates Come From

**crates.io** - The official Rust package registry
- Website: https://crates.io
- Contains over 100,000+ community-published crates
- Free and open source
- Automatically used by Cargo when you add dependencies

**Alternative Sources:**
```toml
# From a Git repository
my-crate = { git = "https://github.com/user/repo" }

# From a local path
my-crate = { path = "../my-local-crate" }
```

### 3. Installing Dependencies

Dependencies are **automatically installed** when you run:

```bash
cargo build    # Compile your project
cargo run      # Compile and run
cargo check    # Check if code compiles (faster, no executable)
```

**What Happens:**
1. Cargo reads `Cargo.toml`
2. Downloads crates from crates.io (or other sources)
3. Compiles all dependencies
4. Caches compiled code in `target/` directory
5. Links everything together

**First build is slow, subsequent builds are fast!** Cargo caches everything.

### 4. The `Cargo.lock` File

When you first build a project, Cargo creates a `Cargo.lock` file that records:
- Exact versions of all dependencies
- Exact versions of transitive dependencies (dependencies of your dependencies)

**Should you commit `Cargo.lock`?**
- ✅ **YES** for binary crates (applications) - ensures reproducible builds
- ❌ **NO** for library crates - lets users get the latest compatible versions

### 5. Using Crates in Your Code

Once declared in `Cargo.toml`, import them with `use` statements:

```rust
// Import the entire crate
use rand;

// Import specific items
use rand::Rng;

// Import multiple items
use serde::{Serialize, Deserialize};

// Import with alias
use std::collections::HashMap as Map;

// Import everything (use sparingly!)
use rand::*;
```

## The Standard Library (`std`)

Rust comes with a built-in standard library that's **always available** without adding to `Cargo.toml`:

```rust
use std::sync::Mutex;        // Thread-safe locks
use std::collections::HashMap; // Hash maps
use std::fs::File;           // File operations
use std::io::Read;           // I/O traits
```

The standard library includes:
- Collections (Vec, HashMap, HashSet, etc.)
- File I/O and networking
- Threading and concurrency
- String manipulation
- Error handling
- And much more!

## Common Crates You'll Use

### Essential Crates

| Crate | Purpose | Example Use |
|-------|---------|-------------|
| `serde` | Serialization/deserialization | JSON, YAML, TOML parsing |
| `tokio` | Async runtime | Async/await, networking |
| `rand` | Random number generation | Games, simulations |
| `clap` | Command-line argument parsing | CLI tools |
| `reqwest` | HTTP client | API calls, web scraping |

### Web Development

| Crate | Purpose |
|-------|---------|
| `axum` | Web framework |
| `actix-web` | Web framework |
| `rocket` | Web framework |
| `tauri` | Desktop app framework |

### Database

| Crate | Purpose |
|-------|---------|
| `sqlx` | SQL database toolkit |
| `diesel` | ORM and query builder |
| `mongodb` | MongoDB driver |

## Cargo Commands for Crates

```bash
# Add a dependency (modifies Cargo.toml)
cargo add rand

# Add a dependency with features
cargo add serde --features derive

# Remove a dependency
cargo rm rand

# Update dependencies to latest compatible versions
cargo update

# Check for outdated dependencies
cargo outdated

# Search for crates on crates.io
cargo search rand

# View dependency tree
cargo tree

# Clean build artifacts (frees disk space)
cargo clean
```

## Example: Our Hangman Game Dependencies

In the Hangman game, we use these crates:

```toml
[dependencies]
tauri = { version = "1.5", features = ["shell-open"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rand = "0.8"
```

**What each does:**
- **tauri** - Framework for building desktop apps with web technologies
- **serde** - Converts Rust structs to/from JSON for frontend communication
- **serde_json** - JSON-specific serialization (used by serde)
- **rand** - Generates random numbers to pick words

## Crates vs Libraries: The Same Thing!

In Rust terminology:
- **Crate** = The Rust term for a package/library
- **Library** = General programming term for reusable code
- **Package** = Another general term (used in other languages)

They all mean the same thing in Rust! The Rust community prefers "crate" because:
1. It's unique to Rust
2. It matches the package registry name (crates.io)
3. It sounds cool 📦

## Best Practices

1. **Keep dependencies minimal** - Only add what you need
2. **Check crate quality** - Look at:
   - Download count on crates.io
   - Last update date
   - Documentation quality
   - GitHub stars/activity
3. **Update regularly** - Run `cargo update` periodically
4. **Audit for security** - Use `cargo audit` to check for vulnerabilities
5. **Read documentation** - Use `cargo doc --open` to view all dependency docs

## Resources

- **crates.io** - https://crates.io - Browse and search crates
- **docs.rs** - https://docs.rs - Automatic documentation for all crates
- **Cargo Book** - https://doc.rust-lang.org/cargo/ - Official Cargo documentation
- **Rust Book** - https://doc.rust-lang.org/book/ - Learn Rust fundamentals

## Summary

- **Crates = Libraries** (same thing, different name)
- Declared in `Cargo.toml`, automatically downloaded from crates.io
- Use `cargo add` to add dependencies
- Import with `use` statements in your code
- Standard library (`std`) is always available
- Build artifacts cached in `target/` (gitignored)
- `Cargo.lock` ensures reproducible builds
