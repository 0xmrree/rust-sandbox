use sha2::{Digest, Sha256};
use crate::traits::Hashable;

/// Represents a coinbase transaction (block reward)
#[derive(Debug, Clone)]
pub struct Transaction {
    /// Coinbase reward amount
    pub amount: u64,
    /// Recipient (miner)
    pub recipient: String,
}

impl Transaction {
    pub fn new_coinbase(recipient: String) -> Self {
        Transaction {
            amount: 50, // Block reward
            recipient,
        }
    }
}

impl Hashable for Transaction {
    fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}:{}", self.amount, self.recipient));
        format!("{:x}", hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new_coinbase("miner1".to_string());
        assert_eq!(tx.amount, 50);
        assert_eq!(tx.recipient, "miner1");
    }

    #[test]
    fn test_transaction_hash() {
        let tx = Transaction::new_coinbase("miner1".to_string());
        let hash = tx.hash();
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex chars
    }

    #[test]
    fn test_transaction_hash_consistency() {
        let tx1 = Transaction::new_coinbase("miner1".to_string());
        let tx2 = Transaction::new_coinbase("miner1".to_string());
        assert_eq!(tx1.hash(), tx2.hash());
    }

    #[test]
    fn test_transaction_hash_different_recipients() {
        let tx1 = Transaction::new_coinbase("miner1".to_string());
        let tx2 = Transaction::new_coinbase("miner2".to_string());
        assert_ne!(tx1.hash(), tx2.hash());
    }
}
