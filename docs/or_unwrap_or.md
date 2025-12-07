# Rust Option Combinators: `.or()` vs `.unwrap_or()`

## Quick Reference

### `.or(other_option)`
- **Input:** `Option<T>.or(Option<T>)`
- **Output:** `Option<T>`
- **Purpose:** Try first option, fallback to second if `None`
- **Chainable:** Yes - stays in `Option` land

```rust
Some(5).or(Some(10))  // Some(5)
None.or(Some(10))     // Some(10)
```

### `.unwrap_or(default_value)`
- **Input:** `Option<T>.unwrap_or(T)`
- **Output:** `T` (plain value)
- **Purpose:** Extract value or use default
- **Chainable:** No - exits `Option` land

```rust
Some(5).unwrap_or(10)  // 5
None.unwrap_or(10)     // 10
```

## Example Chain

```rust
primary_port
    .or(backup_port)        // Option<u16> -> Option<u16>
    .or(discovery_port)     // Option<u16> -> Option<u16>
    .unwrap_or(8080)        // Option<u16> -> u16 (done!)
```

## Common Mistake

```rust
// ❌ Won't compile
email.unwrap_or(backup).unwrap_or("default")
//    └─ Returns String, not Option<String>

// ✅ Correct
email.or(backup).unwrap_or("default")
```

## Rule of Thumb
- Use `.or()` to chain multiple fallbacks
- Use `.unwrap_or()` once at the end for final default