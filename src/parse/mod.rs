//! Parsing module for duration expressions.
//!
//! This module contains the tokenizer and grammar parser for converting
//! human-readable duration strings into `std::time::Duration` values.

pub mod grammar;
pub mod tokens;

pub use grammar::Parser;
pub use tokens::{Token, Tokenizer};