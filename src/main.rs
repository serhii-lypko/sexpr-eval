mod eval;
mod reader;

// use eval::eval;

/*
    Interesting features to implement:
    - [ ] Condition branches
    - [ ] Variables
    - [ ] Functions
    - [ ] Basic list manipulations
    - [ ] Quoting

    - [ ] Pretty printing
    - [ ] Basic REPL with autocompletion based on simple prefix-trie


    Optimizations & improvements:
    - [ ] Result types and error handling
    - [ ] Module system
    - [ ] More tests
    - [ ] Tail recursion optimization
    - [ ] Trampoline (stack machine?)
*/

fn main() {
    // let source = "(+ 3 (* 12 (/ 800 2)))".to_string();
    let source = "(if (> 1 2) a b)".to_string();

    let lexems = read(source);

    // dbg!(lexems);

    let eval_res = eval(lexems);
    dbg!(&eval_res);
}

// TODO -> for now eval result can be only usize. But need to evolve to more complex structure.
// #[derive(Debug)]
// struct EvalRes(isize);

#[derive(Debug)]
enum ResultValue {
    Number(isize),
    Symbol(String),
}

/*
    TODO -> implement with:
    - [ ] Basic recursion
    - [ ] Trampoline
*/
// TODO -> needs to retun Result
fn eval(lexeme: Lexeme) -> ResultValue {
    match lexeme {
        Lexeme::Number(number) => todo!(),
        Lexeme::Symbol(_) => todo!(),
        Lexeme::List(lexemes) => {
            if let Some(res) = try_eval_simple_arithmetics(lexemes) {
                return res;
            }

            todo!()
        }
    }
}

// TODO -> needs to return Result
fn try_eval_simple_arithmetics(lexems: Vec<Lexeme>) -> Option<ResultValue> {
    let head = &lexems[0];

    match head {
        Lexeme::Symbol(maybe_operator) => match maybe_operator.as_str() {
            "+" => {
                let res =
                    handle_builtin_arithmetics(lexems, |lhs, rhs| ResultValue::Number(lhs + rhs));
                Some(res)
            }
            "*" => {
                let res =
                    handle_builtin_arithmetics(lexems, |lhs, rhs| ResultValue::Number(lhs * rhs));
                Some(res)
            }
            "-" => {
                let res =
                    handle_builtin_arithmetics(lexems, |lhs, rhs| ResultValue::Number(lhs - rhs));
                Some(res)
            }
            "/" => {
                let res =
                    handle_builtin_arithmetics(lexems, |lhs, rhs| ResultValue::Number(lhs / rhs));
                Some(res)
            }
            _ => {
                todo!()
            }
        },
        _ => None,
    }
}

// TODO -> needs to return Result
fn handle_builtin_arithmetics<T>(lexems: Vec<Lexeme>, handler: T) -> ResultValue
where
    T: FnOnce(isize, isize) -> ResultValue,
{
    if lexems.len() != 3 {
        // TODO -> return Error
        todo!()
    }

    let lhs = lexems[1].clone();
    let rhs = lexems[2].clone();

    let res = match (&lhs, &rhs) {
        (Lexeme::Number(left_val), Lexeme::Number(right_val)) => handler(*left_val, *right_val),
        (Lexeme::List(_), Lexeme::Number(right_val)) => match eval(lhs) {
            ResultValue::Number(lhs_val) => handler(lhs_val, *right_val),
            ResultValue::Symbol(_) => {
                // TODO -> handle error
                todo!()
            }
        },
        (Lexeme::Number(left_val), Lexeme::List(_)) => match eval(rhs) {
            ResultValue::Number(rhs_val) => handler(*left_val, rhs_val),
            ResultValue::Symbol(_) => {
                // TODO -> handle error
                todo!()
            }
        },
        (Lexeme::List(_), Lexeme::List(_)) => {
            let lhs_reduction_res = eval(lhs);
            let rhs_reduction_res = eval(rhs);

            match (lhs_reduction_res, rhs_reduction_res) {
                (ResultValue::Number(left_val), ResultValue::Number(right_val)) => {
                    handler(left_val, right_val)
                }
                _ => {
                    // TODO -> handle error
                    todo!()
                }
            }
        }
        _ => {
            // TODO -> return error
            todo!()
        }
    };

    res
}

// TODO -> handle floats
#[derive(Debug, Clone, PartialEq, Eq)]
enum Lexeme {
    Number(isize),
    Symbol(String),
    List(Vec<Lexeme>),
}

fn read(source: String) -> Lexeme {
    let (lexeme_res, _) = handle_read(&source);
    lexeme_res
}

// TODO -> error handling and handling of invalid inputs
// TODO -> handle negative numbers
// TODO -> handle newlines
// TODO -> handle errors
fn handle_read(source: &str) -> (Lexeme, usize) {
    let mut res: Vec<Lexeme> = vec![];

    let mut remaining: &str = &source;
    let mut offset = 0;

    while let Some(ch) = remaining.chars().next() {
        let char_bytes = ch.len_utf8();

        match ch {
            '(' => {
                let (list, offset_advance) = handle_read(&remaining[char_bytes..]);
                res.push(list);
                remaining = &remaining[(char_bytes + offset_advance)..];
                offset += offset_advance + char_bytes;
                continue;
            }
            ')' => return (Lexeme::List(res), offset + 1),
            ' ' => {
                remaining = &remaining[char_bytes..];
                offset += char_bytes;
                continue;
            }
            _ => {
                if ch.is_numeric() {
                    // let (number, offset_advance) = process_number(&remaining);
                    let (symbol_lexeme, offset_advance) =
                        process_lexeme(&remaining, |&&b| b.is_ascii_digit());

                    // FIXME: unwrap
                    let number_lexeme = symbol_lexeme.parse::<isize>().unwrap();
                    res.push(Lexeme::Number(number_lexeme));
                    remaining = &remaining[offset_advance..];
                    offset += offset_advance;
                    continue;
                }

                // All the reset should be interpreted as symbol lexems
                let (symbol_lexeme, offset_advance) =
                    process_lexeme(&remaining, |&&b| b != b' ' && b != b')');
                res.push(Lexeme::Symbol(symbol_lexeme.to_string()));
                remaining = &remaining[offset_advance..];
                offset += offset_advance;
                continue;
            }
        }
    }

    // Eliminating redundant nested structure
    let output = if res.len() == 1 {
        res[0].clone()
    } else {
        Lexeme::List(res)
    };

    (output, offset)
}

fn process_lexeme<T>(source: &str, matcher: T) -> (&str, usize)
where
    T: FnMut(&&u8) -> bool,
{
    let bytes = source.as_bytes();
    let offset = bytes.iter().take_while(matcher).count();

    let lexeme_bytes = &bytes[..offset];
    let lexeme_str = unsafe { std::str::from_utf8_unchecked(lexeme_bytes) };

    (lexeme_str, offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_simple_addition() {
        let source = "(+ 10 15)".to_string();

        let expected = Lexeme::List(vec![
            Lexeme::Symbol("+".to_string()),
            Lexeme::Number(10),
            Lexeme::Number(15),
        ]);
        let res = read(source);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_read_nested_addition() {
        let source = "(+ (* 3 2) 15)".to_string();

        let expected = Lexeme::List(vec![
            Lexeme::Symbol("+".to_string()),
            Lexeme::List(vec![
                Lexeme::Symbol("*".to_string()),
                Lexeme::Number(3),
                Lexeme::Number(2),
            ]),
            Lexeme::Number(15),
        ]);
        let res = read(source);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_read_multiple_operations() {
        let source = "(+ 5 10 20)".to_string();

        let expected = Lexeme::List(vec![
            Lexeme::Symbol("+".to_string()),
            Lexeme::Number(5),
            Lexeme::Number(10),
            Lexeme::Number(20),
        ]);
        let res = read(source);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_read_deeply_nested() {
        let source = "(+ (* 2 (/ 10 5)) 3)".to_string();

        let expected = Lexeme::List(vec![
            Lexeme::Symbol("+".to_string()),
            Lexeme::List(vec![
                Lexeme::Symbol("*".to_string()),
                Lexeme::Number(2),
                Lexeme::List(vec![
                    Lexeme::Symbol("/".to_string()),
                    Lexeme::Number(10),
                    Lexeme::Number(5),
                ]),
            ]),
            Lexeme::Number(3),
        ]);
        let res = read(source);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_read_empty_list() {
        let source = "()".to_string();

        let expected = Lexeme::List(vec![]);
        let res = read(source);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_eval_simple_addition() {
        let source = "(+ 10 15)".to_string();
        let lexeme = read(source);
        let result = eval(lexeme);

        match result {
            ResultValue::Number(n) => assert_eq!(n, 25),
            _ => panic!("Expected number result"),
        }
    }

    #[test]
    fn test_eval_simple_multiplication() {
        let source = "(* 7 6)".to_string();
        let lexeme = read(source);
        let result = eval(lexeme);

        match result {
            ResultValue::Number(n) => assert_eq!(n, 42),
            _ => panic!("Expected number result"),
        }
    }

    #[test]
    fn test_eval_nested_arithmetic() {
        let source = "(+ (* 3 2) 15)".to_string();
        let lexeme = read(source);
        let result = eval(lexeme);

        match result {
            ResultValue::Number(n) => assert_eq!(n, 21), // 3*2=6, 6+15=21
            _ => panic!("Expected number result"),
        }
    }

    #[test]
    fn test_eval_arithmetics_complex_nested() {
        let source = "(* (+ 2 3) (- 10 4))".to_string();
        let lexeme = read(source);
        let result = eval(lexeme);

        match result {
            ResultValue::Number(n) => assert_eq!(n, 30), // (2+3)*(10-4) = 5*6 = 30
            _ => panic!("Expected number result"),
        }
    }
}
