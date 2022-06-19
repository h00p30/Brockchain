extern crate sha2;

#[path = "transaction.rs"]
pub mod transaction;

use secp256k1::PublicKey;
use sha2::{Digest, Sha256};

use self::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub timestamp: String,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: usize,
}

impl Block {
    pub fn new(
        timestamp: String,
        transactions: Vec<Transaction>,
        previous_hash: String,
    ) -> Result<Block, std::io::Error> {
        let mut block = Block {
            timestamp,
            transactions,
            previous_hash,
            hash: "".to_string(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        Ok(block)
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{:?}{}{}",
            self.timestamp, self.transactions, self.previous_hash, self.nonce
        ));
        let result = hasher.finalize();
        let string = format!("{:x}", result);
        string
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let mut hash = self.hash.clone();
        while &hash[..difficulty] != vec!['0'; difficulty].into_iter().collect::<String>() {
            self.nonce += 1;
            hash = self.calculate_hash();
        }

        self.hash = hash;

        println!("Block mined: {}", self.hash);
    }

    pub fn has_valid_transactions(&self, reward_pubkey: PublicKey) -> bool {
        for tx in self.transactions.iter() {
            if !tx.is_valid(reward_pubkey) {
                return false;
            }
        }
        true
    }
}
