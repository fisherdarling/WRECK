use crate::ast::AstKind;
/// This takes in a perfectly simplified Regex tree and creates an NFA
use crate::ast::AstNode;
use std::collections::{BTreeMap, BTreeSet};

pub struct NFAGenerator {
    // pub root: AstNode,
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
    pub fn new(alpha: BTreeSet<char>) -> Self {
        Self {
            // root: root,
            transitions: BTreeMap::new(),
            lambda_transitions: BTreeMap::new(),
            highest_state_number: 1,
            alpha: alpha,
        }
    }

    // pub fn simplify_root(&mut self) {
    // self.add_to_table(&xself.root, 0, 1);
    // }

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
            AstKind::Char(c) => self.leaf_child(c, current_state, next_state),
            AstKind::Dot => self.leaf_dot(current_state, next_state),
            AstKind::Lambda => self.leaf_lambda(current_state, next_state),
            AstKind::Alt => self.node_alt(node, current_state, next_state),
            AstKind::Kleene => self.node_kleene(node, current_state, next_state),
            AstKind::Seq => self.node_seq(node, current_state, next_state),

            _ => panic!("Bad AST node kind in add_to_table: #{:?}", node.kind),
        }
        false
    }

    pub fn node_seq(&mut self, node: &AstNode, this: usize, next: usize) {
        let new_states: Vec<usize> = (0..node.children.len())
            .map(|_| self.get_new_state())
            .collect();

        for i in 0..node.children.len() - 1 {
            self.add_to_table(&node.children[i], new_states[i], new_states[i + 1]);
        }
        self.add_to_table(
            node.children.last().unwrap(),
            *new_states.last().unwrap(),
            next,
        );
    }

    pub fn leaf_child(&mut self, value: char, this: usize, next: usize) {
        self.insert_to_trans(this, next, value);
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn testing_leaf_node() {
        // let mut r = AstNode::new(AstKind::Seq);
        // r.children.push(AstNode::new(AstKind::Char('b')));
        let r = AstNode::new(AstKind::Char('b'));

        let mut alpha: BTreeSet<char> = BTreeSet::new();
        alpha.insert('a');
        alpha.insert('b');
        alpha.insert('c');
        alpha.insert('d');
        alpha.insert('e');

        let mut simple = NFAGenerator::new(alpha);
        simple.add_to_table(&r, 0, 1);

        let expected_l = BTreeMap::new();

        let mut expected_t = BTreeMap::new();

        expected_t.insert((0, 'b'), 1);

        assert_eq!(simple.lambda_transitions, expected_l);
        assert_eq!(simple.transitions, expected_t);
    }

    #[test]
    fn testing_leaf_dot() {
        let r = AstNode::new(AstKind::Dot);

        let mut alpha: BTreeSet<char> = BTreeSet::new();
        alpha.insert('a');
        alpha.insert('b');
        alpha.insert('c');
        alpha.insert('d');
        alpha.insert('e');

        let mut simple = NFAGenerator::new(alpha);
        simple.add_to_table(&r, 0, 1);

        let mut expected_t = BTreeMap::new();
        expected_t.insert((0, 'a'), 1);
        expected_t.insert((0, 'b'), 1);
        expected_t.insert((0, 'c'), 1);
        expected_t.insert((0, 'd'), 1);
        expected_t.insert((0, 'e'), 1);

        assert_eq!(simple.lambda_transitions, BTreeMap::new());
        assert_eq!(simple.transitions, expected_t);
    }

    #[test]
    fn test_seq() {
        let a_to_e_alpha: BTreeSet<char> = ['a', 'b', 'c', 'd', 'e'].iter().cloned().collect();
        let mut r = AstNode::new(AstKind::Seq);
        r.children.push(AstNode::new(AstKind::Char('b')));
        r.children.push(AstNode::new(AstKind::Char('c')));

        let mut simple = NFAGenerator::new(a_to_e_alpha);
        simple.add_to_table(&r, 0, 1);

        let mut expected_t = BTreeMap::new();
        expected_t.insert((0, 'b'), 1);
        expected_t.insert((1, 'c'), 2);

        assert_eq!(simple.lambda_transitions, BTreeMap::new());
        assert_eq!(simple.transitions, expected_t);
    }
}
