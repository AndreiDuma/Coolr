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

use crate::regex::lexer::{Token, Tokens};
use crate::regex::Ast;
use crate::regex::Result;

use super::lexer::{Tokens2, Tokens3};

////

fn fail(msg: &str) -> Result<Ast> {
    Err(msg)?
}

pub fn parse(re: &str) -> Result<Ast> {
    // let mut tokens = tokens(re.chars()).peekable();

    let mut tokens = Tokens2::new(re.chars());

    alternation(&mut tokens)
}

fn alternation(tokens: &mut Tokens2) -> Result<Ast> {
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

fn concatenation(tokens: &mut Tokens2) -> Result<Ast> {
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

fn repetition(tokens: &mut Tokens2) -> Result<Ast> {
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

fn parentheses(_tokens: &mut Tokens2) -> Result<Ast> {
    fail("bla")
}
