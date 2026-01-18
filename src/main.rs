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
    - [ ] Basic REPL
    - [ ] REPL with autocompletion based on simple prefix-trie


    Optimizations & improvements:
    - [ ] Result types and error handling
    - [ ] Module system
    - [ ] More tests
    - [ ] Tail recursion optimization
    - [ ] Trampoline (stack machine?)
*/

fn main() {
    // let source = r#"
    //     (define x 10)
    //     (define y (* x 2))
    // "#
    // .to_string();

    let source = r#"
        (define x 10)
        (define y 20)

        (+ x y)
    "#
    .to_string();

    let lexems = read(source);
    // dbg!(lexems);

    let eval_res = eval(lexems);
    dbg!(&eval_res);
}
