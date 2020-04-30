use silly_lex::Token;
use std::mem;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AstKind {
    Regex,
    Alt,
    AltList,
    Seq,
    SeqList,
    Atom,
    AtomMod(Mod),
    Nucleus,
    CharRng,
    Lambda,
    Dot,
    Char(char),
}

impl AstKind {
    pub fn char(&self) -> anyhow::Result<char> {
        if let AstKind::Char(c) = self {
            Ok(*c)
        } else {
            Err(anyhow::anyhow!("AstKind was not a Char"))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstNode {
    pub kind: AstKind,
    pub children: Vec<AstNode>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mod {
    Kleene,
    Plus,
    Lambda,
}

// simplify a node and all children
pub fn simplify_to_ast(node: AstNode) -> AstNode {
    match node.kind {
        AstKind::Regex => simplify_regex(node),
        AstKind::Atom => simplify_atom(node),
        AstKind::Seq => simplify_seq(node),
        AstKind::SeqList => simplify_seq_list(node),
        AstKind::CharRng => simplify_char_rng(node),
        _ => node,
    }
}

pub fn simplify_regex(node: AstNode) -> AstNode {
    simplify_to_ast(node.children[0].clone())
}

pub fn simplify_atom(node: AstNode) -> AstNode {
    todo!()
}

pub fn simplify_seq(node: AstNode) -> AstNode {
    todo!()
}

pub fn simplify_seq_list(node: AstNode) -> AstNode {
    todo!()
}

pub fn simplify_char_rng(node: AstNode) -> AstNode {
    todo!()
}
