use crate::eval::{Value, eval};
use crate::reader::Lexeme;

use crate::Environment;
use std::cell::RefCell;
use std::rc::Rc;

// TODO -> needs to return Result
pub(super) fn try_eval_simple_arithmetics(
    lexems: Vec<Lexeme>,
    env: Rc<RefCell<Environment>>,
) -> Option<Value> {
    let head = &lexems[0];

    match head {
        Lexeme::Symbol(maybe_operator) => match maybe_operator.as_str() {
            "+" => {
                let res =
                    handle_builtin_arithmetics(lexems, |lhs, rhs| Value::Number(lhs + rhs), env);
                Some(res)
            }
            "*" => {
                let res =
                    handle_builtin_arithmetics(lexems, |lhs, rhs| Value::Number(lhs * rhs), env);
                Some(res)
            }
            "-" => {
                let res =
                    handle_builtin_arithmetics(lexems, |lhs, rhs| Value::Number(lhs - rhs), env);
                Some(res)
            }
            "/" => {
                let res =
                    handle_builtin_arithmetics(lexems, |lhs, rhs| Value::Number(lhs / rhs), env);
                Some(res)
            }
            _ => None,
        },
        _ => None,
    }
}

// TODO -> return Result
fn handle_builtin_arithmetics<T>(
    lexems: Vec<Lexeme>,
    handler: T,
    env: Rc<RefCell<Environment>>,
) -> Value
where
    T: FnOnce(isize, isize) -> Value,
{
    if lexems.len() != 3 {
        // TODO -> return Error
        todo!()
    }

    let lhs = lexems[1].clone();
    let rhs = lexems[2].clone();

    let res = match (&lhs, &rhs) {
        (Lexeme::Number(left_val), Lexeme::Number(right_val)) => handler(*left_val, *right_val),
        (Lexeme::List(_), Lexeme::Number(right_val)) => match eval(lhs, env) {
            Value::Number(lhs_val) => handler(lhs_val, *right_val),
            _ => {
                // TODO -> handle error
                todo!()
            }
        },
        (Lexeme::Number(left_val), Lexeme::List(_)) => match eval(rhs, env) {
            Value::Number(rhs_val) => handler(*left_val, rhs_val),
            _ => {
                // TODO -> handle error
                todo!()
            }
        },
        (Lexeme::List(_), Lexeme::List(_)) => {
            let lhs_reduction_res = eval(lhs, env.clone());
            let rhs_reduction_res = eval(rhs, env);

            match (lhs_reduction_res, rhs_reduction_res) {
                (Value::Number(left_val), Value::Number(right_val)) => handler(left_val, right_val),
                _ => {
                    // TODO -> handle error
                    todo!()
                }
            }
        }

        // -------------- -------------- Variables handling -------------- --------------

        // Try to handle variables
        (Lexeme::Symbol(maybe_var_lhs), Lexeme::Symbol(maybe_var_rhs)) => {
            let lhs_var = check_variable(maybe_var_lhs);
            let rhs_var = check_variable(maybe_var_rhs);

            match (lhs_var, rhs_var) {
                (Some(l_var), Some(r_var)) => {
                    let env_ref = env.borrow();

                    let lhs_val = env_ref.get_var(l_var.to_string());
                    let rhs_val = env_ref.get_var(r_var.to_string());

                    match (lhs_val, rhs_val) {
                        (Some(l_val), Some(r_val)) => match (l_val, r_val) {
                            (Value::Number(l_num), Value::Number(r_num)) => handler(l_num, r_num),
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    }
                }
                _ => unimplemented!(),
            }
        }

        // TODO -> how to handle when list is on one of the sides
        // TODO -> handle when symbol can be also on the left or on the right
        _ => {
            // TODO -> return error
            todo!()
        }
    };

    res
}

fn check_variable(s: &str) -> Option<&str> {
    if s.starts_with('$') { Some(s) } else { None }
}
