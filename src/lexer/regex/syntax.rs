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

pub fn tokenize(iter: impl Iterator<Item = char>) -> impl Iterator<Item = Token> {
    iter.map(|c| match c {
        '|' => Token::Alternation,
        '*' => Token::KleeneStar,
        '(' => Token::LeftParen,
        ')' => Token::RightParen,
        c => Token::Character(c),
    })
    .chain(std::iter::once(Token::EndOfInput))
    .peekable()
}

// struct LexerGeneric<I>
// where
//     I: Iterator<Item = Token>,
// {
//     tokens: Peekable<I>,
// }

// impl<I> LexerGeneric<I>
// where
//     I: Iterator<Item = Token>,
// {
//     fn new(chars: impl Iterator<Item = char>) -> Self {
//         let tokens = chars
//             .map(|c| match c {
//                 '|' => Token::Alternation,
//                 '*' => Token::KleeneStar,
//                 '(' => Token::LeftParen,
//                 ')' => Token::RightParen,
//                 c => Token::Character(c),
//             })
//             .chain(std::iter::once(Token::EndOfInput))
//             .peekable();
//         Self { tokens }
//     }

//     fn next(&mut self) -> Option<Token> {
//         self.tokens.next()
//     }

//     fn peek(&mut self) -> Option<&Token> {
//         self.tokens.peek()
//     }
// }

///

struct Lexer<'a> {
    tokens: Box<dyn Iterator<Item = Token> + 'a>,
}

impl<'a> Lexer<'a> {
    fn new(chars: impl Iterator<Item = char> + 'a) -> Self {
        let tokens = chars
            .map(|c| match c {
                '|' => Token::Alternation,
                '*' => Token::KleeneStar,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                c => Token::Character(c),
            })
            .chain(std::iter::once(Token::EndOfInput))
            .peekable();
        Self {
            tokens: Box::new(tokens),
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    // fn peek(&self) -> Option<Token> {
    //     self.tokens.peek()
    // }
}

fn parse(lexer: &mut Lexer) -> Result<Ast> {
    alternation(lexer)
}

fn alternation(lexer: &mut Lexer) -> Result<Ast> {
    let mut alts = vec![concatenation(lexer)?];
    while let Some(token) = lexer.next() {
        match token {
            Token::Alternation => {
                alts.push(concatenation(lexer)?);
            }
            Token::EndOfInput | Token::RightParen => break,
            _ => Err("expected '|', ')', or end of input")?,
        }
    }
    Ok(Ast::Alternation(alts))
}

fn concatenation(lexer: &mut Lexer) -> Result<Ast> {
    let mut parts = vec![repetition(lexer)?];
    while let Some(token) = lexer.next() {
        match token {
            Token::LeftParen | Token::Character(_) => {
                parts.push(repetition(lexer)?);
            }
            Token::EndOfInput | Token::Alternation | Token::RightParen => break,
            _ => Err("expected character, '|', '(', ')' or end of input")?,
        }
    }
    Ok(Ast::Concatenation(parts))
}

fn repetition(lexer: &mut Lexer) -> Result<Ast> {
    let mut rep = parentheses(lexer)?;
    while let Some(token) = lexer.next() {
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

fn parentheses(lexer: &mut Lexer) -> Result<Ast> {
    todo!()
}
