/// Base parsers have the basic functions of parsing and storing values and
/// retaining a copy of the text items they consumed.

pub mod amount;
pub mod date;
pub mod primer;

pub use amount::AmountParser;
pub use date::DateParser;
pub use primer::ParserPrimer;