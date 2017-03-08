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


    fn get_precedence(token: &Token) -> i32 {
        match *token {
            Token::Assign      => 10,
            Token::And         => 12,
            Token::LessThan    => 15,
            Token::GreaterThan => 15,
            Token::EqualTo     => 15,
            Token::NotEqualTo  => 15,
            Token::Plus        => 20,
            Token::Minus       => 20,
            Token::Slash       => 40,
            Token::Asterix     => 40,
            Token::Period      => 100,
            _ => -1,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if let Some(c) = self.read_char() {
            match c {
                '=' => {
                    match self.peek_char() {
                        Some(&'=') => {
                            self.read_char();
                            Some(Token::EqualTo)
                        }
                        
                        _ => {
                            Some(Token::Assign)
                        }
                    }
                }

                '+' => Some(Token::Plus),
                '-' => {
                    match self.peek_char() {
                        Some(&'>') => {
                            self.read_char();
                            Some(Token::Arrow)
                        }
                        
                        _ => {
                            Some(Token::Minus)
                        }
                    }
                },

                '~' => {
                    match self.peek_char() {
                        Some(&'=') => {
                            self.read_char();
                            Some(Token::NotEqualTo)
                        }
                        
                        _ => {
                            Some(Token::Denial)
                        }
                    }
                }

                '&' => {
                    match self.peek_char() {
                        Some(&'&') => {
                            self.read_char();
                            Some(Token::And)
                        }
                        
                        _ => {
                            Some(Token::BinaryAnd)
                        }
                    }
                }

                '*' => Some(Token::Asterix),
                '/' => Some(Token::Slash),
                '<' => Some(Token::LessThan),
                '>' => Some(Token::GreaterThan),
                ',' => Some(Token::Comma),
                '.' => Some(Token::Period),

                ':' => Some(Token::Colon),

                '(' => Some(Token::LParen),
                ')' => Some(Token::RParen),
                '{' => Some(Token::LBrace),
                '}' => Some(Token::RBrace),

                _   => {
                    if Self::is_letter_related(c) {
                        Some(Self::string_to_keyword(self.read_identifier(c)))
                    } else if c.is_digit(10) {
                        let number: String = self.read_number(c);

                        match number.parse::<i64>() {
                            Ok(int) => Some(Token::Integer(int)),
                            _       => {
                                match number.parse::<f64>() {
                                    Ok(float) => Some(Token::Float(float)),
                                    _         => panic!("Invalid number!")
                                }
                            }
                        }

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
            "true"   => Token::True,
            "false"  => Token::False,
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
    Integer(i64),
    Float(f64),

    Assign,
    Denial,
    Asterix,
    Minus,
    Plus,
    Slash,

    GreaterThan,
    LessThan,
    EqualTo,
    NotEqualTo,

    Comma,
    Colon,
    Arrow,

    LBrace,
    RBrace,
    LParen,
    RParen,

    Period,

    Function,
    Lambda,
    If,
    Unless,
    Else,
    Return,
    End,

    And,
    BinaryAnd,

    True,
    False,
}