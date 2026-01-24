use std::cell::RefCell;
use std::rc::Rc;

use crate::Environment;
use crate::eval::{Value, eval};
use crate::reader::Lexeme;

// TODO -> tests
pub(super) fn eval_variable_binding(lexems: Vec<Lexeme>, env: Rc<RefCell<Environment>>) -> Value {
    if lexems.len() != 3 {
        // TODO -> return Error
        todo!()
    }

    let var = &lexems[1];
    let var_name = match var {
        Lexeme::Symbol(name) if name.starts_with('$') => name.to_owned(),
        Lexeme::Symbol(name) => {
            // TODO -> return Error
            panic!("Variable name must start with '$', got: {}", name)
        }
        _ => {
            // TODO -> return Error
            panic!("Expected symbol for variable name, got: {:?}", var)
        }
    };

    let binding_val = &lexems[2];
    let value = match binding_val {
        Lexeme::Number(val) => Value::Number(*val),
        Lexeme::Symbol(val) => Value::Symbol(val.clone()),

        Lexeme::List(_) => eval(binding_val.clone(), env.clone()),

        Lexeme::None => Value::None,
    };

    env.borrow_mut().set_var(var_name, value);

    Value::None
}
