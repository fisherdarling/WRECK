use crate::symbol::Symbol;
use derive_more::{From, Index};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Index, From)]
pub struct Production(Vec<Symbol>);

impl Production {
    pub fn contains_terminal(&self) -> bool {
        self.0.iter().any(|s| s.is_terminal())
    }

    pub fn only_lambda(&self) -> bool {
        self.0.len() == 1 && self.0[0] == Symbol::Lambda
    }

    pub fn symbols(&self) -> &[Symbol] {
        &self.0
    }
}
