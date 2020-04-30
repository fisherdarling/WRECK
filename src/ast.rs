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
    Nucleus,
    CharRng,
    Kleene,
    Plus,
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

impl AstNode {
    pub fn new(kindr: AstKind) -> Self {
        AstNode {
            kind: kindr,
            children: vec![],
        }
    }
}

// simplify a node and all children
pub fn simplify_node(node: AstNode) -> AstNode {
    match node.kind {
        AstKind::Atom => simplify_atom(&node),
        AstKind::Seq => simplify_seq(&node),
        AstKind::SeqList => simplify_seq_list(&node),
        AstKind::CharRng => simplify_char_rng(&node),
        _ => node,
    }
}


pub fn simplify_seq(node: &AstNode) -> AstNode {
    todo!()
}

pub fn simplify_seq_list(node: &AstNode) -> AstNode {
    todo!()
}

pub fn simplify_char_rng(node: &AstNode) -> AstNode {
    todo!()
}

pub fn simplify_atom(atom_node: &AstNode) -> AstNode {
    let mut nucleus = simplify_nucleus(&atom_node.children[0]);
    let atom_mod = &atom_node.children[1].children[0];
    let mut new_atom;
    match atom_mod.kind {
        AstKind::Kleene => new_atom = AstNode::new(AstKind::Kleene),
        AstKind::Plus => new_atom = AstNode::new(AstKind::Plus),
        AstKind::Lambda => new_atom = AstNode::new(AstKind::Alt),
        _ => panic!("Bad Atom Mod")
    }
    new_atom.children.append(&mut nucleus.children);
    new_atom
}

pub fn simplify_nucleus(nucleus_node: &AstNode) -> AstNode {
    let mut new_nuc = AstNode::new(AstKind::Nucleus);
    new_nuc.children.push(nucleus_node.children[0].clone());

    if nucleus_node.children[1].children.len() > 1 {
        let c = nucleus_node.children[0].kind.char().unwrap();
        let m = nucleus_node.children[1].children[1].kind.char().unwrap();
        let start: u8 = c as u8;
        let end: u8 = m as u8;
        let range = start..=end;
        
        for i in range {
            new_nuc.children.push(AstNode::new(AstKind::Char(i as char)));
        }
    } 
    new_nuc
}