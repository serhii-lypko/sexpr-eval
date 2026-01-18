mod eval;
mod reader;

use eval::eval;
use reader::read;

/*
    Interesting features to implement:
    - [x] Condition branches
    - [ ] Execution runtime!
    - [ ] Variables
    - [ ] Functions, lambdas - practice De Bruijn indexes
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
    let source = "(if (> 1 2) (+ 13 12) (if (> 12 13) 2 (* 12 12)))".to_string();

    let lexems = read(source);
    // dbg!(lexems);

    let eval_res = eval(lexems);
    dbg!(&eval_res);
}
