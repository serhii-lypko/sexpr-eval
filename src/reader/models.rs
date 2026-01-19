// TODO -> handle floats
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Lexeme {
    Number(isize),
    Symbol(String),
    List(Vec<Lexeme>),
    None,
}

impl Lexeme {
    pub(crate) fn is_list(&self) -> bool {
        match &self {
            Lexeme::List(_) => true,
            _ => false,
        }
    }

    pub(crate) fn has_inner_lists(&self) -> bool {
        match &self {
            Lexeme::List(lexemes) => {
                if lexemes.is_empty() {
                    return false;
                }

                matches!(lexemes[0], Lexeme::List(_))
            }
            _ => false,
        }
    }
}
