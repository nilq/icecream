use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParserError {
    BadInput,
    UnknownOperator,
    MissingRParen,
    FnMissingName,
    FnMissingParameters,
    OutOfInput,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        match *self {
            ParserError::BadInput            => "Illegal characters found in input stream",
            ParserError::UnknownOperator     => "Unknown operator",
            ParserError::MissingRParen       => "Expected ')",
            ParserError::FnMissingName       => "Function declaration is missing name",
            ParserError::FnMissingParameters => "Function declaration is missing parameters",
            ParserError::OutOfInput           => "Parser fucked up reading of input stream"
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}