use core::fmt::Debug;

type Result<T> = core::result::Result<T, Box<dyn core::error::Error>>;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Ast {
    Empty,
    Character(char),
    // Bracket(Vec<Range<char>>),
    Concatenation(Vec<Ast>),
    Alternation(Vec<Ast>),
    Repetition(Box<Ast>),
}

impl Ast {
    pub fn new(_pattern: &str) -> Result<Self> {
        todo!()
    }

    pub fn empty() -> Result<Self> {
        Ok(Self::Empty)
    }

    pub fn character(chr: char) -> Result<Self> {
        Ok(Self::Character(chr))
    }

    pub fn concatenation(asts: &[Ast]) -> Result<Self> {
        if asts.len() == 0 {
            Err("cannot have a concatenation of zero elements")?;
        }
        Ok(Self::Concatenation(asts.to_vec()))
    }

    pub fn alternation(asts: &[Ast]) -> Result<Self> {
        if asts.len() == 0 {
            Err("cannot have an alternation of zero elements")?;
        }
        Ok(Self::Alternation(asts.to_vec()))
    }

    pub fn repetition(ast: &Ast) -> Result<Self> {
        Ok(Self::Repetition(Box::new(ast.clone())))
    }

    pub fn iter(&self) -> AstIter<'_> {
        AstIter { stack: vec![self] }
    }
}

// impl Debug for Ast {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Ast::Empty => write!(f, "ɛ"),
//             Ast::Character(chr) => write!(f, "{chr}"),
//             Ast::Concatenation(children) => {
//                 write!(f, "(")?;
//                 children
//                     .iter()
//                     .map(|c| c.fmt(f))
//                     .collect::<std::fmt::Result>()?;
//                 write!(f, ")")
//             }
//             Ast::Alternation(children) => {
//                 write!(f, "(")?;
//                 let mut it = children.iter();
//                 // Format the first child.
//                 it.next().unwrap().fmt(f)?;

//                 // Format remaining children using '|' as a separator.
//                 it.map(|c| write!(f, "|{c:?}"))
//                     .collect::<std::fmt::Result>()?;
//                 write!(f, ")")
//             }
//             Ast::Repetition(child) => {
//                 write!(f, "({child:?})*")
//             }
//         }
//     }
// }

/// An iterator that returns references to the sub-expressions of an
/// `Ast` in postorder.
pub struct AstIter<'a> {
    stack: Vec<&'a Ast>,
}

impl<'a> Iterator for AstIter<'a> {
    type Item = &'a Ast;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ast) = self.stack.pop() {
            match *ast {
                Ast::Concatenation(ref children) | Ast::Alternation(ref children) => {
                    self.stack.extend(children);
                }
                Ast::Repetition(ref child) => {
                    self.stack.push(child);
                }
                _ => {}
            }
            return Some(ast);
        }
        None
    }
}
