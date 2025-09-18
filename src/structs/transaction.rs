/// Represents a complete transaction. All fields must be filled (no nulls).
#[derive(Debug, Clone)]
pub struct Transaction {
    /// Date of the transaction as a timestamp (milliseconds since epoch)
    pub date: i64,
    /// Description of the transaction
    pub description: String,
    /// Amount of the transaction
    pub amount: f64,
    /// Balance after the transaction
    pub balance: f64,
}

impl Transaction {
    pub fn new(date: i64, description: String, amount: f64, balance: f64) -> Self {
        Self {
            date,
            description,
            amount,
            balance,
        }
    }
}