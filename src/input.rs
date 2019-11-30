//! Helper module that allows to read input from a file and into a user-specified destination.
//!
//! For most implementations, the input is expected to consist of a list of values of the same type
//! separated by newlines, whitespace or a user-specified delimiter.
//!
//! # Examples
//! ```no_run
//! use aoc_util::input::{FileReader, FromFile};
//!
//! // Read file content directly into `String`
//! let string: String = FileReader::new()
//!     .read_from_file("string.txt")
//!     .unwrap();
//!
//! // Read newline-separated strings into `Vec<String>`
//! let strings: Vec<String> = FileReader::new()
//!     .split_lines()
//!     .read_from_file("string_input.txt")
//!     .unwrap();
//!
//! // Read newline-separated floating point numbers into `Vec<f64>`
//! let doubles: Vec<f64> = FileReader::new()
//!     .split_lines()
//!     .read_from_file("double_input.txt")
//!     .unwrap();
//! ```

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

/// Generic trait to read from file and into a destination of type `T`.
pub trait FromFile<T> {
    /// The error type
    type Error;

    /// Takes a file path and tries to read the file content into a destination of type `T`.
    fn read_from_file<P: AsRef<Path>>(&self, path: P) -> Result<T, Self::Error>;
}

#[derive(Debug)]
/// Generic error type that is returned by `FileReader` if it fails to read the input from file.
pub enum Error<E> {
    /// Returned if the specified file cannot be opened or read (e.g. invalid UTF-8).
    IoError(std::io::Error),
    /// Returned if the input cannot be parsed into the specified data type.
    ParseError(E),
    /// Returned if the input doesn't correspond to the expected format.
    FormatError(String),
}

impl<E: std::fmt::Display> std::fmt::Display for Error<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "{}", e),
            Error::ParseError(e) => write!(f, "{}", e),
            Error::FormatError(s) => write!(f, "{}", s),
        }
    }
}

impl<E> From<std::io::Error> for Error<E> {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

/// Read input from file.
#[derive(Default)]
pub struct FileReader {
    trim: bool,
}

impl FileReader {
    /// Create new `FileReader`.
    pub fn new() -> Self {
        Self { trim: false }
    }

    /// Trim whitespace at the beginning and end.
    pub fn trim(mut self) -> Self {
        self.trim = true;
        self
    }

    /// Split input at newlines.
    pub fn split_lines(self) -> SplitLines {
        SplitLines { trim: self.trim }
    }

    /// Split input at whitespace.
    pub fn split_whitespace(self) -> SplitWhitespace {
        SplitWhitespace { _private: () }
    }

    /// Split input at a specified delimiter.
    pub fn split_char(self, delimiter: char) -> SplitChar {
        SplitChar {
            trim: self.trim,
            delimiter,
        }
    }
}

/// Read input into a `String`.
impl FromFile<String> for FileReader {
    type Error = std::io::Error;

    /// Takes a file path and tries to read the file content into a `String`.
    ///
    /// # Failures
    /// Returns an error if the specified file cannot be opened or contains invalid UTF-8.
    fn read_from_file<P: AsRef<Path>>(&self, path: P) -> Result<String, Self::Error> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)?;

        if self.trim {
            buffer = buffer.trim().to_string();
        }

        Ok(buffer)
    }
}

/// Read input from file and split at newlines. Created using `FileReader::split_lines()`.
pub struct SplitLines {
    trim: bool,
}

impl SplitLines {
    /// Trim whitespace at the beginning and end.
    pub fn trim(mut self) -> Self {
        self.trim = true;
        self
    }
}

/// Read input into a `Vec<T>`. Input is assumed to be a list of values that can be parsed into `T`
/// that are separated by newlines.
impl<T> FromFile<Vec<T>> for SplitLines
where
    T: std::str::FromStr,
{
    type Error = Error<<T as std::str::FromStr>::Err>;

    /// Takes a file path and tries to read the file content into a destination of type `Vec<T>`.
    ///
    /// # Failures
    /// Returns an error if the specified file cannot be opened or contains invalid UTF-8.
    /// Also returns an error if the file contents cannot be parsed into values of type `T`.
    fn read_from_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<T>, Self::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|line| {
                if self.trim {
                    line?.trim().parse().map_err(Error::ParseError)
                } else {
                    line?.parse().map_err(Error::ParseError)
                }
            })
            .collect()
    }
}

/// Read input from file and split at whitespace. Created using `FileReader::split_whitespace()`.
pub struct SplitWhitespace {
    _private: (),
}

/// Read input into a `Vec<T>`. Input is assumed to be a list of values that can be parsed into `T`
/// that are separated by whitespace.
impl<T> FromFile<Vec<T>> for SplitWhitespace
where
    T: std::str::FromStr,
{
    type Error = Error<<T as std::str::FromStr>::Err>;

    /// Takes a file path and tries to read the file content into a destination of type `Vec<T>`.
    ///
    /// # Failures
    /// Returns an error if the specified file cannot be opened or contains invalid UTF-8.
    /// Also returns an error if the file contents cannot be parsed into values of type `T`.
    fn read_from_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<T>, Self::Error> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();

        reader.read_to_string(&mut buffer)?;

        buffer
            .split_whitespace()
            .map(|chunk| chunk.parse().map_err(Error::ParseError))
            .collect()
    }
}

/// Read input from file and split at a specified delimiter. Created using `FileReader::split_char()`.
pub struct SplitChar {
    trim: bool,
    delimiter: char,
}

impl SplitChar {
    /// Trim whitespace at the beginning and end.
    pub fn trim(mut self) -> Self {
        self.trim = true;
        self
    }
}

/// Read input into a `Vec<T>`. Input is assumed to be a list of values that can be parsed into `T`
/// that are separated by a specified delimiter.
impl<T> FromFile<Vec<T>> for SplitChar
where
    T: std::str::FromStr,
{
    type Error = Error<<T as std::str::FromStr>::Err>;

    /// Takes a file path and tries to read the file content into a destination of type `Vec<T>`.
    ///
    /// # Failures
    /// Returns an error if the specified file cannot be opened or contains invalid UTF-8.
    /// Also returns an error if the file contents cannot be parsed into values of type `T`.
    fn read_from_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<T>, Self::Error> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();

        reader.read_to_string(&mut buffer)?;

        buffer
            .split(self.delimiter)
            .map(|chunk| {
                if self.trim {
                    chunk.trim().parse().map_err(Error::ParseError)
                } else {
                    chunk.parse().map_err(Error::ParseError)
                }
            })
            .collect()
    }
}
