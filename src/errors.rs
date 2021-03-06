use error_chain::error_chain;
pub use error_chain::{bail, ChainedError};

error_chain! {
    types {
        LoxError, LoxErrorKind, LoxResultExt, LoxResult;
    }

    errors {
        SyntaxError(desc: String) {
            description("invalid syntax")
            display("SyntaxError: {}", desc)
        }
        UnexpectedEof {
            description("unexpected EOF")
            display("UnexpectedEof")
        }
    }

    foreign_links {
        Io(::std::io::Error);
    }
}
