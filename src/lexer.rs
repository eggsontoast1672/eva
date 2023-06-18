use std::{str::Chars, iter::Peekable};

#[derive(Debug)]
pub enum Token {
    // Single character
    ParenLeft,
    ParenRight,

    // Multiple characters
    Number(String),
    Symbol(String),
}

struct Lexer<'a> {
    current: usize,
    source: Peekable<Chars<'a>>,
    start: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: String) -> Self {
        Self {
            current: 0,
            source: source.chars().peekable(),
            start: 0,
        }
    }

    fn next_atom(&mut self) -> Token {
        let lexeme = self.source.take_while(|c| {
            *c != '(' && *c != ')' && !c.is_ascii_whitespace()
        }).collect::<String>();
        match lexeme.parse::<u64>() {
            Ok(_) => Token::Number(lexeme),
            Err(_) => Token::Symbol(lexeme),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.next()? {
            '(' => Some(Token::ParenLeft),
            ')' => Some(Token::ParenRight),
            byte if byte.is_ascii_whitespace() => self.next(),
            _ => Some(self.next_atom()),
        }
    }
}

pub fn tokenize(source: impl Into<String>) -> Vec<Token> {
    Lexer::new(source.into()).into_iter().collect()
}
