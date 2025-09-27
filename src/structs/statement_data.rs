use crate ::structs::ProtoTransaction;

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
}