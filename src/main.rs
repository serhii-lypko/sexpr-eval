mod eval;
mod reader;

use eval::eval;
use reader::read;

use std::collections::HashMap;

/*
    Interesting features to implement:
    - [x] Condition branches
    - [ ] Execution environment!
    - [ ] Variables
    - [ ] Functions, lambdas - practice De Bruijn indexes
    - [ ] Basic list manipulations
    - [ ] Quoting

    - [ ] Pretty printing
    - [ ] Stdout - display
    - [ ] Basic REPL
    - [ ] REPL with autocompletion based on simple prefix-trie


    Optimizations & improvements:
    - [ ] String interning
    - [ ] Tail recursion optimization
    - [ ] Trampoline (stack machine?)

    String interning: The technique of representing all strings which are equal by a pointer
    or ID that is unique to the contents of that strings, such that O(n) string equality
    check becomes a O(1) pointer equality check.


    Concurrency
    - [ ] Threaded runtime
*/

fn main() {
    let source = r#"
        (+ 10 12)
        (* 3 2)
    "#
    .to_string();

    // let source = r#"
    //     (define x 10)
    //     (define y 20)

    //     (+ x y)
    // "#
    // .to_string();

    // let eval_res = eval(lexems);
    // dbg!(&eval_res);

    let mut runtime = Runtime::new();
    runtime.run(source);
}

// TODO -> implement display

struct Environment {
    //
}

impl Environment {
    fn new() -> Self {
        Environment {}
    }
}

struct Runtime {}

impl Runtime {
    fn new() -> Self {
        Runtime {
            // TODO -> introduce shared Environment (with Rc?)
        }
    }

    fn run(&mut self, source: String) {
        let lexems = read(source);

        dbg!(lexems);
    }
}
