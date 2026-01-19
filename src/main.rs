mod eval;
mod reader;

use eval::eval;
use reader::read;

use crate::eval::Value;
use reader::Lexeme;
use std::collections::HashMap;
use std::rc::Rc;

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

// TODO -> fix comments (now eliminate entire source input)
fn main() {
    // let source = r#"
    //     (+ 10
    //         (* 3 2))

    //     (* 3 2)
    // "#
    // .to_string();

    // let source = r#"
    //     (def-var x 10)
    //     (def-var y 20)

    //     (+ x y)
    // "#
    // .to_string();

    let source = r#"
        (def-var x 10)
        (def-var y 10)
    "#
    .to_string();

    // let eval_res = eval(lexems);
    // dbg!(&eval_res);

    let mut runtime = Runtime::new();
    runtime.run(source);
}

// TODO -> implement display

pub(crate) struct Environment {
    bindings_storage: HashMap<String, Value>,
}

impl Environment {
    fn new() -> Self {
        Environment {
            bindings_storage: HashMap::new(),
        }
    }
}

struct Runtime {
    env: Rc<Environment>,
}

impl Runtime {
    fn new() -> Self {
        let env = Environment::new();

        Runtime { env: Rc::new(env) }
    }

    fn run(&mut self, source: String) {
        let lexeme = read(source);

        // TODO -> not entirely happy with all that clones here
        if let Lexeme::List(expressions) = &lexeme {
            if lexeme.has_inner_lists() {
                for expr in expressions {
                    let eval_res = eval(expr.clone(), self.env.clone());
                }

                return;
            }

            let eval_res = eval(lexeme, self.env.clone());
        }
    }
}
