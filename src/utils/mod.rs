#[cfg(feature="flame")]
mod with_flame;

#[cfg(feature="flame")]
pub use self::with_flame::*;

#[cfg(not(feature="flame"))]
mod no_flame;

#[cfg(not(feature="flame"))]
pub use self::no_flame::*;
