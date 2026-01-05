# timekit

**timekit** is a lightweight, pure Rust library for parsing and formatting
human-readable time durations with a predictable grammar and explicit error
handling.

It converts strings that humans naturally write—such as `1h 30m`, `2h30m`,
`45s`, or `00:12:05`—into `std::time::Duration`, and can also format durations
back into readable forms.

This crate is intentionally **language-centric**:
- No CLI
- No OS dependencies
- No security or async abstractions
- No panics

The focus is on **correctness, API clarity, and idiomatic Rust design**.

---

## Features

- Parse human-friendly durations into `std::time::Duration`
- Support for both unit-based and clock-style formats
- Strict, predictable grammar
- Explicit error types (no vague failures)
- Zero runtime panics
- Small, well-defined public API

---

## Supported Input Formats

### Unit-based
```text
1h 30m
2h30m
45s
1500ms
2d4h
