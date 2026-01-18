#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Value {
    Number(isize),
    Symbol(String),
    Boolean(bool),
}