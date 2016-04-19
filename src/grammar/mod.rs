#[cfg(feature = "trace")]
mod grammar_debug;

#[cfg(not(feature = "trace"))]
mod grammar;

#[cfg(feature = "trace")]
pub use self::grammar_debug::*;

#[cfg(not(feature = "trace"))]
pub use self::grammar::*;

#[cfg(test)]
mod tests;
