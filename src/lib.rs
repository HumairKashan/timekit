//! # timekit
//!
//! A pure Rust library for parsing and formatting human-readable time durations.
//!
//! `timekit` provides a simple, type-safe API for converting between human-readable
//! duration strings and Rust's `std::time::Duration` type. It supports multiple input
//! formats and provides flexible formatting options.
//!
//! ## Features
//!
//! - **Zero panics**: All errors are explicit and returned as `Result` types
//! - **Strict grammar**: Predictable parsing with clear error messages
//! - **Multiple formats**: Supports both unit-based (`1h 30m`) and clock formats (`01:30:00`)
//! - **Flexible formatting**: Multiple output styles (compact, spaced, long, clock)
//! - **Type-safe**: Leverages Rust's type system for correctness
//!
//! ## Supported Input Formats
//!
//! ### Unit-based format
//!
//! Combines values with time units:
//! - `1h 30m` - with spaces
//! - `2h30m45s` - without spaces
//! - `500ms` - milliseconds
//! - `1d 12h` - days and hours
//!
//! Supported units:
//! - `ns`, `nanosecond`, `nanoseconds`
//! - `us`, `microsecond`, `microseconds`
//! - `ms`, `millisecond`, `milliseconds`
//! - `s`, `sec`, `second`, `seconds`
//! - `m`, `min`, `minute`, `minutes`
//! - `h`, `hr`, `hour`, `hours`
//! - `d`, `day`, `days`
//!
//! ### Clock format
//!
//! Traditional time notation:
//! - `MM:SS` - minutes and seconds (e.g., `01:30`)
//! - `HH:MM:SS` - hours, minutes, and seconds (e.g., `02:30:45`)
//!
//! ## Examples
//!
//! ### Basic parsing
//!
//! ```
//! use timekit::parse;
//! use std::time::Duration;
//!
//! // Parse unit-based format
//! let duration = parse("1h 30m").unwrap();
//! assert_eq!(duration, Duration::from_secs(5400));
//!
//! // Parse clock format
//! let duration = parse("01:30:00").unwrap();
//! assert_eq!(duration, Duration::from_secs(5400));
//!
//! // Parse compact format
//! let duration = parse("2h30m45s").unwrap();
//! assert_eq!(duration, Duration::from_secs(9045));
//! ```
//!
//! ### Error handling
//!
//! ```
//! use timekit::{parse, Error};
//!
//! // Invalid unit
//! let result = parse("1x");
//! assert!(matches!(result, Err(Error::InvalidUnit { .. })));
//!
//! // Duplicate units
//! let result = parse("1h 2h");
//! assert!(matches!(result, Err(Error::DuplicateUnit { .. })));
//!
//! // Value out of range
//! let result = parse("01:75"); // 75 seconds invalid
//! assert!(matches!(result, Err(Error::ValueOutOfRange { .. })));
//! ```
//!
//! ### Formatting
//!
//! ```
//! use timekit::format::{format, format_compact, format_long, format_clock};
//! use std::time::Duration;
//!
//! let duration = Duration::from_secs(3665); // 1h 1m 5s
//!
//! assert_eq!(format(duration), "1h 1m 5s");
//! assert_eq!(format_compact(duration), "1h1m5s");
//! assert_eq!(format_long(duration), "1 hour 1 minute 5 seconds");
//! assert_eq!(format_clock(duration), "01:01:05");
//! ```
//!
//! ### Custom formatting
//!
//! ```
//! use timekit::format::{DurationFormatter, FormatStyle, Precision};
//! use std::time::Duration;
//!
//! let duration = Duration::from_secs(3665);
//!
//! let formatted = DurationFormatter::new()
//!     .style(FormatStyle::Long)
//!     .precision(Precision::Minutes)
//!     .format(duration)
//!     .unwrap();
//!
//! assert_eq!(formatted, "1 hour 1 minute");
//! ```

#![deny(missing_docs)]
#![warn(clippy::all)]

pub mod error;
pub mod format;
pub mod parse;

pub use error::{Error, Result};

use std::time::Duration;

/// Parses a human-readable duration string into a `Duration`.
///
/// This is the main entry point for parsing duration strings. It accepts
/// both unit-based formats (e.g., `1h 30m`) and clock formats (e.g., `01:30:00`).
///
/// # Examples
///
/// ```
/// use timekit::parse;
/// use std::time::Duration;
///
/// let duration = parse("1h 30m").unwrap();
/// assert_eq!(duration, Duration::from_secs(5400));
///
/// let duration = parse("2h30m45s").unwrap();
/// assert_eq!(duration, Duration::from_secs(9045));
///
/// let duration = parse("01:30").unwrap();
/// assert_eq!(duration, Duration::from_secs(90));
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The input is empty or invalid
/// - Units are duplicated
/// - Values are out of range
/// - The format is malformed
///
/// See [`Error`] for all possible error types.
pub fn parse(input: &str) -> Result<Duration> {
    parse::Parser::new(input)?.parse()
}

/// Formats a `Duration` into a human-readable string.
///
/// Uses default formatting settings (spaced style, seconds precision).
///
/// # Examples
///
/// ```
/// use timekit::format_duration;
/// use std::time::Duration;
///
/// let duration = Duration::from_secs(3665);
/// assert_eq!(format_duration(duration), "1h 1m 5s");
/// ```
pub fn format_duration(duration: Duration) -> String {
    format::format(duration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_format_roundtrip() {
        let inputs = vec![
            "1h",
            "30m",
            "45s",
            "1h 30m",
            "2h 30m 45s",
        ];

        for input in inputs {
            let duration = parse(input).unwrap();
            let formatted = format_duration(duration);
            let reparsed = parse(&formatted).unwrap();
            assert_eq!(duration, reparsed, "Roundtrip failed for: {}", input);
        }
    }

    #[test]
    fn test_api_basic() {
        // Test the public API
        let duration = parse("1h 30m").unwrap();
        assert_eq!(duration.as_secs(), 5400);

        let formatted = format_duration(duration);
        assert_eq!(formatted, "1h 30m");
    }

    #[test]
    fn test_error_types() {
        // Empty input
        assert!(matches!(parse(""), Err(Error::EmptyInput)));
        assert!(matches!(parse("   "), Err(Error::EmptyInput)));

        // Invalid unit
        assert!(matches!(parse("1x"), Err(Error::InvalidUnit { .. })));

        // Duplicate unit
        assert!(matches!(parse("1h 2h"), Err(Error::DuplicateUnit { .. })));

        // Invalid clock format
        assert!(matches!(parse("01:75"), Err(Error::ValueOutOfRange { .. })));
    }
}