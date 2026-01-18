// TODO -> handle floats
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Lexeme {
    Number(isize),
    Symbol(String),
    List(Vec<Lexeme>),
    None,
}
