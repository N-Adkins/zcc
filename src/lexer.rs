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
        Ok(())
    }

    fn replace_newline_slashes(&mut self) {
        self.source = self.source.replace("\\\n", "");
    }
    
    // This is so inefficient to be honest
    fn replace_trigraphs(&mut self) {
        self.source = self.source
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
    
    fn tokenize_next(&mut self) -> CompResult<()> {
        assert!(self.peek_next_char().is_some());
        
        loop {
            match self.peek_next_char() {
                Some(c) => {
                    if c.is_whitespace() {
                        _ = self.eat_next_char();
                    } else {
                        break;
                    }
                }
                None => return Ok(())
            }
        }
        
        // Should always be Some after loop before this
        let next = self.peek_next_char().unwrap();

        if next.is_numeric() {
             // Number
        } else if self.is_identifier(next) {
            self.tokenize_identifier()?;
        }

        Ok(())
    }

    fn tokenize_identifier(&mut self) -> CompResult<()> {
        let next = self.peek_next_char().unwrap();
        assert!(next.is_alphabetic() || next == '_');
        
        // Iterate until whitespace or end of file
        let start = self.index;
        while let Some(c) = self.eat_next_char() {
            if c.is_whitespace() {
                break;
            }
        }

        let ident = &self.source[start..self.index];
        let token = if let Some(keyword) = self.tokenize_keyword(ident) {
            Token::Keyword(keyword)
        } else {
            Token::Identifier(ident.into())
        };

        self.tokens.push(token);

        Ok(())
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

    fn tokenize_keyword(&self, identifier: &str) -> Option<Keyword> {
        let value = KEYWORD_MAP.get(identifier)?;
        Some(*value)
    }

}
