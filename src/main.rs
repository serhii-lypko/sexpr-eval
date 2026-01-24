mod eval;
mod reader;

use eval::eval;
use reader::read;

use crate::eval::Value;
use reader::Lexeme;

// TODO -> how these works
use std::mem::{replace, swap, take};

use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

/*
    Interesting features to implement:
    - [x] Condition branches
    - [ ] Execution environment!
    - [ ] Variables
    - [ ] Functions, lambdas - practice De Bruijn indexes?rfh
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

// TODO -> which design patterns could be applied?

// TODO -> fix comments (now eliminate entire source input)
fn main() {
    // let source = r#"
    //     (def-var $x 10)
    //     (def-var $y 15)

    //     (* (+ $x $y) 12)
    // "#
    // .to_string();

    let source = r#"
        (def-var $x (* (+ 2 3) (- 10 4)))
        (+ $x 11)
    "#
    .to_string();

    let mut runtime = Runtime::new();
    runtime.run(source);
}

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

// TODO -> what is the lifetime of binded variables?
#[derive(Debug)]
pub(crate) struct Environment {
    bindings_storage: HashMap<String, Value>,
}

impl Environment {
    fn new() -> Self {
        Environment {
            bindings_storage: HashMap::new(),
        }
    }

    fn set_var(&mut self, key: String, value: Value) {
        let insert_res = self.bindings_storage.insert(key, value);

        //
    }

    fn get_var(&self, key: String) -> Option<Value> {
        self.bindings_storage.get(&key).cloned()
    }
}

struct Runtime {
    env: Rc<RefCell<Environment>>,
}

impl Runtime {
    fn new() -> Self {
        let env = Environment::new();

        Runtime {
            env: Rc::new(RefCell::new(env)),
        }
    }

    fn run(&mut self, source: String) {
        let lexeme = read(source);

        // TODO -> not entirely happy with all that clones here
        if let Lexeme::List(expressions) = &lexeme {
            if lexeme.has_inner_lists() {
                for expr in expressions {
                    let eval_res = eval(expr.clone(), self.env.clone());

                    dbg!(eval_res);
                }

                return;
            }

            let eval_res = eval(lexeme, self.env.clone());
            // dbg!(eval_res);
        }
    }
}
