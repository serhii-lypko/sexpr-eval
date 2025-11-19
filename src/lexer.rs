use crate::models::Token;

// TODO -> it would be really nice to have position for debugging
#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidOperatorDelimeter,
    InvalidLexemeDelimeter,
    InvalidLexeme,
    InvalidNumber,
}

pub type ParseResult<T> = Result<T, ParseError>;

pub struct Lexer {
    cursor: usize,

    // TODO -> implement using small vec?
    source: Vec<char>,

    output: Vec<Token>,
}

/// Given implementation is rather imperative and fundamentally relies on
/// state modification during parsing
impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            cursor: 0,
            source: source.chars().collect(),
            output: vec![],
        }
    }

    pub fn parse(&mut self) -> ParseResult<Vec<Token>> {
        while self.cursor < self.source.len() {
            let char = self.source[self.cursor];

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

                '=' => self.parse_condition(Token::Equal, Token::EqualEqual),
                '!' => {
                    if self.match_next('=') {
                        self.output.push(Token::NotEuqal);
                        self.advance();
                    } else {
                        return Err(ParseError::InvalidLexeme);
                    }
                }
                '>' => self.parse_condition(Token::Greater, Token::GreaterEqual),
                '<' => self.parse_condition(Token::Less, Token::LessEqual),

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

    fn parse_condition(&mut self, base_t: Token, match_t: Token) {
        if self.match_next('=') {
            self.output.push(match_t);
        } else {
            self.output.push(base_t);
        }

        self.advance();
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

            if buffer.is_empty() {
                return Err(ParseError::InvalidLexeme);
            }

            match buffer.as_str() {
                "def" => self.output.push(Token::Def),
                "if" => self.output.push(Token::If),
                "print" => self.output.push(Token::Print),
                "true" => self.output.push(Token::Bool(true)),
                "false" => self.output.push(Token::Bool(false)),
                // TODO -> and
                // TODO -> not
                _ => self.output.push(Token::Identifier(buffer)),
            }

            self.validate_lexeme_delimeter()?;
        }

        Ok(())
    }

    /// Making sure every operator is followed by whitespace or closing paren
    fn validate_whitespace_delimeter(&self) -> ParseResult<()> {
        if self.is_at_end() {
            return Ok(());
        }

        if !matches!(self.peek(), ' ' | ')') {
            return Err(ParseError::InvalidOperatorDelimeter);
        }

        Ok(())
    }

    fn validate_lexeme_delimeter(&self) -> ParseResult<()> {
        if self.is_at_end() {
            return Ok(());
        }

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

    fn match_next(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.cursor + 1] != c {
            return false;
        }

        self.advance();

        true
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parens() {
        let mut lexer = Lexer::new("()".to_string());
        let tokens = lexer.parse().unwrap();
        assert_eq!(tokens, vec![Token::OpenParen, Token::CloseParen]);
    }

    #[test]
    fn test_arithmetic_operators() {
        let mut lexer = Lexer::new("(+ 1 2)".to_string());
        let tokens = lexer.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::OpenParen,
                Token::Plus,
                Token::Int(1),
                Token::Int(2),
                Token::CloseParen
            ]
        );
    }

    #[test]
    fn test_all_arithmetic_operators() {
        let mut lexer = Lexer::new("(+ - * /)".to_string());
        let tokens = lexer.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::OpenParen,
                Token::Plus,
                Token::Minus,
                Token::Mult,
                Token::Div,
                Token::CloseParen
            ]
        );
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("(123 456 0)".to_string());
        let tokens = lexer.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::OpenParen,
                Token::Int(123),
                Token::Int(456),
                Token::Int(0),
                Token::CloseParen
            ]
        );
    }

    #[test]
    fn test_strings() {
        let mut lexer = Lexer::new(r#"("hello" "world")"#.to_string());
        let tokens = lexer.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::OpenParen,
                Token::String("hello".to_string()),
                Token::String("world".to_string()),
                Token::CloseParen
            ]
        );
    }

    #[test]
    fn test_booleans() {
        let mut lexer = Lexer::new("(true false)".to_string());
        let tokens = lexer.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::OpenParen,
                Token::Bool(true),
                Token::Bool(false),
                Token::CloseParen
            ]
        );
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("(if def print)".to_string());
        let tokens = lexer.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::OpenParen,
                Token::If,
                Token::Def,
                Token::Print,
                Token::CloseParen
            ]
        );
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new("(foo bar _baz)".to_string());
        let tokens = lexer.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::OpenParen,
                Token::Identifier("foo".to_string()),
                Token::Identifier("bar".to_string()),
                Token::Identifier("_baz".to_string()),
                Token::CloseParen
            ]
        );
    }

    #[test]
    fn test_comparison_operators() {
        let mut lexer = Lexer::new("(< > <= >= == !=)".to_string());
        let tokens = lexer.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::OpenParen,
                Token::Less,
                Token::Greater,
                Token::LessEqual,
                Token::GreaterEqual,
                Token::EqualEqual,
                Token::NotEuqal,
                Token::CloseParen
            ]
        );
    }

    #[test]
    fn test_nested_expression() {
        let mut lexer = Lexer::new("(+ (* 2 3) 4)".to_string());
        let tokens = lexer.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::OpenParen,
                Token::Plus,
                Token::OpenParen,
                Token::Mult,
                Token::Int(2),
                Token::Int(3),
                Token::CloseParen,
                Token::Int(4),
                Token::CloseParen
            ]
        );
    }

    #[test]
    fn test_if_expression() {
        let mut lexer = Lexer::new(r#"(if (< 10 12) "yes" "no")"#.to_string());
        let tokens = lexer.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::OpenParen,
                Token::If,
                Token::OpenParen,
                Token::Less,
                Token::Int(10),
                Token::Int(12),
                Token::CloseParen,
                Token::String("yes".to_string()),
                Token::String("no".to_string()),
                Token::CloseParen
            ]
        );
    }

    #[test]
    fn test_complex_expression() {
        let mut lexer = Lexer::new(r#"(if (<= 10 12) (print "hey"))"#.to_string());
        let tokens = lexer.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::OpenParen,
                Token::If,
                Token::OpenParen,
                Token::LessEqual,
                Token::Int(10),
                Token::Int(12),
                Token::CloseParen,
                Token::OpenParen,
                Token::Print,
                Token::String("hey".to_string()),
                Token::CloseParen,
                Token::CloseParen
            ]
        );
    }

    #[test]
    fn test_empty_string() {
        let mut lexer = Lexer::new(r#"("")"#.to_string());
        let tokens = lexer.parse().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::OpenParen,
                Token::String("".to_string()),
                Token::CloseParen
            ]
        );
    }

    #[test]
    fn test_single_number() {
        let mut lexer = Lexer::new("42".to_string());
        let tokens = lexer.parse().unwrap();
        assert_eq!(tokens, vec![Token::Int(42)]);
    }

    #[test]
    fn test_operator_without_space_fails() {
        let mut lexer = Lexer::new("(+1 2)".to_string());
        let result = lexer.parse();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParseError::InvalidOperatorDelimeter);
    }
}
