# Rust Loops

Rust has several types of loops for different use cases.

## The Three Loop Types

| Loop Type | Use Case | Example |
|-----------|----------|---------|
| `loop` | Infinite loop (until `break`) | `loop { ... }` |
| `while` | Loop while condition is true | `while x < 10 { ... }` |
| `for` | Iterate over a collection/range | `for i in 0..10 { ... }` |

## 1. `loop` - Infinite Loop

Runs forever until you explicitly `break` out of it.

### Example from Your Code

```rust
pub fn start_mining(&mut self) {
    println!("🚀 {} started mining...\n", self.id);
    
    loop {  // ← Infinite loop - mines blocks forever
        let start_time = Instant::now();
        
        // Try to mine a block
        while !self.blockchain.try_mine_block(&self.id) {
            // Keep trying different nonces
        }
        
        // Block mined! Wait before next block
        let elapsed = start_time.elapsed();
        let delay = Duration::from_secs(self.blockchain.config.delay_seconds);
        
        if elapsed < delay {
            thread::sleep(delay - elapsed);
        }
        
        self.print_chain();
    }  // ← Loop never ends (unless program is stopped)
}
```

**When to use:**
- When you don't know how many iterations you need
- Server loops, game loops, mining loops
- When the exit condition is complex

### Breaking Out of a Loop

```rust
loop {
    let input = get_user_input();
    
    if input == "quit" {
        break;  // Exit the loop
    }
    
    process(input);
}
```

### Returning a Value from a Loop

```rust
let result = loop {
    let value = try_something();
    
    if value > 100 {
        break value;  // Return value from loop
    }
};
// result now contains the value that was > 100
```

## 2. `while` - Conditional Loop

Runs while a condition is true.

### Example from Your Code

```rust
// Try to mine a block (keep trying until successful)
while !self.blockchain.try_mine_block(&self.id) {
    // Keep trying different nonces
}
// Exits when try_mine_block returns true
```

**When to use:**
- When you have a clear condition to check
- Simpler than `loop` with `if/break`

### More Examples

```rust
// Count down
let mut count = 10;
while count > 0 {
    println!("{}", count);
    count -= 1;
}

// Process until empty
while !queue.is_empty() {
    let item = queue.pop();
    process(item);
}
```

## 3. `for` - Iterator Loop

Iterates over a collection or range.

### Example from Your Code

```rust
pub fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    let hex: String = (0..4)  // ← Range: 0, 1, 2, 3
        .map(|_| format!("{:x}", rng.gen_range(0..16)))
        .collect();
    format!("node-{}", hex)
}
```

### Common `for` Loop Patterns

#### Iterate Over a Range
```rust
// 0 to 9 (excludes 10)
for i in 0..10 {
    println!("{}", i);
}

// 0 to 10 (includes 10)
for i in 0..=10 {
    println!("{}", i);
}
```

#### Iterate Over a Vector
```rust
let numbers = vec![1, 2, 3, 4, 5];

// By reference (can use values after loop)
for num in &numbers {
    println!("{}", num);
}

// By mutable reference (can modify)
for num in &mut numbers {
    *num *= 2;
}

// By value (consumes the vector)
for num in numbers {
    println!("{}", num);
}
// numbers is no longer usable here
```

#### Why Consume by Value?

You might wonder: why would you ever want to consume a collection if it removes it from memory? Here are the good reasons:

**1. You're Done With the Collection**

If you don't need it after the loop, consuming is more efficient:

```rust
fn process_transactions(transactions: Vec<Transaction>) {
    for tx in transactions {  // Consume the vector
        send_to_network(tx);  // tx is moved into this function
    }
    // transactions is gone, but we don't need it anymore anyway
}
```

**2. Moving Ownership Into Another Structure**

Transfer items without cloning:

```rust
fn filter_valid_blocks(blocks: Vec<Block>) -> Vec<Block> {
    let mut valid = Vec::new();
    
    for block in blocks {  // Consume the original
        if block.is_valid {
            valid.push(block);  // Move block into new vector
        }
        // Invalid blocks are dropped (freed)
    }
    
    valid
}

// Alternative with references requires expensive cloning:
for block in &blocks {
    if block.is_valid {
        valid.push(block.clone());  // ← Expensive!
    }
}
```

**3. Types That Don't Implement `Copy`**

For `String`, `Vec`, etc., consuming avoids cloning:

```rust
let messages = vec![
    String::from("Hello"),
    String::from("World"),
];

// Move each String
for msg in messages {
    send_message(msg);  // msg is moved here
}

// vs. with references (requires cloning):
for msg in &messages {
    send_message(msg.clone());  // Less efficient
}
```

**4. Functional Programming Patterns**

```rust
let numbers = vec![1, 2, 3, 4, 5];

let sum: i32 = numbers
    .into_iter()  // Consume the vector
    .filter(|&x| x > 2)
    .map(|x| x * 2)
    .sum();  // Result: 24
```

**5. Avoiding Lifetime Issues**

Consuming simplifies ownership:

```rust
fn get_first_valid(blocks: Vec<Block>) -> Option<Block> {
    for block in blocks {
        if block.is_valid {
            return Some(block);  // Return owned block
        }
    }
    None
}
```

**When NOT to Consume:**

```rust
let blocks = vec![block1, block2, block3];

// ❌ Bad - consumes blocks
for block in blocks {
    println!("{:?}", block);
}
// Can't use blocks anymore!

// ✅ Good - borrows blocks
for block in &blocks {
    println!("{:?}", block);
}
// Can still use blocks here
```

**Key Takeaway:** Consuming by value is efficient when you don't need the collection afterward. Rust's ownership system makes this safe - you can't accidentally use the consumed collection!

#### Iterate with Index
```rust
let items = vec!["a", "b", "c"];

for (index, item) in items.iter().enumerate() {
    println!("{}: {}", index, item);
}
// Output:
// 0: a
// 1: b
// 2: c
```

## Loop Control Keywords

### `break` - Exit the Loop
```rust
for i in 0..100 {
    if i == 50 {
        break;  // Stop at 50
    }
    println!("{}", i);
}
```

### `continue` - Skip to Next Iteration
```rust
for i in 0..10 {
    if i % 2 == 0 {
        continue;  // Skip even numbers
    }
    println!("{}", i);  // Only prints odd numbers
}
```

### Labeled Loops (Breaking Outer Loops)
```rust
'outer: loop {
    println!("Outer loop");
    
    loop {
        println!("Inner loop");
        break 'outer;  // Break out of outer loop
    }
}
```

## Comparing Loop Types

### Same Task, Different Loops

```rust
// Using loop
let mut i = 0;
loop {
    if i >= 10 {
        break;
    }
    println!("{}", i);
    i += 1;
}

// Using while
let mut i = 0;
while i < 10 {
    println!("{}", i);
    i += 1;
}

// Using for (most idiomatic!)
for i in 0..10 {
    println!("{}", i);
}
```

**Prefer `for` when possible** - it's the most Rust-idiomatic way!

## In Your Proof-of-Work Code

```rust
// Infinite mining loop
loop {
    // Inner conditional loop - keep trying nonces
    while !self.blockchain.try_mine_block(&self.id) {
        // Keep trying
    }
    
    // Delay and continue mining
}
```

This uses:
1. **Outer `loop`** - mine blocks forever
2. **Inner `while`** - keep trying until a block is mined

## Performance Note

All three loop types compile to the same efficient machine code. Choose based on readability:
- **`for`** - when iterating over something (most common)
- **`while`** - when you have a clear condition
- **`loop`** - when you need infinite loop or complex exit logic

## Quick Reference

```rust
// Infinite loop
loop {
    // ...
    if done { break; }
}

// Conditional loop
while condition {
    // ...
}

// Range loop
for i in 0..10 {
    // ...
}

// Collection loop
for item in &collection {
    // ...
}

// Skip iteration
continue;

// Exit loop
break;

// Return value from loop
let x = loop {
    break 42;
};
```

---

**Remember:** Rust's loops are expressions, so `loop` can return values with `break value`! 🦀
