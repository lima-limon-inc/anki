// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

pub use failure::{Error, Fail};

pub type Result<T> = std::result::Result<T, AnkiError>;

#[derive(Debug, Fail)]
pub enum AnkiError {
    #[fail(display = "invalid input: {}", info)]
    InvalidInput { info: String },

    #[fail(display = "invalid card template: {}", info)]
    TemplateParseError { info: String },
}

// error helpers
impl AnkiError {
    pub(crate) fn parse<S: Into<String>>(s: S) -> AnkiError {
        AnkiError::TemplateParseError { info: s.into() }
    }

    pub(crate) fn invalid_input<S: Into<String>>(s: S) -> AnkiError {
        AnkiError::InvalidInput { info: s.into() }
    }
}
