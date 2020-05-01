use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

pub type Regex = String;
pub type ID = String;
pub type TokenOut = Option<String>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LexerConfig {
    pub alphabet: BTreeSet<char>,
    pub regexes: Vec<(Regex, ID, TokenOut)>,
}

impl LexerConfig {
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines().flatten().filter(|l| !l.is_empty());

        let mut config = LexerConfig::default();

        let alpha_line = lines.next().unwrap();
        let alphabet = parse_alphabet(alpha_line);

        println!("Alpha: {:?}", alphabet);

        config
    }
}

fn parse_alphabet(line: String) -> BTreeSet<char> {
    let mut chars = line.chars().filter(|c| !c.is_ascii_whitespace());

    let mut alpha = BTreeSet::new();

    while let Some(c) = decode_char(&mut chars) {
        alpha.insert(c);
    }

    alpha
}

fn decode_char(chars: &mut dyn Iterator<Item = char>) -> Option<char> {
    Some(match chars.next()? {
        'x' => {
            let first = chars.next()?;
            let second = chars.next()?;
            let mut first = first.to_string();
            first.push(second);

            println!("First: {}", first);
            let value: u32 = u32::from_str_radix(&first, 16).unwrap();
            println!("Value: {}", value);
            let char = std::char::from_digit(value, 10).unwrap();

            println!("{:?}", char);

            char
        }
        c => c,
    })
}
