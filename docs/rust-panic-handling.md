# Rust Panic Handling: Best Practices for Production Code

Panics in Rust are unrecoverable errors that cause a program to crash. In production systems, especially those operating in adversarial environments (like the Internet), panics are severe vulnerabilities that must be avoided.

## What is a Panic?

A panic is Rust's way of saying "something went catastrophically wrong, and I can't continue." When a panic occurs:

1. The program prints an error message
2. The stack unwinds (or aborts)
3. The program terminates

```rust
fn main() {
    panic!("This crashes the program!");  // Program ends here
    println!("This never runs");
}
```

## Why Panics are Dangerous

### In Adversarial Environments

If your application is exposed to the Internet or untrusted input:

- **Denial of Service (DoS)**: Attackers can crash your service by triggering panics
- **Data Loss**: Panics can interrupt critical operations
- **Cascading Failures**: One panic can bring down entire systems
- **Security Vulnerability**: Predictable crashes can be exploited

### Example: Vulnerable Code

```rust
// ❌ DANGEROUS: Attacker can crash the server
fn process_request(data: &[u8]) {
    let first_byte = data[0];  // Panics if data is empty!
    // Process first_byte...
}
```

An attacker sends an empty request → panic → server crashes.

## The Golden Rule

**Always prefer `Result` or `Option` over panicking.**

Instead of crashing, return an error that can be handled gracefully.

## Safe Alternatives to Common Panic Sources

### 1. Array/Vector Indexing

#### ❌ Dangerous: Direct Indexing

```rust
let numbers = vec![1, 2, 3];
let value = numbers[10];  // PANIC: index out of bounds
```

#### ✅ Safe: Use `.get()`

```rust
let numbers = vec![1, 2, 3];

// Returns Option<&T>
match numbers.get(10) {
    Some(value) => println!("Value: {}", value),
    None => println!("Index out of bounds"),
}

// Or with the ? operator
fn get_value(numbers: &[i32], index: usize) -> Option<i32> {
    Some(*numbers.get(index)?)
}
```

#### ✅ Safe: Use `.get()` with `?`

```rust
fn process_array(data: &[u8]) -> Result<u8, String> {
    let first = data.get(0)
        .ok_or("Array is empty")?;
    Ok(*first)
}
```

### 2. Unwrapping Results and Options

#### ❌ Dangerous: `.unwrap()`

```rust
let file = File::open("config.txt").unwrap();  // PANIC if file doesn't exist
```

#### ✅ Safe: Propagate Errors with `?`

```rust
fn read_config() -> Result<String, std::io::Error> {
    let mut file = File::open("config.txt")?;  // Return error instead of panic
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

#### ✅ Safe: Handle Errors Explicitly

```rust
match File::open("config.txt") {
    Ok(file) => {
        // Process file
    }
    Err(e) => {
        eprintln!("Failed to open config: {}", e);
        // Use default config or return error
    }
}
```

### 3. Division by Zero

#### ❌ Dangerous: Direct Division

```rust
fn calculate(a: i32, b: i32) -> i32 {
    a / b  // PANIC if b is 0
}
```

#### ✅ Safe: Check Before Dividing

```rust
fn calculate(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Division by zero".to_string());
    }
    Ok(a / b)
}
```

#### ✅ Safe: Use `checked_div()`

```rust
fn calculate(a: i32, b: i32) -> Option<i32> {
    a.checked_div(b)  // Returns None if b is 0
}
```

### 4. String/Slice Operations

#### ❌ Dangerous: Unwrapping

```rust
let text = "hello";
let first_char = text.chars().next().unwrap();  // PANIC if empty
```

#### ✅ Safe: Handle None Case

```rust
let text = "hello";
let first_char = text.chars().next()
    .ok_or("String is empty")?;
```

### 5. Parsing

#### ❌ Dangerous: `.unwrap()`

```rust
let number: i32 = "42".parse().unwrap();  // PANIC if parse fails
```

#### ✅ Safe: Handle Parse Errors

```rust
fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

// Or with error context
fn parse_number_with_context(s: &str) -> Result<i32, String> {
    s.parse()
        .map_err(|e| format!("Failed to parse '{}': {}", s, e))
}
```

## When `.expect()` is Acceptable

If you have a **provable invariant** that guarantees no panic, use `.expect()` with a detailed message.

### ✅ Acceptable: With Clear Reasoning

```rust
fn process_data(data: &[u8]) -> Result<u8, String> {
    // We just checked the length, so this can't panic
    if data.len() < 10 {
        return Err("Data too short".to_string());
    }
    
    // SAFETY: We verified data.len() >= 10 above, so index 5 is valid
    let value = data.get(5)
        .expect("Index 5 is valid because we checked length >= 10");
    
    Ok(*value)
}
```

### ✅ Acceptable: Initialization Invariants

```rust
use std::sync::Mutex;

lazy_static! {
    // This can only fail if the system is out of memory,
    // which is a fatal error anyway
    static ref CACHE: Mutex<HashMap<String, String>> = 
        Mutex::new(HashMap::new());
}

fn get_cache() -> &'static Mutex<HashMap<String, String>> {
    &CACHE
}
```

### Key Points for `.expect()`:
1. **Always include a helpful message** explaining why it won't panic
2. **Document the invariant** in a comment
3. **Use sparingly** - most cases should use `?` instead

## Comparison: `.unwrap()` vs `.expect()` vs `?`

```rust
// ❌ NEVER: No context when it panics
let value = some_option.unwrap();

// ⚠️ RARELY: Only when you have provable invariants
let value = some_option.expect("This is safe because [detailed reason]");

// ✅ PREFERRED: Propagate the error
let value = some_option.ok_or("Helpful error message")?;
```

## Handling Untrusted Input

When dealing with data from the Internet or users:

### ✅ Validate Everything

```rust
fn handle_request(data: &[u8]) -> Result<Response, Error> {
    // Validate length
    if data.is_empty() {
        return Err(Error::EmptyRequest);
    }
    
    if data.len() > MAX_REQUEST_SIZE {
        return Err(Error::RequestTooLarge);
    }
    
    // Validate format
    let header = data.get(0..4)
        .ok_or(Error::InvalidHeader)?;
    
    // Safe to process now
    process_validated_data(data)
}
```

### ✅ Use Bounds Checking

```rust
fn parse_packet(data: &[u8]) -> Result<Packet, ParseError> {
    // Always use .get() for untrusted indices
    let version = data.get(0)
        .ok_or(ParseError::TooShort)?;
    
    let length = data.get(1)
        .ok_or(ParseError::TooShort)?;
    
    let payload = data.get(2..*length as usize)
        .ok_or(ParseError::InvalidLength)?;
    
    Ok(Packet {
        version: *version,
        payload: payload.to_vec(),
    })
}
```

### ✅ Set Limits

```rust
const MAX_ITERATIONS: usize = 1000;

fn process_loop(data: &[u8]) -> Result<Vec<u8>, Error> {
    let mut result = Vec::new();
    let mut iterations = 0;
    
    for byte in data {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            return Err(Error::TooManyIterations);
        }
        
        result.push(process_byte(*byte)?);
    }
    
    Ok(result)
}
```

## Common Panic-Causing Operations

### Operations That Can Panic

| Operation | Panic Condition | Safe Alternative |
|-----------|----------------|------------------|
| `vec[i]` | Index out of bounds | `vec.get(i)?` |
| `array[i]` | Index out of bounds | `array.get(i)?` |
| `.unwrap()` | `None` or `Err` | `?` or `match` |
| `a / b` | `b == 0` | `a.checked_div(b)?` |
| `a % b` | `b == 0` | `a.checked_rem(b)?` |
| `.first().unwrap()` | Empty collection | `.first().ok_or(...)?` |
| `.last().unwrap()` | Empty collection | `.last().ok_or(...)?` |
| `String::from_utf8(v).unwrap()` | Invalid UTF-8 | `String::from_utf8(v)?` |

### Safe Checked Operations

Rust provides checked versions of many operations:

```rust
// Arithmetic
let result = a.checked_add(b)?;      // Returns None on overflow
let result = a.checked_sub(b)?;      // Returns None on underflow
let result = a.checked_mul(b)?;      // Returns None on overflow
let result = a.checked_div(b)?;      // Returns None if b == 0
let result = a.checked_rem(b)?;      // Returns None if b == 0

// Conversion
let small: u8 = large.try_into()?;   // Returns Err if out of range
```

## Error Handling Patterns

### Pattern 1: Early Returns

```rust
fn validate_and_process(data: &[u8]) -> Result<Output, Error> {
    // Validate early, return errors immediately
    if data.is_empty() {
        return Err(Error::Empty);
    }
    
    let header = data.get(0..4)
        .ok_or(Error::InvalidHeader)?;
    
    let body = data.get(4..)
        .ok_or(Error::NoBody)?;
    
    // Now safe to process
    Ok(process(header, body))
}
```

### Pattern 2: Custom Error Types

```rust
#[derive(Debug)]
enum ParseError {
    TooShort,
    InvalidFormat,
    OutOfBounds { index: usize, len: usize },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::TooShort => write!(f, "Data is too short"),
            ParseError::InvalidFormat => write!(f, "Invalid format"),
            ParseError::OutOfBounds { index, len } => 
                write!(f, "Index {} out of bounds (len: {})", index, len),
        }
    }
}

impl std::error::Error for ParseError {}
```

### Pattern 3: Fallible Iterators

```rust
fn process_all(items: &[Item]) -> Result<Vec<Output>, Error> {
    items.iter()
        .map(|item| process_item(item))  // Returns Result
        .collect()  // Collects into Result<Vec<_>, _>
}
```

## Testing for Panics

When you must test code that might panic:

```rust
#[test]
#[should_panic(expected = "index out of bounds")]
fn test_panic_behavior() {
    let v = vec![1, 2, 3];
    let _ = v[10];  // This should panic
}

// Better: Test that safe code doesn't panic
#[test]
fn test_safe_access() {
    let v = vec![1, 2, 3];
    assert_eq!(v.get(10), None);  // Returns None, doesn't panic
}
```

## Panic Recovery (Advanced)

In rare cases, you might need to catch panics:

```rust
use std::panic;

fn risky_operation() -> Result<String, Box<dyn std::error::Error>> {
    let result = panic::catch_unwind(|| {
        // Code that might panic
        dangerous_function()
    });
    
    match result {
        Ok(value) => Ok(value),
        Err(_) => Err("Operation panicked".into()),
    }
}
```

**Warning**: This should be a last resort. It's better to fix the panic source.

## Best Practices Summary

1. **Never use `.unwrap()` in production code**
   - Use `?` to propagate errors
   - Use `match` or `if let` to handle errors explicitly

2. **Use `.expect()` only with detailed reasoning**
   - Include a message explaining why it's safe
   - Document the invariant in a comment

3. **Prefer `.get()` over direct indexing**
   - `array.get(i)?` instead of `array[i]`
   - Especially for untrusted input

4. **Use checked arithmetic**
   - `.checked_add()`, `.checked_div()`, etc.
   - Prevents overflow/underflow panics

5. **Validate all untrusted input**
   - Check lengths, bounds, formats
   - Set reasonable limits

6. **Return `Result` or `Option`**
   - Let callers decide how to handle errors
   - Enables graceful degradation

7. **Write defensive code**
   - Assume inputs can be malicious
   - Validate everything from external sources

8. **Test error paths**
   - Ensure errors are handled correctly
   - Use fuzzing for untrusted input

## Quick Reference: Panic-Free Checklist

Before deploying code, verify:

- [ ] No `.unwrap()` calls (except in tests)
- [ ] All `.expect()` calls have detailed justification
- [ ] No direct array/vector indexing with untrusted indices
- [ ] Division operations check for zero
- [ ] Arithmetic operations use checked variants where overflow is possible
- [ ] All external input is validated
- [ ] Error types are descriptive
- [ ] Errors propagate with `?` operator
- [ ] Tests cover error cases

## Conclusion

In production systems, especially those exposed to adversarial environments:

- **Panics are vulnerabilities** - They enable DoS attacks
- **Always return errors** - Use `Result` and `Option`
- **Validate untrusted input** - Never trust data from the Internet
- **Use safe alternatives** - `.get()`, checked operations, `?` operator
- **Document invariants** - If using `.expect()`, explain why it's safe

Remember: A panic in production is a bug. Design your code to handle errors gracefully, and your systems will be more robust, secure, and reliable.
