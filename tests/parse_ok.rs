//! Integration tests for successful duration parsing.

use std::time::Duration;
use timekit::parse;

#[test]
fn test_parse_hours() {
    assert_eq!(parse("1h").unwrap(), Duration::from_secs(3600));
    assert_eq!(parse("2h").unwrap(), Duration::from_secs(7200));
    assert_eq!(parse("24h").unwrap(), Duration::from_secs(86400));
}

#[test]
fn test_parse_minutes() {
    assert_eq!(parse("1m").unwrap(), Duration::from_secs(60));
    assert_eq!(parse("30m").unwrap(), Duration::from_secs(1800));
    assert_eq!(parse("90m").unwrap(), Duration::from_secs(5400));
}

#[test]
fn test_parse_seconds() {
    assert_eq!(parse("1s").unwrap(), Duration::from_secs(1));
    assert_eq!(parse("45s").unwrap(), Duration::from_secs(45));
    assert_eq!(parse("3600s").unwrap(), Duration::from_secs(3600));
}

#[test]
fn test_parse_milliseconds() {
    assert_eq!(parse("1ms").unwrap(), Duration::from_millis(1));
    assert_eq!(parse("500ms").unwrap(), Duration::from_millis(500));
    assert_eq!(parse("1500ms").unwrap(), Duration::from_millis(1500));
}

#[test]
fn test_parse_microseconds() {
    assert_eq!(parse("1us").unwrap(), Duration::from_micros(1));
    assert_eq!(parse("500us").unwrap(), Duration::from_micros(500));
    assert_eq!(parse("1000us").unwrap(), Duration::from_millis(1));
}

#[test]
fn test_parse_nanoseconds() {
    assert_eq!(parse("1ns").unwrap(), Duration::from_nanos(1));
    assert_eq!(parse("500ns").unwrap(), Duration::from_nanos(500));
    assert_eq!(parse("1000ns").unwrap(), Duration::from_micros(1));
}

#[test]
fn test_parse_days() {
    assert_eq!(parse("1d").unwrap(), Duration::from_secs(86400));
    assert_eq!(parse("7d").unwrap(), Duration::from_secs(604800));
    assert_eq!(parse("30d").unwrap(), Duration::from_secs(2592000));
}

#[test]
fn test_parse_combined_with_spaces() {
    assert_eq!(parse("1h 30m").unwrap(), Duration::from_secs(5400));
    assert_eq!(parse("2h 15m 30s").unwrap(), Duration::from_secs(8130));
    assert_eq!(parse("1d 12h 30m").unwrap(), Duration::from_secs(131400));
}

#[test]
fn test_parse_combined_no_spaces() {
    assert_eq!(parse("1h30m").unwrap(), Duration::from_secs(5400));
    assert_eq!(parse("2h15m30s").unwrap(), Duration::from_secs(8130));
    assert_eq!(parse("1h30m45s").unwrap(), Duration::from_secs(5445));
}

#[test]
fn test_parse_long_unit_names() {
    assert_eq!(parse("1 hour").unwrap(), Duration::from_secs(3600));
    assert_eq!(parse("2 hours").unwrap(), Duration::from_secs(7200));
    assert_eq!(parse("30 minutes").unwrap(), Duration::from_secs(1800));
    assert_eq!(parse("45 seconds").unwrap(), Duration::from_secs(45));
}

#[test]
fn test_parse_alternative_units() {
    assert_eq!(parse("1hr").unwrap(), Duration::from_secs(3600));
    assert_eq!(parse("30min").unwrap(), Duration::from_secs(1800));
    assert_eq!(parse("45sec").unwrap(), Duration::from_secs(45));
}

#[test]
fn test_parse_clock_mm_ss() {
    assert_eq!(parse("01:30").unwrap(), Duration::from_secs(90));
    assert_eq!(parse("05:45").unwrap(), Duration::from_secs(345));
    assert_eq!(parse("59:59").unwrap(), Duration::from_secs(3599));
}

#[test]
fn test_parse_clock_hh_mm_ss() {
    assert_eq!(parse("01:30:45").unwrap(), Duration::from_secs(5445));
    assert_eq!(parse("02:15:30").unwrap(), Duration::from_secs(8130));
    assert_eq!(parse("23:59:59").unwrap(), Duration::from_secs(86399));
}

#[test]
fn test_parse_clock_no_leading_zeros() {
    assert_eq!(parse("1:30").unwrap(), Duration::from_secs(90));
    assert_eq!(parse("1:30:45").unwrap(), Duration::from_secs(5445));
    assert_eq!(parse("0:05").unwrap(), Duration::from_secs(5));
}

#[test]
fn test_parse_zero_values() {
    assert_eq!(parse("0s").unwrap(), Duration::ZERO);
    assert_eq!(parse("0h 0m 0s").unwrap(), Duration::ZERO);
    assert_eq!(parse("0:00").unwrap(), Duration::ZERO);
    assert_eq!(parse("00:00:00").unwrap(), Duration::ZERO);
}

#[test]
fn test_parse_large_values() {
    assert_eq!(parse("1000h").unwrap(), Duration::from_secs(3600000));
    assert_eq!(parse("365d").unwrap(), Duration::from_secs(31536000));
    assert_eq!(parse("10000s").unwrap(), Duration::from_secs(10000));
}

#[test]
fn test_parse_case_insensitive() {
    assert_eq!(parse("1H").unwrap(), Duration::from_secs(3600));
    assert_eq!(parse("30M").unwrap(), Duration::from_secs(1800));
    assert_eq!(parse("45S").unwrap(), Duration::from_secs(45));
    assert_eq!(parse("1HOUR").unwrap(), Duration::from_secs(3600));
}

#[test]
fn test_parse_whitespace_handling() {
    assert_eq!(parse("  1h  ").unwrap(), Duration::from_secs(3600));
    assert_eq!(parse("1h  30m").unwrap(), Duration::from_secs(5400));
    assert_eq!(parse("  1h  30m  45s  ").unwrap(), Duration::from_secs(5445));
}

#[test]
fn test_parse_mixed_subseconds() {
    assert_eq!(
        parse("1s 500ms").unwrap(),
        Duration::from_millis(1500)
    );
    assert_eq!(
        parse("2s 250ms 500us").unwrap(),
        Duration::new(2, 250_500_000)
    );
}

#[test]
fn test_parse_all_units_combined() {
    let result = parse("1d 2h 3m 4s 5ms 6us 7ns").unwrap();
    assert_eq!(result.as_secs(), 93784);
    assert_eq!(result.subsec_nanos(), 5_006_007);
}

#[test]
fn test_parse_complex_durations() {
    assert_eq!(parse("1h30m45s500ms").unwrap(), Duration::from_millis(5445500));
    assert_eq!(parse("2d 12h").unwrap(), Duration::from_secs(216000));
    assert_eq!(parse("90m 30s").unwrap(), Duration::from_secs(5430));
}

#[test]
fn test_parse_singular_vs_plural() {
    assert_eq!(parse("1 second").unwrap(), Duration::from_secs(1));
    assert_eq!(parse("2 seconds").unwrap(), Duration::from_secs(2));
    assert_eq!(parse("1 minute").unwrap(), Duration::from_secs(60));
    assert_eq!(parse("2 minutes").unwrap(), Duration::from_secs(120));
}