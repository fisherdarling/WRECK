use crate::symbol::Symbol;
use derive_more::{From, Index};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Index, From)]
pub struct Production(Vec<Symbol>);
