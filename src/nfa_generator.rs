/// This takes in a perfectly simplified Regex tree and creates an NFA
use ast::AstNode;

pub struct NFAGenerator {
    pub root: AstNode,
    // This is the "T" from here: https://cs.mcprogramming.com/static/comp/hr/2290bf6e443cd3c7/lga-re-semantic-analysis.pdf
    // First usize: leftmost col
    // char, top most char
    // third usize is the number inside the table
    pub transitions: BTreeMap<(usize, char), usize>,

    // same pattern as transitions, but top is number not char?
    pub lambdaTransitions: BTreeMap<(usize, usize), usize>,

    currentState: usize,
    nextState: usize,
}

impl NFAGenerator {
    pub fn new(root: AstNode) -> Self {
        Self {
            root: root,
            transitions: BTreeMap::new,
            currentState: 0,
            nextState: 0,
        }
    }

    /// Returns weather or not a change has been made
    pub fn addToTable(node: AstNode) -> bool {
        match node {
            Seq -> {
                if node.transitions.len() == 1 {
                    let child = node.transitions[0];
                    match child {
                        Char(c) => {
                            // leafChild case
                            self.transitions.insert((self.currentState, c), self.nextState)
                        }
                        _ => {todo!()}
                    }
                }
            }
        }
        false
    }
}
