# Proof-of-Work Blockchain Simulator

A simple blockchain simulator demonstrating proof-of-work mining using Rust best practices.

## Features

- **Proof-of-Work Mining**: Blocks are mined by finding nonces that produce hashes below a ceiling value
- **Coinbase Transactions**: Each block contains a coinbase transaction rewarding the miner
- **Real-time Display**: Shows the last 3 blocks in the chain after each successful mine
- **Configurable**: Easily adjust mining difficulty and delay between blocks

## Rust Best Practices Demonstrated

### 1. **Composition Over Inheritance**
- `Node` **contains** a `Blockchain` (composition)
- `Blockchain` **contains** `Block`s (composition)
- `Block` **contains** `Transaction`s (composition)
- No inheritance - just clean, explicit relationships

### 2. **Traits for Shared Behavior**
```rust
trait Hashable {
    fn hash(&self) -> String;
}

trait Validatable {
    fn is_valid(&self) -> bool;
}
```
- `Block` and `Transaction` both implement `Hashable`
- Traits define shared behavior without inheritance

### 3. **Proper Naming Conventions**
- `snake_case` for functions: `try_nonce()`, `start_mining()`
- `PascalCase` for types: `Block`, `Blockchain`, `Node`
- `SCREAMING_SNAKE_CASE` would be used for constants

### 4. **Documentation Comments**
- Every struct and method has `///` doc comments
- Explains purpose and behavior clearly

### 5. **Explicit Mutability**
```rust
fn try_nonce(&mut self, ceiling: i32) -> bool
```
- Methods that modify state take `&mut self`
- Immutable by default, explicit when mutable

### 6. **Error Handling**
- Uses `Option` and `Result` patterns (implicitly through `unwrap()`)
- No exceptions or hidden control flow

## Architecture

```
Node
 └─ Blockchain (composition)
     └─ Vec<Block> (composition)
         └─ Vec<Transaction> (composition)
```

### Key Components

#### Config
Global configuration accessible throughout the app:
- `ceiling`: Proof-of-work difficulty (default: `i32::MAX` for easy mining)
- `delay_seconds`: Delay after mining a block (default: 1 second)

#### Transaction
Represents a coinbase transaction (block reward):
- `amount`: Reward amount (50 coins)
- `recipient`: Miner ID
- Implements `Hashable` trait

#### Block
Represents a block in the blockchain:
- `index`: Block number
- `transactions`: Array of transactions
- `nonce`: Proof-of-work nonce
- `prev_hash`: Hash of previous block (SHA-256)
- `is_valid`: Whether proof-of-work is valid
- **Methods:**
  - `try_nonce()`: Attempts a random nonce, mutates block, returns true if valid
  - `hash()`: Computes SHA-256 hash of transactions + prev_hash + nonce
  - `genesis()`: Creates the first block (always valid)

#### Blockchain
Manages the chain of blocks:
- `blocks`: Vector of blocks
- `config`: Configuration
- **Methods:**
  - `try_mine_block()`: Creates new block, tries to mine it, adds if valid
  - `latest_block()`: Returns reference to last block
  - `last_n_blocks()`: Returns last N blocks for display

#### Node
Represents a mining node:
- `id`: Random 4-digit hex ID (e.g., "node-a3f2")
- `blockchain`: The blockchain (composition!)
- **Methods:**
  - `start_mining()`: Infinite loop that mines blocks with 1-second delays
  - `print_chain()`: Displays last 3 blocks with hash, nonce, and validity

## How It Works

1. **Node Creation**: Node generates random ID (e.g., "node-a3f2")
2. **Genesis Block**: Blockchain starts with genesis block (nonce 0x0, always valid)
3. **Mining Loop**:
   - Create new block with coinbase transaction
   - Call `try_nonce()` repeatedly until valid nonce found
   - Add block to chain when `is_valid` is true
   - Wait 1 second (or remaining time)
   - Print last 3 blocks
4. **Display**: Shows block index, last 8 chars of hash, last 8 chars of nonce, validity

## Project Structure

The project is organized into multiple modules following Rust best practices:

```
src/
├── main.rs           # Entry point
├── config.rs         # Global configuration
├── traits.rs         # Hashable and Validatable traits
├── transaction.rs    # Transaction implementation
├── block.rs          # Block implementation with PoW
├── blockchain.rs     # Blockchain management
└── node.rs           # Mining node implementation
```

Each module contains:
- Implementation code
- Unit tests (in `#[cfg(test)]` modules)
- Documentation comments

## Running the Simulator

```bash
# Build
cargo build --release

# Run
cargo run --release

# Run with custom config (modify Config::default() in config.rs)
```

## Running Tests

The project includes comprehensive unit tests for all components:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run tests for a specific module
cargo test block::tests
cargo test blockchain::tests
cargo test transaction::tests

# Run a specific test
cargo test test_genesis_block
```

### Test Coverage

Current test coverage includes:

**Config Module (2 tests)**
- ✅ Default configuration
- ✅ Custom configuration

**Transaction Module (4 tests)**
- ✅ Transaction creation
- ✅ Hash generation
- ✅ Hash consistency
- ✅ Different recipients produce different hashes

**Block Module (6 tests)**
- ✅ Genesis block creation
- ✅ New block creation
- ✅ Block hashing
- ✅ Hash consistency
- ✅ Proof-of-work with high ceiling (should succeed)
- ✅ Proof-of-work with zero ceiling (should fail)

**Blockchain Module (7 tests)**
- ✅ Blockchain initialization with genesis
- ✅ Genesis block properties
- ✅ Mining a single block
- ✅ Mining multiple blocks
- ✅ Getting last N blocks
- ✅ Handling requests for more blocks than available
- ✅ Mining with impossible ceiling

**Node Module (3 tests)**
- ✅ Node creation with random ID
- ✅ ID format validation
- ✅ ID uniqueness

**Total: 22 tests, all passing ✅**

### Test Output

```bash
$ cargo test
running 22 tests
test block::tests::test_genesis_block ... ok
test block::tests::test_new_block ... ok
test block::tests::test_block_hash ... ok
test block::tests::test_block_hash_consistency ... ok
test block::tests::test_try_nonce_with_max_ceiling ... ok
test block::tests::test_try_nonce_with_zero_ceiling ... ok
test blockchain::tests::test_blockchain_creation ... ok
test blockchain::tests::test_blockchain_genesis ... ok
test blockchain::tests::test_mine_block ... ok
test blockchain::tests::test_mine_multiple_blocks ... ok
test blockchain::tests::test_last_n_blocks ... ok
test blockchain::tests::test_last_n_blocks_more_than_available ... ok
test blockchain::tests::test_mine_with_impossible_ceiling ... ok
test config::tests::test_config_default ... ok
test config::tests::test_config_new ... ok
test node::tests::test_generate_id_format ... ok
test node::tests::test_generate_id_uniqueness ... ok
test node::tests::test_node_creation ... ok
test transaction::tests::test_transaction_creation ... ok
test transaction::tests::test_transaction_hash ... ok
test transaction::tests::test_transaction_hash_consistency ... ok
test transaction::tests::test_transaction_hash_different_recipients ... ok

test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Example Output

```
⛏️  Proof-of-Work Blockchain Simulator

⚙️  Configuration:
   Ceiling: 2147483647 (almost always mines)
   Delay: 1 second(s)

🚀 node-a3f2 started mining...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 node-a3f2 - Chain Status (Last 3 blocks)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Block #0   | Hash: ...00000000 | Nonce: ...00000000 | Valid: ✅
Block #1   | Hash: ...a3f2b8c9 | Nonce: ...7d4e9f12 | Valid: ✅
Block #2   | Hash: ...5c8a1e3f | Nonce: ...2b6d8a4c | Valid: ✅
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Adjusting Difficulty

To make mining more challenging, modify the ceiling value in `Config::default()`:

```rust
impl Config {
    fn default() -> Self {
        Config {
            ceiling: 0x0FFFFFFF, // Much harder! (1/16th of max)
            delay_seconds: 1,
        }
    }
}
```

Lower ceiling = harder mining = more nonce attempts needed.

## Dependencies

- `sha2`: SHA-256 hashing for blocks
- `rand`: Random nonce generation
- `hex`: Hex encoding for display

## What This Demonstrates

This project shows how Rust's approach to OOP (composition + traits) creates:
- ✅ **Clear ownership**: Each component owns its data
- ✅ **Explicit relationships**: No hidden inheritance dependencies
- ✅ **Easy to understand**: Just follow the composition chain
- ✅ **Easy to test**: Each component can be tested independently
- ✅ **No fragile base classes**: Changes to one component don't break others

Compare this to an inheritance-based approach where `Node extends Blockchain extends Block` - that would create tight coupling and fragile dependencies!

## License

MIT
