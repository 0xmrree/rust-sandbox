# The `::` Operator in Rust

The `::` (double colon) operator is Rust's **path separator** used to access items within modules, types, and namespaces.

## Common Uses

### 1. **Associated Functions (Static Methods)**
Functions that belong to a type but don't take `self` as a parameter:

```rust
// Calling an associated function on Transaction
let coinbase = Transaction::new_coinbase("genesis".to_string());

// Calling an associated function on String
let text = String::from("hello");
```

### 2. **Module Items**
Accessing functions, structs, or constants from modules:

```rust
use sha2::{Digest, Sha256};

// Using Sha256 from the sha2 module
let mut hasher = Sha256::new();
```

### 3. **Enum Variants**
Accessing variants of an enum:

```rust
let result = Result::Ok(42);
let option = Option::Some(value);
```

### 4. **Constants and Statics**
```rust
let max = std::u64::MAX;
```

## Example from Your Code

```rust
pub fn genesis() -> Self {
    let coinbase = Transaction::new_coinbase("genesis".to_string());
    //              ^^^^^^^^^^^::^^^^^^^^^^^^^
    //              Type        Associated function
    Block {
        // ...
        prev_hash: "0".repeat(64),
        //         ^^^::^^^^^^
        //         &str  method (different - uses dot)
    }
}
```

## Key Distinction: `::` vs `.`

- **`::`** - Access items **on the type itself** (associated functions, constants)
- **`.`** - Call methods **on an instance** of the type

```rust
// :: accesses the type
let tx = Transaction::new_coinbase("miner".to_string());

// . calls method on the instance
let hash = tx.hash();
```

## Think of it as...

`::` is like accessing a "static" member in other languages - it belongs to the type/module, not to any particular instance.
