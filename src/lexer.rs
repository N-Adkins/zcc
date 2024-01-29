use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::comp_error::*;

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Auto, Break, Case, Char, Const, Continue, Default, Do,
    Double, Else, Enum, Extern, Float, For, Goto, If,
    Int, Long, Register, Return, Short, Signed, Sizeof, Static,
    Struct, Switch, Typedef, Union, Unsigned, Void, Volatile, While,
}

lazy_static! {
    static ref KEYWORD_MAP: HashMap<&'static str, Keyword> = HashMap::from([
        ("auto", Keyword::Auto),
        ("break", Keyword::Break),
        ("case", Keyword::Case),
        ("char", Keyword::Char),
        ("const", Keyword::Const),
        ("continue", Keyword::Continue),
        ("default", Keyword::Default),
        ("do", Keyword::Do),
        ("double", Keyword::Double),
        ("else", Keyword::Else),
        ("enum", Keyword::Enum),
        ("extern", Keyword::Extern),
        ("float", Keyword::Float),
        ("for", Keyword::For),
        ("goto", Keyword::Goto),
        ("if", Keyword::If),
        ("int", Keyword::Int),
        ("long", Keyword::Long),
        ("register", Keyword::Register),
        ("return", Keyword::Return),
        ("short", Keyword::Short),
        ("signed", Keyword::Signed),
        ("sizeof", Keyword::Sizeof),
        ("static", Keyword::Static),
        ("struct", Keyword::Struct),
        ("switch", Keyword::Switch),
        ("typedef", Keyword::Typedef),
        ("union", Keyword::Union),
        ("unsigned", Keyword::Unsigned),
        ("void", Keyword::Void),
        ("volatile", Keyword::Volatile),
        ("while", Keyword::While),
    ]);
}

#[derive(Debug)]
pub enum Constant {
    Char(char),
    Integer(i32),
    Float(f32),
}

#[derive(Debug)]
pub enum Operator {
    
}

#[derive(Debug)]
pub enum Punctuator {
    
}

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
pub struct Lexer {
    source: String,
    index: usize,
    tokens: Vec<Token>
}

impl<'a> Lexer {
    
    pub fn new(source: &'a str) -> Self {
        Self {
            source: String::from(source),
            index: 0,
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) -> CompResult<()> {
        self.phase_one();
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
    
    // This is so inefficient right now to be honest
    fn replace_trigraphs(&mut self) {
        self.source = self.source
            .replace(r"??=", r"#")
            .replace(r"??(", r"[")
            .replace(r"??/", r"\\")
            .replace(r"??)", r"]")
            .replace(r"??'", r"^")
            .replace(r"??<", r"{")
            .replace(r"??!", r"|")
            .replace(r"??>", r"}")
            .replace(r"??-", r"~")
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
