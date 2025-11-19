mod lexer;
mod models;

use lexer::Lexer;

// - literal values: numbers, strings, booleans
// - basic arithmetics: +, -, *, /
// - lazy if statements with basic conditions: <, >, <=, >=,  ==, !=
// - while loop (using recursion?)
// - handling comments
// - handling newlines and whitespaces
//
// - Bonus: REPL!
// - Bonus: pretty-print

/*
    *Implementation overview
    - complete parsing (incl bindings and booleans)
    - basic arithmetic eval
    -

*/

fn main() {
    // let source = "(+ 353 1222)".to_string();
    // let source = r#"(+ 10 "hello")"#.to_string();
    // let source = r#"(10 def "awesome" if "hello" 21)"#.to_string();

    // let source = r#"(def _name "John")"#.to_string();
    // let source = r#"(if (< 10 12) (print "hey"))"#.to_string();
    let source = r#"(if (<= 10 12) (print "hey"))"#.to_string();

    let mut lexer = Lexer::new(source);

    match lexer.parse() {
        Ok(tokens) => {
            dbg!(tokens);
        }
        Err(e) => {
            eprintln!("Lexing error: {:?}", e);
        }
    }
}
