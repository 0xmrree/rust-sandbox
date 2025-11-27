# Rust's Implicit Returns (No `return` Keyword!)

In Rust, you often see functions return values **without using the `return` keyword**. This is called an **implicit return** or **expression-based return**.

## The Rule

**If the last line of a function has NO semicolon (`;`), it's automatically returned.**

## Example from Your Code

```rust
pub fn try_nonce(&mut self, ceiling: i32) -> bool {
    // ... code ...
    
    // Check if hash is below ceiling
    if hash_value < ceiling {
        self.is_valid = true;
        true  // ← NO semicolon = this is returned!
    } else {
        false  // ← NO semicolon = this is returned!
    }
}  // Function returns bool (either true or false)
```

Notice:
- `true` has **no semicolon** → it's returned
- `false` has **no semicolon** → it's returned
- The function signature says `-> bool`, so it returns a boolean

## The Semicolon Makes the Difference!

### ❌ With Semicolon (Statement - Returns Nothing)
```rust
fn broken() -> i32 {
    42;  // ← Semicolon = statement, value is discarded
}  // ERROR: function should return i32 but returns ()
```

### ✅ Without Semicolon (Expression - Returns Value)
```rust
fn works() -> i32 {
    42  // ← No semicolon = expression, value is returned
}  // Returns 42
```

## You CAN Use `return` (But Usually Don't Need To)

### Explicit Return (Less Common in Rust)
```rust
pub fn try_nonce(&mut self, ceiling: i32) -> bool {
    if hash_value < ceiling {
        self.is_valid = true;
        return true;  // ← Explicit return with semicolon
    } else {
        return false;  // ← Explicit return with semicolon
    }
}
```

### Implicit Return (Idiomatic Rust)
```rust
pub fn try_nonce(&mut self, ceiling: i32) -> bool {
    if hash_value < ceiling {
        self.is_valid = true;
        true  // ← Implicit return, no semicolon
    } else {
        false  // ← Implicit return, no semicolon
    }
}
```

Both work, but **implicit returns are more idiomatic** (the Rust way).

## When to Use Explicit `return`

Use the `return` keyword when you need to **return early** from a function:

```rust
pub fn validate_block(&self) -> bool {
    // Early return if invalid
    if self.transactions.is_empty() {
        return false;  // Exit early
    }
    
    // Early return if hash is wrong
    if !self.check_hash() {
        return false;  // Exit early
    }
    
    // Implicit return at the end
    true  // All checks passed
}
```

## More Examples from Typical Rust Code

### Example 1: Simple Function
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // ← No semicolon = returned
}
```

### Example 2: Block Returns Value
```rust
fn get_status(valid: bool) -> String {
    if valid {
        "Valid".to_string()  // ← Returned if true
    } else {
        "Invalid".to_string()  // ← Returned if false
    }
}
```

### Example 3: Match Expression
```rust
fn describe_number(n: i32) -> &'static str {
    match n {
        0 => "zero",      // ← Each arm returns a value
        1 => "one",
        _ => "other",
    }  // ← No semicolon on the match = returned
}
```

## In Your Block Code

Let's look at another example from your codebase:

```rust
pub fn new(index: u64, transactions: Vec<Transaction>, prev_hash: String) -> Self {
    Block {
        index,
        transactions,
        nonce: 0,
        prev_hash,
        is_valid: false,
    }  // ← No semicolon = this Block is returned
}
```

The `Block { ... }` struct initialization has no semicolon, so it's automatically returned!

## Key Takeaways

1. **No semicolon on last line** = implicit return (value is returned)
2. **Semicolon on last line** = statement (value is discarded, returns `()`)
3. **`return` keyword** = explicit return (use for early exits)
4. **Implicit returns are idiomatic** = the Rust way

## Quick Comparison

| Style | Code | When to Use |
|-------|------|-------------|
| Implicit | `true` | Last expression in function (idiomatic) |
| Explicit | `return true;` | Early returns in middle of function |

## Why Rust Does This

Rust treats almost everything as an **expression** that produces a value:
- `if/else` is an expression
- `match` is an expression
- Blocks `{ }` are expressions
- Function bodies are expressions

This makes code more concise and functional-style!

## Practice Spotting Implicit Returns

```rust
// What does this return?
fn mystery() -> i32 {
    let x = 5;
    let y = 10;
    x + y  // ← Returns 15 (no semicolon!)
}

// What about this?
fn mystery2() -> () {
    let x = 5;
    let y = 10;
    x + y;  // ← Returns nothing/() (semicolon discards the value!)
}
```

---

**Remember:** In Rust, the last expression without a semicolon is automatically returned. It's not confusing once you get used to it - it's actually quite elegant! 🦀
