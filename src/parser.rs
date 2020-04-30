#![allow(unused)]

use crate::ast::{AstKind, AstNode};
use crate::{cfg::CFG, ll_table::LLTable, symbol::*};
use derive_more::*;
use silly_lex::{Token, TokenKind};
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum ParseTree {
    Marker,
    Node(AstNode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parser<'c, 't> {
    cfg: &'c CFG,
    table: &'t LLTable<'c>,
}

impl<'c, 't> Parser<'c, 't> {
    pub fn new(cfg: &'c CFG, table: &'t LLTable<'c>) -> Self {
        Self { cfg, table }
    }

    pub fn parse(&self, mut stream: &mut Peekable<impl Iterator<Item = Token>>) -> AstNode {
        let mut root = NonTerminal::new("RE");
        self.parse_non_terminal(stream, &root)
    }

    fn parse_symbol(
        &self,
        stream: &mut Peekable<impl Iterator<Item = Token>>,
        symbol: &Symbol,
    ) -> Option<AstNode> {
        match symbol {
            Symbol::Terminal(t) => self.parse_terminal(stream, t),
            Symbol::NonTerminal(nt) => Some(self.parse_non_terminal(stream, nt)),
            Symbol::Lambda => None,
        }
    }

    fn parse_terminal(
        &self,
        stream: &mut Peekable<impl Iterator<Item = Token>>,
        terminal: &Terminal,
    ) -> Option<AstNode> {
        println!("Parsing Terminal: {:?} <- {:?}", terminal, stream.peek());

        if let Some(token) = stream.peek() {
            if token.kind.to_string() == terminal.terminal() {
                let next = stream.next()?;

                return if terminal.terminal() == "char" {
                    let node = AstNode::new(AstKind::Char(next.data.chars().next().unwrap()));

                    Some(node)
                } else {
                    // If we should make a node from it (e.g. Kleene or Plus) then do it,
                    // else return None
                    Some(AstNode::new(AstKind::from_str(terminal.terminal())?))
                };
            }
        }

        None
    }

    fn parse_non_terminal(
        &self,
        mut stream: &mut Peekable<impl Iterator<Item = Token>>,
        non_terminal: &NonTerminal,
    ) -> AstNode {
        println!("Parsing NT: {:?} <- {:?}", non_terminal, stream.peek());

        let mut node = AstNode::new(
            AstKind::from_str(non_terminal.non_terminal()).expect("Expected a non_terminal"),
        );

        if let Some(token) = stream.peek() {
            // let nt = NonTerminal::new(token.kind.to_string());
            let terminal = Terminal::new(token.kind.to_string());

            if let Some(p) = self.table.table[non_terminal][&terminal] {
                for symbol in self.cfg.productions[p].symbols() {
                    if let Some(new_node) = self.parse_symbol(&mut stream, symbol) {
                        node.children.push(new_node);
                    }
                }
            } else {
                node.children.push(AstNode::new(AstKind::Lambda));
            }
            // else {
            //     panic!(format!(
            //         "Syntax Error, No Transition: [{:?}][{:?}]",
            //         non_terminal, terminal
            //     ));
            // }
        }

        node
    }
    // pub fn parse(&self, stream: impl Iterator<Item = Token>) -> Option<AstNode> {
    //     let mut stream = stream;
    //     let mut stack: Vec<ParseTree> = vec![ParseTree::from(AstNode::new(AstKind::Regex))];

    //     let mut root = AstNode::new(AstKind::Regex);
    //     let mut current: &mut AstNode = &mut root;

    //     // while let Some(node) = stack.pop() {
    //     //     if let ParseTree::Node(node) = node {
    //     //         if node.kind.is_non_terminal() {
    //     //             let nt = NonTerminal::new(node.kind.as_nt()?);

    //     //             if let Some(token) = stream.peek() {
    //     //                 let t = Terminal::new(token.kind.to_string());
    //     //                 let rule =
    //     //                     self.table.table[&nt][&t].unwrap_or_else(|| panic!("Syntax Error"));
    //     //                 let production = &self.cfg.productions[rule];

    //     //                 stack.push(ParseTree::Marker);

    //     //                 for symbol in production.symbols().iter().rev() {
    //     //                     stack.push(ParseTree::from(AstNode::new(
    //     //                         AstKind::from_str(symbol.as_str()).unwrap(),
    //     //                     )));
    //     //                 }
    //     //             }

    //     //             let node = AstNode::new(AstKind::from_str(nt.non_terminal()).unwrap());
    //     //             current.children.push(node);
    //     //             let last = current.children.last_mut().unwrap();
    //     //             current = last;
    //     //         }
    //     //     }
    //     // }

    //     // while let Some(node) = stack.pop() {
    //     //     if node.kind.is_terminal() {
    //     //         // If the node is a terminal, we need to read from input stream.
    //     //         let next_token = stream.next().expect("Handle EOI");

    //     //         if next_token.kind.to_string() == node.kind.as_str() {
    //     //             continue;
    //     //         } else {
    //     //             panic!(format!(
    //     //                 "Syntax Error: {:?} != {:?}",
    //     //                 next_token.kind, node.kind
    //     //             ));
    //     //         }
    //     //     //

    //     //     // if next_token.kind.to_string()
    //     //     } else {
    //     //         let nt = NonTerminal::new(node.kind.as_nt()?);

    //     //         if let Some(token) = stream.peek() {
    //     //             let t = Terminal::new(token.kind.to_string());
    //     //             let rule = self.table.table[&nt][&t].unwrap_or_else(|| panic!("Syntax Error"));
    //     //             let production = &self.cfg.productions[rule];

    //     //             for symbol in production.symbols().iter().rev() {
    //     //                 stack.push(AstNode::new(AstKind::from_str(symbol.as_str()).unwrap()));
    //     //             }
    //     //         }
    //     //     }
    //     // }

    //     None
    // }
}

// if let Some(token) = stream.peek() {
//     let nt = NonTerminal::new("RE");
//     let t = Terminal::new(token.kind.to_string());

//     if let Some(p) = self.table.table[&nt][&t] {
//         for symbol in self.cfg.productions[p].symbols() {
//             if let Some(node) = self.parse_symbol(&mut stream, symbol) {
//                 root.children.push(node);
//             }
//         }
//     } else {
//         panic!("Syntax Error");
//     }
// }

// root
