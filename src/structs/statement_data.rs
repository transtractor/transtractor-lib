use crate ::structs::ProtoTransaction;
use chrono::{DateTime, Utc, TimeZone, Datelike};

#[derive(Clone)]
pub struct StatementData {
    pub start_date: Option<i64>,
    pub start_date_year: Option<i32>,
    pub opening_balance: Option<f64>,
    pub closing_balance: Option<f64>,
    pub proto_transactions: Vec<ProtoTransaction>,
}

impl StatementData {
    pub fn new() -> Self {
        Self {
            start_date: None,
            start_date_year: None,
            opening_balance: None,
            closing_balance: None,
            proto_transactions: Vec::new(),
        }
    }

    pub fn opening_balance(&self) -> Option<f64> { self.opening_balance }
    pub fn closing_balance(&self) -> Option<f64> { self.closing_balance }
    pub fn start_date(&self) -> Option<i64> { self.start_date }
    pub fn start_date_year(&self) -> Option<i32> { self.start_date_year }

    // Setters for the fields
    pub fn set_start_date(&mut self, date: i64) {
        self.start_date = Some(date);
        self.start_date_year = Utc.timestamp_millis_opt(date).single().map(|dt| dt.year());
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
        if let Some(ms) = self.start_date {
            if let Some(dt) = DateTime::<Utc>::from_timestamp_millis(ms) {
                println!("  Start Date: {}", dt.format("%d %b %Y"));
            } else {
                println!("  Start Date: {}", ms);
            }
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
            let date_str = match tx.date {
                Some(ms) => match DateTime::<Utc>::from_timestamp_millis(ms) {
                    Some(dt) => dt.format("%d %b %Y").to_string(),
                    None => ms.to_string(),
                },
                None => "Not set".to_string(),
            };
            let amount_str = match tx.amount {
                Some(a) => format!("{:.2}", a),
                None => "Not set".to_string(),
            };
            let balance_str = match tx.balance {
                Some(b) => format!("{:.2}", b),
                None => "Not set".to_string(),
            };
            println!(
                "    {}: date={}, description=\"{}\", amount={}, balance={}",
                i + 1,
                date_str,
                tx.description,
                amount_str,
                balance_str
            );
        }
    }
}

impl Default for StatementData {
    fn default() -> Self { Self::new() }
}