#[path = "block.rs"]
pub mod block;
#[path = "transaction.rs"]
pub mod transaction;

use block::Block;
use secp256k1::PublicKey;
use std::mem;
use std::time::{SystemTime, UNIX_EPOCH};

use self::block::transaction::Transaction;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: usize,
    pub reward_pubkey: PublicKey,
}

impl Blockchain {
    pub fn new(
        difficulty: usize,
        mining_reward: usize,
        reward_pubkey: PublicKey,
    ) -> Result<Blockchain, std::io::Error> {
        let mut chain: Vec<Block> = Vec::new();
        let pending_transactions: Vec<Transaction> = Vec::new();
        let gen = Blockchain::create_genesis_block();
        chain.push(gen);
        let blockchain = Blockchain {
            chain,
            difficulty,
            pending_transactions,
            mining_reward,
            reward_pubkey,
        };
        Ok(blockchain)
    }

    pub fn mine_pending_transactions(&mut self, miner: PublicKey) {
        let pending_transactions: Vec<Transaction> = Vec::new();
        let transactions = mem::replace(&mut self.pending_transactions, pending_transactions);
        let mut block = Block::new(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
            transactions,
            self.get_latest_block().hash.clone(),
        )
        .expect("Could not create block");
        block.mine_block(self.difficulty);
        self.chain.push(block);
        self.pending_transactions.push(
            Transaction::new(miner, self.reward_pubkey, self.mining_reward)
                .expect("Counld not create transaction"),
        );
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        assert!(tx.is_valid(self.reward_pubkey) == true);
        self.pending_transactions.push(tx);
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.get(self.chain.len() - 1).unwrap()
    }

    pub fn get_balance_of(&self, address: PublicKey) -> usize {
        let mut balance: usize = 0;

        for block in self.chain.iter() {
            for tx in block.transactions.iter() {
                if tx.from == address {
                    balance -= tx.amount;
                }

                if tx.to == address {
                    balance += tx.amount;
                }
            }
        }
        balance
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = self.chain.get(i).expect("Could not get current block");
            let previous = self.chain.get(i - 1).expect("Could not get current block");

            if !current.has_valid_transactions(self.reward_pubkey) {
                return false;
            }

            if current.hash != current.calculate_hash() {
                return false;
            }

            if current.previous_hash != previous.calculate_hash() {
                return false;
            }
        }
        true
    }

    fn create_genesis_block() -> Block {
        let transactions: Vec<Transaction> = Vec::new();

        Block::new("today".to_string(), transactions, "0".to_string())
            .expect("Could not create block")
    }
}
