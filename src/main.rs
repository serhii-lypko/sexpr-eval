// - literal values: numbers, strings, booleans
// - basic arithmetics: +, -, *, /
// - lazy if statements with basic conditions: <, >, <=, >=,  ==, !=
// - while loop (using recursion?)
// - handling comments
// - handling newlines and whitespaces

// - Bonus: REPL!
// - Bonus: pretty-print

/*
    (+ 3 8)

    (+ (- 7 3) 4)

    (if (> 5 3) "bigger" "smaller")

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

    // Keywords
    If,
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
        self.validate_operator_delimeter()?;
        self.output.push(token);

        Ok(())
    }

    /// Collecting literals and keywords
    fn try_parse_lexeme(&mut self) -> ParseResult<()> {
        // TODO -> check is string

        self.try_parse_number()?;

        // TODO -> collect keywords

        // TODO -> collect booleans

        Ok(())
    }

    fn parse_string(&mut self) {
        //
    }

    /// Given parser & interpreter does not support fractional parts
    fn try_parse_number(&mut self) -> ParseResult<()> {
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

        Ok(())
    }

    /// Making sure every operator is followed by whitespace
    fn validate_operator_delimeter(&self) -> ParseResult<()> {
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
        self.source[self.cursor]
    }

    fn advance(&mut self) {
        self.cursor += 1;
    }
}

fn main() {
    let source = "(+ 353 1222)".to_string();

    let mut lexer = Lexer::new(source);
    let lexing_res = lexer.parse();

    dbg!(lexing_res);
}
