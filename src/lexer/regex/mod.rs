mod ast;
mod lexer;
mod syntax;

use std::error::Error;

pub use ast::*;
pub use syntax::*;

type Result<T> = core::result::Result<T, Box<dyn Error>>;
