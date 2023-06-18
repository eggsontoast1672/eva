#[derive(Debug)]
pub enum Token {
    // Single character
    ParenLeft,
    ParenRight,

    // Multiple characters
    Number(String),
    Symbol(String),
}

struct Lexer {
    current: usize,
    source: Vec<u8>,
    start: usize,
}

impl Lexer {
    fn new(source: String) -> Self {
        Self {
            current: 0,
            source: source.into_bytes(),
            start: 0,
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

    fn next_number(&mut self) -> String {
        while let Some(byte) = self.peek_byte() {
            if !Self::is_numeric(byte) {
                break;
            }
            self.next_byte();
        }
        String::from_utf8_lossy(&self.source[self.start..self.current]).to_string()
    }

    fn next_symbol(&mut self) -> String {
        todo!()
    }

    fn peek_byte(&self) -> Option<u8> {
        self.source.get(self.current).copied()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.start = self.current;
        match self.next_byte()? {
            b'(' => Some(Token::ParenLeft),
            b')' => Some(Token::ParenRight),
            byte if Self::is_numeric(byte) => Some(Token::Number(self.next_number())),
            byte if Self::is_whitespace(byte) => self.next(),
            _ => Some(Token::Symbol(self.next_symbol())),
        }
    }
}

pub fn tokenize(source: impl Into<String>) -> Vec<Token> {
    Lexer::new(source.into()).into_iter().collect()
}
