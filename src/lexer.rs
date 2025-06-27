//! This module defines the `Lexer` struct and the `LexerRule` trait.
//! The `Lexer` is responsible for tokenizing the source code based on the provided rules.
//! The `LexerRule` trait defines the interface for lexer rules that can be used to generate
//!
//! It also provides some utilities for common lexer rules.

// TODO: refactor

use crate::{error::Error, source::Source, token::Token};

/// A trait that defines the interface for lexer rules.
pub trait LexerRule<'a, T> {
    /// This method is called to get a token from the lexer.
    /// /// It should return `Ok(Some(token))` if a token is found,
    /// `Ok(None)` if no token is found,
    /// or `Err(error)` if an error occurs.
    ///
    /// If an error occurs, the lexer will stop processing and return the error.
    /// Otherwise, it will continue to the next rule.
    fn get_token(&self, lexer: &mut Lexer<'a, T>) -> Result<Option<Token<T>>, Error>;
    /// This method returns `true` if the rule generates a token,
    /// and `false` if it does not.
    ///
    /// This is used to determine whether the lexer should jump back to the previous position
    /// if the rule does not generate a token.
    ///
    /// For example, a rule that skips whitespace does not generate a token,
    /// so the lexer will jump back to the previous position if it does not find a token.
    fn generates_token(&self) -> bool {
        true
    }
}

/// A struct which is responsible for tokenizing the source code.
pub struct Lexer<'a, T> {
    /// The source code to be tokenized.
    pub source: &'a Source<'a>,
    /// The current position in the source code.
    pub position: usize,
    /// The current character being processed.
    pub current_char: Option<char>,
    /// The rules used to tokenize the source code.
    rules: Vec<Box<dyn LexerRule<'a, T>>>,
}

impl<'a, T> Lexer<'a, T> {
    /// Creates a new `Lexer` instance with the given source code and rules.
    pub fn new(source: &'a Source<'a>, rules: Vec<Box<dyn LexerRule<'a, T>>>) -> Self {
        let mut lexer = Lexer {
            source,
            position: 0,
            current_char: None,
            rules,
        };

        if lexer.position < lexer.source.code.len() {
            lexer.current_char = Some(lexer.source.code[lexer.position..].chars().next().unwrap());
        } else {
            lexer.current_char = None;
        }

        lexer
    }

    /// Advances the lexer to the next character in the source code.
    pub fn advance(&mut self) {
        if self.position < self.source.code.len() - 1 {
            self.position += 1;
            self.current_char = Some(self.source.code[self.position..].chars().next().unwrap());
        } else {
            self.current_char = None;
        }
    }

    /// Jumps to a specific position in the source code.
    pub fn jump_to(&mut self, position: usize) {
        if position < self.source.code.len() {
            self.position = position;
            self.current_char = Some(self.source.code[self.position..].chars().next().unwrap());
        } else {
            self.position = self.source.code.len() + 1;
            self.current_char = None;
        }
    }

    /// Attempts to get the next token from the lexer using the defined rules.
    ///
    /// If a token is found, it returns `Ok(Some(token))`.
    /// If no token is found, it returns `Ok(None)`.
    /// If an error occurs, it returns `Err(error)`.
    pub fn get_token(&mut self) -> Result<Option<Token<T>>, Error> {
        // TODO: refactor this to avoid using unsafe?

        let self_ptr = self as *mut Self;

        for rule in &self.rules {
            let prev_position = self.position;
            let token = unsafe { rule.get_token(&mut *self_ptr) }?;

            if let Some(token) = token {
                return Ok(Some(token));
            } else if rule.generates_token() {
                unsafe {
                    (*self_ptr).jump_to(prev_position);
                }
            }
        }

        Ok(None)
    }
}

/// This module provides utility functions and common lexer rules.
pub mod utils {
    use crate::lexer::LexerRule;

    /// A macro to create a vector of lexer rules.
    ///
    /// Example usage:
    /// ```rust
    /// use runic::lexer::{utils::SkipWhitespaceRule, utils::rules_vec};
    ///
    /// let rules = rules_vec![SkipWhitespaceRule]; // vec![Box::new(SkipWhitespaceRule)]
    /// ```
    #[allow(unused_macros)]
    macro_rules! rules_vec {
        ($($rule:expr),* $(,)?) => {
            vec![$(Box::new($rule) as Box<dyn LexerRule<'_, _>>),*]
        };
    }

    /// A lexer rule that skips whitespace characters.
    pub struct SkipWhitespaceRule;
    impl<'a, T> LexerRule<'a, T> for SkipWhitespaceRule {
        fn get_token(
            &self,
            lexer: &mut super::Lexer<'a, T>,
        ) -> Result<Option<crate::token::Token<T>>, crate::error::Error> {
            while let Some(c) = lexer.current_char {
                if c.is_whitespace() {
                    lexer.advance();
                } else {
                    break;
                }
            }
            Ok(None)
        }

        fn generates_token(&self) -> bool {
            false
        }
    }

    #[allow(unused_imports)]
    pub(crate) use rules_vec;

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{lexer::Lexer, source::Source};

        #[test]
        fn test_skip_whitespace_rule() {
            let source = Source::from_str("test_input.txt", "     let x = 10;");
            let rules = rules_vec![SkipWhitespaceRule];

            let mut lexer = Lexer::<String>::new(&source, rules);
            let token = lexer.get_token().unwrap();

            assert!(token.is_none());
            assert_eq!(lexer.position, 5);
            assert_eq!(lexer.current_char, Some('l'));
        }

        #[test]
        fn test_rules_vec_macro() {
            let rules: Vec<Box<dyn LexerRule<'_, String> + 'static>> =
                rules_vec![SkipWhitespaceRule];
            assert_eq!(rules.len(), 1);
            assert!(rules[0].generates_token() == false);
        }
    }

    // TODO: add more utils
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error::Error, source::Source, span::Span, token::Token};

    #[test]
    fn test_lexer_new() {
        let source = Source::from_str("test_input.txt", "let x = 10;");
        let rules = utils::rules_vec![utils::SkipWhitespaceRule];
        let lexer = Lexer::<u8>::new(&source, rules);

        assert_eq!(lexer.position, 0);
        assert_eq!(lexer.current_char, Some('l'));
    }

    #[test]
    fn test_lexer_advance() {
        let source = Source::from_str("test_input.txt", "let x = 10;");
        let rules = utils::rules_vec![utils::SkipWhitespaceRule];
        let mut lexer = Lexer::<u8>::new(&source, rules);

        lexer.advance();
        assert_eq!(lexer.position, 1);
        assert_eq!(lexer.current_char, Some('e'));
    }

    #[test]
    fn test_lexer_jump_to() {
        let source = Source::from_str("test_input.txt", "let x = 10;");
        let rules = utils::rules_vec![utils::SkipWhitespaceRule];
        let mut lexer = Lexer::<u8>::new(&source, rules);

        lexer.jump_to(4);
        assert_eq!(lexer.position, 4);
        assert_eq!(lexer.current_char, Some('x'));
    }

    #[test]
    fn test_lexer_get_token() {
        let source = Source::from_str("test_input.txt", "let x = 10;");

        struct TestRule;
        impl<'a> LexerRule<'a, String> for TestRule {
            fn get_token(
                &self,
                lexer: &mut Lexer<'a, String>,
            ) -> Result<Option<Token<String>>, Error> {
                if lexer.current_char == Some('l') {
                    lexer.advance();
                    Ok(Some(Token::new("let".to_string(), Span::new(0, 3))))
                } else {
                    Ok(None)
                }
            }
        }

        let rules = utils::rules_vec![utils::SkipWhitespaceRule, TestRule];
        let mut lexer = Lexer::<String>::new(&source, rules);
        let token = lexer.get_token().unwrap();

        assert!(token.is_some());

        let token = token.unwrap();

        assert_eq!(token.kind, "let");
        assert_eq!(token.span.start, 0);
        assert_eq!(token.span.end, 3);
    }
}
