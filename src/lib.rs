use silly_lex::{Lexer, Token};

pub mod ast;
// ast!(
//     Lit |isize|,
//     Expr: enum Expr {
//         BinOp: struct BinOp {
//             op: enum Op {
//                 Plus,
//                 Minus,
//                 Times,
//                 Divide,
//             },
//             lhs: Box<Expr>,
//             rhs: Box<Expr>,
//         },
//         |Lit|
//     }
// );

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
