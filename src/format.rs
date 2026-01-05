//! Formatting durations into human-readable strings.
//!
//! This module provides functionality to convert `std::time::Duration`
//! values back into human-readable string representations.

use std::time::Duration;

use crate::error::{Error, Result};

/// Output style for duration formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatStyle {
    /// Compact format with minimal spacing (e.g., "2h30m45s")
    Compact,
    /// Spaced format with spaces between components (e.g., "2h 30m 45s")
    Spaced,
    /// Long format with full unit names (e.g., "2 hours 30 minutes 45 seconds")
    Long,
    /// Clock format (e.g., "02:30:45" or "30:45")
    Clock,
}

/// Builder for formatting durations with custom options.
#[derive(Debug, Clone)]
pub struct DurationFormatter {
    style: FormatStyle,
    precision: Precision,
}

/// Precision level for duration formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Precision {
    /// Show all non-zero components
    Full,
    /// Show up to seconds (ignore milliseconds and smaller)
    Seconds,
    /// Show up to minutes (ignore seconds and smaller)
    Minutes,
    /// Show up to hours (ignore minutes and smaller)
    Hours,
}

impl Default for DurationFormatter {
    fn default() -> Self {
        Self {
            style: FormatStyle::Spaced,
            precision: Precision::Seconds,
        }
    }
}

impl DurationFormatter {
    /// Creates a new formatter with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the formatting style.
    pub fn style(mut self, style: FormatStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the precision level.
    pub fn precision(mut self, precision: Precision) -> Self {
        self.precision = precision;
        self
    }

    /// Formats a duration according to the configured options.
    pub fn format(&self, duration: Duration) -> Result<String> {
        match self.style {
            FormatStyle::Clock => self.format_clock(duration),
            _ => self.format_unit_based(duration),
        }
    }

    /// Formats a duration in clock format (HH:MM:SS or MM:SS).
    fn format_clock(&self, duration: Duration) -> Result<String> {
        let total_seconds = duration.as_secs();

        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        if hours > 0 {
            Ok(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
        } else {
            Ok(format!("{:02}:{:02}", minutes, seconds))
        }
    }

    /// Formats a duration in unit-based format.
    fn format_unit_based(&self, duration: Duration) -> Result<String> {
        let total_seconds = duration.as_secs();
        let subsec_nanos = duration.subsec_nanos();

        if total_seconds == 0 && subsec_nanos == 0 {
            return Ok(match self.style {
                FormatStyle::Long => "0 seconds".to_string(),
                _ => "0s".to_string(),
            });
        }

        let mut components = Vec::new();

        // Days
        let days = total_seconds / 86400;
        let remainder = total_seconds % 86400;

        // Hours
        let hours = remainder / 3600;
        let remainder = remainder % 3600;

        // Minutes
        let minutes = remainder / 60;
        let seconds = remainder % 60;

        // Milliseconds, microseconds, nanoseconds
        let millis = subsec_nanos / 1_000_000;
        let micros = (subsec_nanos % 1_000_000) / 1_000;
        let nanos = subsec_nanos % 1_000;

        // Add components based on precision
        if days > 0 {
            components.push(self.format_component(days, "d", "day", "days"));
        }

        if hours > 0 {
            components.push(self.format_component(hours, "h", "hour", "hours"));
        }

        if minutes > 0 && !matches!(self.precision, Precision::Hours) {
            components.push(self.format_component(minutes, "m", "minute", "minutes"));
        }

        if seconds > 0 && matches!(self.precision, Precision::Full | Precision::Seconds) {
            components.push(self.format_component(seconds, "s", "second", "seconds"));
        }

        if millis > 0 && self.precision == Precision::Full {
            components.push(self.format_component(millis as u64, "ms", "millisecond", "milliseconds"));
        }

        if micros > 0 && self.precision == Precision::Full {
            components.push(self.format_component(micros as u64, "us", "microsecond", "microseconds"));
        }

        if nanos > 0 && self.precision == Precision::Full {
            components.push(self.format_component(nanos as u64, "ns", "nanosecond", "nanoseconds"));
        }

        if components.is_empty() {
            return Ok(match self.style {
                FormatStyle::Long => "0 seconds".to_string(),
                _ => "0s".to_string(),
            });
        }

        let separator = match self.style {
            FormatStyle::Compact => "",
            FormatStyle::Spaced | FormatStyle::Long => " ",
            FormatStyle::Clock => unreachable!(),
        };

        Ok(components.join(separator))
    }

    /// Formats a single component (value + unit).
    fn format_component(&self, value: u64, short: &str, singular: &str, plural: &str) -> String {
        match self.style {
            FormatStyle::Long => {
                if value == 1 {
                    format!("{} {}", value, singular)
                } else {
                    format!("{} {}", value, plural)
                }
            }
            FormatStyle::Compact | FormatStyle::Spaced => {
                format!("{}{}", value, short)
            }
            FormatStyle::Clock => unreachable!(),
        }
    }
}

/// Formats a duration using default settings (spaced, seconds precision).
pub fn format(duration: Duration) -> String {
    DurationFormatter::new()
        .format(duration)
        .unwrap_or_else(|_| "0s".to_string())
}

/// Formats a duration in compact style.
pub fn format_compact(duration: Duration) -> String {
    DurationFormatter::new()
        .style(FormatStyle::Compact)
        .format(duration)
        .unwrap_or_else(|_| "0s".to_string())
}

/// Formats a duration in long style with full unit names.
pub fn format_long(duration: Duration) -> String {
    DurationFormatter::new()
        .style(FormatStyle::Long)
        .format(duration)
        .unwrap_or_else(|_| "0 seconds".to_string())
}

/// Formats a duration in clock format (HH:MM:SS or MM:SS).
pub fn format_clock(duration: Duration) -> String {
    DurationFormatter::new()
        .style(FormatStyle::Clock)
        .format(duration)
        .unwrap_or_else(|_| "00:00".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_basic() {
        let duration = Duration::from_secs(3665); // 1h 1m 5s
        assert_eq!(format(duration), "1h 1m 5s");
    }

    #[test]
    fn test_format_compact() {
        let duration = Duration::from_secs(3665);
        assert_eq!(format_compact(duration), "1h1m5s");
    }

    #[test]
    fn test_format_long() {
        let duration = Duration::from_secs(3661); // 1h 1m 1s
        assert_eq!(format_long(duration), "1 hour 1 minute 1 second");
    }

    #[test]
    fn test_format_long_plural() {
        let duration = Duration::from_secs(7322); // 2h 2m 2s
        assert_eq!(format_long(duration), "2 hours 2 minutes 2 seconds");
    }

    #[test]
    fn test_format_clock() {
        let duration = Duration::from_secs(3665);
        assert_eq!(format_clock(duration), "01:01:05");
    }

    #[test]
    fn test_format_clock_no_hours() {
        let duration = Duration::from_secs(125);
        assert_eq!(format_clock(duration), "02:05");
    }

    #[test]
    fn test_format_zero() {
        let duration = Duration::ZERO;
        assert_eq!(format(duration), "0s");
    }

    #[test]
    fn test_format_precision_minutes() {
        let duration = Duration::from_secs(3665);
        let formatted = DurationFormatter::new()
            .precision(Precision::Minutes)
            .format(duration)
            .unwrap();
        assert_eq!(formatted, "1h 1m");
    }

    #[test]
    fn test_format_milliseconds() {
        let duration = Duration::from_millis(1500);
        let formatted = DurationFormatter::new()
            .precision(Precision::Full)
            .format(duration)
            .unwrap();
        assert_eq!(formatted, "1s 500ms");
    }
}