use super::{
    errors::*,
    parser::{AstNode, Parser},
};

use std::io::{self, BufRead, Write};

pub struct Lox {}

impl Lox {
    pub fn run<I: BufRead, O: Write>(&mut self, mut input: I, mut output: O) -> LoxResult<()> {
        let mut parser = Parser::default();
        for maybe_node in parser.iter_ast_nodes(&mut input) {
            let node = maybe_node?;
            self.execute_node(&node, &mut output)?;
        }
        Ok(())
    }

    pub fn run_interactive<I, O, E>(
        &mut self,
        mut input: I,
        mut output: O,
        mut errput: E,
    ) -> io::Result<()>
    where
        I: BufRead,
        O: Write,
        E: Write,
    {
        let put = |text: &[u8], output: &mut O| -> io::Result<()> {
            output.write_all(text)?;
            output.flush()
        };

        let mut parser = Parser::default();
        let mut nodes = vec![];
        let mut line = vec![];
        let mut ends_with_newline = false;

        put(b"> ", &mut output)?;
        while input.read_until(b'\n', &mut line)? > 0 {
            ends_with_newline = *line.last().unwrap() == b'\n';
            let mut continuation = false;

            for maybe_node in parser.iter_ast_nodes(&mut &line[..]) {
                // NB: by contract, nothing can come from this iterator after error.
                match maybe_node {
                    Ok(node) => nodes.push(node),
                    Err(LoxError(LoxErrorKind::UnexpectedEof, _)) => continuation = true,
                    Err(err) => {
                        writeln!(errput, "{}", err)?;
                        nodes.clear();
                    }
                }
            }

            if continuation && ends_with_newline {
                put(b". ", &mut output)?;
            } else if !continuation {
                for node in nodes.drain(..) {
                    let result = self.execute_node(&node, &mut output);
                    if let Err(err) = result {
                        writeln!(errput, "{}", err)?;
                        break;
                    }
                }
                if ends_with_newline {
                    put(b"> ", &mut output)?;
                } else {
                    put(b"\n> ", &mut output)?;
                }
                ends_with_newline = false;
            }

            line.clear();
        }

        if !ends_with_newline {
            put(b"\n", &mut output)?;
        }

        Ok(())
    }

    fn execute_node<O: Write>(&mut self, _node: &AstNode, _output: &mut O) -> LoxResult<()> {
        Ok(())
    }
}

impl Default for Lox {
    fn default() -> Self {
        Lox {}
    }
}
