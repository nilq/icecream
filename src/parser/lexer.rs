use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn read_identifier(&mut self, c: char) -> String {
        let mut ident = String::new();

        ident.push(c);

        while let Some(&c) = self.peek_char() {
            if Self::is_letter_related(c) {
                ident.push(self.read_char().unwrap());
            } else {
                break;
            }
        }

        ident
    }

    fn read_number(&mut self, c: char) -> String {
        let mut number = String::new();

        number.push(c);

        while let Some(&c) = self.peek_char() {
            if Self::is_number_related(c, number.len() > 0) {
                number.push(self.read_char().unwrap());
            } else {
                break;
            }
        }

        number
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if let Some(c) = self.read_char() {
            match c {
                '=' => {
                    if let Some(&'=') = self.peek_char() {
                        let _ = self.read_char();
                        Some(Token::Eq)
                    } else {
                        Some(Token::Assign)
                    }
                }

                '+' => Some(Token::Plus),
                '-' => {
                    if let Some(&'>') = self.peek_char() {
                        let _ = self.read_char();
                        Some(Token::Arrow)
                    } else {
                        Some(Token::Minus)
                    }
                },

                '~' => {
                    if let Some(&'=') = self.peek_char() {
                        let _ = self.read_char();
                        Some(Token::NotEq)
                    } else {
                        Some(Token::Denial)
                    }
                }

                '*' => Some(Token::Asterix),
                '/' => Some(Token::Slash),
                '<' => Some(Token::Lt),
                '>' => Some(Token::Gt),
                ',' => Some(Token::Comma),

                ':' => Some(Token::Colon),

                '(' => Some(Token::LParen),
                ')' => Some(Token::RParen),
                '{' => Some(Token::LBrace),
                '}' => Some(Token::RBrace),

                _   => {
                    if Self::is_letter_related(c) {
                        Some(Self::string_to_keyword(self.read_identifier(c)))
                    } else if c.is_digit(10) {
                        Some(Token::Integer(self.read_number(c)))
                    } else {
                        Some(Token::Fucked(c))
                    }
                }
            }
        } else {
            None
        }
    }

    fn is_letter_related(c: char) -> bool {
        c.is_alphabetic() || c == '_'
                          || c == '?'
                          || c == '!'
    }

    fn is_number_related(c: char, in_num: bool) -> bool {
        c.is_digit(10) || (c == '.' && in_num)
    }

    fn string_to_keyword(id: String) -> Token {
        match id.as_str() {
            "func"   => Token::Function,
            "lambda" => Token::Lambda,
            "if"     => Token::If,
            "unless" => Token::Unless,
            "else"   => Token::Else,
            "return" => Token::Return,
            "end"    => Token::End,
            _        => Token::Identifier(id),
        }
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() {
                let _ = self.read_char();
            } else {
                break;
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Fucked(char),

    Identifier(String),
    Integer(String),

    Assign,
    Denial,
    Asterix,
    Minus,
    Plus,
    Slash,

    Gt,
    Lt,
    Eq,
    NotEq,

    Comma,
    Colon,
    Arrow,

    LBrace,
    RBrace,
    LParen,
    RParen,

    Function,
    Lambda,
    If,
    Unless,
    Else,
    Return,
    End,
}