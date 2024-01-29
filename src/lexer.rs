use crate::comp_error::*;
use crate::lang::*;

#[derive(Debug)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Constant(Constant),
    StringLiteral(String),
    Operator(Operator),
    Punctuator(Punctuator),
}

#[derive(Debug)]
pub enum PreprocessNumber {
    Integer(i64),
    Floating(f64),
}

#[derive(Debug)]
pub enum PreprocessToken {
    HeaderName(String),
    Identifier(String),
    Number(PreprocessNumber),
    CharacterConstant(char),
    StringLiteral(String),
    Operator(Operator),
    Punctuator(Punctuator),
    Other(char),
}

#[derive(Debug)]
pub struct Lexer {
    source: String,
    line: usize,
    col: usize,
    index: usize,
    tokens: Vec<Token>,
    pp_tokens: Vec<PreprocessToken>,
}

impl<'a> Lexer {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: String::from(source),
            line: 1,
            col: 1,
            index: 0,
            tokens: Vec::new(),
            pp_tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) -> CompResult<()> {
        self.phase_one();
        self.phase_two();
        self.phase_three()?;
        Ok(())
    }

    // Source character mapping and trigraph sequence
    // mapping
    fn phase_one(&mut self) {
        self.replace_trigraphs();
    }

    // Deleting newlines with preceding backslashes
    fn phase_two(&mut self) {
        self.replace_newline_slashes();
    }

    // Preprocessing tokenizing
    fn phase_three(&mut self) -> CompResult<()> {

        while let Some(c) = self.peek_next_char() {
            if c.is_whitespace() {
                _ = self.eat_next_char();
                continue;
            };
            self.pp_tokenize_next()?;
        } 

        Ok(())
    }

    fn replace_newline_slashes(&mut self) {
        self.source = self.source.replace("\\\n", "");
    }

    // This is so inefficient to be honest
    fn replace_trigraphs(&mut self) {
        self.source = self
            .source
            .replace(r"??=", r"#")
            .replace(r"??(", r"[")
            .replace(r"??/", r"\") // this might be wrong
            .replace(r"??)", r"]")
            .replace(r"??'", r"^")
            .replace(r"??<", r"{")
            .replace(r"??!", r"|")
            .replace(r"??>", r"}")
            .replace(r"??-", r"~");
    }

    fn pp_tokenize_next(&mut self) -> CompResult<()> {
        let next = self.peek_next_char().expect("Precondition"); 

        if next == '\'' {
            self.pp_tokenize_char_constant()?;
        } else if next == '\"' {
            self.pp_tokenize_string_literal()?;
        } else if next.is_numeric() {
            self.pp_tokenize_number();
        } else if self.is_identifier(next) {
            self.pp_tokenize_identifier();
        } else {
            todo!("Need to implement operators etc");
        }

        Ok(())
    }

    fn pp_tokenize_char_constant(&mut self) -> CompResult<()> {
        let start_line = self.line;
        let start_col = self.col;

        let begin = self.eat_next_char().expect("Precondition");
        assert_eq!(begin, '\'');

        let literal = match self.eat_next_char() {
            Some(c) => c,
            None => {
                return Err(CompErrorBuilder::new()
                    .code(ErrorCode::UnterminatedCharConstant)
                    .message("Expected character, found end of source".into())
                    .source(self.source.clone(), start_line)
                    .highlight(start_col - 1, start_col)
                    .highlight_message("Started here".into())
                    .build())
            }
        };
        
        let next_line = self.line;
        let next_col = self.col;
        match self.eat_next_char() {
            Some('\'') => (),
            Some(c) => {
                return Err(CompErrorBuilder::new()
                    .code(ErrorCode::UnterminatedCharConstant)
                    .message(format!("Expected `\'`, found `{}`", c))
                    .source(self.source.clone(), next_line)
                    .highlight(start_col - 1, next_col)
                    .highlight_message("Invalid termination".into())
                    .build())
            }
            None => {
                return Err(CompErrorBuilder::new()
                    .code(ErrorCode::UnterminatedCharConstant)
                    .message("Expected `\'`, found end of source".into())
                    .source(self.source.clone(), start_line)
                    .highlight(start_col - 1, start_col)
                    .highlight_message("Started here".into())
                    .build())
            }
        }

        self.pp_tokens
            .push(PreprocessToken::CharacterConstant(literal));

        Ok(())
    }

    fn pp_tokenize_string_literal(&mut self) -> CompResult<()> {
        let start_line = self.line;
        let start_col = self.col;

        let begin = self.eat_next_char().expect("Precondition");
        assert_eq!(begin, '\"');

        let start = self.index;
        loop {
            match self.eat_next_char() {
                Some('\"') => break,
                Some(_) => (),
                None => {
                    return Err(CompErrorBuilder::new()
                        .code(ErrorCode::UnterminatedStringLiteral)
                        .message("Expected character, found end of source".into())
                        .source(self.source.clone(), start_line)
                        .highlight(start_col - 1, start_col)
                        .highlight_message("Started here".into())
                        .build())
                }
            };
        }

        let literal = &self.source[start..(self.index - 1)];

        self.pp_tokens
            .push(PreprocessToken::StringLiteral(String::from(literal)));

        Ok(())
    }

    fn pp_tokenize_number(&mut self) {
        let start = self.index;
        while let Some(c) = self.peek_next_char() {
            if c.is_numeric() {
                _ = self.eat_next_char();
            } else {
                break;
            }
        }

        // TODO: Float literals
        let num_raw = &self.source[start..self.index];

        let num = num_raw
            .parse::<i64>()
            .expect("Should only ever be a number");

        self.pp_tokens
            .push(PreprocessToken::Number(PreprocessNumber::Integer(num)));
    }

    fn pp_tokenize_identifier(&mut self) {
        let start = self.index;
        while let Some(c) = self.peek_next_char() {
            if self.is_identifier(c) {
                _ = self.eat_next_char();
            } else {
                break;
            }
        }

        let ident_raw = &self.source[start..self.index];

        self.pp_tokens.push(PreprocessToken::Identifier(String::from(ident_raw)));
    }

    fn peek_next_char(&self) -> Option<char> {
        self.source.chars().nth(self.index)
    }

    fn eat_next_char(&mut self) -> Option<char> {
        let maybe_char = self.source.chars().nth(self.index);
        self.index += 1;
        if let Some(c) = maybe_char {
            if c == '\n' {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
        }
        maybe_char
    }

    fn is_identifier(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn get_keyword(&self, identifier: &str) -> Option<Keyword> {
        let value = KEYWORD_MAP.get(identifier)?;
        Some(*value)
    }
}
