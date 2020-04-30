use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid Symbol: {0:?}")]
    SymbolParseError(String),
}
