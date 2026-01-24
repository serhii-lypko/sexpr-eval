use crate::eval::Value;

// TODO -> needs to return Result
pub(super) fn try_eval_arithmetics(values: &Vec<Value>) -> Option<Value> {
    let head = &values[0];

    if values.len() != 3 {
        // TODO -> return Error
        todo!()
    }

    let lhs = &values[1];
    let rhs = &values[2];

    match head {
        Value::Symbol(maybe_operator) => match maybe_operator.as_str() {
            "+" => {
                let res = handle_builtin_arithmetics(lhs, rhs, |lhs, rhs| Value::Number(lhs + rhs));
                Some(res)
            }
            "*" => {
                let res = handle_builtin_arithmetics(lhs, rhs, |lhs, rhs| Value::Number(lhs * rhs));
                Some(res)
            }
            "-" => {
                let res = handle_builtin_arithmetics(lhs, rhs, |lhs, rhs| Value::Number(lhs - rhs));
                Some(res)
            }
            "/" => {
                let res = handle_builtin_arithmetics(lhs, rhs, |lhs, rhs| Value::Number(lhs / rhs));
                Some(res)
            }
            _ => None,
        },
        _ => None,
    }
}

// TODO -> return Result
fn handle_builtin_arithmetics<T>(lhs: &Value, rhs: &Value, handler: T) -> Value
where
    T: FnOnce(isize, isize) -> Value,
{
    match (lhs, rhs) {
        (Value::Number(l_val), Value::Number(r_val)) => handler(*l_val, *r_val),
        _ => {
            // TODO -> error handling
            todo!()
        }
    }
}
