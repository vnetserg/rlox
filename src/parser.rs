use super::{
    errors::*,
    tokenizer::{TokenIterator, Tokenizer},
};

use std::{io::BufRead, iter::Iterator};

pub struct AstNode {}

#[derive(Default)]
pub struct Parser {
    tokenizer: Tokenizer,
    state: ParserState,
}

#[derive(Default)]
struct ParserState {}

impl Parser {
    pub fn iter_ast_nodes<'a, R: BufRead>(
        &'a mut self,
        input: &'a mut R,
    ) -> AstNodeIterator<'a, R> {
        AstNodeIterator {
            _state: &mut self.state,
            tokens: self.tokenizer.iter_tokens(input),
        }
    }
}

pub struct AstNodeIterator<'a, R: BufRead> {
    _state: &'a mut ParserState,
    tokens: TokenIterator<'a, R>,
}

impl<'a, R: BufRead> Iterator for AstNodeIterator<'a, R> {
    type Item = LoxResult<AstNode>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(maybe_token) = self.tokens.next() {
            match maybe_token {
                Ok(token) => println!("{:?}", token),
                Err(err) => return Some(Err(err)),
            }
        }
        None
    }
}
