use parser::lexer::{Token, Lexer};
use parser::error::ParserError;

#[derive(Debug, Clone)]
pub enum Statement {
    If(Box<Expr>, Box<Statement>),
    IfElse(Box<Expr>, Box<Statement>, Box<Statement>),
    Var(String, Option<Box<Expr>>),
    Block(Box<Vec<Statement>>),
    Expr(Box<Expr>),
    Return,
    ReturnWithVal(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Text(String),
    Identifier(String),
    FnCall(String, Box<Vec<Expr>>),
    Dot(Box<Expr>, Box<Expr>),
    Index(String, Box<Expr>),
    Assignment(Box<Expr>, Box<Expr>),
    True,
    False,
}

fn parse_main<'a>(input: &mut Lexer<'a>) {
    if let Some(token) = input.next_token() {
        match token {
            Token::Integer(ref a)    => Ok(Expr::Integer(a.clone())),
            Token::Text(ref a)       => Ok(Expr::Text(a.clone())),
        }
    }
}