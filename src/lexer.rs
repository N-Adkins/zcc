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

#[derive(Debug, Clone, Copy)]
pub struct PreprocessMetadata {
    pub line: usize,
    pub col: usize,
    pub index: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HeaderNameType {
    Included,
    Local,
}

#[derive(Debug)]
pub enum PreprocessToken {
    HeaderName(String, HeaderNameType, PreprocessMetadata),
    Identifier(String, PreprocessMetadata),
    Number(String, PreprocessMetadata),
    CharacterConstant(char, PreprocessMetadata),
    StringLiteral(String, PreprocessMetadata),
    Operator(String, PreprocessMetadata),
    Punctuator(String, PreprocessMetadata),
    Other(char, PreprocessMetadata),
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
            if c == '\n' {
                let token = PreprocessToken::Other(
                    c,
                    PreprocessMetadata {
                        index: self.index,
                        col: self.col,
                        line: self.line,
                    },
                );
                self.pp_tokens.push(token);
                _ = self.eat_next_char();
                continue;
            } else if c.is_whitespace() {
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

        if next == '\"' || next == '<' && self.pp_tokens.len() >= 2 {
            if let Some(PreprocessToken::Identifier(ident, _)) = self.pp_tokens.last() {
                if let Some(PreprocessToken::Punctuator(hash, _)) =
                    self.pp_tokens.iter().nth(self.pp_tokens.len() - 2)
                {
                    if ident == "include" && hash == "#" {
                        self.pp_tokenize_header_name()?;
                        return Ok(());
                    }
                }
            }
        }

        if next == '\'' {
            self.pp_tokenize_char_constant()?;
        } else if next == '\"' {
            self.pp_tokenize_string_literal()?;
        } else if next.is_numeric() {
            self.pp_tokenize_number();
        } else if self.is_identifier(next) {
            self.pp_tokenize_identifier();
        } else {
            self.pp_tokenize_specials();
        }

        Ok(())
    }

    fn pp_tokenize_header_name(&mut self) -> CompResult<()> {
        let start_index = self.index;
        let start_line = self.line;
        let start_col = self.col;

        let begin = self.eat_next_char().expect("Precondition");
        assert!(begin == '\"' || begin == '<');

        let header_type = if begin == '\"' {
            HeaderNameType::Local
        } else {
            HeaderNameType::Included
        };

        let start = self.index;
        loop {
            match self.eat_next_char() {
                Some(c) => {
                    if (header_type == HeaderNameType::Local && c == '\"')
                        || (header_type == HeaderNameType::Included && c == '>')
                    {
                        break;
                    }
                }
                None => {
                    let expected = match header_type {
                        HeaderNameType::Local => '\"',
                        HeaderNameType::Included => '>',
                    };
                    return Err(CompErrorBuilder::new()
                        .code(ErrorCode::UnterminatedHeaderName)
                        .message(format!("Expected `{}`, found end of source", expected))
                        .source(self.source.clone(), start_line)
                        .highlight(start_col - 1, start_col)
                        .highlight_message("Started here".into())
                        .build());
                }
            };
        }

        let literal = &self.source[start..(self.index - 1)];

        self.pp_tokens.push(PreprocessToken::HeaderName(
            String::from(literal),
            header_type,
            PreprocessMetadata {
                index: start_index,
                col: start_col,
                line: start_line,
            },
        ));

        Ok(())
    }

    fn pp_tokenize_char_constant(&mut self) -> CompResult<()> {
        let start_index = self.index;
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

        self.pp_tokens.push(PreprocessToken::CharacterConstant(
            literal,
            PreprocessMetadata {
                index: start_index,
                col: start_col,
                line: start_line,
            },
        ));

        Ok(())
    }

    fn pp_tokenize_string_literal(&mut self) -> CompResult<()> {
        let start_index = self.index;
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

        self.pp_tokens.push(PreprocessToken::StringLiteral(
            String::from(literal),
            PreprocessMetadata {
                index: start_index,
                col: start_col,
                line: start_line,
            },
        ));

        Ok(())
    }

    fn pp_tokenize_number(&mut self) {
        let start_index = self.index;
        let start_line = self.line;
        let start_col = self.col;

        while let Some(c) = self.peek_next_char() {
            if c.is_numeric() {
                _ = self.eat_next_char();
            } else {
                break;
            }
        }

        // TODO: Float literals
        let num_raw = &self.source[start_index..self.index];

        /*
        let num = num_raw
            .parse::<i64>()
            .expect("Should only ever be a number");
        */

        self.pp_tokens.push(PreprocessToken::Number(
            String::from(num_raw),
            PreprocessMetadata {
                index: start_index,
                line: start_line,
                col: start_col,
            },
        ));
    }

    fn pp_tokenize_identifier(&mut self) {
        let start_index = self.index;
        let start_col = self.col;
        let start_line = self.line;

        while let Some(c) = self.peek_next_char() {
            if self.is_identifier(c) {
                _ = self.eat_next_char();
            } else {
                break;
            }
        }

        let ident_raw = &self.source[start_index..self.index];

        self.pp_tokens.push(PreprocessToken::Identifier(
            String::from(ident_raw),
            PreprocessMetadata {
                index: start_index,
                col: start_col,
                line: start_line,
            },
        ));
    }

    fn pp_tokenize_specials(&mut self) {
        let first = self.peek_next_char().expect("Precondition");
        let highest: usize = if self.peek_offset_char(2).is_some() {
            2
        } else if self.peek_offset_char(1).is_some() {
            1
        } else {
            0
        };

        let metadata = PreprocessMetadata {
            index: self.index,
            line: self.line,
            col: self.col,
        };

        if first == '#' && self.col == 1 {
            let raw = &self.source[self.index..(self.index + 1)];
            self.pp_tokens
                .push(PreprocessToken::Punctuator(raw.into(), metadata));
            self.eat_chars(1);
            return ();
        };

        // This is badly optimized
        for i in (0..=highest).rev() {
            let slice = &self.source[self.index..(self.index + i)];
            println!("{}", slice);
            if OPERATOR_MAP.get(slice).is_some() {
                self.pp_tokens
                    .push(PreprocessToken::Operator(slice.into(), metadata));
                self.eat_chars(i);
                return ();
            } else if PUNCTUATOR_MAP.get(slice).is_some() {
                self.pp_tokens
                    .push(PreprocessToken::Punctuator(slice.into(), metadata));
                self.eat_chars(i);
                return ();
            }
        }

        self.pp_tokens.push(PreprocessToken::Other(first, metadata));
        self.eat_next_char();
    }

    fn peek_next_char(&self) -> Option<char> {
        self.source.chars().nth(self.index)
    }

    fn peek_offset_char(&self, index: usize) -> Option<char> {
        self.source.chars().nth(self.index + index)
    }

    fn eat_next_char(&mut self) -> Option<char> {
        let maybe_char = self.source.chars().nth(self.index);
        self.index += 1;
        if let Some(c) = maybe_char {
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        maybe_char
    }

    fn eat_chars(&mut self, amount: usize) {
        for _ in 0..amount {
            self.eat_next_char();
        }
    }

    fn is_identifier(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn get_keyword(&self, identifier: &str) -> Option<Keyword> {
        let value = KEYWORD_MAP.get(identifier)?;
        Some(*value)
    }
}
