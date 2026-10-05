

pub type Ledger = Vec<Transaction>;

#[derive(Clone, Debug)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: f64) -> Self {
        Self { sender, recipient, amount }
    }
}
