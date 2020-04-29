use std::fmt;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::Chars;

use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
struct Args {
    input_file: PathBuf,
}

fn main() {
    let args = Args::from_args();
    let regex = read_to_string(args.input_file).unwrap();

    for token in Lexer::new(&regex).iter() {
        println!("{}", token);
    }
}

#[derive(Debug, Clone)]
pub struct Lexer<'f> {
    chars: Chars<'f>,
    // chars: CharIndicies,
    lookahead: Option<char>,
    // position: usize,
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
            Some('*') => Token::new("kleene", "*"),
            Some('+') => Token::new("plus", "+"),
            Some('(') => Token::new("open", "("),
            Some(')') => Token::new("close", ")"),
            Some('.') => Token::new("dot", "."),
            Some('-') => Token::new("dash", "-"),
            Some('|') => Token::new("pipe", "|"),
            Some('\\') => match self.eat() {
                Some('n') => Token::new("char", "x0a"),
                Some('s') => Token::new("char", "x20"),
                Some('\\') => Token::new("char", "\\"),
                Some(c) => Token::new("char", c),
                None => panic!("Backslash characters must escape another character."),
            },
            Some(c) => Token::new("char", c),
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
    pub symbol: &'static str,
    pub data: String,
}

impl Token {
    pub fn new(symbol: &'static str, data: impl ToString) -> Self {
        Self {
            symbol,
            data: data.to_string(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.symbol, self.data)
    }
}
