# Enums vs Structs in Rust

Rust has both **enums** and **structs** because they solve different problems. Here's when to use each.

## The Core Difference

**Struct** = "AND" - A thing that has **all** of these fields
**Enum** = "OR" - A thing that can be **one of** these variants

## Structs: Product Types (AND)

Use a struct when you need **all** the fields together:

```rust
// A user HAS a name AND an email AND an age
struct User {
    name: String,
    email: String,
    age: u32,
}

let user = User {
    name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
    age: 30,
};
// user has ALL three fields at the same time
```

**Think:** "This thing IS MADE UP OF these parts"

## Enums: Sum Types (OR)

Use an enum when you need **one of** several possibilities:

```rust
// A payment can be CreditCard OR PayPal OR Cash (not all at once!)
enum Payment {
    CreditCard { number: String, cvv: String },
    PayPal { email: String },
    Cash { amount: f64 },
}

let payment = Payment::CreditCard {
    number: "1234-5678".to_string(),
    cvv: "123".to_string(),
};
// payment is ONE of the variants, not all of them
```

**Think:** "This thing CAN BE one of these options"

## When to Use Structs

### 1. Modeling Data with Fixed Fields

```rust
// A point always has x AND y
struct Point {
    x: f64,
    y: f64,
}

// A rectangle always has width AND height
struct Rectangle {
    width: f64,
    height: f64,
}

// A person always has a name AND birthdate
struct Person {
    name: String,
    birthdate: String,
}
```

### 2. Grouping Related Data

```rust
// Configuration has all these settings
struct Config {
    host: String,
    port: u16,
    timeout: u64,
    debug: bool,
}
```

### 3. Building Objects/Entities

```rust
// A car has all these properties
struct Car {
    make: String,
    model: String,
    year: u32,
    mileage: u32,
}
```

## When to Use Enums

### 1. Representing Different States

```rust
// A connection is in ONE state at a time
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}
```

### 2. Representing Different Types of Things

```rust
// A message can be ONE of these types
enum Message {
    Text(String),
    Image { url: String, width: u32, height: u32 },
    Video { url: String, duration: u32 },
}
```

### 3. Handling Success or Failure

```rust
// A result is EITHER success OR failure
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 4. Optional Values

```rust
// A value is EITHER present OR absent
enum Option<T> {
    Some(T),
    None,
}
```

### 5. Different Variants with Different Data

```rust
// Different shapes have different properties
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}
```

## Side-by-Side Comparison

### Struct Example: User Profile
```rust
// A user profile HAS all of these
struct UserProfile {
    username: String,
    email: String,
    age: u32,
    is_premium: bool,
}

// Every user has ALL these fields
let user = UserProfile {
    username: "alice".to_string(),
    email: "alice@example.com".to_string(),
    age: 30,
    is_premium: true,
};
```

### Enum Example: User Status
```rust
// A user IS IN one of these states
enum UserStatus {
    Active,
    Suspended { reason: String, until: String },
    Banned { reason: String },
    Deleted,
}

// A user is in ONE status at a time
let status = UserStatus::Suspended {
    reason: "Violation of terms".to_string(),
    until: "2024-12-31".to_string(),
};
```

## Combining Structs and Enums

Often you use them together:

```rust
// Struct for the user data
struct User {
    id: u32,
    name: String,
    email: String,
    status: UserStatus,  // Enum for the state
}

// Enum for the different states
enum UserStatus {
    Active,
    Suspended { reason: String },
    Banned,
}

let user = User {
    id: 1,
    name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
    status: UserStatus::Active,
};
```

## Real-World Examples

### E-commerce System

```rust
// Struct: Product has all these properties
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: ProductCategory,  // Enum
}

// Enum: A product is in ONE category
enum ProductCategory {
    Electronics,
    Clothing,
    Food,
    Books,
}

// Enum: An order is in ONE state
enum OrderStatus {
    Pending,
    Processing,
    Shipped { tracking_number: String },
    Delivered,
    Cancelled { reason: String },
}

// Struct: Order has all these fields
struct Order {
    id: u32,
    product_id: u32,
    quantity: u32,
    status: OrderStatus,  // Enum
}
```

### File System

```rust
// Enum: A file system entry is EITHER a file OR a directory
enum FileSystemEntry {
    File { name: String, size: u64, content: Vec<u8> },
    Directory { name: String, entries: Vec<FileSystemEntry> },
}

// Struct: File metadata has all these properties
struct FileMetadata {
    name: String,
    size: u64,
    created: String,
    modified: String,
}
```

## Decision Tree

Ask yourself:

**"Does this thing have ALL of these properties at the same time?"**
- ✅ YES → Use a **struct**
- ❌ NO → Use an **enum**

**"Can this thing be in different states or forms?"**
- ✅ YES → Use an **enum**
- ❌ NO → Use a **struct**

**"Do I need to represent multiple possibilities?"**
- ✅ YES → Use an **enum**
- ❌ NO → Use a **struct**

## Common Patterns

### Pattern 1: Struct with Enum Field
```rust
struct Task {
    id: u32,
    title: String,
    status: TaskStatus,  // Enum for state
}

enum TaskStatus {
    Todo,
    InProgress,
    Done,
}
```

### Pattern 2: Enum with Struct Variants
```rust
enum Event {
    UserCreated(User),      // Struct as data
    UserUpdated(User),      // Struct as data
    UserDeleted { id: u32 },
}

struct User {
    id: u32,
    name: String,
}
```

### Pattern 3: Nested Enums and Structs
```rust
struct Response {
    status_code: u16,
    body: ResponseBody,  // Enum
}

enum ResponseBody {
    Json(String),
    Html(String),
    Binary(Vec<u8>),
    Empty,
}
```

## Key Differences Summary

| Aspect | Struct | Enum |
|--------|--------|------|
| Purpose | Group related data | Represent alternatives |
| Fields | All fields present | One variant at a time |
| Think | "AND" (has all) | "OR" (is one of) |
| Example | `User { name, email, age }` | `Status::Active` or `Status::Banned` |
| Pattern Matching | Access fields directly | Must match on variant |
| Memory | Size of all fields | Size of largest variant |

## Quick Examples

```rust
// ✅ Struct: A book HAS all these
struct Book {
    title: String,
    author: String,
    pages: u32,
}

// ✅ Enum: A book format IS one of these
enum BookFormat {
    Hardcover,
    Paperback,
    Ebook,
    Audiobook,
}

// ✅ Together:
struct Book {
    title: String,
    author: String,
    pages: u32,
    format: BookFormat,  // One format at a time
}
```

## Summary

- **Structs** = "This thing **has** all of these properties"
- **Enums** = "This thing **is** one of these options"

Both are essential! Structs model entities with fixed properties, while enums model choices, states, and alternatives. Together, they give you powerful tools to model your domain accurately and safely.
