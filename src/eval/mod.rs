mod arithmetic;
mod bindings;
mod conditional;
mod models;

#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::rc::Rc;

use crate::Environment;
use crate::reader::Lexeme;
pub(crate) use models::Value;

// (+ 3 2)
// (+ 5 (* 3 2))
// (+ 10 $a)

const SPECIAL_FORMS: [&str; 3] = ["if", "def-var", "def-fn"];

// TODO -> tests for variables

// TODO -> needs to retun Result
pub(crate) fn eval(lexeme: Lexeme, env: Rc<RefCell<Environment>>) -> Value {
    match lexeme {
        Lexeme::None => Value::None,
        Lexeme::Number(number) => Value::Number(number),
        Lexeme::Symbol(symbol) => match resolve_symbol(&symbol, env) {
            Some(val) => val,
            None => Value::Symbol(symbol),
        },
        Lexeme::List(lexemes) => resolve_list(lexemes, env),
    }
}

// TODO -> handle error cases when variable does not starts from $ by mistake
fn resolve_symbol(symbol: &String, env: Rc<RefCell<Environment>>) -> Option<Value> {
    match check_variable(&symbol) {
        Some(var_name) => match env.borrow().get_var(var_name) {
            Some(val) => Some(val.clone()),
            None => {
                // TODO -> return error variable not found
                unimplemented!()
            }
        },
        None => None,
    }
}

fn resolve_list(lexemes: Vec<Lexeme>, env: Rc<RefCell<Environment>>) -> Value {
    // SPECIAL_FORMS

    let head = &lexemes[0];

    match head {
        Lexeme::Symbol(symbol) => {
            // User-defined functions will fall in the same scoupe as arithmetics,
            // and special forms have finit number of cases - so start from them
            let is_special_form = check_special_form(&symbol);

            if is_special_form {
                return try_eval_special_form(lexemes, env);
            }

            let values: Vec<Value> = lexemes
                .into_iter()
                .map(|lexeme| eval(lexeme, env.clone()))
                .collect();

            if let Some(res) = arithmetic::try_eval_arithmetics(&values) {
                return res;
            }

            bindings::eval_function(values, env)
        }
        _ => todo!(),
    }
}

// TODO -> needs to return Result
fn try_eval_special_form(lexems: Vec<Lexeme>, env: Rc<RefCell<Environment>>) -> Value {
    let head = &lexems[0];

    if let Lexeme::Symbol(head_symbol) = head {
        match head_symbol.as_str() {
            "if" => return conditional::eval_lazy_if(lexems, env),

            "def-var" => return bindings::set_variable_binding(lexems, env),
            "def-fn" => return bindings::set_function_binding(lexems, env),

            // Return Symbol as is
            _ => return Value::Symbol(head_symbol.clone()),
        }
    }

    unimplemented!()
}

fn check_special_form(s: &str) -> bool {
    SPECIAL_FORMS.contains(&s)
}

fn check_variable(s: &str) -> Option<&str> {
    if s.starts_with('$') { Some(s) } else { None }
}
