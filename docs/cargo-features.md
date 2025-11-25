# Cargo Features: Optional Functionality in Dependencies

## What Are Features?

**Features** are optional functionality that crates can provide. They allow you to:
- Enable/disable parts of a crate
- Reduce compile times by only including what you need
- Minimize binary size
- Add optional dependencies

Think of features like "add-ons" or "plugins" for a crate. You only enable the ones you need.

## Why Do Features Exist?

### Problem Without Features
Imagine a crate that supports:
- JSON serialization
- XML serialization
- YAML serialization
- Binary serialization

If all of these were always included, you'd get:
- ❌ Longer compile times
- ❌ Larger binary size
- ❌ Unnecessary dependencies
- ❌ More code to maintain

### Solution With Features
The crate can make each format optional:
```toml
[dependencies]
my-serializer = { version = "1.0", features = ["json"] }  # Only JSON
```

Now you get:
- ✅ Faster compile times
- ✅ Smaller binary
- ✅ Only the dependencies you need
- ✅ Cleaner, more focused code

## How to Use Features

### Basic Syntax

```toml
[dependencies]
# No features (default features only)
rand = "0.8"

# With specific features
serde = { version = "1.0", features = ["derive"] }

# Multiple features
tokio = { version = "1.0", features = ["full", "macros"] }

# Disable default features
regex = { version = "1.0", default-features = false }

# Disable defaults and add specific features
serde = { version = "1.0", default-features = false, features = ["derive"] }
```

### Real Examples from Our Hangman Game

```toml
[dependencies]
# Tauri with shell-open feature
tauri = { version = "1.5", features = ["shell-open"] }
# Enables: Opening URLs in the default browser

# Serde with derive feature
serde = { version = "1.0", features = ["derive"] }
# Enables: #[derive(Serialize, Deserialize)] macros
```

## Common Features You'll Encounter

### 1. `derive` Feature (serde)
```toml
serde = { version = "1.0", features = ["derive"] }
```

**What it does:**
- Enables `#[derive(Serialize, Deserialize)]` macros
- Automatically implements serialization for your structs

**Without derive:**
```rust
// Manual implementation (tedious!)
impl Serialize for MyStruct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        // ... lots of boilerplate code
    }
}
```

**With derive:**
```rust
// Automatic implementation (easy!)
#[derive(Serialize, Deserialize)]
struct MyStruct {
    name: String,
    age: u32,
}
```

### 2. `full` Feature (tokio)
```toml
tokio = { version = "1.0", features = ["full"] }
```

**What it does:**
- Enables ALL tokio features
- Useful for learning or prototyping
- Not recommended for production (includes unnecessary code)

**Better approach for production:**
```toml
tokio = { version = "1.0", features = ["rt-multi-thread", "macros", "net"] }
# Only includes: runtime, macros, and networking
```

### 3. `macros` Feature (tokio)
```toml
tokio = { version = "1.0", features = ["macros"] }
```

**What it does:**
- Enables `#[tokio::main]` and `#[tokio::test]` macros
- Makes async code easier to write

### 4. Feature Combinations

```toml
# Web server with specific features
actix-web = { version = "4.0", features = ["rustls", "cookies"] }
# Enables: TLS support via rustls + cookie handling

# Database with async support
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres"] }
# Enables: Tokio runtime + TLS + PostgreSQL driver
```

## Default Features

Most crates have **default features** that are enabled automatically.

### Example: `serde`
```toml
# In serde's Cargo.toml (simplified)
[features]
default = ["std"]  # "std" feature is enabled by default
std = []           # Enables standard library support
derive = []        # NOT enabled by default
```

### Disabling Default Features

Sometimes you want to disable defaults (e.g., for `no_std` environments):

```toml
[dependencies]
serde = { version = "1.0", default-features = false, features = ["derive"] }
# Disables "std" but enables "derive"
```

## How Crates Define Features

Inside a crate's `Cargo.toml`, features are defined like this:

```toml
[features]
# Default features (enabled automatically)
default = ["std", "json"]

# Individual features
std = []                    # Empty = just a flag
json = ["serde_json"]       # Enables serde_json dependency
xml = ["quick-xml"]         # Enables quick-xml dependency
full = ["json", "xml"]      # Enables multiple features
```

## Finding Available Features

### Method 1: Check crates.io
1. Go to https://crates.io
2. Search for the crate
3. Look at the "Feature Flags" section

### Method 2: Check docs.rs
1. Go to https://docs.rs
2. Search for the crate
3. Look for "Feature Flags" in the documentation

### Method 3: Check the Source
1. Look at the crate's `Cargo.toml` on GitHub
2. Find the `[features]` section

### Method 4: Use `cargo tree`
```bash
# Show features being used
cargo tree -e features
```

## Feature Best Practices

### ✅ DO:
1. **Only enable features you need**
   ```toml
   tokio = { version = "1.0", features = ["rt-multi-thread", "net"] }
   ```

2. **Read the crate's documentation**
   - Understand what each feature does
   - Check for recommended feature combinations

3. **Use `default-features = false` for embedded/no_std**
   ```toml
   serde = { version = "1.0", default-features = false }
   ```

4. **Document why you need specific features**
   ```toml
   [dependencies]
   # Need derive for automatic serialization in API responses
   serde = { version = "1.0", features = ["derive"] }
   ```

### ❌ DON'T:
1. **Don't blindly use `full` features**
   ```toml
   # Bad: Includes everything, even what you don't need
   tokio = { version = "1.0", features = ["full"] }
   ```

2. **Don't enable conflicting features**
   - Some features are mutually exclusive
   - Read the documentation!

3. **Don't forget to test with minimal features**
   - Ensure your code works with only necessary features

## Real-World Examples

### Example 1: Web API Server
```toml
[dependencies]
# Async runtime with networking
tokio = { version = "1.0", features = ["rt-multi-thread", "macros", "net"] }

# Web framework with TLS
actix-web = { version = "4.0", features = ["rustls"] }

# Serialization with derive macros
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Database with async PostgreSQL
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres"] }
```

### Example 2: CLI Tool
```toml
[dependencies]
# Command-line parsing with colors and suggestions
clap = { version = "4.0", features = ["derive", "color", "suggestions"] }

# Serialization for config files
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"

# HTTP client with TLS
reqwest = { version = "0.11", features = ["rustls-tls", "json"] }
```

### Example 3: Embedded System (no_std)
```toml
[dependencies]
# Disable std library support
serde = { version = "1.0", default-features = false, features = ["derive"] }

# Embedded-friendly random numbers
rand = { version = "0.8", default-features = false }
```

## Feature Unification

**Important:** If multiple crates in your dependency tree enable the same feature, it gets enabled for everyone.

```toml
[dependencies]
crate-a = { version = "1.0", features = ["feature-x"] }
crate-b = { version = "1.0" }  # Doesn't enable feature-x
```

If `crate-b` also depends on the same underlying crate, and `crate-a` enables `feature-x`, then `feature-x` will be enabled for both!

This is called **feature unification** and is usually what you want, but be aware of it.

## Checking What Features Are Active

```bash
# Show dependency tree with features
cargo tree -e features

# Show features for a specific package
cargo tree -e features -p serde

# Check what features your crate provides
cargo metadata --format-version 1 | grep features
```

## Summary

- **Features** = Optional functionality in crates
- Enable with: `features = ["feature-name"]`
- Disable defaults with: `default-features = false`
- Benefits: Faster builds, smaller binaries, fewer dependencies
- Find available features on crates.io or docs.rs
- Only enable what you need!

## Quick Reference

```toml
# Basic feature usage
crate = { version = "1.0", features = ["feature1", "feature2"] }

# Disable defaults
crate = { version = "1.0", default-features = false }

# Disable defaults + enable specific features
crate = { version = "1.0", default-features = false, features = ["feature1"] }

# No features (just defaults)
crate = "1.0"
```

## Resources

- **Cargo Book - Features**: https://doc.rust-lang.org/cargo/reference/features.html
- **crates.io**: https://crates.io - Check "Feature Flags" section
- **docs.rs**: https://docs.rs - Read feature documentation
