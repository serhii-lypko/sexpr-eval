mod eval;
mod reader;

// use eval::eval;

/*
    TODO -> features
    - Pretty printing (but print what exactly? AST?)
    - Basic REPL
    - Repl with autocompletion based on simple prefix-trie
    -------------------------------------------------------

*/

fn main() {
    // let source = "(+ 512 3)".to_string();
    let source = "123 4".to_string();

    // let source = r#"(if (< 10 12) (print "hey"))"#.to_string();
    // let source = r#"(if (<= 10 12) (print "hey"))"#.to_string();

    read(source);

    //
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
                let (symbol_lexeme, offset_advance) = process_lexeme(&remaining, |&&b| b != b' ');
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

    // #[test]
    // fn playground() {
    //     let foo = "hello world".to_string();

    //     let ptr = foo.as_ptr();

    //     unsafe {
    //         let slice = slice::from_raw_parts(ptr, 4);

    //         dbg!(String::from_utf8(slice.to_vec()).unwrap());
    //     };
    // }

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
    fn test_read_empty_list() {
        let source = "()".to_string();

        let expected = Lexeme::List(vec![]);
        let res = read(source);

        assert_eq!(expected, res);
    }
}
