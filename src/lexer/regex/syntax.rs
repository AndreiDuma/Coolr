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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token {
    Character(char),
    Alternation,
    KleeneStar,
    LeftParen,
    RightParen,
    EndOfInput,
}

// struct Lexer<I>
// where
//     I: Iterator<Item = char>,
// {
//     chars: I,
// }

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
