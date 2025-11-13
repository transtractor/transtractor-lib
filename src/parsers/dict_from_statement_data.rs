use crate::structs::StatementData;
use std::collections::HashMap;

/// Enum to hold different column data types for the dictionary
#[derive(Debug, Clone, PartialEq)]
pub enum ColumnData {
    DateColumn(Vec<i64>),
    IndexColumn(Vec<usize>),
    StringColumn(Vec<String>),
    AmountColumn(Vec<f64>),
    BalanceColumn(Vec<f64>),
}

impl ColumnData {
    pub fn len(&self) -> usize {
        match self {
            ColumnData::DateColumn(v) => v.len(),
            ColumnData::IndexColumn(v) => v.len(),
            ColumnData::StringColumn(v) => v.len(),
            ColumnData::AmountColumn(v) => v.len(),
            ColumnData::BalanceColumn(v) => v.len(),
        }
    }
}

/// Convert StatementData to a dictionary suitable for creating a Pandas DataFrame.
/// 
/// The dictionary contains lists of values for each field with all required data present:
/// - 'date': List of transaction dates (as i64 timestamps)
/// - 'transaction_index': List of transaction indices (as usize, renamed from 'index' to avoid Pandas conflicts)
/// - 'description': List of transaction descriptions (as String)
/// - 'amount': List of transaction amounts (as f64)
/// - 'balance': List of transaction balances (as f64)
/// 
/// # Panics
/// Panics if any transaction is missing date, amount, or balance data. This should not happen
/// after all fixers have been applied to the StatementData.
/// 
/// # Arguments
/// * `sd` - The StatementData containing proto_transactions to convert
/// 
/// # Returns
/// A HashMap where keys are column names and values are ColumnData enums containing the typed data
pub fn dict_from_statement_data(sd: &StatementData) -> HashMap<String, ColumnData> {
    let mut result: HashMap<String, ColumnData> = HashMap::new();
    
    // Initialize vectors for each column with proper types
    let mut dates: Vec<i64> = Vec::new();
    let mut transaction_indices: Vec<usize> = Vec::new();
    let mut descriptions: Vec<String> = Vec::new();
    let mut amounts: Vec<f64> = Vec::new();
    let mut balances: Vec<f64> = Vec::new();
    
    // Extract data from each proto_transaction
    for (i, proto_transaction) in sd.proto_transactions.iter().enumerate() {
        // Date: required, panic if missing
        dates.push(proto_transaction.date.unwrap_or_else(|| {
            panic!("Transaction at index {} is missing date. This should not happen after fixers.", i)
        }));
        
        // Transaction index: always present
        transaction_indices.push(proto_transaction.index);
        
        // Description: always present
        descriptions.push(proto_transaction.description.clone());
        
        // Amount: required, panic if missing  
        amounts.push(proto_transaction.amount.unwrap_or_else(|| {
            panic!("Transaction at index {} is missing amount. This should not happen after fixers.", i)
        }));
        
        // Balance: required, panic if missing
        balances.push(proto_transaction.balance.unwrap_or_else(|| {
            panic!("Transaction at index {} is missing balance. This should not happen after fixers.", i)
        }));
    }
    
    // Insert all columns into the result HashMap with original types
    result.insert("date".to_string(), ColumnData::DateColumn(dates));
    result.insert("transaction_index".to_string(), ColumnData::IndexColumn(transaction_indices));
    result.insert("description".to_string(), ColumnData::StringColumn(descriptions));
    result.insert("amount".to_string(), ColumnData::AmountColumn(amounts));
    result.insert("balance".to_string(), ColumnData::BalanceColumn(balances));
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::ProtoTransaction;

    fn create_test_statement_data() -> StatementData {
        StatementData {
            proto_transactions: vec![
                ProtoTransaction {
                    date: Some(1609459200000), // 2021-01-01 00:00:00 UTC
                    index: 0,
                    description: "Opening balance".to_string(),
                    amount: Some(0.0), // Changed from None to 0.0
                    balance: Some(1000.0),
                },
                ProtoTransaction {
                    date: Some(1609545600000), // 2021-01-02 00:00:00 UTC
                    index: 0,
                    description: "Purchase at store".to_string(),
                    amount: Some(-50.25),
                    balance: Some(949.75),
                },
                ProtoTransaction {
                    date: Some(1609545600000), // Same day
                    index: 1,
                    description: "ATM withdrawal".to_string(),
                    amount: Some(-100.0),
                    balance: Some(849.75),
                },
                ProtoTransaction {
                    date: Some(1609632000000), // 2021-01-03 00:00:00 UTC - Changed from None
                    index: 0,
                    description: "Deposit".to_string(),
                    amount: Some(25.0),
                    balance: Some(874.75), // Changed from None to calculated balance
                },
            ],
            opening_balance: Some(1000.0),
            closing_balance: Some(874.75), // Updated to match final balance
            start_date: Some(1609459200000),
            start_date_year: Some(2021),
            key: Some("test_statement".to_string()),
            errors: Vec::new(),
        }
    }

    #[test]
    fn test_dict_from_statement_data_basic() {
        let sd = create_test_statement_data();
        let dict = dict_from_statement_data(&sd);
        
        // Check that all expected columns are present
        assert!(dict.contains_key("date"));
        assert!(dict.contains_key("transaction_index"));
        assert!(dict.contains_key("description"));
        assert!(dict.contains_key("amount"));
        assert!(dict.contains_key("balance"));
        
        // Check data length consistency
        let len = sd.proto_transactions.len();
        assert_eq!(dict["date"].len(), len);
        assert_eq!(dict["transaction_index"].len(), len);
        assert_eq!(dict["description"].len(), len);
        assert_eq!(dict["amount"].len(), len);
        assert_eq!(dict["balance"].len(), len);
    }

    #[test]
    fn test_dict_from_statement_data_values() {
        let sd = create_test_statement_data();
        let dict = dict_from_statement_data(&sd);
        
        // Test dates
        if let ColumnData::DateColumn(dates) = &dict["date"] {
            assert_eq!(dates[0], 1609459200000);
            assert_eq!(dates[1], 1609545600000);
            assert_eq!(dates[2], 1609545600000);
            assert_eq!(dates[3], 1609632000000); // No longer None
        } else {
            panic!("Expected DateColumn");
        }
        
        // Test transaction indices (renamed from 'index')
        if let ColumnData::IndexColumn(indices) = &dict["transaction_index"] {
            assert_eq!(indices[0], 0);
            assert_eq!(indices[1], 0);
            assert_eq!(indices[2], 1);
            assert_eq!(indices[3], 0);
        } else {
            panic!("Expected IndexColumn");
        }
        
        // Test descriptions
        if let ColumnData::StringColumn(descriptions) = &dict["description"] {
            assert_eq!(descriptions[0], "Opening balance");
            assert_eq!(descriptions[1], "Purchase at store");
            assert_eq!(descriptions[2], "ATM withdrawal");
            assert_eq!(descriptions[3], "Deposit"); // Updated description
        } else {
            panic!("Expected StringColumn");
        }
        
        // Test amounts
        if let ColumnData::AmountColumn(amounts) = &dict["amount"] {
            assert_eq!(amounts[0], 0.0); // No longer None, now 0.0
            assert_eq!(amounts[1], -50.25);
            assert_eq!(amounts[2], -100.0);
            assert_eq!(amounts[3], 25.0);
        } else {
            panic!("Expected AmountColumn");
        }
        
        // Test balances
        if let ColumnData::BalanceColumn(balances) = &dict["balance"] {
            assert_eq!(balances[0], 1000.0);
            assert_eq!(balances[1], 949.75);
            assert_eq!(balances[2], 849.75);
            assert_eq!(balances[3], 874.75); // No longer None
        } else {
            panic!("Expected BalanceColumn");
        }
    }

    #[test]
    fn test_dict_from_statement_data_empty() {
        let sd = StatementData {
            proto_transactions: vec![],
            opening_balance: None,
            closing_balance: None,
            start_date: None,
            start_date_year: None,
            key: None,
            errors: Vec::new(),
        };
        
        let dict = dict_from_statement_data(&sd);
        
        // Check that all columns are present but empty
        assert!(dict.contains_key("date"));
        assert!(dict.contains_key("transaction_index"));
        assert!(dict.contains_key("description"));
        assert!(dict.contains_key("amount"));
        assert!(dict.contains_key("balance"));
        
        assert_eq!(dict["date"].len(), 0);
        assert_eq!(dict["transaction_index"].len(), 0);
        assert_eq!(dict["description"].len(), 0);
        assert_eq!(dict["amount"].len(), 0);
        assert_eq!(dict["balance"].len(), 0);
    }

    #[test]
    fn test_dict_from_statement_data_single_transaction() {
        let sd = StatementData {
            proto_transactions: vec![
                ProtoTransaction {
                    date: Some(1609459200000),
                    index: 5,
                    description: "Single transaction".to_string(),
                    amount: Some(123.45),
                    balance: Some(123.45),
                },
            ],
            opening_balance: None,
            closing_balance: None,
            start_date: None,
            start_date_year: None,
            key: None,
            errors: Vec::new(),
        };
        
        let dict = dict_from_statement_data(&sd);
        
        // Check single values with proper type matching
        if let ColumnData::DateColumn(dates) = &dict["date"] {
            assert_eq!(dates[0], 1609459200000);
        } else {
            panic!("Expected DateColumn");
        }
        
        if let ColumnData::IndexColumn(indices) = &dict["transaction_index"] {
            assert_eq!(indices[0], 5);
        } else {
            panic!("Expected IndexColumn");
        }
        
        if let ColumnData::StringColumn(descriptions) = &dict["description"] {
            assert_eq!(descriptions[0], "Single transaction");
        } else {
            panic!("Expected StringColumn");
        }
        
        if let ColumnData::AmountColumn(amounts) = &dict["amount"] {
            assert_eq!(amounts[0], 123.45);
        } else {
            panic!("Expected AmountColumn");
        }
        
        if let ColumnData::BalanceColumn(balances) = &dict["balance"] {
            assert_eq!(balances[0], 123.45);
        } else {
            panic!("Expected BalanceColumn");
        }
    }

    #[test]
    #[should_panic(expected = "Transaction at index 0 is missing date")]
    fn test_dict_from_statement_data_panics_on_missing_date() {
        let sd = StatementData {
            proto_transactions: vec![
                ProtoTransaction {
                    date: None, // Missing date should cause panic
                    index: 0,
                    description: "Transaction".to_string(),
                    amount: Some(100.0),
                    balance: Some(100.0),
                },
            ],
            opening_balance: None,
            closing_balance: None,
            start_date: None,
            start_date_year: None,
            key: None,
            errors: Vec::new(),
        };
        
        dict_from_statement_data(&sd);
    }

    #[test]
    #[should_panic(expected = "Transaction at index 1 is missing amount")]
    fn test_dict_from_statement_data_panics_on_missing_amount() {
        let sd = StatementData {
            proto_transactions: vec![
                ProtoTransaction {
                    date: Some(1609459200000),
                    index: 0,
                    description: "Transaction 1".to_string(),
                    amount: Some(100.0),
                    balance: Some(100.0),
                },
                ProtoTransaction {
                    date: Some(1609545600000),
                    index: 1,
                    description: "Transaction 2".to_string(),
                    amount: None, // Missing amount should cause panic
                    balance: Some(50.0),
                },
            ],
            opening_balance: None,
            closing_balance: None,
            start_date: None,
            start_date_year: None,
            key: None,
            errors: Vec::new(),
        };
        
        dict_from_statement_data(&sd);
    }

    #[test]
    #[should_panic(expected = "Transaction at index 2 is missing balance")]
    fn test_dict_from_statement_data_panics_on_missing_balance() {
        let sd = StatementData {
            proto_transactions: vec![
                ProtoTransaction {
                    date: Some(1609459200000),
                    index: 0,
                    description: "Transaction 1".to_string(),
                    amount: Some(100.0),
                    balance: Some(100.0),
                },
                ProtoTransaction {
                    date: Some(1609545600000),
                    index: 1,
                    description: "Transaction 2".to_string(),
                    amount: Some(-50.0),
                    balance: Some(50.0),
                },
                ProtoTransaction {
                    date: Some(1609632000000),
                    index: 2,
                    description: "Transaction 3".to_string(),
                    amount: Some(25.0),
                    balance: None, // Missing balance should cause panic
                },
            ],
            opening_balance: None,
            closing_balance: None,
            start_date: None,
            start_date_year: None,
            key: None,
            errors: Vec::new(),
        };
        
        dict_from_statement_data(&sd);
    }
}
