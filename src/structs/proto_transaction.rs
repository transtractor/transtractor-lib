use crate::structs::transaction::Transaction;

/// Represents an incomplete transaction.
/// Serves as a temporary structure to hold transaction data before it is fully parsed, validated, and filled.
#[derive(Debug, Clone)]
pub struct ProtoTransaction {
    /// Date of the transaction as a timestamp (milliseconds since epoch)
    pub date: Option<i64>,
    /// Description of the transaction
    pub description: String,
    /// Amount of the transaction
    pub amount: Option<f64>,
    /// Balance after the transaction
    pub balance: Option<f64>,
}

impl ProtoTransaction {
    /// Create a new ProtoTransaction.
    pub fn new(
        date: Option<i64>,
        description: String,
        amount: Option<f64>,
        balance: Option<f64>,
    ) -> Self {
        Self {
            date,
            description,
            amount,
            balance,
        }
    }

    /// Returns true if all required fields are present and description is not empty.
    pub fn is_ready(&self) -> bool {
        self.date.is_some()
            && self.amount.is_some()
            && self.balance.is_some()
            && !self.description.is_empty()
    }

    /// Converts to a Transaction if all fields are present.
    pub fn to_transaction(&self) -> Result<Transaction, String> {
        if !self.is_ready() {
            return Err("Cannot convert to Transaction: fields are missing".to_string());
        }
        Ok(Transaction::new(
            self.date.unwrap(),
            self.description.clone(),
            self.amount.unwrap(),
            self.balance.unwrap(),
        ))
    }
}