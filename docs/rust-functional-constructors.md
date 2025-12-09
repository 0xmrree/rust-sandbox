# Rust Functional Constructors

## Overview

One of Rust's most elegant features is that **enum variant constructors are functions**. This means you can pass them around, store them in variables, and use them anywhere a function is expected.

## The Core Concept

When you define an enum variant, Rust automatically creates a constructor function:

```rust
enum Option<T> {
    Some(T),  // Creates: fn Some<T>(T) -> Option<T>
    None,     // Creates: fn None<T>() -> Option<T>
}
```

## Practical Examples

### 1. Using Constructors with Iterator Methods

Instead of writing verbose closures, use the constructor directly:

```rust
// ❌ Verbose way
let numbers = vec![1, 2, 3];
let options: Vec<Option<i32>> = numbers
    .into_iter()
    .map(|n| Some(n))
    .collect();

// ✅ Clean way
let options: Vec<Option<i32>> = numbers
    .into_iter()
    .map(Some)  // Some is a function!
    .collect();
```

### 2. Converting Between Types

```rust
// Convert Result to Option
let results: Vec<Result<i32, _>> = vec![Ok(1), Err("error"), Ok(3)];
let options: Vec<Option<i32>> = results
    .into_iter()
    .filter_map(Result::ok)  // ok() is also a function!
    .map(Some)
    .collect();
```

### 3. Storing Constructors in Variables

```rust
enum Message {
    Text(String),
    Number(i32),
    Flag(bool),
}

// Store constructor in a variable
let constructor = Message::Text;
let msg = constructor(String::from("Hello"));

// Use in conditional logic
let make_message = if use_text {
    Message::Text
} else {
    Message::Number
};
```

### 4. Passing Constructors to Functions

```rust
fn transform<T, U, F>(items: Vec<T>, wrapper: F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    items.into_iter().map(wrapper).collect()
}

// Use with different constructors
let numbers = vec![1, 2, 3];
let options = transform(numbers.clone(), Some);  // Vec<Option<i32>>
let results = transform(numbers, Ok::<_, String>);  // Vec<Result<i32, String>>
```

## Real-World Use Cases

### Parsing with Error Handling

```rust
fn parse_numbers(strings: Vec<&str>) -> Vec<Option<i32>> {
    strings
        .iter()
        .map(|s| s.parse().ok())  // Result -> Option
        .collect()
}

let inputs = vec!["1", "2", "invalid", "4"];
let parsed = parse_numbers(inputs);
// [Some(1), Some(2), None, Some(4)]
```

### Building Data Structures

```rust
#[derive(Debug)]
enum Node {
    Leaf(i32),
    Branch(Box<Node>, Box<Node>),
}

// Create leaves using constructor
let leaves: Vec<Node> = vec![1, 2, 3, 4]
    .into_iter()
    .map(Node::Leaf)
    .collect();
```

### Event Systems

```rust
enum Event {
    Click(i32, i32),
    KeyPress(char),
    Scroll(i32),
}

// Factory function that returns different constructors
fn get_event_constructor(event_type: &str) -> Box<dyn Fn(i32) -> Event> {
    match event_type {
        "scroll" => Box::new(Event::Scroll),
        _ => panic!("Unknown event type"),
    }
}
```

## Performance Benefits

Using constructors directly instead of closures can lead to:

1. **Better optimization** - Compiler can inline more aggressively
2. **Less code generation** - No closure overhead
3. **Clearer intent** - More readable and idiomatic

## Common Patterns

### Wrapping Collections

```rust
// Wrap all items in Some
let items = vec![1, 2, 3];
let wrapped: Vec<Option<i32>> = items.into_iter().map(Some).collect();

// Wrap all items in Ok
let results: Vec<Result<i32, String>> = items
    .into_iter()
    .map(Ok)
    .collect();
```

### Filtering and Wrapping

```rust
let numbers = vec![1, 2, 3, 4, 5];
let evens: Vec<Option<i32>> = numbers
    .into_iter()
    .filter(|n| n % 2 == 0)
    .map(Some)
    .collect();
```

### Chaining Transformations

```rust
let strings = vec!["1", "2", "3"];
let doubled: Vec<Option<i32>> = strings
    .iter()
    .filter_map(|s| s.parse().ok())  // Parse to Option<i32>
    .map(|n| n * 2)                   // Double the value
    .map(Some)                        // Wrap in Some again
    .collect();
```

## Key Takeaways

1. ✅ Enum variant constructors are first-class functions
2. ✅ Use them directly instead of writing `|x| Some(x)`
3. ✅ They can be stored in variables and passed around
4. ✅ This leads to cleaner, more functional code
5. ✅ Better performance through compiler optimizations

## Try It Yourself

```rust
fn main() {
    // Example 1: Map with Some
    let nums = vec![1, 2, 3];
    let opts: Vec<Option<i32>> = nums.into_iter().map(Some).collect();
    println!("{:?}", opts);
    
    // Example 2: Store constructor
    let constructor = Some;
    let value = constructor(42);
    println!("{:?}", value);
    
    // Example 3: Custom enum
    enum Status {
        Active(String),
        Inactive,
    }
    
    let names = vec!["Alice", "Bob", "Charlie"];
    let statuses: Vec<Status> = names
        .into_iter()
        .map(String::from)
        .map(Status::Active)  // Constructor as function!
        .collect();
}
```

## Related Concepts

- **Higher-order functions** - Functions that take or return functions
- **Functional programming** - Using functions as values
- **Method references** - Similar to Java's `::` operator
- **Closure elision** - When you can skip writing closures

---

**Pro Tip**: Whenever you write `|x| SomeVariant(x)`, ask yourself: "Can I just use `SomeVariant` directly?" The answer is usually yes!
