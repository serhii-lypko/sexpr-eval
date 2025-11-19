mod eval;
mod lexer;
mod models;
mod parser;

use eval::eval;
use lexer::Lexer;
use parser::build_ast;

/*
    * Memory model:
    - Garbage collected
    - Cons cells (linked lists) as fundamental structure
    - Symbol tables for variable/function lookups

*/

// TODO -> pretty print (but for which stage exactly?)

fn main() {
    let source = "(+ 10 15)".to_string();

    // let source = r#"(if (< 10 12) (print "hey"))"#.to_string();
    // let source = r#"(if (<= 10 12) (print "hey"))"#.to_string();

    let mut lexer = Lexer::new(source);

    match lexer.lex() {
        Ok(tokens) => {
            dbg!(&tokens);

            let ast = build_ast(tokens);
        }
        Err(e) => {
            eprintln!("Lexing error: {:?}", e);
        }
    }
}
