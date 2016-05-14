#![allow(clippy)]

#[cfg(any(feature = "trace", test))]
mod grammar_debug;

#[cfg(not(any(feature = "trace", test)))]
mod grammar;

#[cfg(any(feature = "trace", test))]
pub use self::grammar_debug::*;

#[cfg(not(any(feature = "trace", test)))]
pub use self::grammar::*;

#[cfg(test)]
mod tests;
