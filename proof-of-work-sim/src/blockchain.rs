use crate::block::Block;
use crate::config::Config;
use crate::transaction::Transaction;
use crate::traits::Hashable;

/// Represents a blockchain (chain of blocks)
pub struct Blockchain {
    /// Chain of blocks
    pub blocks: Vec<Block>,
    /// Configuration
    pub config: Config,
}

impl Blockchain {
    /// Create a new blockchain with genesis block
    pub fn new(config: Config) -> Self {
        let genesis = Block::genesis();
        Blockchain {
            blocks: vec![genesis],
            config,
        }
    }

    /// Get the latest block
    pub fn latest_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }

    /// Try to mine a new block
    /// Returns true if a block was successfully mined and added
    pub fn try_mine_block(&mut self, miner_id: &str) -> bool {
        // Create new block with coinbase transaction
        let coinbase = Transaction::new_coinbase(miner_id.to_string());
        let prev_hash = self.latest_block().hash();
        let index = self.blocks.len() as u64;
        
        let mut new_block = Block::new(index, vec![coinbase], prev_hash);
        
        // Try to find a valid nonce
        if new_block.try_nonce(self.config.ceiling) {
            // Valid nonce found! Add block to chain
            self.blocks.push(new_block);
            true
        } else {
            false
        }
    }

    /// Get the last N blocks
    pub fn last_n_blocks(&self, n: usize) -> Vec<&Block> {
        let start = if self.blocks.len() > n {
            self.blocks.len() - n
        } else {
            0
        };
        self.blocks[start..].iter().collect()
    }

    /// Get the length of the blockchain
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    /// Check if blockchain is empty (should never be, due to genesis)
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_creation() {
        let config = Config::default();
        let blockchain = Blockchain::new(config);
        assert_eq!(blockchain.len(), 1); // Should have genesis block
        assert!(!blockchain.is_empty());
    }

    #[test]
    fn test_blockchain_genesis() {
        let config = Config::default();
        let blockchain = Blockchain::new(config);
        let genesis = blockchain.latest_block();
        assert_eq!(genesis.index, 0);
        assert!(genesis.is_valid);
    }

    #[test]
    fn test_mine_block() {
        let config = Config::default();
        let mut blockchain = Blockchain::new(config);
        
        // Keep trying until we successfully mine a block
        let mut success = false;
        for _ in 0..100 {
            if blockchain.try_mine_block("miner1") {
                success = true;
                break;
            }
        }
        assert!(success);
        assert_eq!(blockchain.len(), 2); // Genesis + 1 new block
    }

    #[test]
    fn test_mine_multiple_blocks() {
        let config = Config::default();
        let mut blockchain = Blockchain::new(config);
        
        // Mine 5 blocks (keep trying until each succeeds)
        for i in 0..5 {
            let mut success = false;
            for _ in 0..100 {
                if blockchain.try_mine_block(&format!("miner{}", i)) {
                    success = true;
                    break;
                }
            }
            assert!(success);
        }
        
        assert_eq!(blockchain.len(), 6); // Genesis + 5 blocks
    }

    #[test]
    fn test_last_n_blocks() {
        let config = Config::default();
        let mut blockchain = Blockchain::new(config);
        
        // Mine 5 blocks (keep trying until each succeeds)
        for i in 0..5 {
            while !blockchain.try_mine_block(&format!("miner{}", i)) {
                // Keep trying
            }
        }
        
        let last_3 = blockchain.last_n_blocks(3);
        assert_eq!(last_3.len(), 3);
        assert_eq!(last_3[0].index, 3);
        assert_eq!(last_3[1].index, 4);
        assert_eq!(last_3[2].index, 5);
    }

    #[test]
    fn test_last_n_blocks_more_than_available() {
        let config = Config::default();
        let blockchain = Blockchain::new(config);
        
        let last_10 = blockchain.last_n_blocks(10);
        assert_eq!(last_10.len(), 1); // Only genesis block
    }

    #[test]
    fn test_mine_with_impossible_ceiling() {
        let config = Config::new(0, 1); // Impossible ceiling
        let mut blockchain = Blockchain::new(config);
        
        let result = blockchain.try_mine_block("miner1");
        assert!(!result); // Should fail
        assert_eq!(blockchain.len(), 1); // Still only genesis
    }
}
