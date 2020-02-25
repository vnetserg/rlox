pub use error_chain::{bail, ChainedError};
use error_chain::error_chain;

error_chain! {
    types {
        LoxError, LoxErrorKind, LoxResultExt, LoxResult;
    }

    errors {
        SyntaxError(desc: String) {
            description("invalid syntax")
            display("invalid syntax: {}", desc)
        }
    }
}
