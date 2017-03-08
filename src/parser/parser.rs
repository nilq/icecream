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
    IntConst(i64),
    Identifier(String),
    FnCall(String, Box<Vec<Expr>>),
    Dot(Box<Expr>, Box<Expr>),
    Index(String, Box<Expr>),
    Assignment(Box<Expr>, Box<Expr>),
    True,
    False,
}
