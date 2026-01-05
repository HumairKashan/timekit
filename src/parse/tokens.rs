//! Tokenization for duration strings.
//!
//! This module breaks down input strings into a stream of tokens
//! that can be consumed by the grammar parser.

use crate::error::{Error, Result};

/// A token in a duration expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// A numeric value (e.g., "123", "45")
    Number(u64),
    /// A time unit (e.g., "h", "min", "seconds")
    Unit(String),
    /// A colon separator for clock format
    Colon,
    /// End of input
    Eof,
}

/// A tokenizer that produces tokens from an input string.
#[derive(Debug)]
pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
}

impl Tokenizer {
    /// Creates a new tokenizer for the given input string.
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    /// Returns the current position in the input.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Peeks at the current character without consuming it.
    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    /// Consumes and returns the current character.
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += 1;
        Some(ch)
    }

    /// Skips whitespace characters.
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Parses a number token.
    fn parse_number(&mut self) -> Result<Token> {
        let start_pos = self.position;
        let mut num_str = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        num_str.parse::<u64>().map(Token::Number).map_err(|e| {
            Error::InvalidNumber {
                value: num_str,
                position: start_pos,
                reason: e.to_string(),
            }
        })
    }

    /// Parses a unit token.
    fn parse_unit(&mut self) -> Result<Token> {
        let mut unit = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_alphabetic() {
                unit.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        Ok(Token::Unit(unit))
    }

    /// Returns the next token from the input.
    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        match self.peek() {
            None => Ok(Token::Eof),
            Some(':') => {
                self.advance();
                Ok(Token::Colon)
            }
            Some(ch) if ch.is_ascii_digit() => self.parse_number(),
            Some(ch) if ch.is_alphabetic() => self.parse_unit(),
            Some(ch) => Err(Error::InvalidCharacter {
                character: ch,
                position: self.position,
            }),
        }
    }

    /// Collects all tokens from the input.
    pub fn tokenize(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let tokens = Tokenizer::new("1h 30m").tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Number(1),
            Token::Unit("h".to_string()),
            Token::Number(30),
            Token::Unit("m".to_string()),
            Token::Eof,
        ]);
    }

    #[test]
    fn test_tokenize_no_spaces() {
        let tokens = Tokenizer::new("2h30m45s").tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Number(2),
            Token::Unit("h".to_string()),
            Token::Number(30),
            Token::Unit("m".to_string()),
            Token::Number(45),
            Token::Unit("s".to_string()),
            Token::Eof,
        ]);
    }

    #[test]
    fn test_tokenize_clock_format() {
        let tokens = Tokenizer::new("01:30:45").tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Number(1),
            Token::Colon,
            Token::Number(30),
            Token::Colon,
            Token::Number(45),
            Token::Eof,
        ]);
    }

    #[test]
    fn test_invalid_character() {
        let result = Tokenizer::new("1h@30m").tokenize();
        assert!(matches!(result, Err(Error::InvalidCharacter { .. })));
    }

    #[test]
    fn test_number_overflow() {
        let result = Tokenizer::new("999999999999999999999999h").tokenize();
        assert!(matches!(result, Err(Error::InvalidNumber { .. })));
    }
}