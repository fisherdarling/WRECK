use crate::symbol::NonTerminal;
use petgraph::dot::{Config, Dot};
use petgraph::Graph;
use silly_lex::Token;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::mem;
use std::path::Path;

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
    AtomMod,
    Char(char),
}

impl fmt::Display for AstKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AstKind::Char(c) => write!(f, "{}", c),
            AstKind::Kleene => write!(f, "*"),
            AstKind::Dot => write!(f, "."),
            AstKind::Lambda => write!(f, "λ"),
            _ => write!(f, "{:?}", self),
        }
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

    pub fn from_str(s: &str) -> Option<AstKind> {
        Some(match s {
            "RE" => AstKind::Regex,
            "ALT" => AstKind::Alt,
            "ALTLIST" => AstKind::AltList,
            "SEQ" => AstKind::Seq,
            "SEQLIST" => AstKind::SeqList,
            "ATOM" => AstKind::Atom,
            "ATOMMOD" => AstKind::AtomMod,
            "NUCLEUS" => AstKind::Nucleus,
            "CHARRNG" => AstKind::CharRng,
            "char" => AstKind::Char('\0'),
            "dot" => AstKind::Dot,
            "kleene" => AstKind::Kleene,
            "plus" => AstKind::Plus,
            // "open" => AstKind::Char('\0'),
            // "close" => AstKind::Char('\0'),
            _ => None?,
        })
    }

    pub fn as_nt(&self) -> Option<&str> {
        Some(match self {
            AstKind::Regex => "RE",
            AstKind::Alt => "ALT",
            AstKind::AltList => "ALTLIST",
            AstKind::Seq => "SEQ",
            AstKind::SeqList => "SEQLIST",
            AstKind::Atom => "ATOM",
            AstKind::Nucleus => "NUCLEUS",
            AstKind::CharRng => "CHARRNG",
            AstKind::AtomMod => "ATOMMOD",
            AstKind::Kleene => None?,
            AstKind::Plus => None?,
            AstKind::Lambda => None?,
            AstKind::Dot => None?,
            AstKind::Char(_) => None?,
        })
    }

    pub fn as_str(&self) -> &str {
        match self {
            AstKind::Regex => "RE",
            AstKind::Alt => "ALT",
            AstKind::AltList => "ALTLIST",
            AstKind::Seq => "SEQ",
            AstKind::SeqList => "SEQLIST",
            AstKind::Atom => "ATOM",
            AstKind::Nucleus => "NUCLEUS",
            AstKind::CharRng => "CHARRNG",
            AstKind::AtomMod => "ATOMMOD",
            AstKind::Kleene => "kleene",
            AstKind::Plus => "plus",
            AstKind::Lambda => "lambda",
            AstKind::Dot => "dot",
            AstKind::Char(_) => "char",
        }
    }

    // pub fn token_type(&self) -> TokenKind {}

    pub fn is_terminal(&self) -> bool {
        match self {
            AstKind::Regex => false,
            AstKind::Alt => false,
            AstKind::AltList => false,
            AstKind::Seq => false,
            AstKind::SeqList => false,
            AstKind::Atom => false,
            AstKind::Nucleus => false,
            AstKind::CharRng => false,
            AstKind::AtomMod => false,
            AstKind::Kleene => true,
            AstKind::Plus => true,
            AstKind::Lambda => true,
            AstKind::Dot => true,
            AstKind::Char(_) => true,
        }
    }

    pub fn is_non_terminal(&self) -> bool {
        !self.is_terminal()
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
    pub fn export_graph(&self, file_path: impl AsRef<Path>) {
        let graph = self.create_pet_graph();
        let mut f = File::create(file_path).unwrap();
        let output = format!("{}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
        f.write_all(&output.as_bytes())
            .expect("could not write file");
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

    fn create_pet_graph_rec(
        &self,
        mut graph: Graph<AstKind, usize>,
        node: &AstNode,
        parent: petgraph::graph::NodeIndex,
    ) -> Graph<AstKind, usize> {
        for child in node.children.iter() {
            let cnode = graph.add_node(child.kind);
            graph.add_edge(parent, cnode, 0);

            graph = self.create_pet_graph_rec(graph, child, cnode);
        }
        graph
    }
}

pub fn simplify_RE(root_node: &AstNode) -> AstNode {
    simplify_alt(&root_node.children[0])
}

pub fn simplify_plus(mut node: AstNode) -> AstNode {
    println!("Hi");
    let mut new_seq = AstNode::new(AstKind::Seq);
    let mut kleene = AstNode::new(AstKind::Kleene);
    let mut copy = node.clone();
    kleene.children.append(&mut node.children);
    new_seq.children.append(&mut copy.children);
    new_seq.children.push(kleene);

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
        },
        AstKind::Plus => {
            new_atom.children.push(simplify_plus(nucleus));
            new_atom
        },
        AstKind::Lambda => {
            new_atom.children.append(&mut nucleus.children);
            new_atom
        },
        _ => panic!("Bad Atom Mod"),
    }
}

pub fn simplify_nucleus(nucleus_node: &AstNode) -> AstNode {
    let mut new_nuc = AstNode::new(AstKind::Nucleus);
    if nucleus_node.children.len() == 1 {
        let dot = AstNode::new(AstKind::Dot);
        new_nuc.children.push(dot);
        return new_nuc
    }
    if nucleus_node.children[1].kind == AstKind::CharRng {
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
        } else {
            new_nuc.children.push(nucleus_node.children[0].clone());
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

    if alt_node.children[1].children.len() == 1 {
        return seq;
    }

    new_alt.children.append(&mut seq.children);
    new_alt.children.append(&mut alt.children);

    new_alt
}

pub fn simplify_alt_list(altlist_node: &AstNode) -> AstNode {
    let mut new_alt = AstNode::new(AstKind::Alt);
    if (altlist_node.children.len() == 1) {
        return new_alt;
    }

    let mut seq = simplify_seq(&altlist_node.children[1]);
    let mut alt = simplify_alt_list(&altlist_node.children[2]);

    new_alt.children.push(seq);
    new_alt.children.append(&mut alt.children);

    new_alt
}

pub fn simplify_seq(node: &AstNode) -> AstNode {
    let mut new_seq = AstNode::new(AstKind::Seq);

    if node.children[0].kind == AstKind::Lambda {
        new_seq.children.push(AstNode::new(AstKind::Lambda));
        return new_seq;
    }

    let mut atom = simplify_atom(&node.children[0]);
    let mut seqlist = simplify_seq_list(&node.children[1]);

    new_seq.children.append(&mut atom.children);
    new_seq.children.append(&mut seqlist.children);

    new_seq
}

pub fn simplify_seq_list(node: &AstNode) -> AstNode {
    let mut new_seq = AstNode::new(AstKind::Seq);

    if node.children[0].kind == AstKind::Lambda {
        return new_seq;
    }

    let mut atom = simplify_atom(&node.children[0]);
    let mut seqlist = simplify_seq_list(&node.children[1]);

    new_seq.children.append(&mut atom.children);
    new_seq.children.append(&mut seqlist.children);

    new_seq
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::create_dir;
    use std::process::Command;

    #[test]
    fn simple_graphing() {
        let mut r = AstNode::new(AstKind::Char('b'));
        let mut a = AstNode::new(AstKind::Regex);
        let mut b = AstNode::new(AstKind::Seq);
        let mut c = AstNode::new(AstKind::SeqList);
        let mut d = AstNode::new(AstKind::Kleene);
        let e = AstNode::new(AstKind::Regex);

        a.children.push(c);
        a.children.push(e);
        b.children.push(d);

        r.children.push(a);
        r.children.push(b);

        match create_dir("test_output/") {
            Ok(_) => println!("Making output dir"),
            Err(_) => println!("Output dir already exists"),
        };

        r.export_graph("test_output/simple.dot");
        Command::new("dot")
            .arg("-Tpng")
            .arg("test_output/simple.dot")
            .arg("-o")
            .arg("test_output/simple.png")
            .output()
            .expect("failed to execute process");
    }

    #[test]
    fn concrete_simplification() {
        // Create a nucleus
        let a = AstNode::new(AstKind::Char('a'));
        let b = AstNode::new(AstKind::Char('-'));
        let c = AstNode::new(AstKind::Char('d'));
        let mut d = AstNode::new(AstKind::CharRng);
        let mut e = AstNode::new(AstKind::Nucleus);
        d.children.push(b);
        d.children.push(c);
        e.children.push(a);
        e.children.push(d);

        ///create atom
        let f = AstNode::new(AstKind::Lambda);
        let mut g = AstNode::new(AstKind::AtomMod);
        let mut h = AstNode::new(AstKind::Atom);
        g.children.push(f);
        h.children.push(e);
        h.children.push(g);

        //// Another Nucleus
        let i = AstNode::new(AstKind::Dot);
        let j = AstNode::new(AstKind::Lambda);
        let mut k = AstNode::new(AstKind::CharRng);
        let mut l = AstNode::new(AstKind::Nucleus);
        l.children.push(i);
        k.children.push(j);
        l.children.push(k);

        ///// Another atom
        let m = AstNode::new(AstKind::Lambda);
        let mut n = AstNode::new(AstKind::AtomMod);
        let mut o = AstNode::new(AstKind::Atom);
        n.children.push(m);
        o.children.push(l);
        o.children.push(n);

        ////// First Seqlist
        let p = AstNode::new(AstKind::Lambda);
        let mut q = AstNode::new(AstKind::SeqList);
        let mut r = AstNode::new(AstKind::SeqList);
        q.children.push(p);
        r.children.push(o);
        r.children.push(q);

        /////// Sequence
        let mut s = AstNode::new(AstKind::Seq);
        s.children.push(h);
        s.children.push(r);

        //////// Final Alt!
        let t = AstNode::new(AstKind::Lambda);
        let mut u = AstNode::new(AstKind::AltList);
        let mut v = AstNode::new(AstKind::Alt);
        u.children.push(t);
        v.children.push(s);
        v.children.push(u);

        ///////// REGEX, FUCKING FINALLY!
        let x = AstNode::new(AstKind::Char('$'));
        let mut y = AstNode::new(AstKind::Regex);
        y.children.push(v);
        y.children.push(x);

        match create_dir("test_output/") {
            Ok(_) => println!("Making output dir"),
            Err(_) => println!("Output dir already exists"),
        };

        y.export_graph("test_output/concrete.dot");
        Command::new("dot")
            .arg("-Tpng")
            .arg("test_output/concrete.dot")
            .arg("-o")
            .arg("test_output/concrete.png")
            .output()
            .expect("failed to execute process");

        let simple = simplify_RE(&y);
        simple.export_graph("test_output/AST.dot");
        Command::new("dot")
            .arg("-Tpng")
            .arg("test_output/AST.dot")
            .arg("-o")
            .arg("test_output/AST.png")
            .output()
            .expect("failed to execute process");
    }
}
