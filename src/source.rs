//! This module defines the `Source` struct, which encapsulates source code and its associated filename.

/// Represents source code along with its filename.
#[derive(Debug)]
pub struct Source<'a> {
    /// The filename of the source code.
    pub filename: &'a str,
    /// The actual source code as a string.
    pub code: String,
}

impl<'a> Source<'a> {
    /// Creates a new `Source` instance, reading the source code from the given filename.
    pub fn new(filename: &'a str) -> Result<Self, std::io::Error> {
        let code = std::fs::read_to_string(filename)?;
        Ok(Source { filename, code })
    }

    /// Creates a new `Source` instance from a string slice.
    pub fn from_str(filename: &'a str, code: &'a str) -> Self {
        Source {
            filename,
            code: code.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_new() {
        let filename = "test_file.txt";
        let code = "fn main() { println!(\"Hello, world!\"); }";
        std::fs::write(filename, code).unwrap();

        let source = Source::new(filename).unwrap();
        assert_eq!(source.filename, filename);
        assert_eq!(source.code, code);

        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_source_from_str() {
        let filename = "test_file.txt";
        let code = "fn main() { println!(\"Hello, world!\"); }";
        let source = Source::from_str(filename, code);

        assert_eq!(source.filename, filename);
        assert_eq!(source.code, code);
    }
}
