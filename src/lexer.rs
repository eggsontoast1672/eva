#[derive(Debug)]
pub enum Token {
    // Single character
    Minus,
    ParenLeft,
    ParenRight,
    Plus,
    Slash,
    Star,

    // Multiple characters
    Number(u64),
}

struct Lexer {
    current: usize,
    source: Vec<u8>,
}

impl Lexer {
    fn new(source: String) -> Self {
        Self {
            current: 0,
            source: source.into_bytes(),
        }
    }

    fn is_numeric(byte: u8) -> bool {
        byte.is_ascii_digit()
    }

    fn is_whitespace(byte: u8) -> bool {
        byte.is_ascii_whitespace()
    }

    fn next_byte(&mut self) -> Option<u8> {
        let byte = self.source.get(self.current).copied();
        self.current += 1;
        byte
    }

    fn next_number(&mut self) -> Token {
        todo!()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_byte()? {
            b'-' => Some(Token::Minus),
            b'(' => Some(Token::ParenLeft),
            b')' => Some(Token::ParenRight),
            b'+' => Some(Token::Plus),
            b'/' => Some(Token::Slash),
            b'*' => Some(Token::Star),
            byte if Self::is_numeric(byte) => Some(self.next_number()),
            byte if Self::is_whitespace(byte) => self.next(),
            byte => panic!("unrecognized byte '{}'", byte as char),
        }
    }
}

pub fn tokenize(source: impl Into<String>) -> Vec<Token> {
    Lexer::new(source.into()).into_iter().collect()
}
