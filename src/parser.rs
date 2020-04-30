use crate::ast::{AstKind, AstNode};
use crate::{cfg::CFG, ll_table::LLTable, symbol::*};
use silly_lex::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parser<'c, 't> {
    cfg: &'c CFG,
    table: &'t LLTable<'c>,
}

impl<'c, 't> Parser<'c, 't> {
    pub fn new(cfg: &'c CFG, table: &'t LLTable<'c>) -> Self {
        Self { cfg, table }
    }

    pub fn parse(&self, stream: impl Iterator<Item = Token>) -> Option<AstNode> {
        let mut stream = stream.peekable();
        let mut stack: Vec<AstNode> = vec![AstNode::new(AstKind::Regex)];

        let mut lookahead: Option<&Token> = stream.peek();

        while let Some(node) = stack.pop() {
            let nt = NonTerminal::new(node.kind.as_nt()?);

            if let Some(token) = lookahead.take() {
                let t = Terminal::new(token.kind.to_string());
                let rule = self.table.table[&nt][&t].unwrap_or_else(|| panic!("Syntax Error"));
                let production = &self.cfg.productions[rule];

                for symbol in production.symbols().iter().rev() {
                    stack.push(AstNode::new(AstKind::from_str(symbol.as_str()).unwrap()));
                }
            }
        }

        None
    }
}
