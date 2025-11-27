# Rust Multithreading - Simple Guide

Multithreading lets your program do multiple things at the same time by running code on different CPU cores.

## Why Multithreading?

- **Parallel execution** - Do multiple tasks simultaneously
- **Better performance** - Use all CPU cores
- **Responsiveness** - Keep UI responsive while doing work

## Basic Thread Creation

### Spawning a Thread

```rust
use std::thread;

fn main() {
    // Spawn a new thread
    thread::spawn(|| {
        println!("Hello from a thread!");
    });
    
    println!("Hello from main!");
}
```

**Problem:** The spawned thread might not finish before the program exits!

### Waiting for Threads to Finish

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });
    
    // Wait for the thread to finish
    handle.join().unwrap();
    
    println!("Thread finished!");
}
```

## Example: Multiple Mining Nodes

Here's how you might run multiple mining nodes in parallel:

```rust
use std::thread;

fn main() {
    let mut handles = vec![];
    
    // Spawn 3 mining nodes
    for i in 0..3 {
        let handle = thread::spawn(move || {
            println!("Node {} started mining", i);
            // Mining logic here
            loop {
                // Mine blocks...
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads (they run forever in this case)
    for handle in handles {
        handle.join().unwrap();
    }
}
```

## Moving Data Into Threads

Use `move` to transfer ownership into the thread:

```rust
let data = vec![1, 2, 3];

thread::spawn(move || {
    // data is now owned by this thread
    println!("{:?}", data);
});

// data is no longer accessible here
```

## Sharing Data Between Threads

### Problem: Can't Share Mutable Data Directly

```rust
let mut counter = 0;

thread::spawn(|| {
    counter += 1;  // ❌ ERROR: can't capture mutable reference
});
```

### Solution 1: Arc + Mutex (Shared Mutable State)

`Arc` = Atomic Reference Counted (thread-safe shared ownership)  
`Mutex` = Mutual Exclusion (only one thread can access at a time)

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Wrap data in Arc<Mutex<T>>
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);  // Clone the Arc (cheap)
        
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();  // Lock the mutex
            *num += 1;  // Modify the data
            // Lock is automatically released when num goes out of scope
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());  // Result: 10
}
```

### Solution 2: Channels (Message Passing)

Send data between threads using channels:

```rust
use std::sync::mpsc;  // Multiple Producer, Single Consumer
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    // Spawn thread that sends data
    thread::spawn(move || {
        tx.send("Hello from thread!").unwrap();
    });
    
    // Receive data in main thread
    let message = rx.recv().unwrap();
    println!("{}", message);
}
```

## Real-World Example: Parallel Block Mining

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn mine_blocks_parallel(num_miners: usize) {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let mut handles = vec![];
    
    for miner_id in 0..num_miners {
        let blockchain = Arc::clone(&blockchain);
        
        let handle = thread::spawn(move || {
            loop {
                // Lock blockchain to try mining
                let mut chain = blockchain.lock().unwrap();
                
                if chain.try_mine_block(&format!("miner-{}", miner_id)) {
                    println!("Miner {} mined a block!", miner_id);
                }
                
                // Release lock (drop happens here)
                drop(chain);
                
                // Small delay to let other miners try
                thread::sleep(Duration::from_millis(10));
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

## Thread Sleep

Pause a thread for a duration:

```rust
use std::thread;
use std::time::Duration;

thread::sleep(Duration::from_secs(1));      // Sleep 1 second
thread::sleep(Duration::from_millis(500));  // Sleep 500 milliseconds
```

### Example from Your Code

```rust
pub fn start_mining(&mut self) {
    loop {
        let start_time = Instant::now();
        
        // Mine a block...
        
        let elapsed = start_time.elapsed();
        let delay = Duration::from_secs(self.blockchain.config.delay_seconds);
        
        // Sleep to maintain consistent block time
        if elapsed < delay {
            thread::sleep(delay - elapsed);
        }
    }
}
```

## Common Patterns

### Pattern 1: Worker Pool
```rust
// Spawn N worker threads
let mut workers = vec![];
for i in 0..num_workers {
    let handle = thread::spawn(move || {
        // Worker logic
    });
    workers.push(handle);
}

// Wait for all workers
for worker in workers {
    worker.join().unwrap();
}
```

### Pattern 2: Producer-Consumer
```rust
let (tx, rx) = mpsc::channel();

// Producer thread
thread::spawn(move || {
    for i in 0..10 {
        tx.send(i).unwrap();
    }
});

// Consumer thread
thread::spawn(move || {
    for received in rx {
        println!("Got: {}", received);
    }
});
```

## Key Types for Multithreading

| Type | Purpose | Use Case |
|------|---------|----------|
| `thread::spawn()` | Create a new thread | Run code in parallel |
| `JoinHandle` | Handle to a thread | Wait for thread to finish |
| `Arc<T>` | Shared ownership | Share data across threads |
| `Mutex<T>` | Mutual exclusion | Protect mutable data |
| `RwLock<T>` | Read-write lock | Multiple readers, one writer |
| `mpsc::channel()` | Message passing | Send data between threads |

## Thread Safety Rules

Rust enforces thread safety at compile time:

✅ **Can send between threads:**
- Types that implement `Send` trait
- Most types are `Send`

✅ **Can share between threads:**
- Types that implement `Sync` trait
- Immutable references are `Sync`
- `Arc<Mutex<T>>` is `Sync`

❌ **Cannot share:**
- `Rc<T>` (not thread-safe, use `Arc<T>` instead)
- Raw mutable references without synchronization

## Quick Reference

```rust
use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;

// Spawn a thread
let handle = thread::spawn(|| {
    // Thread code
});

// Wait for thread
handle.join().unwrap();

// Share mutable data
let data = Arc::new(Mutex::new(0));
let data_clone = Arc::clone(&data);

// Lock and modify
let mut value = data.lock().unwrap();
*value += 1;

// Send messages
let (tx, rx) = mpsc::channel();
tx.send(42).unwrap();
let msg = rx.recv().unwrap();

// Sleep
thread::sleep(Duration::from_secs(1));
```

## Common Pitfalls

### 1. Forgetting `move`
```rust
let data = vec![1, 2, 3];
thread::spawn(|| {
    println!("{:?}", data);  // ❌ ERROR: need move
});
```

**Fix:** Add `move`
```rust
thread::spawn(move || {
    println!("{:?}", data);  // ✅ OK
});
```

### 2. Deadlock
```rust
let data = Arc::new(Mutex::new(0));

// Thread 1 locks and never releases
let lock1 = data.lock().unwrap();
// Thread 2 tries to lock - DEADLOCK!
let lock2 = data.lock().unwrap();
```

**Fix:** Release locks quickly, use `drop(lock)` explicitly

### 3. Using `Rc` Instead of `Arc`
```rust
let data = Rc::new(0);  // ❌ Not thread-safe
```

**Fix:** Use `Arc` for thread-safe reference counting
```rust
let data = Arc::new(0);  // ✅ Thread-safe
```

---

**Remember:** Rust's ownership system prevents data races at compile time. If it compiles, it's thread-safe! 🦀
