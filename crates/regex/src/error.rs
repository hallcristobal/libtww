// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libtww::std::fmt;

use syntax;

/// An error that occurred during parsing or compiling a regular expression.
#[derive(Debug)]
pub enum Error {
    /// A syntax error.
    Syntax(syntax::Error),
    /// The compiled program exceeded the set size limit.
    /// The argument is the size limit imposed.
    CompiledTooBig(usize),
    /// **DEPRECATED:** Will be removed on next major version bump.
    ///
    /// This error is no longer used. (A `RegexSet` can now contain zero or
    /// more regular expressions.)
    InvalidSet,
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}

impl ::libtww::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Syntax(ref err) => err.description(),
            Error::CompiledTooBig(_) => "compiled program too big",
            Error::InvalidSet => "sets must contain 2 or more regular expressions",
            Error::__Nonexhaustive => unreachable!(),
        }
    }

    fn cause(&self) -> Option<&::libtww::std::error::Error> {
        match *self {
            Error::Syntax(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Syntax(ref err) => err.fmt(f),
            Error::CompiledTooBig(limit) => {
                write!(f, "Compiled regex exceeds size limit of {} bytes.", limit)
            }
            Error::InvalidSet => write!(f, "Sets must contain 2 or more regular expressions."),
            Error::__Nonexhaustive => unreachable!(),
        }
    }
}

impl From<syntax::Error> for Error {
    fn from(err: syntax::Error) -> Error {
        Error::Syntax(err)
    }
}
