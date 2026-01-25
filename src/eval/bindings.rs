use std::cell::RefCell;
use std::rc::Rc;

use crate::Environment;
use crate::eval::{Value, eval};
use crate::reader::Lexeme;

// TODO -> tests
pub(super) fn set_variable_binding(lexems: Vec<Lexeme>, env: Rc<RefCell<Environment>>) -> Value {
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

    env.borrow_mut().set_var(&var_name, value);

    Value::None
}

// (def-fn add ($x $y) (+ $x $y))
// (add 10 12)
pub(super) fn set_function_binding(lexems: Vec<Lexeme>, env: Rc<RefCell<Environment>>) -> Value {
    if lexems.len() != 4 {
        // TODO -> return Error
        todo!()
    }

    let fn_name = lexems[1].clone();
    let fn_name = match fn_name {
        Lexeme::Symbol(fn_name) => fn_name,
        _ => {
            // TODO -> return type error
            todo!()
        }
    };

    let parameters = lexems[2].clone();
    let body = lexems[3].clone();

    env.borrow_mut().set_fn_bindings(fn_name, parameters, body);

    Value::None
}

// (add (+ 9 1) 7) -> will recieve [add, 10, 7] as values.
pub(super) fn eval_function(values: Vec<Value>, env: Rc<RefCell<Environment>>) -> Value {
    let fn_descriptor = match values[0].clone() {
        Value::Symbol(descr) => descr,
        _ => {
            // TODO -> return error
            todo!()
        }
    };

    let parameter_values: Vec<Value> = values.into_iter().skip(1).collect();

    match env.borrow().get_fn_bindings(fn_descriptor) {
        Some(bindings) => {
            if bindings.parameters.len() != parameter_values.len() {
                // TODO -> return error
                todo!()
            }

            let env = Environment::build_with_var_bindings(bindings.parameters, parameter_values);
            eval(bindings.body, Rc::new(RefCell::new(env)))
        }
        None => todo!(),
    }
}
