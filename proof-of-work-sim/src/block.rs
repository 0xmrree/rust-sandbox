use rand::Rng;
use sha2::{Digest, Sha256};
use crate::traits::{Hashable, Validatable};
use crate::transaction::Transaction;

/// Represents a block in the blockchain
#[derive(Debug, Clone)]
pub struct Block {
    /// Block index
    pub index: u64,
    /// Transactions in this block
    pub transactions: Vec<Transaction>,
    /// Nonce for proof-of-work
    pub nonce: u64,
    /// Hash of the previous block
    pub prev_hash: String,
    /// Whether this block has valid proof-of-work
    pub is_valid: bool,
}

impl Block {
    /// Create a new block
    pub fn new(index: u64, transactions: Vec<Transaction>, prev_hash: String) -> Self {
        Block {
            index,
            transactions,
            nonce: 0,
            prev_hash,
            is_valid: false,
        }
    }

    /// Create the genesis block (first block)
    pub fn genesis() -> Self {
        let coinbase = Transaction::new_coinbase("genesis".to_string());
        Block {
            index: 0,
            transactions: vec![coinbase],
            nonce: 0,
            prev_hash: "0".repeat(64), // 64 zeros for genesis
            is_valid: true, // Genesis is always valid
        }
    }

    /// Try a random nonce for proof-of-work
    /// Returns true if the nonce produces a valid hash
    pub fn try_nonce(&mut self, ceiling: i32) -> bool {
        // Generate random nonce
        let mut rng = rand::thread_rng();
        self.nonce = rng.gen();

        // Calculate hash with this nonce
        let hash = self.hash();
        
        // Convert first 8 hex chars to i32 for comparison
        let hash_value = i32::from_str_radix(&hash[..8], 16).unwrap_or(i32::MAX);
        
        // Check if hash is below ceiling
        if hash_value < ceiling {
            self.is_valid = true;
            true
        } else {
            false
        }
    }
}

impl Hashable for Block {
    fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        
        // Hash all transactions
        for tx in &self.transactions {
            hasher.update(tx.hash());
        }
        
        // Add previous block hash
        hasher.update(&self.prev_hash);
        
        // Add nonce
        hasher.update(self.nonce.to_string());
        
        format!("{:x}", hasher.finalize())
    }
}

impl Validatable for Block {
    fn is_valid(&self) -> bool {
        self.is_valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_block() {
        let genesis = Block::genesis();
        assert_eq!(genesis.index, 0);
        assert_eq!(genesis.nonce, 0);
        assert_eq!(genesis.prev_hash, "0".repeat(64));
        assert!(genesis.is_valid);
        assert_eq!(genesis.transactions.len(), 1);
    }

    #[test]
    fn test_new_block() {
        let tx = Transaction::new_coinbase("miner1".to_string());
        let block = Block::new(1, vec![tx], "prev_hash".to_string());
        assert_eq!(block.index, 1);
        assert_eq!(block.nonce, 0);
        assert_eq!(block.prev_hash, "prev_hash");
        assert!(!block.is_valid);
    }

    #[test]
    fn test_block_hash() {
        let tx = Transaction::new_coinbase("miner1".to_string());
        let block = Block::new(1, vec![tx], "prev_hash".to_string());
        let hash = block.hash();
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex chars
    }

    #[test]
    fn test_try_nonce_with_max_ceiling() {
        let tx = Transaction::new_coinbase("miner1".to_string());
        let mut block = Block::new(1, vec![tx], "prev_hash".to_string());
        
        // With i32::MAX ceiling, should succeed within a few tries
        let mut success = false;
        for _ in 0..100 {
            if block.try_nonce(i32::MAX) {
                success = true;
                break;
            }
        }
        assert!(success);
        assert!(block.is_valid);
    }

    #[test]
    fn test_try_nonce_with_zero_ceiling() {
        let tx = Transaction::new_coinbase("miner1".to_string());
        let mut block = Block::new(1, vec![tx], "prev_hash".to_string());
        
        // With 0 ceiling, should always fail
        let result = block.try_nonce(0);
        assert!(!result);
        assert!(!block.is_valid);
    }

    #[test]
    fn test_block_hash_consistency() {
        let tx = Transaction::new_coinbase("miner1".to_string());
        let mut block = Block::new(1, vec![tx], "prev_hash".to_string());
        block.nonce = 12345;
        
        let hash1 = block.hash();
        let hash2 = block.hash();
        assert_eq!(hash1, hash2);
    }
}
