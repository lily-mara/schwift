use super::grammar;
use super::statement::Statement;
use super::value::Value;
use super::Operator;
use rand::{thread_rng, Rng};
use std::io;
use std::process;

pub type SwResult<T> = Result<T, ErrorKind>;
pub type SwErResult<T> = Result<T, Error>;

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

#[derive(Debug)]
pub struct Error {
    pub place: Statement,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    UnknownVariable(String),
    IndexUnindexable(Value),
    SyntaxError(grammar::ParseError),
    IndexOutOfBounds(Value, usize),
    IOError(io::Error),
    UnexpectedType(String, Value),
    InvalidBinaryExpression(Value, Value, Operator),
    InvalidArguments(String, usize, usize),
    NoReturn(String),
    NonFunctionCallInDylib(Statement),
}

impl From<io::Error> for ErrorKind {
    fn from(err: io::Error) -> Self {
        ErrorKind::IOError(err)
    }
}

impl PartialEq for ErrorKind {
    fn eq(&self, other: &ErrorKind) -> bool {
        use self::ErrorKind::*;

        match (self, other) {
            (&IndexUnindexable(ref s), &IndexUnindexable(ref o)) => s == o,
            (&SyntaxError(ref s), &SyntaxError(ref o)) => s == o,
            (&UnknownVariable(ref s), &UnknownVariable(ref o))
            | (&NoReturn(ref s), &NoReturn(ref o)) => s == o,
            (&InvalidArguments(ref sn, ss1, ss2), &InvalidArguments(ref on, os1, os2)) => {
                sn == on && ss1 == os1 && ss2 == os2
            }
            (&IndexOutOfBounds(ref sv, si), &IndexOutOfBounds(ref ov, oi)) => sv == ov && si == oi,
            (&NonFunctionCallInDylib(ref s), &NonFunctionCallInDylib(ref o)) => s == o,
            (&IOError(_), &IOError(_)) => true,
            (&UnexpectedType(ref ss, ref sv), &UnexpectedType(ref os, ref ov)) => {
                ss == os && sv == ov
            }
            (
                &InvalidBinaryExpression(ref sv1, ref sv2, ref so),
                &InvalidBinaryExpression(ref ov1, ref ov2, ref oo),
            ) => sv1 == ov1 && sv2 == ov2 && so == oo,
            _ => false,
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind, place: Statement) -> Self {
        Error { kind, place }
    }

    pub fn panic_message(&self) -> String {
        use self::ErrorKind::*;

        match self.kind {
            UnknownVariable(ref name) => format!("There's no {} in this universe, Morty!", name),
            NoReturn(ref fn_name) => format!(
                "Morty, your function has to return a value! {} just runs and dies like \
                 an animal!",
                fn_name
            ),
            IndexUnindexable(ref value) => format!(
                "I'll try and say this slowly Morty. You can't index that. It's a {}",
                value.type_str()
            ),
            SyntaxError(ref err) => format!(
                "If you're going to start trying to construct sub-programs in your \
                 programs Morty, you'd better make sure you're careful! {:?}",
                err
            ),
            IndexOutOfBounds(ref list, ref index) => format!(
                "This isn't your mom's wine bottle Morty, you can't just keep asking for \
                 more, there's not that much here! You want {}, but your cob only has {} \
                 kernels on it!",
                index,
                list.len().unwrap()
            ),
            IOError(ref err) => format!(
                "Looks like we're having a comm-burp-unications problem Morty: {:?}",
                err
            ),
            UnexpectedType(ref expected, ref value) => format!(
                "I asked for a {}, not a {} Morty.",
                expected,
                value.type_str()
            ),
            InvalidBinaryExpression(ref lhs, ref rhs, ref op) => format!(
                "It's like apples and space worms Morty! You can't {:?} a {} and a {}!",
                op,
                lhs.type_str(),
                rhs.type_str()
            ),
            InvalidArguments(ref name, expected, actual) => format!(
                "I'm confused Morty, a minute ago you said that {} takes {} paramaters, \
                 but you just tried to give it {}. WHICH IS IT MORTY?",
                name, expected, actual
            ),
            NonFunctionCallInDylib(_) => {
                "Is this a miniverse, or a microverse, or a teeny-verse? All I know is \
                 you fucked up."
                    .into()
            }
        }
    }

    pub fn full_panic_message(&self, filename: &str) -> String {
        let type_msg = self.panic_message();
        let quote = random_quote();

        println!("{}", filename);

        let source_part = self.place.get_source(filename).unwrap();

        format!(
            r#"
    You made a Rickdiculous mistake:

    {}
    {}

    {}

    "#,
            source_part, type_msg, quote
        )
    }

    pub fn panic(&self, source: &str) {
        println!("{}", self.full_panic_message(source));
        process::exit(1);
    }
}

fn random_quote() -> &'static str {
    let mut rng = thread_rng();
    rng.choose(&QUOTES).unwrap()
}
