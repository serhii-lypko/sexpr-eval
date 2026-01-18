use crate::reader::read;
use crate::eval::{eval, Value};

#[test]
fn test_eval_simple_addition() {
    let source = "(+ 10 15)".to_string();
    let lexeme = read(source);
    let result = eval(lexeme);

    match result {
        Value::Number(n) => assert_eq!(n, 25),
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_eval_simple_multiplication() {
    let source = "(* 7 6)".to_string();
    let lexeme = read(source);
    let result = eval(lexeme);

    match result {
        Value::Number(n) => assert_eq!(n, 42),
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_eval_nested_arithmetic() {
    let source = "(+ (* 3 2) 15)".to_string();
    let lexeme = read(source);
    let result = eval(lexeme);

    match result {
        Value::Number(n) => assert_eq!(n, 21), // 3*2=6, 6+15=21
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_eval_arithmetics_complex_nested() {
    let source = "(* (+ 2 3) (- 10 4))".to_string();
    let lexeme = read(source);
    let result = eval(lexeme);

    match result {
        Value::Number(n) => assert_eq!(n, 30), // (2+3)*(10-4) = 5*6 = 30
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_eval_if_true_condition() {
    let source = "(if (> 5 3) 42 99)".to_string();
    let lexeme = read(source);
    let result = eval(lexeme);

    match result {
        Value::Number(n) => assert_eq!(n, 42), // 5 > 3 is true, so return 42
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_eval_if_false_condition() {
    let source = "(if (> 3 5) 42 99)".to_string();
    let lexeme = read(source);
    let result = eval(lexeme);

    match result {
        Value::Number(n) => assert_eq!(n, 99), // 3 > 5 is false, so return 99
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_eval_nested_if() {
    let source = "(if (> 1 2) (+ 13 12) (if (> 12 13) 2 (* 12 12)))".to_string();
    let lexeme = read(source);
    let result = eval(lexeme);

    match result {
        Value::Number(n) => assert_eq!(n, 144), // 1 > 2 is false, so eval (if (> 12 13) 2 (* 12 12))
                                                 // 12 > 13 is false, so eval (* 12 12) = 144
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_eval_if_with_arithmetic_condition() {
    let source = "(if (< (+ 2 3) 10) (* 4 5) (/ 100 2))".to_string();
    let lexeme = read(source);
    let result = eval(lexeme);

    match result {
        Value::Number(n) => assert_eq!(n, 20), // (+ 2 3) = 5, 5 < 10 is true, so (* 4 5) = 20
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_eval_if_with_nested_arithmetic() {
    let source = "(if (>= (* 2 3) 6) (+ 10 (* 2 5)) 0)".to_string();
    let lexeme = read(source);
    let result = eval(lexeme);

    match result {
        Value::Number(n) => assert_eq!(n, 20), // (* 2 3) = 6, 6 >= 6 is true, so (+ 10 (* 2 5)) = 10 + 10 = 20
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_eval_comparison_operators() {
    let test_cases = vec![
        ("(> 5 3)", true),
        ("(< 3 5)", true),
        ("(>= 6 6)", true),
        ("(<= 4 7)", true),
        ("(== 8 8)", true),
        ("(!= 5 7)", true),
        ("(> 3 5)", false),
        ("(< 5 3)", false),
        ("(>= 4 6)", false),
        ("(<= 7 4)", false),
        ("(== 5 7)", false),
        ("(!= 8 8)", false),
    ];

    for (source, expected) in test_cases {
        let full_source = format!("(if {} 1 0)", source);
        let lexeme = read(full_source);
        let result = eval(lexeme);
        
        match result {
            Value::Number(n) => {
                let actual = n == 1;
                assert_eq!(actual, expected, "Failed for expression: {}", source);
            }
            _ => panic!("Expected number result for: {}", source),
        }
    }
}