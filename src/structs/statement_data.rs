use crate ::structs::ProtoTransaction;

#[derive(Clone)]
pub struct StatementData {
    pub start_date: Option<i64>,
    pub opening_balance: Option<f64>,
    pub closing_balance: Option<f64>,
    pub proto_transactions: Vec<ProtoTransaction>,
}

impl StatementData {
    pub fn new() -> Self {
        Self {
            start_date: None,
            opening_balance: None,
            closing_balance: None,
            proto_transactions: Vec::new(),
        }
    }

    pub fn opening_balance(&self) -> Option<f64> { self.opening_balance }
    pub fn closing_balance(&self) -> Option<f64> { self.closing_balance }
    pub fn start_date(&self) -> Option<i64> { self.start_date }

    // Setters for the fields
    pub fn set_start_date(&mut self, date: i64) {
        self.start_date = Some(date);
    }

    pub fn set_opening_balance(&mut self, balance: f64) {
        self.opening_balance = Some(balance);
    }

    pub fn set_closing_balance(&mut self, balance: f64) {
        self.closing_balance = Some(balance);
    }

    pub fn add_proto_transaction(&mut self, proto_tx: ProtoTransaction) {
        self.proto_transactions.push(proto_tx);
    }

    pub fn print(&self) {
        println!("Statement Data:");
        if let Some(date) = self.start_date {
            println!("  Start Date: {}", date);
        } else {
            println!("  Start Date: Not set");
        }
        if let Some(balance) = self.opening_balance {
            println!("  Opening Balance: {:.2}", balance);
        } else {
            println!("  Opening Balance: Not set");
        }
        if let Some(balance) = self.closing_balance {
            println!("  Closing Balance: {:.2}", balance);
        } else {
            println!("  Closing Balance: Not set");
        }
        println!("  Proto Transactions:");
        for (i, tx) in self.proto_transactions.iter().enumerate() {
            println!("    {}: {:?}", i + 1, tx);
        }
    }
}

impl Default for StatementData {
    fn default() -> Self { Self::new() }
}