use std::error::Error;

#[derive(Debug)]
pub enum ParserError {
    BadInput,
    UnknownOperator,
    MissingRParen,
    FnMissingName,
    FnMissingParameters,
}

impl Error for ParserError {
    fn description(&self) -> &str {
        ParserError::BadInput           => "Illegal characters found in input stream",
        ParserError::UnknownOperator    => "Unknown operator",
        ParserError::MissingRParen      => "Expected ')",
        ParseError::FnMissingName       => "Function declaration is missing name",
        ParseError::FnMissingParameters => "Function declaration is missing parameters",
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}