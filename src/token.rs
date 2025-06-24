//! This module defines the `Token` struct, which represents a token in the source code.

use crate::{source::Source, span::Span};

/// Represents a token in the source code.
pub struct Token<'a, T> {
    /// The kind of token
    pub kind: T,
    /// The span in the source code where the token is located.
    pub span: Span,
    /// The source code where the token is located.
    pub source: &'a Source<'a>,
}

impl<'a, T> Token<'a, T> {
    /// Creates a new `Token`.
    pub fn new(kind: T, span: Span, source: &'a Source<'a>) -> Self {
        Token { kind, span, source }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::Span;

    #[test]
    fn test_token_creation() {
        let source = Source::from_str("test_file.txt", "let x = 42;");
        let span = Span::new(0, 10);
        let token = Token::new("let", span, &source);

        assert_eq!(token.kind, "let");
        assert_eq!(token.span.start, 0);
        assert_eq!(token.span.end, 10);
        assert_eq!(token.source.code, "let x = 42;");
    }
}
