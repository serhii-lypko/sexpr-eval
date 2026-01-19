use crate::eval::{Value, eval};
use crate::reader::Lexeme;

use crate::Environment;
use std::rc::Rc;

// (if (< 3 5) a b)
pub(super) fn eval_lazy_if(lexems: Vec<Lexeme>, mut env: Rc<Environment>) -> Value {
    if lexems.len() != 4 {
        // TODO -> return Error
        todo!()
    }

    let cond = &lexems[1];
    let lhs = &lexems[2];
    let rhs = &lexems[3];

    let is_truthy = eval_cond(cond.clone(), env.clone());

    if is_truthy {
        eval(lhs.clone(), env)
    } else {
        eval(rhs.clone(), env)
    }
}

// TODO -> return Result
fn eval_cond(cond: Lexeme, mut env: Rc<Environment>) -> bool {
    match cond {
        Lexeme::List(lexemes) => {
            if lexemes.len() != 3 {
                // TODO -> return Err
                todo!()
            }

            let cond_head = &lexemes[0];
            let lhs = &lexemes[1];
            let rhs = &lexemes[2];

            let lhs_reduction_res = eval(lhs.clone(), env.clone());
            let rhs_reduction_res = eval(rhs.clone(), env);

            match cond_head {
                Lexeme::Symbol(symbol_head) => match symbol_head.as_str() {
                    // Expected to compare only numbers
                    ">" => handle_bultin_comparison(
                        lhs_reduction_res,
                        rhs_reduction_res,
                        |left_val, right_val| left_val > right_val,
                    ),
                    "<" => handle_bultin_comparison(
                        lhs_reduction_res,
                        rhs_reduction_res,
                        |left_val, right_val| left_val < right_val,
                    ),
                    ">=" => handle_bultin_comparison(
                        lhs_reduction_res,
                        rhs_reduction_res,
                        |left_val, right_val| left_val >= right_val,
                    ),
                    "<=" => handle_bultin_comparison(
                        lhs_reduction_res,
                        rhs_reduction_res,
                        |left_val, right_val| left_val <= right_val,
                    ),

                    // Equality comparison applicable to all Value types
                    "==" => lhs_reduction_res == rhs_reduction_res,
                    "!=" => lhs_reduction_res != rhs_reduction_res,
                    _ => todo!(),
                },
                _ => todo!(),
            }
        }
        _ => {
            // TODO -> return Err
            todo!()
        }
    }
}

fn handle_bultin_comparison<T>(lhs: Value, rhs: Value, handler: T) -> bool
where
    T: FnOnce(isize, isize) -> bool,
{
    match (lhs, rhs) {
        (Value::Number(left_val), Value::Number(right_val)) => {
            return handler(left_val, right_val);
        }
        _ => {
            // TODO -> retunr Err
            todo!()
        }
    }
}
