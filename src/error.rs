//! This module provides error handling utilities.

use colored::*;

use crate::{
    source::Source,
    span::{Span, location_to_line_col},
};

/// Represents an advanced error.
pub struct Error<'a> {
    /// The error message describing the issue.
    message: String,
    /// The source code where the error occurred.
    source: &'a Source<'a>,
    /// The span in the source code where the error occurred.
    span: Span,
    /// The context of the error, if any.
    context: Vec<String>,
    /// Notes or additional information about the error.
    notes: Vec<String>,
}

impl<'a> Error<'a> {
    /// Creates a new `Error`
    pub fn new(message: String, source: &'a Source<'a>, span: Span) -> Self {
        Error {
            message,
            source,
            span,
            context: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// Creates a new `Error`, adding the given context to the error.
    pub fn with_context(mut self, context: String) -> Self {
        self.context.push(context);
        self
    }

    /// Creates a new `Error`, adding the given note to the error.
    pub fn with_note(mut self, note: String) -> Self {
        self.notes.push(note);
        self
    }

    /// Displays the error in a human-readable format.
    pub fn display(&self) {
        let (start_line, start_col) = location_to_line_col(&self.source.code, self.span.start);
        let (end_line, mut end_col) = location_to_line_col(&self.source.code, self.span.end);
        end_col -= 1;

        let number_of_spaces = start_line.max(end_line).to_string().len();

        eprintln!(
            "{}{} {}",
            "error".red().bold(),
            ":".bold(),
            self.message.bold()
        );

        if start_line == end_line {
            if start_col == end_col {
                eprintln!(
                    "{}{} {}:{}:{}",
                    " ".repeat(number_of_spaces),
                    "-->".cyan().bold(),
                    self.source.filename,
                    start_line,
                    start_col
                );
            } else {
                eprintln!(
                    "{}{} {}:{}:{}-{}",
                    " ".repeat(number_of_spaces),
                    "-->".cyan().bold(),
                    self.source.filename,
                    start_line,
                    start_col,
                    end_col
                );
            }
        } else {
            eprintln!(
                "{}{} {}:{}:{}-{}:{}",
                " ".repeat(number_of_spaces),
                "-->".cyan().bold(),
                self.source.filename,
                start_line,
                start_col,
                end_line,
                end_col
            );
        }

        let lines = self.source.code.lines().collect::<Vec<&str>>();
        let lines = lines
            .iter()
            .skip(start_line - 1)
            .take(end_line - start_line + 1);

        eprintln!("{} {}", " ".repeat(number_of_spaces), "|".cyan().bold());

        for (line_index, line) in lines.enumerate() {
            let line_number = start_line + line_index;

            if line_number == start_line && line_number == end_line {
                eprintln!(
                    "{}{} {} {}",
                    line_number.to_string().cyan().bold(),
                    " ".repeat(number_of_spaces - line_number.to_string().len()),
                    "|".cyan().bold(),
                    line
                );
                eprintln!(
                    "{} {} {}{}",
                    " ".repeat(number_of_spaces),
                    "|".cyan().bold(),
                    " ".repeat(start_col - 1),
                    "^".repeat(end_col - start_col + 1).red().bold()
                );
                continue;
            }

            eprintln!(
                "{}{} {} {}",
                line_number.to_string().cyan().bold(),
                " ".repeat(number_of_spaces - line_number.to_string().len()),
                "|".cyan().bold(),
                line
            );

            if line_number == start_line {
                eprintln!(
                    "{} {} {}{}",
                    " ".repeat(number_of_spaces),
                    "|".cyan().bold(),
                    " ".repeat(start_col - 1),
                    "^".repeat(line.len() - start_col + 1).red().bold()
                );
            } else if line_number == end_line {
                eprintln!(
                    "{} {} {}",
                    " ".repeat(number_of_spaces),
                    "|".cyan().bold(),
                    "^".repeat(end_col + 1).red().bold()
                );
            } else {
                eprintln!(
                    "{} {} {}",
                    " ".repeat(number_of_spaces),
                    "|".cyan().bold(),
                    "^".repeat(line.len()).red().bold()
                );
            }
        }

        if !self.context.is_empty() || !self.notes.is_empty() {
            eprintln!("{} {}", " ".repeat(number_of_spaces), "|".cyan().bold());
        }

        for context in self.context.iter() {
            eprintln!(
                "{} {} {}",
                " ".repeat(number_of_spaces),
                "=".cyan().bold(),
                context
            );
        }

        for note in self.notes.iter() {
            eprintln!(
                "{} {} {} {}",
                " ".repeat(number_of_spaces),
                "=".cyan().bold(),
                "note:".bold(),
                note
            );
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO: check stdout for expected output

    use super::*;
    use crate::span::Span;

    #[test]
    fn test_error_display_single_line() {
        let source = Source::from_str(
            "test.rs",
            "fn main() {\n    println!(\"Hello, world!\");\n}",
        );
        let span = Span { start: 12, end: 25 };
        let error = Error::new("Syntax error".to_string(), &source, span)
            .with_context("In function main".to_string())
            .with_note("Check the syntax".to_string());
        error.display();
    }

    #[test]
    fn test_error_display_multi_line() {
        let source = Source::from_str(
            "test.rs",
            "fn main() {\n    println!(\"Hello, world!\");\n}",
        );
        let span = Span { start: 12, end: 40 };
        let error = Error::new("Syntax error".to_string(), &source, span)
            .with_context("In function main".to_string())
            .with_note("Check the syntax".to_string());
        error.display();
    }
}
