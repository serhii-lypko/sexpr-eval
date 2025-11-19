use crate::models::Token;

pub(crate) enum ASTNode {
    List,
    Atom(AtomKind),
}

pub(crate) enum AtomKind {
    Symbol,
    String,
    Int,
}

// (+ 10 15)
/*
   [
    OpenParen,
    Plus,
    Int(
        10,
    ),
    Int(
        15,
    ),
    CloseParen,
  ]
*/

// TODO -> return result?
pub(crate) fn build_ast(tokens: Vec<Token>) -> ASTNode {
    //

    todo!()
}
