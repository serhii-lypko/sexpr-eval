mod arithmetic;
mod conditional;
mod models;

#[cfg(test)]
mod tests;

use crate::reader::Lexeme;
pub(crate) use models::Value;

// TODO -> needs to retun Result
pub(crate) fn eval(lexeme: Lexeme) -> Value {
    match lexeme {
        Lexeme::None => Value::None,
        Lexeme::Number(number) => Value::Number(number),
        Lexeme::Symbol(symbol) => Value::Symbol(symbol),
        Lexeme::List(lexemes) => {
            if let Some(res) = arithmetic::try_eval_simple_arithmetics(lexemes.clone()) {
                return res;
            }

            try_eval_special_form(lexemes)
        }
    }
}

// TODO -> needs to return Result
fn try_eval_special_form(lexems: Vec<Lexeme>) -> Value {
    let head = &lexems[0];

    if let Lexeme::Symbol(head_symbol) = head {
        match head_symbol.as_str() {
            "if" => return conditional::eval_lazy_if(lexems),

            // Return Symbol as is
            _ => return Value::Symbol(head_symbol.clone()),
        }
    }

    todo!()
}
