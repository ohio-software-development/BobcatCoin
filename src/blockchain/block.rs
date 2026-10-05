use crate::blockchain::ledger::Ledger;

const DIFFICULTY: u32 = 4; // this is just a placeholder

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
    // Constructor
    fn new (index: u64, ledger: Ledger, prev_hash: String, difficulty: u32) -> Self;

    // Calculate hach based on block content
    fn calculate_hash(&self) -> String;

    // Mine or proof of work
    fn mine(&mut self);

    // Validity checks
    fn is_valid(&self) -> bool;
    fn has_valid_hash(&self) -> bool;
    fn is_valid_link(&self, prev_block: &Self) -> bool;
}
