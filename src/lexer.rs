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

impl Lexer {
    
    pub fn new(source: String) -> Self {
        Self {
            source,
            index: 0,
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) -> CompResult<()> {
        Ok(())
    }
    
    fn tokenize_keyword(identifier: &str) -> Option<Keyword> {
        let value = KEYWORD_MAP.get(identifier)?;
        Some(*value)
    }
}
