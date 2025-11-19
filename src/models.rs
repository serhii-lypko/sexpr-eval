/// While in traditional
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Token {
    // Base
    OpenParen,
    CloseParen,
    Semicolon,

    // Arithmetic
    Plus,
    Minus,
    Mult,
    Div,

    // Conditions
    Equal,
    EqualEqual,
    NotEuqal,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Literals
    String(String),
    Int(isize),
    Bool(bool),

    Identifier(String),

    // Keywords
    If,
    Def,
    Print,
    And,
    Not,

    // TODO -> recursive collections?
    List,
}
