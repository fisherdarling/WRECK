use crate::ast::AstKind;
/// This takes in a perfectly simplified Regex tree and creates an NFA
use crate::ast::AstNode;
use std::collections::{BTreeMap, BTreeSet};

pub struct NFAGenerator {
    pub root: AstNode,
    // This is the "T" from here: https://cs.mcprogramming.com/static/comp/hr/2290bf6e443cd3c7/lga-re-semantic-analysis.pdf
    // First usize: leftmost col
    // char, top most char
    // third usize is the number inside the table
    pub transitions: BTreeMap<(usize, char), usize>,

    // same pattern as transitions, but top is number not char?
    pub lambda_transitions: BTreeMap<(usize, usize), usize>,

    pub highest_state_number: usize,

    pub alpha: BTreeSet<char>,
}

impl NFAGenerator {
    pub fn new(root: AstNode) -> Self {
        Self {
            root: root,
            transitions: BTreeMap::new(),
            lambda_transitions: BTreeMap::new(),
            highest_state_number: 0,
            alpha: BTreeSet::new(),
        }
    }

    pub fn get_new_state(&mut self) -> usize {
        self.highest_state_number += 1;
        self.highest_state_number
    }

    pub fn insert_to_trans(&mut self, current: usize, next: usize, value: char) {
        self.transitions.insert((current, value), next);
    }

    /// Returns weather or not a change has been made. `curent_state` is "this" in psuedocode, `next_state` is next
    pub fn add_to_table(
        &mut self,
        node: &AstNode,
        current_state: usize,
        next_state: usize,
    ) -> bool {
        match node.kind {
            AstKind::Char(c) => {
                self.insert_to_trans(current_state, next_state, c.clone());
            }
            AstKind::Seq => {
                todo!();
            }
            _ => todo!(),
        }
        false
    }
}
