// - literal values: numbers, strings, booleans
// - basic arithmetics: +, -, *, /
// - lazy if statements with basic conditions: <, >, <=, >=,  ==, !=
// - while loop (using recursion?)
// - handling comments
// - handling newlines and whitespaces

// - Bonus: REPL!
// - Bonus: pretty-print

/*
    *Implementation overview
    - complete parsing (incl bindings and booleans)
    - basic arithmetic eval
    -

*/

#[derive(Debug)]
enum ParseError {
    InvalidOperatorDelimeter,
    InvalidLexemeDelimeter,
    InvalidLexeme,
    InvalidNumber,
}

type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
enum Token {
    // Base
    OpenParen,
    CloseParen,
    Semicolon,

    // Arithmetic
    Plus,
    Minus,
    Mult,
    Div,

    // Literals
    String(String),
    Int(isize),
    Bool(bool),

    Identifier(String),

    // Keywords
    If,
    Def,
    And,
    Not,

    // TODO -> recursive collections?
    List,
}

pub(crate) struct Lexer {
    cursor: usize,

    // TODO -> rethinking to work with string indices?
    source: Vec<char>,
    output: Vec<Token>,
}

/// Given implementation is rather imperative and fundamentally relies on
/// state modification during parsing
impl Lexer {
    pub(crate) fn new(source: String) -> Self {
        Lexer {
            cursor: 0,
            source: source.chars().collect(),
            output: vec![],
        }
    }

    pub(crate) fn parse(&mut self) -> ParseResult<Vec<Token>> {
        while self.cursor < self.source.len() {
            let char = self.source[self.cursor];

            // TODO -> handle EOF

            match char {
                '(' => self.parse_base(Token::OpenParen),
                ')' => self.parse_base(Token::CloseParen),
                ';' => self.parse_base(Token::Semicolon),

                // TODO -> handle comments (incl multiline)
                // TODO -> handle new lines
                ' ' => self.advance(),

                '+' => self.try_parse_arithmetic(Token::Plus)?,
                '-' => {
                    // TODO -> needs to handle negative numbers as single atoms
                    self.try_parse_arithmetic(Token::Minus)?;
                }
                '*' => self.try_parse_arithmetic(Token::Mult)?,
                '/' => self.try_parse_arithmetic(Token::Div)?,

                // TODO -> conditions: ==, !=, <, >, <=, >=

                //
                _ => self.try_parse_lexeme()?,
            }
        }

        Ok(std::mem::take(&mut self.output))
    }

    fn parse_base(&mut self, token: Token) {
        self.advance();
        self.output.push(token);
    }

    fn try_parse_arithmetic(&mut self, token: Token) -> ParseResult<()> {
        self.advance();
        self.validate_whitespace_delimeter()?;
        self.output.push(token);

        Ok(())
    }

    // TODO -> need to carefully test this function
    /// Collecting literals and keywords
    fn try_parse_lexeme(&mut self) -> ParseResult<()> {
        self.try_parse_number()?;

        if self.peek() == '"' {
            self.try_parse_string()?;
        } else {
            self.try_parse_keyword_or_identifier()?;
        }

        // TODO -> try_parse_identifier

        // TODO -> collect booleans

        Ok(())
    }

    fn try_parse_string(&mut self) -> ParseResult<()> {
        let mut buffer = String::new();

        self.advance();

        while self.peek() != '"' {
            if self.is_at_end() {
                break;
            }

            buffer.push(self.peek());
            self.advance();
        }

        self.advance();
        self.validate_lexeme_delimeter()?;
        self.output.push(Token::String(buffer));

        Ok(())
    }

    /// Given parser does not support fractional parts
    fn try_parse_number(&mut self) -> ParseResult<()> {
        if self.peek().is_ascii_digit() {
            let mut result = 0;
            let mut collected_digits: Vec<isize> = vec![];

            while self.cursor < self.source.len() && self.peek().is_ascii_digit() {
                let digit = (self.peek() as u8 - b'0') as isize;
                collected_digits.push(digit);
                self.advance();
            }

            if collected_digits.is_empty() {
                return Err(ParseError::InvalidLexeme);
            }

            let mut max_exponent = collected_digits.len() - 1;
            for digit in collected_digits {
                let with_position = digit * 10_isize.pow(max_exponent as u32);

                // Stops at the minimum value (0 for unsigned types) instead of wrapping/overflowing
                max_exponent = max_exponent.saturating_sub(1);
                result += with_position;
            }

            self.validate_lexeme_delimeter()?;
            self.output.push(Token::Int(result));
        }

        Ok(())
    }

    /// Given parser supports only alphabetic characters and underscore for binding names
    fn try_parse_keyword_or_identifier(&mut self) -> ParseResult<()> {
        let is_alpha =
            |c: char| -> bool { (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_' };

        let mut buffer = String::new();

        if is_alpha(self.peek()) {
            while is_alpha(self.peek()) {
                if self.is_at_end() {
                    break;
                }

                buffer.push(self.peek());
                self.advance();
            }
        }

        if buffer.is_empty() {
            return Err(ParseError::InvalidLexeme);
        }

        match buffer.as_str() {
            "def" => self.output.push(Token::Def),
            "if" => self.output.push(Token::If),
            // TODO -> and
            // TODO -> not
            _ => self.output.push(Token::Identifier(buffer)),
        }

        self.validate_whitespace_delimeter()?;

        Ok(())
    }

    /// Making sure every operator is followed by whitespace
    fn validate_whitespace_delimeter(&self) -> ParseResult<()> {
        if self.peek() != ' ' {
            return Err(ParseError::InvalidOperatorDelimeter);
        }

        Ok(())
    }

    fn validate_lexeme_delimeter(&self) -> ParseResult<()> {
        if !matches!(self.peek(), ' ' | ')' | ']' | '}') {
            return Err(ParseError::InvalidLexemeDelimeter);
        }

        Ok(())
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source[self.cursor]
    }

    fn advance(&mut self) {
        self.cursor += 1;
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.source.len()
    }
}

fn main() {
    // let source = "(+ 353 1222)".to_string();
    // let source = r#"(+ 10 "hello")"#.to_string();
    // let source = r#"(10 def "awesome" if "hello" 21)"#.to_string();
    let source = r#"(def _name "John")"#.to_string();

    let mut lexer = Lexer::new(source);
    let lexing_res = lexer.parse();

    dbg!(lexing_res);
}
