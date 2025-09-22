/// Parses a day string and returns the day as u32 if valid (1-31), or None if invalid.
pub fn parse_day(day_str: &str) -> Option<u32> {
    let day = day_str.trim().parse::<u32>().ok()?;
    if day >= 1 && day <= 31 {
        Some(day)
    } else {
        None
    }
}

/// Parses a month string (e.g. "Jan", "March", "12") and returns the month number (1 = Jan, 12 = Dec).
/// Returns None if the input is not a valid month.
pub fn parse_month(month_str: &str) -> Option<u32> {
    // If a number is passed, return it as-is (1-based)
    if let Ok(num) = month_str.trim().parse::<u32>() {
        if num >= 1 && num <= 12 {
            return Some(num);
        }
    }
    match month_str.trim().to_ascii_lowercase().as_str() {
        "jan" | "january" => Some(1),
        "feb" | "february" => Some(2),
        "mar" | "march" => Some(3),
        "apr" | "april" => Some(4),
        "may" => Some(5),
        "jun" | "june" => Some(6),
        "jul" | "july" => Some(7),
        "aug" | "august" => Some(8),
        "sep" | "september" => Some(9),
        "oct" | "october" => Some(10),
        "nov" | "november" => Some(11),
        "dec" | "december" => Some(12),
        _ => None,
    }
}

/// Parses a year string and returns the year as u32 if valid, or None if invalid.
/// - 2-digit years are interpreted as 2000+year.
/// - Years in [1970, 2100) are accepted.
/// - Otherwise returns None.
pub fn parse_year(year_str: &str) -> Option<u32> {
    let year = year_str.trim().parse::<u32>().ok()?;
    if year < 100 {
        Some(year + 2000)
    } else if year >= 1970 && year < 2100 {
        Some(year)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Day tests
    #[test]
    fn test_parse_day_valid() {
        assert_eq!(parse_day("1"), Some(1));
        assert_eq!(parse_day("15"), Some(15));
        assert_eq!(parse_day("31"), Some(31));
    }

    #[test]
    fn test_parse_day_invalid() {
        assert_eq!(parse_day("0"), None);
        assert_eq!(parse_day("32"), None);
        assert_eq!(parse_day("abc"), None);
    }

    // Month tests
    #[test]
    fn test_parse_month_numeric() {
        assert_eq!(parse_month("1"), Some(1));
        assert_eq!(parse_month("12"), Some(12));
        assert_eq!(parse_month("0"), None);
        assert_eq!(parse_month("13"), None);
    }

    #[test]
    fn test_parse_month_text() {
        assert_eq!(parse_month("Jan"), Some(1));
        assert_eq!(parse_month("january"), Some(1));
        assert_eq!(parse_month("Feb"), Some(2));
        assert_eq!(parse_month("March"), Some(3));
        assert_eq!(parse_month("october"), Some(10));
        assert_eq!(parse_month("DEC"), Some(12));
        assert_eq!(parse_month("foo"), None);
    }

    // Year tests
    #[test]
    fn test_parse_year_two_digit() {
        assert_eq!(parse_year("23"), Some(2023));
        assert_eq!(parse_year("99"), Some(2099));
        assert_eq!(parse_year("00"), Some(2000));
    }

    #[test]
    fn test_parse_year_four_digit() {
        assert_eq!(parse_year("1970"), Some(1970));
        assert_eq!(parse_year("2024"), Some(2024));
        assert_eq!(parse_year("2099"), Some(2099));
        assert_eq!(parse_year("2100"), None);
        assert_eq!(parse_year("1969"), None);
    }

    #[test]
    fn test_parse_year_invalid() {
        assert_eq!(parse_year("abc"), None);
        assert_eq!(parse_year(""), None);
        assert_eq!(parse_year("3000"), None);
    }
}