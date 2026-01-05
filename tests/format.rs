//! Integration tests for successful duration formatting.

use std::time::Duration;
use timekit::format::{
    format, format_clock, format_compact, format_long, DurationFormatter, FormatStyle, Precision,
};

#[test]
fn test_format_default() {
    assert_eq!(format(Duration::from_secs(3600)), "1h");
    assert_eq!(format(Duration::from_secs(5400)), "1h 30m");
    assert_eq!(format(Duration::from_secs(5445)), "1h 30m 45s");
}

#[test]
fn test_format_compact() {
    assert_eq!(format_compact(Duration::from_secs(3600)), "1h");
    assert_eq!(format_compact(Duration::from_secs(5400)), "1h30m");
    assert_eq!(format_compact(Duration::from_secs(5445)), "1h30m45s");
}

#[test]
fn test_format_long_singular() {
    assert_eq!(format_long(Duration::from_secs(3600)), "1 hour");
    assert_eq!(format_long(Duration::from_secs(60)), "1 minute");
    assert_eq!(format_long(Duration::from_secs(1)), "1 second");
}

#[test]
fn test_format_long_plural() {
    assert_eq!(format_long(Duration::from_secs(7200)), "2 hours");
    assert_eq!(format_long(Duration::from_secs(120)), "2 minutes");
    assert_eq!(format_long(Duration::from_secs(5)), "5 seconds");
}

#[test]
fn test_format_long_combined() {
    assert_eq!(
        format_long(Duration::from_secs(3661)),
        "1 hour 1 minute 1 second"
    );
    assert_eq!(
        format_long(Duration::from_secs(7322)),
        "2 hours 2 minutes 2 seconds"
    );
}

#[test]
fn test_format_clock_hh_mm_ss() {
    assert_eq!(format_clock(Duration::from_secs(3661)), "01:01:01");
    assert_eq!(format_clock(Duration::from_secs(7322)), "02:02:02");
    assert_eq!(format_clock(Duration::from_secs(86399)), "23:59:59");
}

#[test]
fn test_format_clock_mm_ss() {
    assert_eq!(format_clock(Duration::from_secs(90)), "01:30");
    assert_eq!(format_clock(Duration::from_secs(345)), "05:45");
    assert_eq!(format_clock(Duration::from_secs(59)), "00:59");
}

#[test]
fn test_format_zero() {
    assert_eq!(format(Duration::ZERO), "0s");
    assert_eq!(format_compact(Duration::ZERO), "0s");
    assert_eq!(format_long(Duration::ZERO), "0 seconds");
    assert_eq!(format_clock(Duration::ZERO), "00:00");
}

#[test]
fn test_format_days() {
    assert_eq!(format(Duration::from_secs(86400)), "1d");
    assert_eq!(format(Duration::from_secs(172800)), "2d");
    assert_eq!(format_long(Duration::from_secs(86400)), "1 day");
    assert_eq!(format_long(Duration::from_secs(172800)), "2 days");
}

#[test]
fn test_format_days_and_hours() {
    assert_eq!(format(Duration::from_secs(90000)), "1d 1h");
    assert_eq!(format_compact(Duration::from_secs(90000)), "1d1h");
    assert_eq!(format_long(Duration::from_secs(90000)), "1 day 1 hour");
}

#[test]
fn test_format_milliseconds_full_precision() {
    let duration = Duration::from_millis(1500);
    let formatted = DurationFormatter::new()
        .precision(Precision::Full)
        .format(duration)
        .unwrap();
    assert_eq!(formatted, "1s 500ms");
}

#[test]
fn test_format_microseconds_full_precision() {
    let duration = Duration::from_micros(1500);
    let formatted = DurationFormatter::new()
        .precision(Precision::Full)
        .format(duration)
        .unwrap();
    assert_eq!(formatted, "1ms 500us");
}

#[test]
fn test_format_nanoseconds_full_precision() {
    let duration = Duration::from_nanos(1500);
    let formatted = DurationFormatter::new()
        .precision(Precision::Full)
        .format(duration)
        .unwrap();
    assert_eq!(formatted, "1us 500ns");
}

#[test]
fn test_format_precision_hours() {
    let duration = Duration::from_secs(5445); // 1h 30m 45s
    let formatted = DurationFormatter::new()
        .precision(Precision::Hours)
        .format(duration)
        .unwrap();
    assert_eq!(formatted, "1h");
}

#[test]
fn test_format_precision_minutes() {
    let duration = Duration::from_secs(5445); // 1h 30m 45s
    let formatted = DurationFormatter::new()
        .precision(Precision::Minutes)
        .format(duration)
        .unwrap();
    assert_eq!(formatted, "1h 30m");
}

#[test]
fn test_format_precision_seconds() {
    let duration = Duration::from_millis(5445500); // 1h 30m 45s 500ms
    let formatted = DurationFormatter::new()
        .precision(Precision::Seconds)
        .format(duration)
        .unwrap();
    assert_eq!(formatted, "1h 30m 45s");
}

#[test]
fn test_format_precision_full() {
    let duration = Duration::new(5445, 500_000_000); // 1h 30m 45s 500ms
    let formatted = DurationFormatter::new()
        .precision(Precision::Full)
        .format(duration)
        .unwrap();
    assert_eq!(formatted, "1h 30m 45s 500ms");
}

#[test]
fn test_format_builder_chaining() {
    let duration = Duration::from_secs(5445);

    let formatted = DurationFormatter::new()
        .style(FormatStyle::Compact)
        .precision(Precision::Minutes)
        .format(duration)
        .unwrap();

    assert_eq!(formatted, "1h30m");
}

#[test]
fn test_format_all_styles() {
    let duration = Duration::from_secs(3665); // 1h 1m 5s

    let spaced = DurationFormatter::new()
        .style(FormatStyle::Spaced)
        .format(duration)
        .unwrap();
    assert_eq!(spaced, "1h 1m 5s");

    let compact = DurationFormatter::new()
        .style(FormatStyle::Compact)
        .format(duration)
        .unwrap();
    assert_eq!(compact, "1h1m5s");

    let long = DurationFormatter::new()
        .style(FormatStyle::Long)
        .format(duration)
        .unwrap();
    assert_eq!(long, "1 hour 1 minute 5 seconds");

    let clock = DurationFormatter::new()
        .style(FormatStyle::Clock)
        .format(duration)
        .unwrap();
    assert_eq!(clock, "01:01:05");
}

#[test]
fn test_format_large_values() {
    let duration = Duration::from_secs(604800); // 7 days
    assert_eq!(format(duration), "7d");

    let duration = Duration::from_secs(31536000); // 365 days
    assert_eq!(format(duration), "365d");
}

#[test]
fn test_format_only_milliseconds() {
    let duration = Duration::from_millis(500);
    let formatted = DurationFormatter::new()
        .precision(Precision::Full)
        .format(duration)
        .unwrap();
    assert_eq!(formatted, "500ms");
}

#[test]
fn test_format_complex_subseconds() {
    let duration = Duration::new(1, 123_456_789); // 1s 123ms 456us 789ns
    let formatted = DurationFormatter::new()
        .precision(Precision::Full)
        .format(duration)
        .unwrap();
    assert_eq!(formatted, "1s 123ms 456us 789ns");
}

#[test]
fn test_format_hours_only() {
    assert_eq!(format(Duration::from_secs(7200)), "2h");
    assert_eq!(format_compact(Duration::from_secs(7200)), "2h");
    assert_eq!(format_long(Duration::from_secs(7200)), "2 hours");
}

#[test]
fn test_format_minutes_only() {
    assert_eq!(format(Duration::from_secs(1800)), "30m");
    assert_eq!(format_compact(Duration::from_secs(1800)), "30m");
    assert_eq!(format_long(Duration::from_secs(1800)), "30 minutes");
}

#[test]
fn test_format_seconds_only() {
    assert_eq!(format(Duration::from_secs(45)), "45s");
    assert_eq!(format_compact(Duration::from_secs(45)), "45s");
    assert_eq!(format_long(Duration::from_secs(45)), "45 seconds");
}

#[test]
fn test_format_mixed_units() {
    let duration = Duration::from_secs(93784); // 1d 2h 3m 4s
    assert_eq!(format(duration), "1d 2h 3m 4s");
    assert_eq!(format_compact(duration), "1d2h3m4s");
}

#[test]
fn test_format_with_subseconds_default_precision() {
    // Default precision is Seconds, so subseconds should be ignored
    let duration = Duration::new(10, 500_000_000);
    assert_eq!(format(duration), "10s");
}

#[test]
fn test_format_long_with_all_units() {
    let duration = Duration::from_secs(93784); // 1d 2h 3m 4s
    let formatted = DurationFormatter::new()
        .style(FormatStyle::Long)
        .format(duration)
        .unwrap();
    assert_eq!(formatted, "1 day 2 hours 3 minutes 4 seconds");
}

#[test]
fn test_format_clock_large_hours() {
    let duration = Duration::from_secs(360000); // 100 hours
    assert_eq!(format_clock(duration), "100:00:00");
}

#[test]
fn test_format_roundtrip_compatibility() {
    // These should format in a way that can be parsed back
    let durations = vec![
        Duration::from_secs(3600),
        Duration::from_secs(5400),
        Duration::from_secs(5445),
    ];

    for duration in durations {
        let formatted = format(duration);
        // Just verify it produces valid output
        assert!(!formatted.is_empty());
        assert!(formatted.chars().any(|c| c.is_alphabetic()));
    }
}