use crate::structs::StatementData;

pub mod balances;
pub mod fields;

pub use balances::check_balances;
pub use fields::check_fields;

/// Apply all checkers to the StatementData
pub fn check_statement_data(statement: &mut StatementData) {
    check_fields(statement);
    check_balances(statement);
}