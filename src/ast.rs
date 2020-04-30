use silly_lex::Token;
use std::mem;
use petgraph::Graph;
use petgraph::dot::{Dot, Config};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::fmt;

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

impl fmt::Display for AstKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
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

    // Export a graph to something that Graphvis can us 
    pub fn export_graph(self, file_path: PathBuf) {
        let graph = self.create_pet_graph();
        let mut f = File::create(file_path).unwrap();
        let output = format!("{}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
        f.write_all(&output.as_bytes()).expect("could not write file");
    }

    fn create_pet_graph(&self) -> Graph<AstKind, usize> {
        let mut graph = Graph::<_, usize>::new();
        let root = graph.add_node(self.kind);

        for child in self.children.iter() {
            let cnode = graph.add_node(child.kind);
            graph.add_edge(root, cnode, 0);
            graph = self.create_pet_graph_rec(graph, child, cnode);
        }
        graph
    }

    fn create_pet_graph_rec(&self, mut graph: Graph<AstKind, usize>, node: &AstNode, parent: petgraph::graph::NodeIndex) -> Graph <AstKind, usize> {
        for child in node.children.iter() {
            let cnode = graph.add_node(child.kind);
            graph.add_edge(parent, cnode, 0);
            
            graph = self.create_pet_graph_rec(graph, child, cnode);
        }
        graph
    }
}

pub fn simplify_RE(root_node: AstNode) -> AstNode {
    simplify_alt(&root_node.children[0])
}

pub fn simplify_plus(node: AstNode) -> AstNode {
    let mut new_seq = AstNode::new(AstKind::Seq);
    let mut kleene = AstNode::new(AstKind::Kleene);
    let copy = node.clone();
    new_seq.children.push(node);
    new_seq.children.push(kleene);
    new_seq.children[1].children.push(copy);
    new_seq
}

pub fn simplify_atom(atom_node: &AstNode) -> AstNode {
    let mut nucleus = simplify_nucleus(&atom_node.children[0]);
    let atom_mod = &atom_node.children[1].children[0];
    let mut new_atom = AstNode::new(AstKind::Atom);
    match atom_mod.kind {
        AstKind::Kleene => {
            let mut kleene = AstNode::new(AstKind::Kleene);
            kleene.children.append(&mut nucleus.children);
            new_atom.children.push(kleene);
            new_atom
        }
        AstKind::Plus => {
            new_atom.children.push(simplify_plus(nucleus));
            new_atom
        }
        AstKind::Lambda => {
            new_atom.children.append(&mut nucleus.children);
            new_atom
        }
        _ => panic!("Bad Atom Mod"),
    }
}

pub fn simplify_nucleus(nucleus_node: &AstNode) -> AstNode {
    let mut new_nuc = AstNode::new(AstKind::Nucleus);
    if nucleus_node.children[1].kind == AstKind::CharRng {
        new_nuc.children.push(nucleus_node.children[0].clone());
        if nucleus_node.children[1].children.len() > 1 {
            let mut alt = AstNode::new(AstKind::Alt);
            let c = nucleus_node.children[0].kind.char().unwrap();
            let m = nucleus_node.children[1].children[1].kind.char().unwrap();
            let start: u8 = c as u8;
            let end: u8 = m as u8;
            let range = start..=end;

            for i in range {
                alt.children.push(AstNode::new(AstKind::Char(i as char)));
            }
            new_nuc.children.push(alt);
        }
        return new_nuc;
    } else {
        // we're dealing with an alt!
        let alt = simplify_alt(&nucleus_node.children[1]);
        new_nuc.children.push(alt);
        return new_nuc;
    }
}

pub fn simplify_alt(alt_node: &AstNode) -> AstNode {
    let mut new_alt = AstNode::new(AstKind::Alt);
    let mut seq = simplify_seq(&alt_node.children[0]);
    let mut alt = simplify_alt_list(&alt_node.children[1]);

    if alt.children.len() == 1 {
        return seq;
    }

    new_alt.children.append(&mut seq.children);
    new_alt.children.append(&mut alt.children);

    new_alt
}

pub fn simplify_alt_list(altlist_node: &AstNode) -> AstNode {
    let mut new_alt = AstNode::new(AstKind::Alt);
    if (altlist_node.children.len() == 1) {
        new_alt.children.push(AstNode::new(AstKind::Lambda));
        return new_alt;
    }

    let mut seq = simplify_seq(&altlist_node.children[1]);
    let mut alt = simplify_alt_list(&altlist_node.children[2]);

    new_alt.children.append(&mut seq.children);
    new_alt.children.append(&mut alt.children);

    new_alt
}

pub fn simplify_seq(node: &AstNode) -> AstNode {
    let mut new_seq = AstNode::new(AstKind::Seq);

    if node.children[0].kind == AstKind::Lambda {
        new_seq.children.push(AstNode::new(AstKind::Lambda));
        return new_seq;
    }

    let atom = simplify_atom(&node.children[0]);
    let mut seqlist = simplify_seq_list(&node.children[1]);

    new_seq.children.push(atom);
    new_seq.children.append(&mut seqlist.children);

    new_seq
}

pub fn simplify_seq_list(node: &AstNode) -> AstNode {
    let mut new_seq = AstNode::new(AstKind::Seq);

    if node.children[0].kind == AstKind::Lambda {
        return new_seq;
    }

    let atom = simplify_atom(&node.children[0]);
    let mut seqlist = simplify_seq_list(&node.children[1]);

    new_seq.children.push(atom);
    new_seq.children.append(&mut seqlist.children);

    new_seq
}
