use super::errors::*;

use std::io::{Read, Write};

pub struct Lox {}

impl Lox {
    pub fn new() -> Self {
        Lox {}
    }

    pub fn execute<R: Read, W: Write>(&mut self, mut input: R, mut output: W) -> LoxResult<()> {
        let mut buffer = [0u8; 1024];
        loop {
            let n = input.read(&mut buffer).chain_err(|| "read_failed")?;
            if n == 0 {
                break;
            }
            output.write(&buffer[..n]).chain_err(|| "write failed")?;
        }
        Ok(())
    }
}

impl Default for Lox {
    fn default() -> Self {
        Lox::new()
    }
}
