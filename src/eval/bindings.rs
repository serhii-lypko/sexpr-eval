use std::rc::Rc;

use crate::Environment;
use crate::eval::{Value, eval};
use crate::reader::Lexeme;

pub(super) fn eval_variable_binding(lexems: Vec<Lexeme>, mut env: Rc<Environment>) -> Value {
    // if lexems.len() != 4 {
    //     // TODO -> return Error
    //     todo!()
    // }

    dbg!(lexems);

    // let cond = &lexems[1];
    // let lhs = &lexems[2];
    // let rhs = &lexems[3];

    // let is_truthy = eval_cond(cond.clone(), env.clone());

    // if is_truthy {
    //     eval(lhs.clone(), env)
    // } else {
    //     eval(rhs.clone(), env)
    // }

    todo!()
}
