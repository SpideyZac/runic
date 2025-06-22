//! This module defines the `Span` struct, which represents a span of text in a source file.
//! It also provides utilities for working with spans.

/// A `Span` represents a contiguous region in a source file, defined by its start and end byte indices.
#[derive(Debug)]
pub struct Span {
    /// The starting byte index of the span (inclusive).
    pub start: usize,
    /// The ending byte index of the span (exclusive).
    pub end: usize,
}

impl Span {
    /// Creates a new `Span` from the given start and end byte indices.
    ///
    /// # Panics
    ///
    /// Panics if `start` is greater than or equal to `end`.
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start < end, "Span start must be less than end");
        Span { start, end }
    }
}

/// Converts a byte index in the source string to a (line, column) tuple.
///
/// Lines and columns are 1-based.
///
/// Column of the newline character is + 1 of the last character in the line.
///
/// # Usage
///
/// ```rust
/// use runic::span::location_to_line_col;
///
/// let source = "Hello\nWorld";
/// let index = 6; // Byte index of 'W'
/// let (line, col) = location_to_line_col(source, index);
/// assert_eq!((line, col), (2, 1)); // 'W' is on line 2, column 1
/// ```
pub fn location_to_line_col(source: &str, index: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;

    for (i, c) in source.char_indices() {
        if i == index {
            break;
        }

        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    (line, col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_new() {
        let span = Span::new(5, 10);
        assert_eq!(span.start, 5);
        assert_eq!(span.end, 10);
    }

    #[test]
    #[should_panic(expected = "Span start must be less than end")]
    fn test_span_new_invalid() {
        Span::new(10, 5);
    }

    #[test]
    fn test_location_to_line_col() {
        let source = "Hello\nWorld";
        assert_eq!(location_to_line_col(source, 0), (1, 1)); // 'H'
        assert_eq!(location_to_line_col(source, 4), (1, 5)); // 'o'
        assert_eq!(location_to_line_col(source, 5), (1, 6)); // '\n'
        assert_eq!(location_to_line_col(source, 6), (2, 1)); // 'W'
        assert_eq!(location_to_line_col(source, 10), (2, 5)); // 'd'
    }
}
