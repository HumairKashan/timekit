//! Grammar parser for duration expressions.
//!
//! This module implements the parsing logic that converts a token stream
//! into a `std::time::Duration`. It supports two main formats:
//!
//! 1. Unit-based: `1h 30m`, `2h30m45s`, `500ms`
//! 2. Clock format: `MM:SS` or `HH:MM:SS`

use std::collections::HashSet;
use std::time::Duration;

use crate::error::{Error, Result};
use crate::parse::tokens::{Token, Tokenizer};

/// Supported time units and their multipliers in seconds.
const UNITS: &[(&str, u64)] = &[
    ("ns", 0),           // Special case: handled in nanoseconds
    ("nanosecond", 0),
    ("nanoseconds", 0),
    ("us", 0),           // Special case: handled in microseconds
    ("microsecond", 0),
    ("microseconds", 0),
    ("ms", 0),           // Special case: handled in milliseconds
    ("millisecond", 0),
    ("milliseconds", 0),
    ("s", 1),
    ("sec", 1),
    ("second", 1),
    ("seconds", 1),
    ("m", 60),
    ("min", 60),
    ("minute", 60),
    ("minutes", 60),
    ("h", 3600),
    ("hr", 3600),
    ("hour", 3600),
    ("hours", 3600),
    ("d", 86400),
    ("day", 86400),
    ("days", 86400),
];

/// A duration component with its value and unit.
#[derive(Debug, Clone)]
struct Component {
    value: u64,
    unit: String,
}

/// Parser for duration expressions.
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Creates a new parser from an input string.
    pub fn new(input: &str) -> Result<Self> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(Error::EmptyInput);
        }

        let tokens = Tokenizer::new(trimmed).tokenize()?;
        Ok(Self {
            tokens,
            position: 0,
        })
    }

    /// Peeks at the current token without consuming it.
    fn peek(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }

    /// Consumes and returns the current token.
    fn advance(&mut self) -> &Token {
        let token = self.peek();
        if *token != Token::Eof {
            self.position += 1;
        }
        &self.tokens[self.position - 1]
    }

    /// Checks if the input looks like clock format (contains colons).
    fn is_clock_format(&self) -> bool {
        self.tokens.iter().any(|t| matches!(t, Token::Colon))
    }

    /// Parses a clock format string (MM:SS or HH:MM:SS).
    fn parse_clock_format(&mut self) -> Result<Duration> {
        let mut parts = Vec::new();

        loop {
            match self.peek() {
                Token::Number(n) => {
                    parts.push(*n);
                    self.advance();
                }
                Token::Colon => {
                    self.advance();
                }
                Token::Eof => break,
                _ => {
                    return Err(Error::InvalidClockFormat {
                        input: "invalid token in clock format".to_string(),
                        reason: "expected number or colon".to_string(),
                    });
                }
            }
        }

        // Validate format: must be 2 or 3 parts
        match parts.len() {
            2 => {
                // MM:SS
                let minutes = parts[0];
                let seconds = parts[1];

                if seconds >= 60 {
                    return Err(Error::ValueOutOfRange {
                        component: "seconds".to_string(),
                        value: seconds,
                        max: 59,
                    });
                }

                let total_seconds = minutes
                    .checked_mul(60)
                    .and_then(|m| m.checked_add(seconds))
                    .ok_or(Error::DurationOverflow)?;

                Ok(Duration::from_secs(total_seconds))
            }
            3 => {
                // HH:MM:SS
                let hours = parts[0];
                let minutes = parts[1];
                let seconds = parts[2];

                if minutes >= 60 {
                    return Err(Error::ValueOutOfRange {
                        component: "minutes".to_string(),
                        value: minutes,
                        max: 59,
                    });
                }

                if seconds >= 60 {
                    return Err(Error::ValueOutOfRange {
                        component: "seconds".to_string(),
                        value: seconds,
                        max: 59,
                    });
                }

                let total_seconds = hours
                    .checked_mul(3600)
                    .and_then(|h| h.checked_add(minutes * 60))
                    .and_then(|hm| hm.checked_add(seconds))
                    .ok_or(Error::DurationOverflow)?;

                Ok(Duration::from_secs(total_seconds))
            }
            _ => Err(Error::InvalidClockFormat {
                input: format!("{} parts", parts.len()),
                reason: "expected MM:SS or HH:MM:SS format".to_string(),
            }),
        }
    }

    /// Parses a unit-based duration string.
    fn parse_unit_based(&mut self) -> Result<Duration> {
        let mut components = Vec::new();
        let mut seen_units = HashSet::new();

        loop {
            match self.peek() {
                Token::Number(value) => {
                    let value = *value;
                    self.advance();

                    match self.peek() {
                        Token::Unit(unit) => {
                            let unit = unit.clone();
                            self.advance();

                            // Check for duplicate units
                            let normalized = normalize_unit(&unit);
                            if seen_units.contains(&normalized) {
                                return Err(Error::DuplicateUnit { unit: unit.clone() });
                            }
                            seen_units.insert(normalized);

                            components.push(Component { value, unit });
                        }
                        Token::Eof => {
                            return Err(Error::UnexpectedEndOfInput {
                                expected: "time unit".to_string(),
                            });
                        }
                        _ => {
                            return Err(Error::UnexpectedEndOfInput {
                                expected: "time unit".to_string(),
                            });
                        }
                    }
                }
                Token::Eof => break,
                _ => {
                    return Err(Error::InvalidClockFormat {
                        input: "unexpected token".to_string(),
                        reason: "expected number".to_string(),
                    });
                }
            }
        }

        if components.is_empty() {
            return Err(Error::EmptyInput);
        }

        // Convert components to duration
        let mut duration = Duration::ZERO;

        for component in components {
            let unit_duration = convert_to_duration(component.value, &component.unit)?;
            duration = duration
                .checked_add(unit_duration)
                .ok_or(Error::DurationOverflow)?;
        }

        Ok(duration)
    }

    /// Parses the input and returns a Duration.
    pub fn parse(mut self) -> Result<Duration> {
        if self.is_clock_format() {
            self.parse_clock_format()
        } else {
            self.parse_unit_based()
        }
    }
}

/// Normalizes a unit string to its canonical form.
fn normalize_unit(unit: &str) -> String {
    let lower = unit.to_lowercase();
    match lower.as_str() {
        "ns" | "nanosecond" | "nanoseconds" => "ns".to_string(),
        "us" | "microsecond" | "microseconds" => "us".to_string(),
        "ms" | "millisecond" | "milliseconds" => "ms".to_string(),
        "s" | "sec" | "second" | "seconds" => "s".to_string(),
        "m" | "min" | "minute" | "minutes" => "m".to_string(),
        "h" | "hr" | "hour" | "hours" => "h".to_string(),
        "d" | "day" | "days" => "d".to_string(),
        _ => lower,
    }
}

/// Converts a value and unit to a Duration.
fn convert_to_duration(value: u64, unit: &str) -> Result<Duration> {
    let lower = unit.to_lowercase();

    match lower.as_str() {
        "ns" | "nanosecond" | "nanoseconds" => {
            Ok(Duration::from_nanos(value))
        }
        "us" | "microsecond" | "microseconds" => {
            Ok(Duration::from_micros(value))
        }
        "ms" | "millisecond" | "milliseconds" => {
            Ok(Duration::from_millis(value))
        }
        "s" | "sec" | "second" | "seconds" => {
            Ok(Duration::from_secs(value))
        }
        "m" | "min" | "minute" | "minutes" => {
            value.checked_mul(60)
                .map(Duration::from_secs)
                .ok_or(Error::DurationOverflow)
        }
        "h" | "hr" | "hour" | "hours" => {
            value.checked_mul(3600)
                .map(Duration::from_secs)
                .ok_or(Error::DurationOverflow)
        }
        "d" | "day" | "days" => {
            value.checked_mul(86400)
                .map(Duration::from_secs)
                .ok_or(Error::DurationOverflow)
        }
        _ => Err(Error::InvalidUnit {
            unit: unit.to_string(),
            position: 0, // Position tracking would require more context
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_hours() {
        let duration = Parser::new("2h").unwrap().parse().unwrap();
        assert_eq!(duration, Duration::from_secs(7200));
    }

    #[test]
    fn test_parse_combined() {
        let duration = Parser::new("1h 30m 45s").unwrap().parse().unwrap();
        assert_eq!(duration, Duration::from_secs(5445));
    }

    #[test]
    fn test_parse_no_spaces() {
        let duration = Parser::new("2h30m").unwrap().parse().unwrap();
        assert_eq!(duration, Duration::from_secs(9000));
    }

    #[test]
    fn test_parse_milliseconds() {
        let duration = Parser::new("1500ms").unwrap().parse().unwrap();
        assert_eq!(duration, Duration::from_millis(1500));
    }

    #[test]
    fn test_parse_clock_mm_ss() {
        let duration = Parser::new("01:30").unwrap().parse().unwrap();
        assert_eq!(duration, Duration::from_secs(90));
    }

    #[test]
    fn test_parse_clock_hh_mm_ss() {
        let duration = Parser::new("02:30:45").unwrap().parse().unwrap();
        assert_eq!(duration, Duration::from_secs(9045));
    }

    #[test]
    fn test_duplicate_unit() {
        let result = Parser::new("1h 2h").unwrap().parse();
        assert!(matches!(result, Err(Error::DuplicateUnit { .. })));
    }

    #[test]
    fn test_empty_input() {
        let result = Parser::new("   ");
        assert!(matches!(result, Err(Error::EmptyInput)));
    }

    #[test]
    fn test_invalid_clock_seconds() {
        let result = Parser::new("01:75").unwrap().parse();
        assert!(matches!(result, Err(Error::ValueOutOfRange { .. })));
    }
}