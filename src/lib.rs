use silly_lex::{Lexer, Token};

pub mod ast;
pub mod cfg;
pub mod error;
pub mod ll_table;
pub mod nfa_generator;
pub mod production;
pub mod symbol;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
