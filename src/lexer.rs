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
pub struct Lexer<'a> {
    source: &'a str,
    index: usize,
    tokens: Vec<Token>
}

impl<'a> Lexer<'a> {
    
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            index: 0,
            tokens: Vec::new(),
        }
    }
    
    pub fn tokenize(&mut self) -> CompResult<()> {
        Ok(())
    }

    fn tokenize_identifier(&mut self) -> CompResult<()> {
        // Preconditions
        let next = self.peek_next_char().unwrap();
        assert!(next.is_alphabetic());
        
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
    
    fn tokenize_keyword(&self, identifier: &str) -> Option<Keyword> {
        let value = KEYWORD_MAP.get(identifier)?;
        Some(*value)
    }

}
