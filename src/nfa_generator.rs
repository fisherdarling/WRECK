use crate::ast::AstKind;
/// This takes in a perfectly simplified Regex tree and creates an NFA
use crate::ast::AstNode;
use std::collections::BTreeMap;

pub struct NFAGenerator {
    pub root: AstNode,
    // This is the "T" from here: https://cs.mcprogramming.com/static/comp/hr/2290bf6e443cd3c7/lga-re-semantic-analysis.pdf
    // First usize: leftmost col
    // char, top most char
    // third usize is the number inside the table
    pub transitions: BTreeMap<(usize, char), usize>,

    // same pattern as transitions, but top is number not char?
    pub lambdaTransitions: BTreeMap<(usize, usize), usize>,

    pub highestStateNumber: usize,
}

impl NFAGenerator {
    pub fn new(root: AstNode) -> Self {
        Self {
            root: root,
            transitions: BTreeMap::new(),
            lambdaTransitions: BTreeMap::new(),
            highestStateNumber: 0,
        }
    }

    pub fn getNewState(&mut self) -> usize {
        self.highestStateNumber += 1;
        self.highestStateNumber
    }

    /// Returns weather or not a change has been made
    pub fn addToTable(&mut self, node: &AstNode, currentState: usize, nextState: usize) -> bool {
        match node.kind {
            AstKind::Char(c) => {
                self.transitions
                    .insert((currentState, c.clone()), nextState);
            }
            AstKind::Seq => {
                todo!();
            }
            _ => todo!(),
        }
        false
    }
}
