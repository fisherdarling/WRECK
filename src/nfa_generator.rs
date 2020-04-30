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
    pub lambda_transitions: BTreeMap<(usize, usize), bool>,

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
            AstKind::Dot => self.leaf_dot(current_state, next_state),
            AstKind::Lambda => self.leaf_lambda(current_state, next_state),
            AstKind::Alt => self.node_alt(node, current_state, next_state),
            AstKind::AtomMod(amod) => match amod {
                Kleene => self.node_kleene(node, current_state, next_state),
                _ => todo!(),
            },
            _ => todo!(),
        }
        false
    }

    pub fn leaf_dot(&mut self, this: usize, next: usize) {
        for c in &self.alpha {
            // have to inline this instead of calling insert_to_trans because borrow checking :(
            self.transitions.insert((this, c.clone()), next);
        }
    }

    pub fn leaf_lambda(&mut self, this: usize, next: usize) {
        self.lambda_transitions.insert((this, next), true);
    }

    pub fn node_alt(&mut self, node: &AstNode, this: usize, next: usize) {
        let new_states: Vec<usize> = (0..node.children.len())
            .map(|_| self.get_new_state())
            .collect();

        for (i, state) in new_states.iter().enumerate() {
            self.add_to_table(&node.children[i], *state, next);
        }
    }

    pub fn node_kleene(&mut self, node: &AstNode, this: usize, next: usize) {
        self.lambda_transitions.insert((this, next), true);
        self.add_to_table(&node.children[0], this, next);
        self.lambda_transitions.insert((next, this), true);
    }
}
