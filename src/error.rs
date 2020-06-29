use std::error;
use std::fmt;
use std::num::ParseIntError;
use time::ComponentRangeError;

type Result<T> = std::result::Result<T, ParserError>;

/// An error type indicating that a component provided to a parser method was invalid, causing a failure.
#[derive(Debug, PartialEq)]
pub enum ParserError {
    // defer to the FromStr api for more information
    StringIsNotNum(ParseIntError),
    // defer to the time crate for more information
    RangeError(ComponentRangeError),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParserError::StringIsNotNum(ref e) => write!(
                f,
                "Make sure that the given string only contains numerical values."
            ),
            ParserError::RangeError(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for ParserError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            ParserError::StringIsNotNum(ref e) => Some(e),
            ParserError::RangeError(ref e) => Some(e),
        }
    }
}

impl From<ComponentRangeError> for ParserError {
    fn from(err: ComponentRangeError) -> ParserError {
        ParserError::RangeError(err)
    }
}

impl From<ParseIntError> for ParserError {
    fn from(err: ParseIntError) -> ParserError {
        ParserError::StringIsNotNum(err)
    }
}
