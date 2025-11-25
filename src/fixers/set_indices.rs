use crate::structs::StatementData;

/// Reset prototransaction indices based on their order within each day.
/// This fixer should be applied after implicit_balance to lock the order
/// so that transactions can be reordered safely without breaking the running balance.
///
/// # Panics
/// Panics if dates are found out of order - the transtractor isn't set up to deal with this.
pub fn fix_set_indices(sd: &mut StatementData) {
    if sd.proto_transactions.is_empty() {
        return;
    }

    let mut prev_date: Option<i64> = None;
    let mut current_day: Option<i64> = None;
    let mut day_index = 0;

    for (i, proto_transaction) in sd.proto_transactions.iter_mut().enumerate() {
        // Validate that transaction has a date (should be guaranteed by earlier fixers)
        let current_date = match proto_transaction.date {
            Some(date) => date,
            None => panic!(
                "Transaction at position {} does not have a date. This should not happen after date fixers.",
                i
            ),
        };

        // Check that dates are in chronological order (panic if not)
        if let Some(prev) = prev_date {
            if current_date < prev {
                panic!(
                    "Transaction dates are out of order at position {}: current date {} < previous date {}. The transtractor isn't set up to deal with this.",
                    i, current_date, prev
                );
            }
        }
        prev_date = Some(current_date);

        // Check if we've moved to a new day and reset indices
        if current_day != Some(current_date) {
            current_day = Some(current_date);
            day_index = 0; // Reset index for new day
        }

        // Set the index for this transaction
        proto_transaction.index = day_index;
        day_index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::{ProtoTransaction, StatementData};

    fn create_proto_transaction(date: i64, index: usize) -> ProtoTransaction {
        ProtoTransaction {
            date: Some(date),
            index,
            description: format!("Transaction {}", index),
            amount: Some(100.0),
            balance: None,
        }
    }

    #[test]
    fn test_fix_set_indices_empty_transactions() {
        let mut sd = StatementData {
            proto_transactions: vec![],
            account_number: None,
            opening_balance: None,
            closing_balance: None,
            start_date: None,
            start_date_year: None,
            key: None,
            errors: Vec::new(),
        };

        fix_set_indices(&mut sd);
        assert_eq!(sd.proto_transactions.len(), 0);
    }

    #[test]
    fn test_fix_set_indices_single_transaction() {
        let mut sd = StatementData {
            proto_transactions: vec![create_proto_transaction(1000, 5)],
            account_number: None,
            opening_balance: None,
            closing_balance: None,
            start_date: None,
            start_date_year: None,
            key: None,
            errors: Vec::new(),
        };

        fix_set_indices(&mut sd);
        assert_eq!(sd.proto_transactions[0].index, 0);
    }

    #[test]
    fn test_fix_set_indices_all_none() {
        let mut sd = StatementData {
            proto_transactions: vec![
                create_proto_transaction(1000, 5),
                create_proto_transaction(1000, 10),
                create_proto_transaction(1000, 15),
            ],
            account_number: None,
            opening_balance: None,
            closing_balance: None,
            start_date: None,
            start_date_year: None,
            key: None,
            errors: Vec::new(),
        };

        fix_set_indices(&mut sd);

        // All should have sequential indices within the same day
        assert_eq!(sd.proto_transactions[0].index, 0);
        assert_eq!(sd.proto_transactions[1].index, 1);
        assert_eq!(sd.proto_transactions[2].index, 2);
    }

    #[test]
    fn test_fix_set_indices_different_days() {
        let mut sd = StatementData {
            proto_transactions: vec![
                create_proto_transaction(1000, 10), // Day 1, transaction 0
                create_proto_transaction(1000, 20), // Day 1, transaction 1
                create_proto_transaction(2000, 30), // Day 2, transaction 0
                create_proto_transaction(2000, 40), // Day 2, transaction 1
                create_proto_transaction(3000, 50), // Day 3, transaction 0
            ],
            account_number: None,
            opening_balance: None,
            closing_balance: None,
            start_date: None,
            start_date_year: None,
            key: None,
            errors: Vec::new(),
        };

        fix_set_indices(&mut sd);

        // Day 1 transactions
        assert_eq!(sd.proto_transactions[0].index, 0);
        assert_eq!(sd.proto_transactions[1].index, 1);

        // Day 2 transactions (indices reset)
        assert_eq!(sd.proto_transactions[2].index, 0);
        assert_eq!(sd.proto_transactions[3].index, 1);

        // Day 3 transaction (index reset)
        assert_eq!(sd.proto_transactions[4].index, 0);
    }

    #[test]
    #[should_panic(expected = "Transaction at position 1 does not have a date")]
    fn test_fix_set_indices_panics_on_missing_date() {
        let mut sd = StatementData {
            proto_transactions: vec![
                create_proto_transaction(1000, 0),
                ProtoTransaction {
                    date: None, // Missing date should cause panic
                    index: 1,
                    description: "No date transaction".to_string(),
                    amount: Some(100.0),
                    balance: None,
                },
            ],
            account_number: None,
            opening_balance: None,
            closing_balance: None,
            start_date: None,
            start_date_year: None,
            key: None,
            errors: Vec::new(),
        };

        fix_set_indices(&mut sd);
    }

    #[test]
    #[should_panic(expected = "Transaction dates are out of order at position 1")]
    fn test_fix_set_indices_panics_on_out_of_order_dates() {
        let mut sd = StatementData {
            proto_transactions: vec![
                create_proto_transaction(2000, 0), // Later date first
                create_proto_transaction(1000, 1), // Earlier date second - should panic
            ],
            account_number: None,
            opening_balance: None,
            closing_balance: None,
            start_date: None,
            start_date_year: None,
            key: None,
            errors: Vec::new(),
        };

        fix_set_indices(&mut sd);
    }
}
