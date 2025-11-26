use rand::Rng;
use std::thread;
use std::time::{Duration, Instant};
use crate::blockchain::Blockchain;
use crate::config::Config;
use crate::traits::Hashable;

/// Represents a mining node
pub struct Node {
    /// Node ID (e.g., "node-a3f2")
    pub id: String,
    /// The blockchain this node maintains
    pub blockchain: Blockchain,
}

impl Node {
    /// Create a new node with random ID
    pub fn new(config: Config) -> Self {
        let id = Self::generate_id();
        Node {
            id,
            blockchain: Blockchain::new(config),
        }
    }

    /// Generate a random node ID (e.g., "node-a3f2")
    pub fn generate_id() -> String {
        let mut rng = rand::thread_rng();
        let hex: String = (0..4)
            .map(|_| format!("{:x}", rng.gen_range(0..16)))
            .collect();
        format!("node-{}", hex)
    }

    /// Start mining blocks
    pub fn start_mining(&mut self) {
        println!("🚀 {} started mining...\n", self.id);
        
        loop {
            let start_time = Instant::now();
            
            // Try to mine a block (keep trying until successful)
            while !self.blockchain.try_mine_block(&self.id) {
                // Keep trying different nonces
            }
            
            // Block mined! Calculate remaining delay time
            let elapsed = start_time.elapsed();
            let delay = Duration::from_secs(self.blockchain.config.delay_seconds);
            
            if elapsed < delay {
                thread::sleep(delay - elapsed);
            }
            
            // Print the last 3 blocks
            self.print_chain();
        }
    }

    /// Print the last 3 blocks in the chain
    pub fn print_chain(&self) {
        let blocks = self.blockchain.last_n_blocks(3);
        
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📊 {} - Chain Status (Last {} blocks)", self.id, blocks.len());
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        for block in blocks {
            let hash = block.hash();
            let hash_short = &hash[hash.len() - 8..]; // Last 8 chars
            let nonce_short = format!("{:016x}", block.nonce);
            let nonce_display = &nonce_short[nonce_short.len() - 8..]; // Last 8 chars
            
            println!(
                "Block #{:<3} | Hash: ...{} | Nonce: ...{} | Valid: {}",
                block.index,
                hash_short,
                nonce_display,
                if block.is_valid { "✅" } else { "❌" }
            );
        }
        
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let config = Config::default();
        let node = Node::new(config);
        assert!(node.id.starts_with("node-"));
        assert_eq!(node.id.len(), 9); // "node-" + 4 hex chars
        assert_eq!(node.blockchain.len(), 1); // Genesis block
    }

    #[test]
    fn test_generate_id_format() {
        let id = Node::generate_id();
        assert!(id.starts_with("node-"));
        assert_eq!(id.len(), 9);
        
        // Check that the hex part is valid
        let hex_part = &id[5..];
        assert!(hex_part.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_id_uniqueness() {
        let id1 = Node::generate_id();
        let id2 = Node::generate_id();
        // Very unlikely to be the same (1 in 65536 chance)
        // But this test could theoretically fail
        assert_ne!(id1, id2);
    }
}
