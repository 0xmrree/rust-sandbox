# Understanding Rust `use` Statements

The `use` keyword in Rust brings items (functions, types, traits, modules) into scope. But there are different ways to use it, and each has different effects. Let's explore the differences!

## The Three Patterns

```rust
use rand;           // Import the module
use rand::Rng;      // Import a specific item
use rand::*;        // Import everything (glob import)
```

Each pattern has different use cases and implications. Let's break them down.

## Pattern 1: `use rand;` - Import the Module

```rust
use rand;

fn main() {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..=100);
}
```

**What it does:**
- Brings the `rand` **module** into scope
- You still need to use the `rand::` prefix to access items
- Only the module name is imported, not its contents

**When to use:**
- ✅ When you want to be explicit about where things come from
- ✅ When you use multiple items from the same crate
- ✅ When you want to avoid name conflicts
- ✅ For better code readability (clear what's from `rand`)

**Pros:**
- Clear origin of functions/types
- No namespace pollution
- Easy to see dependencies

**Cons:**
- More verbose (need `rand::` prefix every time)

## Pattern 2: `use rand::Rng;` - Import a Specific Item

```rust
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..=100);  // Rng trait methods available
}
```

**What it does:**
- Brings the `Rng` **trait** into scope
- You can now use `Rng` trait methods without importing the module
- Only imports the specific item you named

**When to use:**
- ✅ When you need specific items (traits, types, functions)
- ✅ Most common pattern in Rust code
- ✅ When you want clean, readable code
- ✅ For traits (you must import traits to use their methods!)

**Pros:**
- Clean and concise
- Only imports what you need
- Standard Rust convention

**Cons:**
- Need to know exactly what to import
- May need multiple `use` statements

### Why Import Traits?

**Important:** In Rust, you must import a trait to use its methods!

```rust
// ❌ This won't work!
fn main() {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..=100);  // ERROR: gen_range not found!
}

// ✅ This works!
use rand::Rng;  // Import the Rng trait

fn main() {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..=100);  // Works! Rng trait is in scope
}
```

The `gen_range` method is defined in the `Rng` trait, so you must import `Rng` to use it.

## Pattern 3: `use rand::*;` - Glob Import (Import Everything)

```rust
use rand::*;

fn main() {
    let mut rng = thread_rng();  // No rand:: prefix needed
    let number = rng.gen_range(1..=100);
}
```

**What it does:**
- Imports **all public items** from the `rand` module
- No need for prefixes
- Everything is directly available

**When to use:**
- ⚠️ Rarely! Generally discouraged
- ✅ In tests (common pattern: `use super::*;`)
- ✅ When working with preludes (e.g., `use std::prelude::*;`)
- ✅ In small, controlled scopes

**Pros:**
- Less typing
- Convenient for exploration/prototyping

**Cons:**
- ❌ Unclear where items come from
- ❌ Can cause name conflicts
- ❌ Makes code harder to read
- ❌ Can import items you don't need
- ❌ Not idiomatic Rust

### Why Glob Imports Are Discouraged

```rust
use rand::*;
use std::collections::*;

fn main() {
    // Where does `thread_rng` come from? rand? std? Not obvious!
    let rng = thread_rng();
    
    // What if both modules have a function with the same name?
    // Compiler error or unexpected behavior!
}
```

## Comparison Table

| Pattern | Syntax | Verbosity | Clarity | Common? |
|---------|--------|-----------|---------|---------|
| Module import | `use rand;` | High (need `rand::`) | Very clear | Sometimes |
| Specific import | `use rand::Rng;` | Low | Clear | **Most common** ✅ |
| Glob import | `use rand::*;` | Very low | Unclear | Rare ⚠️ |

## Real-World Examples

### Example 1: Hangman Game (Our Project)

```rust
// ✅ Good: Import specific items
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

fn start_new_game(state: State<AppState>) -> GameState {
    let mut rng = rand::thread_rng();  // Still need rand:: for functions
    let number = rng.gen_range(0..words.len());  // But Rng methods work!
}
```

**Why this works:**
- `Rng` is a trait, so we import it to use its methods
- `Deserialize` and `Serialize` are traits for the `#[derive()]` macro
- `Mutex` is a type we use directly
- `State` is a type we use in function parameters

### Example 2: Multiple Items from Same Module

```rust
// Option 1: Multiple use statements
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;

// Option 2: Grouped imports (better!)
use std::collections::{HashMap, HashSet, BTreeMap};

// Option 3: Module import
use std::collections;

fn main() {
    let map = collections::HashMap::new();  // More verbose
}
```

### Example 3: Nested Imports

```rust
// Import from nested modules
use std::io::prelude::*;  // Glob import for prelude (acceptable)
use std::fs::File;
use std::io::{self, Read, Write};  // Import module + specific items

fn main() -> io::Result<()> {  // Can use io:: because we imported it
    let mut file = File::open("file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(())
}
```

## Advanced Patterns

### Renaming with `as`

```rust
// Avoid name conflicts by renaming
use std::io::Result as IoResult;
use std::fmt::Result as FmtResult;

fn read_file() -> IoResult<String> { /* ... */ }
fn format_data() -> FmtResult { /* ... */ }
```

### Re-exporting with `pub use`

```rust
// In a library crate
mod internal {
    pub fn helper() {}
}

// Make internal items available at the crate root
pub use internal::helper;

// Users can now do:
// use my_crate::helper;
// Instead of:
// use my_crate::internal::helper;
```

### Importing `self`

```rust
use std::io::{self, Read, Write};

// Now you can use:
// - io::Result
// - Read trait
// - Write trait
```

## The Prelude

Rust has a special module called the **prelude** that's automatically imported:

```rust
// This happens automatically in every Rust file:
use std::prelude::v1::*;
```

The prelude includes commonly used items:
- `Option`, `Some`, `None`
- `Result`, `Ok`, `Err`
- `String`, `Vec`
- `Box`, `Arc`, `Rc`
- Common traits like `Clone`, `Copy`, `Drop`

**That's why you can use these without importing them!**

```rust
// No import needed!
fn main() {
    let v: Vec<i32> = vec![1, 2, 3];
    let s = String::from("hello");
    let result: Result<i32, String> = Ok(42);
}
```

## Best Practices

### ✅ DO:

1. **Import specific items you need**
   ```rust
   use rand::Rng;
   use serde::{Serialize, Deserialize};
   ```

2. **Group imports from the same module**
   ```rust
   use std::io::{self, Read, Write, BufReader};
   ```

3. **Import traits you use**
   ```rust
   use std::io::Read;  // To use .read() method
   ```

4. **Use meaningful names when renaming**
   ```rust
   use std::io::Result as IoResult;
   ```

### ❌ DON'T:

1. **Avoid glob imports in production code**
   ```rust
   use rand::*;  // ❌ Bad: unclear where items come from
   ```

2. **Don't import unused items**
   ```rust
   use rand::{Rng, thread_rng, random};  // ❌ If you only use Rng
   ```

3. **Don't create name conflicts**
   ```rust
   use std::io::Result;
   use std::fmt::Result;  // ❌ Conflict! Use `as` to rename
   ```

## Common Patterns by Use Case

### Working with Files
```rust
use std::fs::File;
use std::io::{self, Read, Write, BufReader};
use std::path::Path;
```

### Web Development
```rust
use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};
```

### Async Programming
```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
```

### Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;  // ✅ Glob import acceptable in tests
    
    #[test]
    fn test_something() {
        // Can use all items from parent module
    }
}
```

## Quick Decision Guide

**When should I use each pattern?**

```
Need a trait's methods? 
    → use crate::Trait;

Need a specific type/function?
    → use crate::Type;

Need multiple items from same module?
    → use crate::{Item1, Item2, Item3};

Want to be very explicit?
    → use crate; (then use crate::item)

Writing tests?
    → use super::*; (acceptable here)

Everything else?
    → Avoid use crate::*;
```

## Summary

| Pattern | Example | Best For |
|---------|---------|----------|
| **Module** | `use rand;` | Explicit code, avoiding conflicts |
| **Specific** | `use rand::Rng;` | **Most common** - clean, clear code ✅ |
| **Glob** | `use rand::*;` | Tests, preludes only ⚠️ |

**Golden Rule:** Import what you need, be specific, and keep it clear where things come from!

## Resources

- **Rust Book - Modules**: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
- **Rust by Example - Use**: https://doc.rust-lang.org/rust-by-example/mod/use.html
- **Rust Reference - Use Declarations**: https://doc.rust-lang.org/reference/items/use-declarations.html
