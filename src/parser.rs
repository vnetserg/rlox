use super::errors::*;

use std::{io::BufRead, iter::Iterator};

pub struct AstNode {}

pub struct Parser {}

impl Default for Parser {
    fn default() -> Self {
        Self {}
    }
}

impl Parser {
    pub fn iter_ast_nodes<'a, R: BufRead>(
        &'a mut self,
        input: &'a mut R,
    ) -> AstNodeIterator<'a, R> {
        AstNodeIterator {
            parser: self,
            input,
        }
    }
}

pub struct AstNodeIterator<'a, R: BufRead> {
    parser: &'a mut Parser,
    input: &'a mut R,
}

impl<'a, R: BufRead> Iterator for AstNodeIterator<'a, R> {
    type Item = LoxResult<AstNode>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
