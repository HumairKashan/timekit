//! Integration tests for formatting edge cases and error handling.

use std::time::Duration;
use timekit::format::{format, format_clock, format_compact, format_long, DurationFormatter, FormatStyle, Precision};

// Note: Most formatting operations are infallible, so these tests focus on
// edge cases and unusual inputs that should still format correctly

#[test]
fn test_format_max_duration() {
    // Maximum Duration value
    let max_duration = Duration::new(u64::MAX, 999_999_999);
    let result = format(max_duration);
    // Should succeed without panicking
    assert!(!result.is_empty());
}

#[test]
fn test_format_very_large_seconds() {
    let duration = Duration::from_secs(u64::MAX);
    let result = format(duration);
    // Should format without error
    assert!(result.contains('d') || result.contains("day"));
}

#[test]
fn test_format_precision_with_zero_subseconds() {
    let duration = Duration::from_secs(60); // Exactly 1 minute, no subseconds
    let result = DurationFormatter::new()
        .precision(Precision::Full)
        .format(duration)
        .unwrap();
    // Should just show "1m", not include zero subsecond components
    assert_eq!(result, "1m");
}

#[test]
fn test_format_all_precision_levels_with_subseconds() {
    let duration = Duration::new(3665, 123_456_789); // 1h 1m 5s + subseconds

    // Hours precision should ignore everything below hours
    let hours = DurationFormatter::new()
        .precision(Precision::Hours)
        .format(duration)
        .unwrap();
    assert_eq!(hours, "1h");

    // Minutes precision should ignore seconds and below
    let minutes = DurationFormatter::new()
        .precision(Precision::Minutes)
        .format(duration)
        .unwrap();
    assert_eq!(minutes, "1h 1m");

    // Seconds precision should ignore subseconds
    let seconds = DurationFormatter::new()
        .precision(Precision::Seconds)
        .format(duration)
        .unwrap();
    assert_eq!(seconds, "1h 1m 5s");

    // Full precision should show everything
    let full = DurationFormatter::new()
        .precision(Precision::Full)
        .format(duration)
        .unwrap();
    assert!(full.contains("ms") && full.contains("us") && full.contains("ns"));
}

#[test]
fn test_format_only_subseconds() {
    // Only nanoseconds, no full seconds
    let duration = Duration::from_nanos(123);
    let result = DurationFormatter::new()
        .precision(Precision::Full)
        .format(duration)
        .unwrap();
    assert_eq!(result, "123ns");

    // Only microseconds
    let duration = Duration::from_micros(456);
    let result = DurationFormatter::new()
        .precision(Precision::Full)
        .format(duration)
        .unwrap();
    assert_eq!(result, "456us");

    // Only milliseconds
    let duration = Duration::from_millis(789);
    let result = DurationFormatter::new()
        .precision(Precision::Full)
        .format(duration)
        .unwrap();
    assert_eq!(result, "789ms");
}

#[test]
fn test_format_seconds_precision_ignores_subseconds() {
    let duration = Duration::new(5, 999_999_999); // 5s + max subseconds
    let result = DurationFormatter::new()
        .precision(Precision::Seconds)
        .format(duration)
        .unwrap();
    // Should only show seconds, not subseconds
    assert_eq!(result, "5s");
}

#[test]
fn test_format_clock_with_many_hours() {
    // Clock format with more than 24 hours
    let duration = Duration::from_secs(100_000); // ~27.7 hours
    let result = format_clock(duration);
    // Should format without error, showing hours > 24
    assert!(result.contains(':'));
}

#[test]
fn test_format_clock_exactly_one_hour() {
    let duration = Duration::from_secs(3600);
    let result = format_clock(duration);
    assert_eq!(result, "01:00:00");
}

#[test]
fn test_format_clock_less_than_one_minute() {
    let duration = Duration::from_secs(30);
    let result = format_clock(duration);
    assert_eq!(result, "00:30");
}

#[test]
fn test_format_long_with_zero_components() {
    // When using long format with precision that eliminates components
    let duration = Duration::from_secs(3600); // Exactly 1 hour
    let result = DurationFormatter::new()
        .style(FormatStyle::Long)
        .precision(Precision::Hours)
        .format(duration)
        .unwrap();
    assert_eq!(result, "1 hour");
}

#[test]
fn test_format_compact_empty_components() {
    // Duration with only hours, no minutes or seconds
    let duration = Duration::from_secs(7200); // 2 hours exactly
    let result = format_compact(duration);
    assert_eq!(result, "2h");
}

#[test]
fn test_format_builder_default_values() {
    // Test that default builder values work correctly
    let duration = Duration::from_secs(90);
    let result = DurationFormatter::new()
        .format(duration)
        .unwrap();
    // Default is Spaced style with Seconds precision
    assert_eq!(result, "1m 30s");
}

#[test]
fn test_format_all_zeros_except_nanoseconds() {
    let duration = Duration::from_nanos(1);
    let result = DurationFormatter::new()
        .precision(Precision::Full)
        .format(duration)
        .unwrap();
    assert_eq!(result, "1ns");
}

#[test]
fn test_format_boundary_between_units() {
    // Test boundary values between units
    let duration = Duration::from_millis(999); // Just under 1 second
    let result = DurationFormatter::new()
        .precision(Precision::Full)
        .format(duration)
        .unwrap();
    assert_eq!(result, "999ms");

    let duration = Duration::from_millis(1000); // Exactly 1 second
    let result = DurationFormatter::new()
        .precision(Precision::Full)
        .format(duration)
        .unwrap();
    assert_eq!(result, "1s");
}

#[test]
fn test_format_precision_removes_trailing_zeros() {
    // When lower precision hides trailing zero components
    let duration = Duration::from_secs(3600); // 1h 0m 0s
    let result = format(duration);
    // Should just show "1h", not "1h 0m 0s"
    assert_eq!(result, "1h");
}

#[test]
fn test_format_long_singular_boundary() {
    // Test singular vs plural at the boundary
    let duration = Duration::from_secs(1);
    assert_eq!(format_long(duration), "1 second");

    let duration = Duration::from_secs(2);
    assert_eq!(format_long(duration), "2 seconds");
}

#[test]
fn test_format_subseconds_only_with_default_precision() {
    // Subseconds with default precision (Seconds) should show as 0s
    let duration = Duration::from_nanos(500);
    let result = format(duration);
    // With Seconds precision, subseconds are ignored, showing 0s
    assert_eq!(result, "0s");
}

#[test]
fn test_format_clock_subseconds_ignored() {
    // Clock format should ignore subseconds
    let duration = Duration::new(90, 999_999_999); // 1m 30s + max subseconds
    let result = format_clock(duration);
    assert_eq!(result, "01:30"); // Subseconds not shown in clock format
}

#[test]
fn test_format_mixed_precision_levels() {
    let duration = Duration::new(3723, 456_789_123); // 1h 2m 3s + subseconds

    // Test each precision level maintains consistency
    let hours = DurationFormatter::new()
        .style(FormatStyle::Long)
        .precision(Precision::Hours)
        .format(duration)
        .unwrap();
    assert!(!hours.contains("minute"));

    let minutes = DurationFormatter::new()
        .style(FormatStyle::Long)
        .precision(Precision::Minutes)
        .format(duration)
        .unwrap();
    assert!(!minutes.contains("second"));
}

#[test]
fn test_format_edge_case_1_nanosecond() {
    let duration = Duration::from_nanos(1);
    let result = DurationFormatter::new()
        .precision(Precision::Full)
        .format(duration)
        .unwrap();
    assert_eq!(result, "1ns");
}

#[test]
fn test_format_edge_case_max_subseconds() {
    let duration = Duration::new(0, 999_999_999);
    let result = DurationFormatter::new()
        .precision(Precision::Full)
        .format(duration)
        .unwrap();
    assert!(result.contains("999ms") && result.contains("999us") && result.contains("999ns"));
}

#[test]
fn test_format_combination_all_styles_with_zero() {
    // Ensure all styles handle zero correctly
    let duration = Duration::ZERO;

    assert_eq!(format(duration), "0s");
    assert_eq!(format_compact(duration), "0s");
    assert_eq!(format_long(duration), "0 seconds");
    assert_eq!(format_clock(duration), "00:00");
}

#[test]
fn test_format_days_with_different_precisions() {
    let duration = Duration::from_secs(86401); // 1d 1s

    let hours_precision = DurationFormatter::new()
        .precision(Precision::Hours)
        .format(duration)
        .unwrap();
    assert_eq!(hours_precision, "1d");

    let seconds_precision = DurationFormatter::new()
        .precision(Precision::Seconds)
        .format(duration)
        .unwrap();
    assert_eq!(seconds_precision, "1d 1s");
}

#[test]
fn test_format_very_small_duration() {
    // Smallest possible non-zero duration
    let duration = Duration::from_nanos(1);
    let result = DurationFormatter::new()
        .precision(Precision::Seconds)
        .format(duration)
        .unwrap();
    // With seconds precision, 1ns rounds down to 0s
    assert_eq!(result, "0s");
}