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

pub fn simplify_to_ast(node: AstNode) -> AstNode {
    for child in node.children.into_iter() {
        match child.kind {
            AstKind::Regex => simplify_regex(child),
            AstKind::Atom => simplify_atom(child),
            AstKind::Seq => simplify_seq(child),
            AstKind::SeqList => simplify_seq_list(child),
            AstKind::CharRng => simplify_char_rng(child),
            _ => (),
        };
    }
    node
}

pub fn simplify_regex(node: AstNode) -> AstNode {
    todo!()
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