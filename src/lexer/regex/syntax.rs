// Grammar for regular expressions
//
// Expr <- Alternation eoi
// Alternation <- Alternation | Concatenation
// Alternation <- Concatenation
// Concatenation <- Concatenation Repetition
// Concatenation <- Repetition
// Repetition <- Repetition '*'
// Repetition <- '(' Alternation ')'
// Repetition <- character
//
// Terminals: eoi, character, '|', '*', '(', ')'

use std::{error::Error, iter::Peekable};

use crate::regex::Ast;

type Result<T> = core::result::Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token {
    Character(char),
    Alternation,
    KleeneStar,
    LeftParen,
    RightParen,
    EndOfInput,
}

pub fn tokens(chars: impl Iterator<Item = char>) -> Peekable<impl Iterator<Item = Token>> {
    chars
        .map(|c| match c {
            '|' => Token::Alternation,
            '*' => Token::KleeneStar,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            c => Token::Character(c),
        })
        .chain(std::iter::once(Token::EndOfInput))
        .peekable()
}

///

struct Tokens<'a> {
    inner: Peekable<Box<dyn Iterator<Item = Token> + 'a>>,
}

impl<'a> Tokens<'a> {
    fn new(chars: impl Iterator<Item = char> + 'a) -> Self {
        let inner = chars
            .map(|c| match c {
                '|' => Token::Alternation,
                '*' => Token::KleeneStar,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                c => Token::Character(c),
            })
            .chain(std::iter::once(Token::EndOfInput));
        let inner = Box::new(inner) as Box<dyn Iterator<Item = Token>>;
        let inner = inner.peekable();
        Self { inner }
    }
}

impl Iterator for Tokens<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

////

pub struct Lexer {
    // tokens: Box<dyn Iterator<Item = Token> + 'a>,
}

impl Lexer {
    pub fn tokens(chars: impl Iterator<Item = char>) -> Peekable<impl Iterator<Item = Token>> {
        chars
            .map(|c| match c {
                '|' => Token::Alternation,
                '*' => Token::KleeneStar,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                c => Token::Character(c),
            })
            .chain(std::iter::once(Token::EndOfInput))
            .peekable()
    }
}

///

fn fail(msg: &str) -> Result<Ast> {
    Err(msg)?
}

pub fn parse(re: &str) -> Result<Ast> {
    // let mut tokens = tokens(re.chars()).peekable();

    let mut tokens = Tokens::new(re.chars());

    alternation(&mut tokens)
}

fn alternation(tokens: &mut Tokens) -> Result<Ast> {
    let mut alts = vec![concatenation(tokens)?];
    while let Some(token) = tokens.next() {
        match token {
            Token::Alternation => {
                alts.push(concatenation(tokens)?);
            }
            Token::EndOfInput | Token::RightParen => break,
            _ => Err("expected '|', ')', or end of input")?,
        }
    }
    Ok(Ast::Alternation(alts))
}

fn concatenation(tokens: &mut Tokens) -> Result<Ast> {
    let mut parts = vec![repetition(tokens)?];
    while let Some(token) = tokens.next() {
        match token {
            Token::LeftParen | Token::Character(_) => {
                parts.push(repetition(tokens)?);
            }
            Token::EndOfInput | Token::Alternation | Token::RightParen => break,
            _ => Err("expected character, '|', '(', ')' or end of input")?,
        }
    }
    Ok(Ast::Concatenation(parts))
}

fn repetition(tokens: &mut Tokens) -> Result<Ast> {
    let mut rep = parentheses(tokens)?;
    while let Some(token) = tokens.next() {
        match token {
            Token::KleeneStar => {
                rep = Ast::Repetition(Box::new(rep));
            }
            Token::EndOfInput | Token::Alternation | Token::RightParen => break,
            _ => Err("expected '*', '|', ')' or end of input")?,
        }
    }
    Ok(rep)
}

fn parentheses(_tokens: &mut Tokens) -> Result<Ast> {
    fail("bla")
}
