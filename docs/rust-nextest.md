# Cargo Nextest: A Better Test Runner for Rust

Nextest is a next-generation test runner for Rust that's faster and more feature-rich than `cargo test`.

## What is Nextest?

Nextest is a drop-in replacement for `cargo test` that provides:
- **Faster test execution** - Runs tests in parallel more efficiently
- **Better output** - Cleaner, more readable test results
- **Advanced features** - Retries, test groups, JUnit output, and more

## Installation

```bash
cargo install cargo-nextest --locked
```

Or using Homebrew (macOS/Linux):
```bash
brew install cargo-nextest
```

## Basic Usage

### Run All Tests

```bash
cargo nextest run
```

This replaces `cargo test`.

### Run Specific Tests

```bash
# Run tests matching a pattern
cargo nextest run test_name

# Run tests in a specific module
cargo nextest run module::

# Run tests in a specific file
cargo nextest run --test integration_test
```

### Run with Options

```bash
# Show output from passing tests
cargo nextest run --no-capture

# Run tests serially (one at a time)
cargo nextest run --test-threads=1

# Run only failed tests from last run
cargo nextest run --failed
```

## Key Features

### 1. Parallel Execution

Nextest runs each test in its own process, providing better isolation and parallelism:

```bash
cargo nextest run  # Automatically uses all CPU cores
```

### 2. Test Retries

Automatically retry flaky tests:

```bash
# Retry failed tests up to 3 times
cargo nextest run --retries 3
```

### 3. Better Output

Nextest provides cleaner, more informative output:
- Shows test duration
- Groups failures at the end
- Color-coded results
- Progress indicators

### 4. JUnit Output

Generate JUnit XML reports for CI/CD:

```bash
cargo nextest run --profile ci
```

### 5. Test Groups

Run specific groups of tests:

```bash
# Run only unit tests
cargo nextest run --lib

# Run only integration tests
cargo nextest run --tests

# Run only doc tests (uses cargo test)
cargo test --doc
```

## Comparison: `cargo test` vs `cargo nextest`

| Feature | `cargo test` | `cargo nextest` |
|---------|-------------|-----------------|
| Speed | Slower | Faster |
| Parallelism | Per-crate | Per-test |
| Output | Verbose | Clean |
| Retries | No | Yes |
| JUnit reports | No | Yes |
| Test isolation | Shared process | Separate processes |

## Common Commands

```bash
# List all tests without running
cargo nextest list

# Run tests with verbose output
cargo nextest run -v

# Run tests in release mode
cargo nextest run --release

# Run with specific number of threads
cargo nextest run -j 4

# Show slow tests
cargo nextest run --profile default --slow-timeout 60
```

## Configuration

Create `.config/nextest.toml` in your project:

```toml
[profile.default]
retries = 1
slow-timeout = { period = "60s" }

[profile.ci]
retries = 3
fail-fast = false
```

Then use it:
```bash
cargo nextest run --profile ci
```

## When to Use Nextest

**Use nextest when:**
- You have a large test suite
- Tests are slow
- You need better CI/CD integration
- You want cleaner output
- You have flaky tests that need retries

**Stick with `cargo test` when:**
- Running doc tests (nextest doesn't support them yet)
- You need specific `cargo test` features
- Your project is very small

## Integration with CI/CD

### GitHub Actions Example

```yaml
- name: Install nextest
  run: cargo install cargo-nextest --locked

- name: Run tests
  run: cargo nextest run --profile ci
```

### Generate Reports

```bash
# Generate JUnit XML
cargo nextest run --profile ci

# Output is in target/nextest/ci/junit.xml
```

## Tips

1. **Combine with cargo watch** for development:
   ```bash
   cargo install cargo-watch
   cargo watch -x "nextest run"
   ```

2. **Use profiles** for different environments (local, CI, etc.)

3. **Check test timing** to identify slow tests:
   ```bash
   cargo nextest run --profile default
   ```

4. **For doc tests**, still use `cargo test`:
   ```bash
   cargo test --doc
   ```

## Summary

Nextest is a modern, faster alternative to `cargo test` with:
- Better parallelism and speed
- Cleaner output
- Test retries for flaky tests
- JUnit reports for CI/CD
- Per-test process isolation

Install it with `cargo install cargo-nextest` and use `cargo nextest run` instead of `cargo test`.
