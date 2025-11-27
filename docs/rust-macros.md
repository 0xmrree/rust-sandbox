# Rust Macros Explained

Macros in Rust are a way to write code that writes other code (metaprogramming). They're identified by the `!` at the end of their name.

## What Are Macros?

Macros are expanded at compile time into regular Rust code. They allow you to:
- Reduce code repetition
- Create domain-specific syntax
- Generate code based on patterns

## Identifying Macros

**Easy rule:** If you see `!` after a name, it's a macro.

```rust
vec![1, 2, 3]        // macro
println!("hello")    // macro
panic!("error")      // macro
```

## Example from Your Code

```rust
pub fn genesis() -> Self {
    let coinbase = Transaction::new_coinbase("genesis".to_string());
    Block {
        index: 0,
        transactions: vec![coinbase],  // ← vec! macro here
        nonce: 0,
        prev_hash: "0".repeat(64),
        is_valid: true,
    }
}
```

### The `vec!` Macro

**What it does:**
```rust
transactions: vec![coinbase]
```

Creates a `Vec<Transaction>` containing one element: `coinbase`.

**What it expands to (approximately):**
```rust
let mut temp_vec = Vec::new();
temp_vec.push(coinbase);
transactions: temp_vec
```

**More examples:**
```rust
// Empty vector
let v: Vec<i32> = vec![];

// Vector with multiple elements
let v = vec![1, 2, 3, 4, 5];

// Vector with repeated value
let v = vec![0; 10];  // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
```

## Common Standard Library Macros

### Output Macros
```rust
println!("Hello, {}!", name);     // Print with newline
print!("No newline");             // Print without newline
eprintln!("Error: {}", err);      // Print to stderr
format!("Value: {}", x);          // Create formatted String
```

### Debugging Macros
```rust
dbg!(my_variable);                // Debug print with file/line info
assert!(x > 0);                   // Runtime assertion
assert_eq!(a, b);                 // Assert equality
assert_ne!(a, b);                 // Assert inequality
```

### Collection Macros
```rust
vec![1, 2, 3]                     // Create Vec
                                  // (HashMap/HashSet require external crates)
```

### Control Flow Macros
```rust
panic!("Something went wrong!");  // Crash the program
unimplemented!();                 // Mark unfinished code
todo!();                          // Mark planned code
unreachable!();                   // Mark code that should never run
```

### Other Useful Macros
```rust
include_str!("file.txt");         // Include file as &str at compile time
env!("PATH");                     // Get environment variable at compile time
concat!("Hello", " ", "World");   // Concatenate at compile time
```

## Types of Macros

### 1. Declarative Macros (macro_rules!)
Pattern-based macros (most common):
```rust
macro_rules! my_macro {
    ($x:expr) => {
        println!("You passed: {}", $x);
    };
}
```

### 2. Procedural Macros
More powerful, written as Rust functions:
- **Derive macros**: `#[derive(Debug, Clone)]`
- **Attribute macros**: `#[test]`
- **Function-like macros**: Custom syntax

## Why Use Macros?

### Without `vec!` macro:
```rust
let mut transactions = Vec::new();
transactions.push(coinbase);
```

### With `vec!` macro:
```rust
let transactions = vec![coinbase];
```

**Benefits:**
- ✅ Less code
- ✅ More readable
- ✅ Less error-prone
- ✅ Compile-time guarantees

## Macros vs Functions

| Feature | Macros | Functions |
|---------|--------|-----------|
| When executed | Compile time | Runtime |
| Can take variable args | ✅ Yes | ❌ No (without tricks) |
| Type checking | After expansion | Before call |
| Syntax | `name!(...)` | `name(...)` |
| Performance | Zero overhead | Function call overhead |

## Creating Your Own Macro

Here's a simple example:

```rust
macro_rules! create_block {
    ($index:expr, $prev:expr) => {
        Block {
            index: $index,
            transactions: vec![],
            nonce: 0,
            prev_hash: $prev,
            is_valid: false,
        }
    };
}

// Usage
let block = create_block!(1, "abc123".to_string());
```

## Key Takeaways

1. **Macros end with `!`** - Easy to spot in code
2. **Compile-time expansion** - No runtime cost
3. **`vec!` is your friend** - Simplest way to create vectors
4. **Common in Rust** - `println!`, `format!`, `assert!`, etc.
5. **Powerful but complex** - Start by using standard macros before writing your own

## In Your Proof-of-Work Code

```rust
// From block.rs
transactions: vec![coinbase]  // Creates Vec<Transaction> with one item

// You might also see these in your code:
println!("Block hash: {}", hash);
format!("{}{}{}{}", index, transactions_str, nonce, prev_hash);
```

---

**Remember:** Macros are code generators. They take some input syntax and produce Rust code at compile time!
