/// TODO: Create smart constructors not allowing invalid ASTs (such as
/// concatenations of zero sub-expressions).
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
    pub fn iter(&self) -> AstIter<'_> {
        AstIter { stack: vec![self] }
    }
}

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
        return None;
    }
}
