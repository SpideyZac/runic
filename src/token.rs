//! This module defines the `Token` struct, which represents a token in the source code.

use crate::span::Span;

/// Represents a token in the source code.
#[derive(Debug)]
pub struct Token<T> {
    /// The kind of token
    pub kind: T,
    /// The span in the source code where the token is located.
    pub span: Span,
}

impl<T> Token<T> {
    /// Creates a new `Token`.
    pub fn new(kind: T, span: Span) -> Self {
        Token { kind, span }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::Span;

    #[test]
    fn test_token_creation() {
        let span = Span::new(0, 10);
        let token = Token::new("let", span);

        assert_eq!(token.kind, "let");
        assert_eq!(token.span.start, 0);
        assert_eq!(token.span.end, 10);
    }
}
