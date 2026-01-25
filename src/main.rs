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
    - [x] Execution environment
    - [x] Variables
    - [x] Functions
    - [ ] Basic list manipulations
    - [ ] Quoting

    - [ ] Pretty printing
    - [ ] Stdout - display
    - [ ] Basic REPL
    - [ ] REPL with autocompletion based on simple prefix-trie

    - [ ] Heterogeneous data


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

    // let source = r#"
    //     (def-var $x (* (+ 2 3) (- 10 4)))
    //     (+ $x 11)
    // "#
    // .to_string();

    // let source = r#"
    //     (def-fn add ($x $y) (* $x $y))
    //     (add (+ 9 1) 7)
    // "#
    // .to_string();

    // NOTE -> identity function
    // let source = r#"
    //     (def-fn add ($k) $k)
    // "#
    // .to_string();

    let source = r#"
        (def-fn sum ($x $y)
            (+ $x $y))

        (def-fn
            add-cond ($x $y)
                (if (< $x $y)
                    (+ $x $y)
                    (* $x $y)))

        (add-cond (sum (sum 100 200) 3) 7)
    "#
    .to_string();

    // TODO -> what about recursion? ⭐️
    // TODO -> factorial
    // TODO -> fib

    let mut runtime = Runtime::new();
    runtime.run(source);
}

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

// TODO -> tests for variables
// TODO -> tests for functions

// TODO -> tests for runtime

// (def-fn add ($x $y) (+ $x $y))
// (add 10 12)

#[derive(Debug)]
pub(crate) struct Environment {
    var_bindings_storage: HashMap<String, Value>,

    // TODO -> &str instead of String?
    fn_bindings_storage: HashMap<String, FnBindings>,
}

#[derive(Debug, Clone)]
struct FnBindings {
    parameters: Vec<String>,
    body: Lexeme,
}

impl Environment {
    fn new() -> Self {
        Environment {
            var_bindings_storage: HashMap::new(),
            fn_bindings_storage: HashMap::new(),
        }
    }

    fn build_with_var_bindings(parameters: Vec<String>, parameter_values: Vec<Value>) -> Self {
        Environment {
            // Fairly simple bindings execution
            var_bindings_storage: parameters.into_iter().zip(parameter_values).collect(),
            fn_bindings_storage: HashMap::new(),
        }
    }

    fn get_fn_bindings(&self, fn_descriptor: String) -> Option<FnBindings> {
        self.fn_bindings_storage.get(&fn_descriptor).cloned()
    }

    fn get_var(&self, key: &str) -> Option<Value> {
        self.var_bindings_storage.get(key).cloned()
    }

    fn set_var(&mut self, key: &str, value: Value) {
        self.var_bindings_storage.insert(key.to_string(), value);
    }

    fn set_fn_bindings(&mut self, fn_name: String, parameters: Lexeme, body: Lexeme) {
        // Positioned variables
        let mut extracted_parameters: Vec<String> = vec![];

        match parameters {
            Lexeme::List(parameters) => {
                for param_lexeme in parameters {
                    match param_lexeme {
                        Lexeme::Symbol(var) => {
                            extracted_parameters.push(var);
                        }
                        _ => {
                            // TODO -> return error of malformed parameters
                            todo!()
                        }
                    }
                }
            }
            _ => {
                // TODO -> return error of malformed parameters
                todo!()
            }
        };

        let bindings = FnBindings {
            parameters: extracted_parameters,
            body,
        };

        self.fn_bindings_storage.insert(fn_name, bindings);
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
