use crate::error::Error;
use anyhow::{anyhow, Result};
// use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Index, Into};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SYMBOL: Regex =
        Regex::new(r#"(?P<lambda>lambda)|(?P<terminal>[a-z][a-z_]*)|(?P<nonterminal>[A-Z][a-zA-Z_]*)|(?P<dollar>\$)"#)
            .unwrap();
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Terminal(String);

impl Terminal {
    pub fn new(i: impl Into<String>) -> Self {
        Self(i.into())
    }

    pub fn terminal(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Terminal {
    fn as_ref(&self) -> &str {
        self.terminal()
    }
}

impl From<String> for Terminal {
    fn from(e: String) -> Self {
        Self(e)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonTerminal(String);

impl NonTerminal {
    pub fn new(i: impl Into<String>) -> Self {
        Self(i.into())
    }

    pub fn non_terminal(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for NonTerminal {
    fn as_ref(&self) -> &str {
        self.non_terminal()
    }
}

impl From<String> for NonTerminal {
    fn from(e: String) -> Self {
        Self(e)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
    Lambda,
}

impl Symbol {
    pub fn from_parse(input: &str) -> Result<Self> {
        if let Some(captures) = SYMBOL.captures(input) {
            if let Some(terminal) = captures.name("terminal") {
                return Ok(Symbol::from_terminal(terminal.as_str().to_string()));
            } else if let Some(nonterminal) = captures.name("nonterminal") {
                return Ok(Symbol::from_non_terminal(nonterminal.as_str().to_string()));
            } else if let Some(dollar) = captures.name("dollar") {
                return Ok(Symbol::from_terminal(dollar.as_str().to_string()));
            } else if let Some(_) = captures.name("lambda") {
                return Ok(Symbol::Lambda);
            }
        }

        Err(anyhow!("Invalid Symbol: {:?}", input))
    }

    pub fn from_terminal(t: impl Into<Terminal>) -> Self {
        Self::Terminal(t.into())
    }

    pub fn from_non_terminal(nt: impl Into<NonTerminal>) -> Self {
        Self::NonTerminal(nt.into())
    }

    pub fn terminal(&self) -> Result<&Terminal> {
        if let Symbol::Terminal(ref t) = self {
            Ok(t)
        } else {
            Err(anyhow!("Symbol {:?} is not a Terminal", self))
        }
    }

    pub fn is_terminal(&self) -> bool {
        if let Symbol::Terminal(_) = self {
            true
        } else {
            false
        }
    }

    pub fn non_terminal(&self) -> Result<&NonTerminal> {
        if let Symbol::NonTerminal(ref t) = self {
            Ok(t)
        } else {
            Err(anyhow!("Symbol {:?} is not a NonTerminal", self))
        }
    }

    pub fn is_non_terminal(&self) -> bool {
        if let Symbol::NonTerminal(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_lambda(&self) -> bool {
        Symbol::Lambda == *self
    }
}

// use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Index, Into};

// #[derive(Default, Debug, Copy, Clone, AsMut, AsRef, Deref, DerefMut, From, Into)]
// pub struct SymbolIdx(usize);

// #[derive(Default, Debug, Copy, Clone, AsMut, AsRef, Deref, DerefMut, From, Into)]
// pub struct TerminalIdx(usize);

// #[derive(Default, Debug, Copy, Clone, AsMut, AsRef, Deref, DerefMut, From, Into)]
// pub struct NonTerminalIdx(usize);
