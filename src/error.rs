//! Error types for timekit parsing and formatting operations.
//!
//! This module defines all error types that can occur during duration parsing
//! and formatting. All errors are explicit and descriptive to aid debugging.

use std::fmt;

/// The main error type for timekit operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The input string is empty or contains only whitespace.
    EmptyInput,

    /// Invalid character encountered during parsing.
    InvalidCharacter {
        /// The invalid character
        character: char,
        /// Position in the input string (0-indexed)
        position: usize,
    },

    /// Invalid time unit encountered.
    InvalidUnit {
        /// The invalid unit string
        unit: String,
        /// Position in the input string (0-indexed)
        position: usize,
    },

    /// Number parsing failed (overflow, invalid format, etc).
    InvalidNumber {
        /// The string that failed to parse
        value: String,
        /// Position in the input string (0-indexed)
        position: usize,
        /// The underlying parsing error
        reason: String,
    },

    /// Duplicate time unit in the same expression.
    DuplicateUnit {
        /// The duplicated unit
        unit: String,
    },

    /// Invalid clock format (MM:SS or HH:MM:SS expected).
    InvalidClockFormat {
        /// The malformed clock string
        input: String,
        /// Explanation of what's wrong
        reason: String,
    },

    /// Value out of valid range.
    ValueOutOfRange {
        /// The component that's out of range (e.g., "seconds", "minutes")
        component: String,
        /// The actual value
        value: u64,
        /// The maximum allowed value
        max: u64,
    },

    /// Unexpected end of input while parsing.
    UnexpectedEndOfInput {
        /// What was expected
        expected: String,
    },

    /// The resulting duration would overflow.
    DurationOverflow,

    /// Invalid format string or formatting error.
    FormatError {
        /// Description of the formatting error
        message: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::EmptyInput => {
                write!(f, "input is empty or contains only whitespace")
            }
            Error::InvalidCharacter { character, position } => {
                write!(f, "invalid character '{}' at position {}", character, position)
            }
            Error::InvalidUnit { unit, position } => {
                write!(f, "invalid time unit '{}' at position {}", unit, position)
            }
            Error::InvalidNumber { value, position, reason } => {
                write!(f, "invalid number '{}' at position {}: {}", value, position, reason)
            }
            Error::DuplicateUnit { unit } => {
                write!(f, "duplicate time unit '{}'", unit)
            }
            Error::InvalidClockFormat { input, reason } => {
                write!(f, "invalid clock format '{}': {}", input, reason)
            }
            Error::ValueOutOfRange { component, value, max } => {
                write!(f, "{} value {} exceeds maximum of {}", component, value, max)
            }
            Error::UnexpectedEndOfInput { expected } => {
                write!(f, "unexpected end of input, expected {}", expected)
            }
            Error::DurationOverflow => {
                write!(f, "duration calculation would overflow")
            }
            Error::FormatError { message } => {
                write!(f, "format error: {}", message)
            }
        }
    }
}

impl std::error::Error for Error {}

/// A specialized Result type for timekit operations.
pub type Result<T> = std::result::Result<T, Error>;