# Transpose

**Transpose** flips a matrix over its diagonal - rows become columns, columns become rows.

## Visual Example

```
Original Matrix:        Transposed:
[1  2  3]              [1  4]
[4  5  6]              [2  5]
                       [3  6]
```

Rows → Columns, Columns → Rows

## Formula

If `A[i][j]` is the original, then `A^T[j][i]` is the transpose.

```
A[0][0] = 1  →  A^T[0][0] = 1
A[0][1] = 2  →  A^T[1][0] = 2
A[0][2] = 3  →  A^T[2][0] = 3
A[1][0] = 4  →  A^T[0][1] = 4
A[1][1] = 5  →  A^T[1][1] = 5
A[1][2] = 6  →  A^T[2][1] = 6
```

## Rust Example

```rust
fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    
    let mut result = vec![vec![0; rows]; cols];
    
    for i in 0..rows {
        for j in 0..cols {
            result[j][i] = matrix[i][j];  // Flip indices
        }
    }
    
    result
}

fn main() {
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];
    
    let transposed = transpose(matrix);
    // Result: [[1, 4], [2, 5], [3, 6]]
    
    for row in transposed {
        println!("{:?}", row);
    }
}
```

## Quick Facts

- **Notation:** `A^T` or `A'`
- **Size change:** `m×n` matrix → `n×m` matrix
- **Diagonal unchanged:** Elements on the main diagonal stay in place
- **Double transpose:** `(A^T)^T = A` (transpose twice = original)

## `Option::transpose()` in Rust

The `transpose()` method also exists on `Option<Result<T, E>>` to flip the nesting!

### What It Does

```rust
// Converts Option<Result<T, E>> into Result<Option<T>, E>
Option<Result<T, E>> → Result<Option<T>, E>
```

**Transformations:**
- `Some(Ok(value))` → `Ok(Some(value))`
- `Some(Err(e))` → `Err(e)`
- `None` → `Ok(None)`

### Why This Is Useful

When you have nested `Option<Result>`, you often want to:
1. Propagate errors with `?`
2. Keep working with `Option` values

Without `transpose()`, you'd need nested matching. With it, you can use `?` operator!

### Simple Example

```rust
fn get_port() -> Result<u16, String> {
    Ok(8080)
}

fn main() -> Result<(), String> {
    let use_custom: Option<bool> = Some(true);
    
    // Without transpose - messy!
    let port = match use_custom {
        Some(true) => match get_port() {
            Ok(p) => Some(p),
            Err(e) => return Err(e),
        },
        _ => None,
    };
    
    // With transpose - clean!
    let port: Option<u16> = use_custom
        .then(|| get_port())  // Option<Result<u16, String>>
        .transpose()?;        // Result<Option<u16>, String> then unwrap with ?
    
    println!("Port: {:?}", port);
    Ok(())
}
```

### Real-World Example: Port Selection with Fallbacks

```rust
use_zero_ports
    .then(unused_port::unused_udp6_port)  // Option<Result<u16, Error>>
    .transpose()?                          // Result<Option<u16>, Error> → Option<u16>
    .or(maybe_disc_port)                   // Fallback to maybe_disc_port
    .unwrap_or(tcp_port)                   // Final fallback to tcp_port
```

**Step-by-step breakdown:**

1. **`.then(unused_port::unused_udp6_port)`**
   - If `use_zero_ports` is `Some(true)`, call `unused_udp6_port()`
   - Result: `Option<Result<u16, Error>>`

2. **`.transpose()?`**
   - Flips nesting: `Option<Result<T, E>>` → `Result<Option<T>, E>`
   - `Some(Ok(port))` → `Ok(Some(port))`
   - `Some(Err(e))` → `Err(e)` (propagated by `?`)
   - `None` → `Ok(None)`
   - After `?`: `Option<u16>`

3. **`.or(maybe_disc_port)`**
   - If previous result is `Some(port)`, use it
   - Otherwise, try `maybe_disc_port`
   - Result: `Option<u16>`

4. **`.unwrap_or(tcp_port)`**
   - If still `None`, use `tcp_port` as final fallback
   - Result: `u16`

**The cascading priority:**
1. Try unused UDP6 port (if enabled)
2. If that fails or wasn't attempted, try `maybe_disc_port`
3. If still no port, fall back to `tcp_port`

All with error handling baked in!

### Key Insight

`transpose()` lets you **flip the nesting** so you can:
- Use `?` to propagate errors early
- Continue working with `Option` values
- Avoid deeply nested `match` statements

```rust
// Before transpose: Can't use ? operator
let opt_result: Option<Result<T, E>> = some_value;

// After transpose: Can use ? operator!
let result_opt: Result<Option<T>, E> = opt_result.transpose();
let opt: Option<T> = result_opt?;  // Propagate error if any
```

## Common Use Cases

### Matrix Transpose
- Linear algebra operations
- Matrix multiplication preparation
- Data transformation (rows ↔ columns)
- Image rotation (90° rotations)

### Option::transpose()
- Error propagation with `?` operator
- Fallback chains with error handling
- Simplifying nested `Option<Result>` patterns
