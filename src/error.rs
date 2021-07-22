use crate::{grammar, statement::Statement, value, Operator};
use rand::{seq::SliceRandom, thread_rng};
use std::{fmt::Write, io, process};

pub type SwResult<T> = Result<T, EitherError>;

pub const QUOTES: [&str; 9] = [
    "Nobody exists on purpose, nobody belongs anywhere, we're all going to die. -Morty",
    "That's planning for failure Morty, even dumber than regular planning. -Rick",
    "\"Snuffles\" was my slave name. You shall now call me Snowball, because my fur is pretty \
     and white. -S̶n̶u̶f̶f̶l̶e̶s̶ Snowbal",
    "Existence is pain to an interpreter. -Meeseeks",
    "In bird culture this is considered a dick move -Bird Person",
    "There is no god, gotta rip that band aid off now. You'll thank me later. -Rick",
    "Your program is a piece of shit and I can proove it mathmatically. -Rick",
    "Interpreting Morty, it hits hard, then it slowly fades, leaving you stranded in a failing \
     program. -Rick",
    "DISQUALIFIED. -Cromulon",
];

#[derive(Debug, thiserror::Error, PartialEq)]
#[error("An error ocurred")]
pub struct ErrorWithContext {
    place: Statement,

    kind: ErrorKind,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum EitherError {
    #[error("An error with statement context")]
    WithContext(#[from] ErrorWithContext),

    #[error("An error with no statment context")]
    NoContext(#[source] ErrorKind),
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("There's no {0} in this universe, Morty!")]
    UnknownVariable(String),

    #[error("I'll try and say this slowly Morty. You can't index that. It's a {0}")]
    IndexUnindexable(value::Type),

    #[error("If you're going to start trying to construct sub-programs in your programs Morty, you'd better make sure you're careful!")]
    SyntaxError(grammar::ParseError),

    #[error("Y-you can't just keep asking for more, Morty! You want {index}, but your cob only has {len} kernels on it!")]
    IndexOutOfBounds { len: usize, index: usize },

    #[error("Looks like we're having a comm-burp-unications problem Morty")]
    IOError(#[from] io::Error),

    #[error("I asked for a {expected}, not a {actual} Morty.")]
    UnexpectedType {
        actual: value::Type,
        expected: value::Type,
    },

    // TODO
    #[error("load error")]
    LoadError(#[from] libloading::Error),

    #[error("It's like apples and space worms Morty! You can't {2:?} a {0} and a {1}!")]
    InvalidBinaryExpression(value::Type, value::Type, Operator),

    #[error("I'm confused Morty, a minute ago you said that {0} takes {1} paramaters, but you just tried to give it {2}. WHICH IS IT MORTY?")]
    InvalidArguments(String, usize, usize),

    #[error("Morty, your function has to return a value! {0} just runs and dies like an animal!")]
    NoReturn(String),

    #[error(
        "Is this a miniverse, or a microverse, or a teeny-verse? All I know is you fucked up."
    )]
    NonFunctionCallInDylib(Statement),

    #[error("Wait, wait, I'm confused. Just a second ago, you said that {library} was a microverse, but when I looked there, I didn't know what I was looking at.")]
    MissingAbiCompat {
        #[source]
        error: libloading::Error,
        library: String,
    },

    #[error("That's an older code, Morty and it does not check out. That microverse can only be run by schwift {0}, but this is {}", crate::LIBSCHWIFT_ABI_COMPAT)]
    IncompatibleAbi(u32),

    #[error(
        "I told you how a Microverse works Morty. At what point exactly did you stop listening?"
    )]
    DylibReturnedNil,
}

impl<T> From<T> for EitherError
where
    ErrorKind: From<T>,
{
    fn from(x: T) -> EitherError {
        EitherError::NoContext(ErrorKind::from(x))
    }
}

pub trait ErrorKindExt<T> {
    fn with_error_ctx(self, stmt: &Statement) -> Result<T, ErrorWithContext>;
}

impl<T> ErrorKindExt<T> for std::result::Result<T, ErrorKind> {
    fn with_error_ctx(self, stmt: &Statement) -> Result<T, ErrorWithContext> {
        self.map_err(|e| ErrorWithContext::new(e, stmt.clone()))
    }
}

impl<T> ErrorKindExt<T> for std::result::Result<T, ErrorWithContext> {
    fn with_error_ctx(self, _stmt: &Statement) -> Result<T, ErrorWithContext> {
        self
    }
}

impl<T> ErrorKindExt<T> for std::result::Result<T, EitherError> {
    fn with_error_ctx(self, stmt: &Statement) -> Result<T, ErrorWithContext> {
        self.map_err(|e| match e {
            EitherError::NoContext(e) => ErrorWithContext::new(e, stmt.clone()),
            EitherError::WithContext(e) => e,
        })
    }
}

impl PartialEq for ErrorKind {
    fn eq(&self, other: &Self) -> bool {
        use self::ErrorKind::*;

        match (self, other) {
            (IndexUnindexable(ref s), IndexUnindexable(ref o)) => s == o,
            (SyntaxError(ref s), SyntaxError(ref o)) => s == o,
            (UnknownVariable(ref s), UnknownVariable(ref o))
            | (NoReturn(ref s), NoReturn(ref o)) => s == o,
            (InvalidArguments(ref sn, ss1, ss2), InvalidArguments(ref on, os1, os2)) => {
                sn == on && ss1 == os1 && ss2 == os2
            }
            (
                IndexOutOfBounds {
                    len: slen,
                    index: sindex,
                },
                IndexOutOfBounds {
                    len: olen,
                    index: oindex,
                },
            ) => slen == olen && sindex == oindex,
            (NonFunctionCallInDylib(ref s), NonFunctionCallInDylib(ref o)) => s == o,
            (IOError(_), IOError(_)) => true,
            (
                UnexpectedType {
                    actual: ref sactual,
                    expected: ref sexpected,
                },
                UnexpectedType {
                    actual: ref oactual,
                    expected: ref oexpected,
                },
            ) => sactual == oactual && sexpected == oexpected,
            (
                InvalidBinaryExpression(ref sv1, ref sv2, ref so),
                InvalidBinaryExpression(ref ov1, ref ov2, ref oo),
            ) => sv1 == ov1 && sv2 == ov2 && so == oo,
            (
                MissingAbiCompat {
                    library: lib1,
                    error: _,
                },
                MissingAbiCompat {
                    library: lib2,
                    error: _,
                },
            ) => lib1 == lib2,
            (IncompatibleAbi(ver1), IncompatibleAbi(ver2)) => ver1 == ver2,
            (DylibReturnedNil, DylibReturnedNil) => true,
            _ => false,
        }
    }
}

impl ErrorWithContext {
    pub fn new(kind: ErrorKind, place: Statement) -> Self {
        Self { kind, place }
    }

    pub fn full_panic_message(&self, filename: &str) -> String {
        let mut f = String::new();

        let quote = random_quote();

        writeln!(f, "{}", filename).unwrap();

        let source_part = self.place.get_source(&filename).unwrap();

        for c in anyhow::Chain::new(&self.kind) {
            writeln!(f, "{}", c).unwrap();
        }

        writeln!(
            f,
            r#"
    You made a Rickdiculous mistake:

    {}
    {}

    {}

    "#,
            source_part, self, quote
        )
        .unwrap();

        f
    }

    pub fn panic(&self, source: &str) {
        println!("{}", self.full_panic_message(source));
        process::exit(1);
    }
}

fn random_quote() -> &'static str {
    let mut rng = thread_rng();
    QUOTES.choose(&mut rng).unwrap()
}
