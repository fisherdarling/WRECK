use std::fmt;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::Chars;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    Kleene,
    Plus,
    Open,
    Close,
    Dot,
    Dash,
    Pipe,
    Char,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            TokenKind::Kleene => "kleene",
            TokenKind::Plus => "plus",
            TokenKind::Open => "open",
            TokenKind::Close => "close",
            TokenKind::Dot => "dot",
            TokenKind::Dash => "dash",
            TokenKind::Char => "char",
            TokenKind::Pipe => "pipe",
        };

        write!(f, "{}", val)
    }
}

#[derive(Debug, Clone)]
pub struct Lexer<'f> {
    chars: Chars<'f>,
    lookahead: Option<char>,
}

impl<'f> Lexer<'f> {
    pub fn eat(&mut self) -> Option<char> {
        self.lookahead = self.chars.next();
        self.lookahead
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.clone().peekable().peek().copied()
    }

    pub fn new(input: &'f str) -> Self {
        Self {
            chars: input.chars(),
            lookahead: None,
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        Some(match self.eat() {
            Some('*') => Token::new(TokenKind::Kleene, "*"),
            Some('+') => Token::new(TokenKind::Plus, "+"),
            Some('(') => Token::new(TokenKind::Open, "("),
            Some(')') => Token::new(TokenKind::Close, ")"),
            Some('.') => Token::new(TokenKind::Dot, "."),
            Some('-') => Token::new(TokenKind::Dash, "-"),
            Some('|') => Token::new(TokenKind::Pipe, "|"),
            Some('\\') => match self.eat() {
                Some('n') => Token::new(TokenKind::Char, "x0a"),
                Some('s') => Token::new(TokenKind::Char, "x20"),
                Some('\\') => Token::new(TokenKind::Char, "\\"),
                Some(c) => Token::new(TokenKind::Char, c),
                None => panic!("Backslash characters must escape another character."),
            },
            Some(c) => Token::new(TokenKind::Char, c),
            None => return None,
        })
    }

    pub fn iter(self) -> impl Iterator<Item = Token> + 'f {
        LexerIter { inner: self }
    }
}

pub struct LexerIter<'f> {
    pub inner: Lexer<'f>,
}

impl<'f> Iterator for LexerIter<'f> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub data: String,
}

impl Token {
    pub fn new(kind: TokenKind, data: impl ToString) -> Self {
        Self {
            kind,
            data: data.to_string(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.kind, self.data)
    }
}
