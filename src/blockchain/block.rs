use crate::blockchain::ledger::Ledger;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};


const DIFFICULTY: u32 = 4; // placeholder

#[derive(Clone, Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub ledger: Ledger,
    pub prev_hash: String,
    pub nonce: u64,
    pub hash: String,
    pub difficulty: u32,
}

trait BlockTrait: Sized + Clone {
    // constructor
    fn new (index: u64, ledger: Ledger, prev_hash: String, difficulty: u32) -> Self;

    // calculate hach based on block content
    fn calculate_hash(&self) -> String;

    // mine, proof of work
    fn mine(&mut self);
    fn verify(&self) -> bool;

    // validity checks
    fn is_valid(&self) -> bool;
    fn has_valid_hash(&self) -> bool;
    fn is_valid_link(&self, prev_block: &Self) -> bool;
}

impl BlockTrait for Block {
    // block implementation

    // constructor
    fn new (index: u64, ledger: Ledger, prev_hash: String, difficulty: u32) -> Block {
        if is_valid(&self) {
            Block {
                index: index,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();,
                ledger: ledger,
                prev_hash: prev_hash,
                nonce: rand::random(),
                hash: String::new(),
                difficulty: difficulty
            };
            block.hash = block.calculate_hash();
            block
        } else {
            println!("Block cannot be created - invalid information.");
        }
    }

    // hash calculation
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();

        // add all fields except hash
        hasher.update(self.index.to_le_bytes());
        hasher.update(self.timestamp.to_le_bytes());
        // ledger once we figure out the structure
        hasher.update(self.prev_hash.to_le_bytes());
        hasher.update(self.nonce.to_le_bytes());
        hasher.update(self.difficulty.to_le_bytes());

        return hasher.finalize();
    }

    fn mine (&mut self) {
        while calculate_hash(&self) > difficulty {
            nonce = rand::random();
        }

        // valid block - broadcast
        hash = calculate_hash(&self);
        if verify(&self) {
            // put into ledger and update network
        }
    }

    // validity checks
    fn is_valid(&self) -> bool {
        has_valid_hash(&self) && is_valid_link(&self, prev_block: prev_block)
    }

    fn has_valid_hash(&self) -> bool {
        hash < difficulty
    }

    fn is_valid_link(&self, prev_block: &Self) -> bool {
        // check previous block

        return self.prev_hash == calculate_hash(prev_block) &&
            self.index == prev_block.index + 1 &&
            self.timestamp <= prev_block.timestamp
    }
}
