# Understanding `self` in Rust Methods

There are **three ways** to take `self` in a method, each with different ownership semantics.

## What Does `&` Mean?

The `&` symbol means **reference** (or "borrow"). It lets you use a value without taking ownership of it.

## The Four Types of Self

| Syntax | Meaning | Ownership | Can Modify? | Common Use |
|--------|---------|-----------|-------------|------------|
| `self` | Take ownership | Consumes value | ❌ No* | Converting (immutable) |
| `mut self` | Take ownership (mutable) | Consumes value | ✅ Yes | Converting (mutable) |
| `&self` | Immutable reference | Borrows | ❌ No | Reading data |
| `&mut self` | Mutable reference | Borrows mutably | ✅ Yes | Changing data |

*`self` can modify, but modifications are lost since the value is consumed

### 1. `self` - Takes Ownership (Consumes)

```rust
impl Block {
    // This method CONSUMES the block
    fn into_string(self) -> String {
        format!("Block #{}", self.index)
    }
}

// Usage:
let block = Block::new(...);
let s = block.into_string();  // block is now GONE, can't use it anymore
// block.hash();  // ❌ ERROR: block was moved
```

**When to use:**
- Converting the value into something else
- When you want to prevent further use of the value
- Rare in most code

### 2. `mut self` - Takes Ownership AND Can Modify (Very Rare!)

```rust
impl Block {
    // This method CONSUMES the block AND can modify it before consuming
    fn into_validated_block(mut self) -> Block {
        self.is_valid = true;  // Can modify before consuming
        self  // Return the modified, consumed block
    }
}

// Usage:
let block = Block::new(...);
let validated = block.into_validated_block();  // block is consumed
// block.hash();  // ❌ ERROR: block was moved
```

**When to use:**
- Need to modify the value AND consume it
- Builder pattern methods that transform and return `self`
- **Extremely rare** - usually `&mut self` is better!

**Why it's rare:**
- If you're consuming the value anyway, modifications often don't matter
- `&mut self` is almost always what you want instead
- Only useful when you need to modify AND return the consumed value

### 3. `&self` - Immutable Reference (Borrows to Read)

```rust
impl Block {
    fn hash(&self) -> String {
        // Can read self.index, self.nonce, etc.
        // Cannot modify anything
    }
}

// Usage:
let block = Block::new(...);
let hash = block.hash();  // block is still usable after
let hash2 = block.hash(); // can call again!
```

**When to use:**
- Reading/inspecting data (most common!)
- Calculating values from existing data
- Getters

### 3. `&mut self` - Mutable Reference (Borrows to Modify)

```rust
impl Block {
    fn try_nonce(&mut self, ceiling: i32) -> bool {
        self.nonce = rng.gen();  // Can modify!
        self.is_valid = true;    // Can modify!
    }
}

// Usage:
let mut block = Block::new(...);  // must be mut
block.try_nonce(1000);  // block is modified but still usable
block.try_nonce(2000);  // can call again!
```

**When to use:**
- Modifying the struct's fields
- Updating state
- Setters

## Examples from Your Block Code

### `&self` - Read-Only Access

```rust
impl Hashable for Block {
    fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        
        // Hash all transactions
        for tx in &self.transactions {
            hasher.update(tx.hash());
        }
        
        // Add previous block hash
        hasher.update(&self.prev_hash);
        // ...
    }
}
```

**Why `&self`?**
- The `hash()` method only **reads** the block's data
- It doesn't change `transactions`, `prev_hash`, `nonce`, etc.
- Multiple parts of code can call `hash()` at the same time (safe!)

### `&mut self` - Can Modify

```rust
impl Block {
    pub fn try_nonce(&mut self, ceiling: i32) -> bool {
        // Generate random nonce
        let mut rng = rand::thread_rng();
        self.nonce = rng.gen();  // ← MODIFYING self.nonce
        
        // Calculate hash with this nonce
        let hash = self.hash();
        
        // Check if hash is below ceiling
        if hash_value < ceiling {
            self.is_valid = true;  // ← MODIFYING self.is_valid
            true
        } else {
            false
        }
    }
}
```

**Why `&mut self`?**
- The `try_nonce()` method **changes** the block's data
- It modifies `self.nonce` and `self.is_valid`
- Only one part of code can have mutable access at a time (prevents bugs!)

## The Rule

```rust
// ❌ This won't compile - &self can't modify
fn broken_method(&self) {
    self.nonce = 42;  // ERROR: cannot assign to immutable field
}

// ✅ This works - &mut self can modify
fn working_method(&mut self) {
    self.nonce = 42;  // OK!
}
```

## Calling These Methods

```rust
let mut block = Block::new(1, transactions, prev_hash);

// Calling &self method (no mut needed on variable)
let hash = block.hash();  // Just reading

// Calling &mut self method (variable must be mut)
block.try_nonce(1000);    // Modifying the block
```

**Important:** To call a method with `&mut self`, the variable itself must be declared as `mut`:

```rust
let block = Block::new(...);        // immutable
block.try_nonce(1000);              // ❌ ERROR

let mut block = Block::new(...);    // mutable
block.try_nonce(1000);              // ✅ OK
```

## Why This Matters

Rust's borrow checker enforces these rules at compile time:

### Safety Rule: Only One Mutable Reference
```rust
let mut block = Block::new(...);

let ref1 = &mut block;
let ref2 = &mut block;  // ❌ ERROR: can't have two mutable borrows
```

This prevents data races and bugs!

### Multiple Immutable References Are OK
```rust
let block = Block::new(...);

let hash1 = block.hash();  // OK
let hash2 = block.hash();  // OK - both just reading
```

## Choosing Between Them

**Use `&self` when:**
- ✅ Method only reads/inspects data
- ✅ Method calculates something from existing data
- ✅ Examples: `hash()`, `is_valid()`, getters

**Use `&mut self` when:**
- ✅ Method changes the struct's fields
- ✅ Method updates state
- ✅ Examples: `try_nonce()`, setters, state transitions

## In Your Proof-of-Work Code

```rust
// Read-only methods use &self
impl Hashable for Block {
    fn hash(&self) -> String { ... }  // Just reading
}

impl Validatable for Block {
    fn is_valid(&self) -> bool { ... }  // Just checking
}

// Modifying methods use &mut self
impl Block {
    pub fn try_nonce(&mut self, ceiling: i32) -> bool {
        self.nonce = rng.gen();      // Changing nonce
        self.is_valid = true;        // Changing validity
        // ...
    }
}
```

## Key Takeaway

Think of it like borrowing a book:
- **`&self`** = Borrow to read (many people can read at once)
- **`&mut self`** = Borrow to write in (only one person can write at a time)

This is Rust's way of preventing bugs at compile time! 🦀
