use super::errors::*;

use std::io::{self, BufRead};

#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenKind {
    // Single-character tokens:
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens:
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals:
    Identifier(String),
    StringLiteral(String),
    Number(f64),

    // Keywords:
    And,
    Class,
    Else,
    Fun,
    For,
    If,
    Nil,
    Or,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub line: u32,
    pub column: u32,
}

#[derive(Default)]
pub struct Tokenizer {
    _literal_head: Option<String>,
}

impl Tokenizer {
    pub fn iter_tokens<'a, R: BufRead>(&'a mut self, input: &'a mut R) -> TokenIterator<'a, R> {
        TokenIterator {
            _tokenizer: self,
            input,
            sealed: false,
        }
    }
}

pub struct TokenIterator<'a, R: BufRead> {
    _tokenizer: &'a mut Tokenizer,
    input: &'a mut R,
    sealed: bool,
}

impl<'a, R: BufRead> TokenIterator<'a, R> {
    fn read_byte(&mut self) -> io::Result<Option<u8>> {
        let mut byte = [0u8; 1];
        match self.input.read(&mut byte[..])? {
            0 => Ok(None),
            _ => Ok(Some(byte[0])),
        }
    }

    fn peek_byte(&mut self) -> io::Result<Option<u8>> {
        let buf = self.input.fill_buf()?;
        Ok(buf.first().cloned())
    }

    fn skip_whitespace(&mut self) -> io::Result<()> {
        loop {
            let buf = self.input.fill_buf()?;
            if buf.is_empty() {
                return Ok(());
            }
            for (ind, &byte) in buf.iter().enumerate() {
                if byte != b' ' && byte != b'\n' && byte != b'\t' {
                    self.input.consume(ind);
                    return Ok(());
                }
            }
            let len = buf.len();
            self.input.consume(len);
        }
    }

    fn match_byte(&mut self, byte: u8) -> io::Result<bool> {
        match self.peek_byte()? {
            None => Ok(false),
            Some(x) => {
                if x == byte {
                    self.input.consume(1);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }

    fn read_token(&mut self) -> LoxResult<Option<Token>> {
        self.skip_whitespace()?;
        let byte = match self.read_byte()? {
            None => return Ok(None), // TODO: handle unfinished string literal
            Some(byte) => byte,
        };
        let kind = match byte {
            b'(' => TokenKind::LeftParen,
            b')' => TokenKind::RightParen,
            b'{' => TokenKind::LeftBrace,
            b'}' => TokenKind::RightBrace,
            b',' => TokenKind::Comma,
            b'.' => TokenKind::Dot,
            b'-' => TokenKind::Minus,
            b'+' => TokenKind::Plus,
            b';' => TokenKind::Semicolon,
            b'*' => TokenKind::Star,
            b'!' => {
                if self.match_byte(b'=')? {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }
            b'=' => {
                if self.match_byte(b'=')? {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }
            b'<' => {
                if self.match_byte(b'=')? {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            b'>' => {
                if self.match_byte(b'=')? {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            _ => bail!(LoxErrorKind::SyntaxError("unexpected character".to_owned())),
        };
        let token = Token {
            kind,
            line: 0,
            column: 0,
        };
        Ok(Some(token))
    }
}

impl<'a, R: BufRead> Iterator for TokenIterator<'a, R> {
    type Item = LoxResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sealed {
            return None;
        }
        let result = self.read_token();
        if result.is_err() {
            self.sealed = true;
        }
        result.transpose()
    }
}
