use crate::blockchain::ledger::Ledger;
use crate::blockchain::ledger::Transaction;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};


const DIFFICULTY: u32 = 4; // placeholder

#[derive(Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
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
    fn mine(&mut self, transaction: Transaction);

    // validity checks
    fn verify(&self) -> bool;
    fn has_valid_hash(&self) -> bool;
    fn is_valid_link(&self, prev_block : &Self) -> bool;
}

impl BlockTrait for Block {
    // block implementation

    // constructor
    fn new (index: u64, ledger: Ledger, prev_hash: String, difficulty: u32) -> Block {
        let mut block = Block {
            index: index,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            ledger: ledger,
            prev_hash: prev_hash,
            nonce: rand::random(),
            hash: String::new(),
            difficulty: difficulty
        };
        block.hash = block.calculate_hash();
        block
    }

    // hash calculation
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();

        // add all fields except hash
        hasher.update(self.index.to_le_bytes());
        hasher.update(self.timestamp.to_le_bytes());
        // ledger once we figure out the structure
        hasher.update(self.prev_hash.as_bytes());
        hasher.update(self.nonce.to_le_bytes());
        hasher.update(self.difficulty.to_le_bytes());

        // format as string
        return hex::encode(hasher.finalize());
    }

    fn mine (&mut self, transaction : Transaction) {
        let p_hash = self.calculate_hash();

        while self.calculate_hash().parse::<u32>().unwrap() > self.difficulty {
            self.nonce = rand::random();
        }

        // valid block - broadcast
        if self.verify() {
            // put into ledger and update network
            self.ledger.push(transaction);

            // new block
            let _block = Block::new(self.index + 1, self.ledger.clone(), p_hash, self.difficulty);
        }
    }

    // validity checks
    fn verify(&self) -> bool {
        // change valid_link parameter to previous block
        self.has_valid_hash() && self.is_valid_link(self)
    }

    fn has_valid_hash(&self) -> bool {
        self.hash.parse::<u32>().unwrap() < self.difficulty
    }

    fn is_valid_link(&self, _prev_block : &Self) -> bool {
        // IMPLEMENT
        return true;
    }
}
