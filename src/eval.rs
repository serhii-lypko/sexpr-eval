use crate::models::Token;

#[derive(Debug, PartialEq)]
pub enum EvalError {
    // InvalidOperatorDelimeter,
    // InvalidLexemeDelimeter,
    // InvalidLexeme,
    // InvalidNumber,
}

pub(crate) type EvalResult<T> = Result<T, EvalError>;

// TODO -> the dispatch might looks like this:
/*
  match list[0] {
      Symbol::If => evaluate_if(list[1], list[2], list[3]),
      Symbol::Plus => evaluate_plus(&list[1..]),
      Symbol::Identifier(name) => lookup_function(name),
      _ => error
  }
*/

/// Evaluates s-expressions (recursively processes lists, resolves symbols)
pub fn eval(tokens: Vec<Token>) -> EvalResult<()> {
    //

    Ok(())
}
