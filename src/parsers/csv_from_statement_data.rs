use crate::structs::StatementData;
use chrono::{DateTime, Utc};
use std::fs::File;

/// Write all transactions in StatementData to CSV format (date, description, amount, balance)
/// Date is in YYYY-MM-DD format, amounts and balances are in standard decimal format.
/// Descriptions are automatically quoted by the CSV writer.
pub fn parse(sd: &StatementData, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(csv_path)?;
    let mut wtr = csv::Writer::from_writer(file);

    // Write header
    wtr.write_record(&["date", "description", "amount", "balance"])?;

    // Write each transaction
    for tx in &sd.proto_transactions {
        // Only write transactions that have all required fields
        if tx.is_ready() {
            let date_str = if let Some(date_ms) = tx.date {
                if let Some(datetime) = DateTime::<Utc>::from_timestamp_millis(date_ms) {
                    datetime.format("%Y-%m-%d").to_string()
                } else {
                    return Err(format!("Invalid timestamp: {}", date_ms).into());
                }
            } else {
                return Err("Transaction missing date".into());
            };

            let amount = tx.amount.ok_or("Transaction missing amount")?;
            let balance = tx.balance.ok_or("Transaction missing balance")?;

            wtr.write_record(&[
                &date_str,
                &tx.description,
                &amount.to_string(),
                &balance.to_string(),
            ])?;
        }
    }

    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::ProtoTransaction;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_empty_statement() {
        let sd = StatementData::new();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let result = parse(&sd, path);
        assert!(result.is_ok());

        let content = fs::read_to_string(path).unwrap();
        assert_eq!(content, "date,description,amount,balance\n");
    }

    #[test]
    fn test_parse_single_transaction() {
        let mut sd = StatementData::new();
        
        let mut tx = ProtoTransaction::new();
        tx.set_date(1672531200000); // 2023-01-01 00:00:00 UTC
        tx.description = "Test Payment".to_string();
        tx.set_amount(100.50);
        tx.set_balance(1000.75);
        
        sd.add_proto_transaction(tx);

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let result = parse(&sd, path);
        assert!(result.is_ok());

        let content = fs::read_to_string(path).unwrap();
        assert_eq!(content, "date,description,amount,balance\n2023-01-01,Test Payment,100.5,1000.75\n");
    }

    #[test]
    fn test_parse_multiple_transactions() {
        let mut sd = StatementData::new();
        
        // First transaction
        let mut tx1 = ProtoTransaction::new();
        tx1.set_date(1672531200000); // 2023-01-01
        tx1.description = "Payment One".to_string();
        tx1.set_amount(50.25);
        tx1.set_balance(950.25);
        sd.add_proto_transaction(tx1);

        // Second transaction
        let mut tx2 = ProtoTransaction::new();
        tx2.set_date(1672617600000); // 2023-01-02
        tx2.description = "Payment Two".to_string();
        tx2.set_amount(-25.00);
        tx2.set_balance(925.25);
        sd.add_proto_transaction(tx2);

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let result = parse(&sd, path);
        assert!(result.is_ok());

        let content = fs::read_to_string(path).unwrap();
        let expected = "date,description,amount,balance\n2023-01-01,Payment One,50.25,950.25\n2023-01-02,Payment Two,-25,925.25\n";
        assert_eq!(content, expected);
    }

    #[test]
    fn test_parse_description_with_quotes_and_commas() {
        let mut sd = StatementData::new();
        
        let mut tx = ProtoTransaction::new();
        tx.set_date(1672531200000);
        tx.description = "Payment to \"Store, Inc.\" for goods".to_string();
        tx.set_amount(123.45);
        tx.set_balance(876.55);
        
        sd.add_proto_transaction(tx);

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let result = parse(&sd, path);
        assert!(result.is_ok());

        let content = fs::read_to_string(path).unwrap();
        
        // Check that we have our date and amounts
        assert!(content.contains("2023-01-01"));
        assert!(content.contains("123.45"));
        assert!(content.contains("876.55"));
        
        // The description should be properly escaped by the CSV writer
        // Let's check that the essential parts are there
        assert!(content.contains("Store") && content.contains("Inc") && content.contains("goods"));
    }

    #[test]
    fn test_parse_skips_incomplete_transactions() {
        let mut sd = StatementData::new();
        
        // Complete transaction
        let mut tx1 = ProtoTransaction::new();
        tx1.set_date(1672531200000);
        tx1.description = "Complete".to_string();
        tx1.set_amount(100.0);
        tx1.set_balance(1000.0);
        sd.add_proto_transaction(tx1);

        // Incomplete transaction (missing amount)
        let mut tx2 = ProtoTransaction::new();
        tx2.set_date(1672617600000);
        tx2.description = "Incomplete".to_string();
        tx2.set_balance(900.0);
        // tx2.amount is None
        sd.add_proto_transaction(tx2);

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let result = parse(&sd, path);
        assert!(result.is_ok());

        let content = fs::read_to_string(path).unwrap();
        // Should only contain the complete transaction
        assert_eq!(content, "date,description,amount,balance\n2023-01-01,Complete,100,1000\n");
    }

    #[test]
    fn test_parse_negative_amounts_and_balances() {
        let mut sd = StatementData::new();
        
        let mut tx = ProtoTransaction::new();
        tx.set_date(1672531200000);
        tx.description = "Overdraft Fee".to_string();
        tx.set_amount(-35.00);
        tx.set_balance(-10.50);
        
        sd.add_proto_transaction(tx);

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let result = parse(&sd, path);
        assert!(result.is_ok());

        let content = fs::read_to_string(path).unwrap();
        assert_eq!(content, "date,description,amount,balance\n2023-01-01,Overdraft Fee,-35,-10.5\n");
    }

    #[test]
    fn test_parse_description_with_comma_gets_quoted() {
        let mut sd = StatementData::new();
        
        let mut tx = ProtoTransaction::new();
        tx.set_date(1672531200000); // 2023-01-01
        tx.description = "Transfer to Smith, John".to_string();
        tx.set_amount(-250.00);
        tx.set_balance(750.00);
        
        sd.add_proto_transaction(tx);

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let result = parse(&sd, path);
        assert!(result.is_ok());

        let content = fs::read_to_string(path).unwrap();
        
        // The CSV writer should automatically quote the description field because it contains a comma
        // Expected format: date,description,amount,balance
        // Should be: 2023-01-01,"Transfer to Smith, John",-250,750
        assert_eq!(content, "date,description,amount,balance\n2023-01-01,\"Transfer to Smith, John\",-250,750\n");
    }
}