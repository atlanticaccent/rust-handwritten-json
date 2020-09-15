use core::result::Result as StdResult;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("missing a right brace")]
    MissingRightBrace,
    #[error("missing a right bracket")]
    MissingRightBracket,
    #[error("missing a double quote")]
    MissingDoubleQuote,

    #[error("missing a colon")]
    MissingColon,
    #[error("missing a key without double quotes")]
    MissingKey,
    #[error("missing a value")]
    MissingValue,
    #[error("missing a non-string value")]
    MissingNonString,

    #[error("expect {expected:?} but actual is {actual:?}")]
    ShouldBe { actual: char, expected: char },
}

pub type Result<T> = StdResult<T, Error>;
