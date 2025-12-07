# Rust Variant Constructors

Variant constructors are special functions that create instances of enum variants. They look like function calls but are actually creating enum values.

## What is a Variant Constructor?

When you define an enum, Rust automatically creates a **constructor function** for each variant:

```rust
enum Option<T> {
    Some(T),    // Creates a constructor: Some(value) -> Option<T>
    None,       // Creates a constructor: None -> Option<T>
}

// Using the constructors:
let x = Some(5);    // Calls the Some constructor
let y = None;       // Uses the None constructor
```

Think of variant constructors as **built-in factory functions** that create enum instances.

## How Variant Constructors Work

### Unit Variants (No Data)

```rust
enum Status {
    Active,
    Inactive,
    Pending,
}

// These are constructors with no parameters
let s1 = Status::Active;
let s2 = Status::Inactive;
let s3 = Status::Pending;
```

### Tuple Variants (Unnamed Fields)

```rust
enum Message {
    Quit,                       // Unit variant
    Move(i32, i32),            // Tuple variant with 2 fields
    Write(String),             // Tuple variant with 1 field
    ChangeColor(u8, u8, u8),   // Tuple variant with 3 fields
}

// Constructors that take parameters:
let msg1 = Message::Quit;
let msg2 = Message::Move(10, 20);
let msg3 = Message::Write(String::from("hello"));
let msg4 = Message::ChangeColor(255, 0, 0);
```

### Struct Variants (Named Fields)

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Point { x: i32, y: i32 },
}

// Constructors with named fields:
let shape1 = Shape::Circle { radius: 5.0 };
let shape2 = Shape::Rectangle { width: 10.0, height: 20.0 };
let shape3 = Shape::Point { x: 0, y: 0 };
```

## Variant Constructors are Functions

This is the key insight: **variant constructors ARE functions**!

```rust
enum Option<T> {
    Some(T),
    None,
}

// Some is a function: fn Some<T>(T) -> Option<T>
// You can use it anywhere a function is expected:

let numbers = vec![1, 2, 3];

// Using Some as a function in map
let options: Vec<Option<i32>> = numbers.into_iter().map(Some).collect();
// Same as: .map(|n| Some(n))

// Result: [Some(1), Some(2), Some(3)]
```

### More Examples of Constructors as Functions

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Ok is a function: fn Ok<T, E>(T) -> Result<T, E>
let results: Vec<Result<i32, String>> = vec![1, 2, 3]
    .into_iter()
    .map(Ok)  // Using Ok as a function!
    .collect();

// Result: [Ok(1), Ok(2), Ok(3)]
```

## Constructor Signatures

Each variant has a specific function signature:

```rust
enum Message {
    Quit,                    // fn Quit() -> Message
    Move(i32, i32),         // fn Move(i32, i32) -> Message
    Write(String),          // fn Write(String) -> Message
}

// These are equivalent:
let m1 = Message::Move(10, 20);
let constructor = Message::Move;  // Store the constructor
let m2 = constructor(10, 20);     // Call it later
```

## Constructors vs Methods

It's important to distinguish between constructors and methods:

```rust
enum Option<T> {
    Some(T),
    None,
}

impl<T> Option<T> {
    // This is a METHOD (called on an instance)
    fn is_some(&self) -> bool {
        match self {
            Some(_) => true,
            None => false,
        }
    }
    
    // This is an ASSOCIATED FUNCTION (like a static method)
    fn from_value(value: T) -> Option<T> {
        Some(value)
    }
}

// Using constructors (create the enum):
let x = Some(5);           // Variant constructor
let y = None;              // Variant constructor

// Using methods (work with the enum):
let check = x.is_some();   // Method call

// Using associated functions:
let z = Option::from_value(10);  // Associated function
```

## Pattern Matching with Constructors

The same constructors used to **create** values are used to **destructure** them:

```rust
enum Message {
    Move(i32, i32),
    Write(String),
}

// Creating with constructor
let msg = Message::Move(10, 20);

// Destructuring with the same pattern
match msg {
    Message::Move(x, y) => println!("Move to ({}, {})", x, y),
    Message::Write(text) => println!("Write: {}", text),
}
```

## Generic Variant Constructors

Constructors work with generic types:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// The type parameters are inferred from usage:
let success: Result<i32, String> = Ok(42);
let failure: Result<i32, String> = Err("error".to_string());

// Or explicitly specified:
let success = Result::<i32, String>::Ok(42);
```

## Common Patterns

### 1. Using Constructors in Iterators

```rust
let numbers = vec![1, 2, 3];

// Wrap all in Some
let options: Vec<Option<i32>> = numbers.into_iter().map(Some).collect();

// Convert to Results
let results: Vec<Result<i32, ()>> = vec![1, 2, 3]
    .into_iter()
    .map(Ok)
    .collect();
```

### 2. Storing Constructors in Variables

```rust
enum Message {
    Text(String),
    Number(i32),
}

// Store the constructor
let make_text = Message::Text;
let make_number = Message::Number;

// Use them later
let msg1 = make_text(String::from("hello"));
let msg2 = make_number(42);
```

### 3. Passing Constructors as Arguments

```rust
fn apply_constructor<T, F>(value: T, constructor: F) -> Option<T>
where
    F: Fn(T) -> Option<T>
{
    constructor(value)
}

let result = apply_constructor(5, Some);  // Some(5)
```

## Real-World Example

```rust
enum HttpResponse {
    Ok { status: u16, body: String },
    Error { status: u16, message: String },
    Redirect { location: String },
}

// Using constructors to create responses
fn handle_request(path: &str) -> HttpResponse {
    match path {
        "/" => HttpResponse::Ok {
            status: 200,
            body: "Welcome!".to_string(),
        },
        "/redirect" => HttpResponse::Redirect {
            location: "/home".to_string(),
        },
        _ => HttpResponse::Error {
            status: 404,
            message: "Not found".to_string(),
        },
    }
}
```

## Constructor Type Signatures

Understanding the actual types:

```rust
enum Option<T> {
    Some(T),
    None,
}

// Some has type: fn(T) -> Option<T>
// None has type: Option<T> (it's already a value, not a function)

let f: fn(i32) -> Option<i32> = Some;  // Some is a function
let x: Option<i32> = None;              // None is a value
```

## Variant Namespacing and Name Conflicts

### Variants are Namespaced to Their Enum

Each enum's variants are **namespaced** to that enum, so you can have multiple enums with the same variant names:

```rust
// Standard library Option
enum Option<T> {
    Some(T),
    None,
}

// Your custom enum - no conflict!
enum MyEnum<T> {
    Some(T, T),  // ✅ Different from Option::Some
    None,        // ✅ Different from Option::None
}
```

### Full Qualified Names

The full name of a variant is `EnumName::VariantName`:

```rust
// These are completely different constructors:
Option::Some(5)        // Option's Some (1 field)
MyEnum::Some(5, 10)    // MyEnum's Some (2 fields)

Option::None           // Option's None
MyEnum::None           // MyEnum's None
```

### Type Inference Helps

Rust uses type inference to figure out which constructor you're using:

```rust
// Rust infers from the type annotation:
let x: Option<i32> = Some(5);        // Uses Option::Some
let y: MyEnum<i32> = Some(5, 10);    // Uses MyEnum::Some

// Or from the number of arguments:
let z = Some(5);        // Must be Option::Some (1 arg)
let w = Some(5, 10);    // Must be MyEnum::Some (2 args)
```

### Wildcard Import Conflicts

The **only** time you get a conflict is with wildcard imports:

```rust
use Option::*;   // Brings Option's Some and None into scope
use MyEnum::*;   // Also brings MyEnum's Some and None into scope

// ❌ Compile error: ambiguous!
let x = Some(5);  // Which Some? Option::Some or MyEnum::Some?
```

**Error message:**
```
error[E0659]: `Some` is ambiguous
   |
   |     let x = Some(5);
   |             ^^^^ ambiguous name
   |
   = note: ambiguous because of multiple glob imports
```

### How to Avoid Conflicts

**Option 1: Use fully qualified names**
```rust
use Option::*;
use MyEnum::*;

let x = Option::Some(5);      // ✅ Explicit
let y = MyEnum::Some(5, 10);  // ✅ Explicit
```

**Option 2: Only wildcard import one**
```rust
use Option::*;  // Wildcard for Option only

let x = Some(5);              // ✅ Uses Option::Some
let y = MyEnum::Some(5, 10);  // ✅ Explicit MyEnum
```

**Option 3: Rename on import**
```rust
use Option::{Some as OptionSome, None as OptionNone};
use MyEnum::{Some as MyEnumSome, None as MyEnumNone};

let x = OptionSome(5);
let y = MyEnumSome(5, 10);
```

**Option 4: Don't use wildcard imports**
```rust
// No wildcards - always be explicit
let x = Option::Some(5);
let y = MyEnum::Some(5, 10);
```

### Key Point: `Some(T)` is Just a Name

There's nothing special about the name "Some" - it's just what the standard library chose:

```rust
// These are functionally identical:
enum Option<T> {
    Some(T),
    None,
}

enum Option<T> {
    HasValue(T),    // Could be named anything!
    NoValue,
}

enum Option<T> {
    Banana(T),      // Even this works!
    Apple,
}
```

The name "Some" is just a convention meaning "there's some value here."

## Key Takeaways

1. **Variant constructors are automatically generated** when you define an enum
2. **They are functions** that create instances of that variant
3. **They can be used anywhere functions are expected** (map, filter, etc.)
4. **They have specific type signatures** based on the variant's fields
5. **The same syntax creates and destructures** enum values
6. **Variants are namespaced** to their enum - no conflicts unless you wildcard import
7. **`Some(T)` is just a tuple variant** - nothing magical about the name

## Quick Reference

| Variant Type | Definition | Constructor Call |
|--------------|------------|------------------|
| Unit | `Quit` | `Message::Quit` |
| Tuple | `Move(i32, i32)` | `Message::Move(10, 20)` |
| Struct | `Point { x: i32, y: i32 }` | `Shape::Point { x: 0, y: 0 }` |

## Summary

Variant constructors are **special functions** automatically created for each enum variant. They:
- Create instances of the enum
- Can be passed around like any other function
- Have type signatures based on the variant's fields
- Are used in both creation and pattern matching

Understanding that `Some`, `None`, `Ok`, and `Err` are just variant constructors helps demystify how Rust's type system works!
