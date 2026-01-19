mod arithmetic;
mod bindings;
mod conditional;
mod models;

#[cfg(test)]
mod tests;

use std::rc::Rc;

use crate::Environment;
use crate::reader::Lexeme;
pub(crate) use models::Value;

// TODO -> needs to retun Result
pub(crate) fn eval(lexeme: Lexeme, mut env: Rc<Environment>) -> Value {
    match lexeme {
        Lexeme::None => Value::None,
        Lexeme::Number(number) => Value::Number(number),
        Lexeme::Symbol(symbol) => Value::Symbol(symbol),
        Lexeme::List(lexemes) => {
            if let Some(res) = arithmetic::try_eval_simple_arithmetics(lexemes.clone(), env.clone())
            {
                return res;
            }

            try_eval_special_form(lexemes, env)
        }
    }
}

// TODO -> needs to return Result
fn try_eval_special_form(lexems: Vec<Lexeme>, mut env: Rc<Environment>) -> Value {
    let head = &lexems[0];

    if let Lexeme::Symbol(head_symbol) = head {
        match head_symbol.as_str() {
            "if" => return conditional::eval_lazy_if(lexems, env),
            "def-var" => return bindings::eval_variable_binding(lexems, env),

            // Return Symbol as is
            _ => return Value::Symbol(head_symbol.clone()),
        }
    }

    todo!()
}
