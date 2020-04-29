use silly_lex::Token;

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
    pub children: Option<Vec<AstNode>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mod {
    Kleene,
    Plus,
    Lambda,
}
