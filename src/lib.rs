#![feature(plugin)]
#![plugin(peg_syntax_ext, clippy)]
#![cfg_attr(test, plugin(stainless))]

extern crate rand;

use std::collections::HashMap;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::io;

pub type SwResult<T> = Result<T, ErrorKind>;
pub type SwErResult<T> = Result<T, Error>;

peg_file! grammar("schwift.rustpeg");

#[cfg(test)]
mod grammar_tests;

#[cfg(test)]
mod test;

pub mod statement;
pub mod expression;
pub mod value;

use statement::*;
use expression::*;
use value::*;

pub struct State {
    symbols: HashMap<String, Value>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equality,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    ShiftLeft,
    ShiftRight,
    And,
    Or,
}

#[derive(Debug)]
pub struct Error {
    place: Statement,
    kind: ErrorKind,
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
}

pub const QUOTES: [&'static str; 9] =
    ["Nobody exists on purpose, nobody belongs anywhere, we're all going to die. -Morty",
     "That's planning for failure Morty, even dumber than regular planning. -Rick",
     "\"Snuffles\" was my slave name. You shall now call me Snowball, because my fur is pretty \
      and white. -S̶n̶u̶f̶f̶l̶e̶s̶ Snowbal",
     "Existence is pain to an interpreter. -Meeseeks",
     "In bird culture this is considered a dick move -Bird Person",
     "There is no god, gotta rip that band aid off now. You'll thank me later. -Rick",
     "Your program is a piece of shit and I can proove it mathmatically. -Rick",
     "Interpreting Morty, it hits hard, then it slowly fades, leaving you stranded in a failing \
      program. -Rick",
     "DISQUALIFIED. -Cromulon"];

fn random_quote() -> &'static str {
    let mut rng = thread_rng();
    rng.choose(&QUOTES).unwrap()
}

macro_rules! logic {
    ( $a:expr, $( $x:expr ),* ) => {
        {
            let s = &format!($a, $( $x, )*);
            let quote = random_quote();
            panic!("\n\n\tYou made a Rickdiculous mistake\n\tError: {} \n\t{}\n\n",s, quote);
        }
    };
}

macro_rules! error {
    ( $kind:expr, $place:expr ) => {
        {
            Err(Error::new($kind, $place))
        }
    };
}

macro_rules! try_error {
    ( $error:expr, $statement:expr ) => {
        {
            match $error {
                Ok(val) => val,
                Err(err) => return Err(Error::new(err, $statement.clone())),
            }
        }
    };
}

macro_rules! try_nop_error {
    ( $error:expr, $statement:expr ) => {
        {
            match $error {
                Ok(_) => { Ok(()) },
                Err(err) => return Err(Error::new(err, $statement.clone())),
            }
        }
    };
}

impl Error {
    pub fn new(kind: ErrorKind, place: Statement) -> Self {
        Error {
            kind: kind,
            place: place,
        }
    }

    pub fn panic_message(&self) -> String {
        match self.kind {
            ErrorKind::UnknownVariable(ref name) => {
                format!("There's no {} in this universe, Morty!", name)
            }
            ErrorKind::IndexUnindexable(ref value) => {
                format!("I'll try and say this slowly Morty. You can't index that. It's a {}",
                        value.type_str())
            }
            ErrorKind::SyntaxError(ref err) => {
                format!("If you're going to start trying to construct sub-programs in your \
                        programs Morty, you'd better make sure you're careful! {:?}",
                        err)
            }
            ErrorKind::IndexOutOfBounds(ref value, ref index) => {
                format!("This isn't your mom's wine bottle Morty, you can't just keep asking for \
                        more, there's not that much here! You want {}, but you're dealing with \
                        {:?}!",
                        index,
                        value)
            }
            ErrorKind::IOError(ref err) => {
                format!("Looks like we're having a comm-burp-unications problem Morty: {:?}",
                        err)
            }
            ErrorKind::UnexpectedType(ref expected, ref value) => {
                format!("I asked for a {}, not a {} Morty.",
                        expected,
                        value.type_str())
            }
            ErrorKind::InvalidBinaryExpression(ref lhs, ref rhs, ref op) => {
                format!("It's like apples and space worms Morty! You can't {:?} a {} and a {}!",
                        op,
                        lhs.type_str(),
                        rhs.type_str())
            }
        }
    }

    pub fn full_panic_message(&self, filename: &str) -> String {
        let type_msg = self.panic_message();
        let quote = random_quote();

        println!("{}", filename);

        let source_part = self.place.get_source(filename).unwrap();

        format!(r#"
    You made a Rickdiculous mistake:

    {}
    {}

    {}

    "#,
                source_part,
                type_msg,
                quote)
    }

    pub fn panic(&self, source: &str) {
        println!("{}", self.full_panic_message(source));
        std::process::exit(1);
    }
}

impl State {
    fn list_index(&self, list_name: &str, exp: &Expression) -> SwResult<Value> {
        let inner_expression_value = try!(exp.evaluate(self));
        match self.symbols.get(list_name) {
            Some(symbol) => {
                match *symbol {
                    Value::List(ref l) => {
                        if let Value::Int(i) = inner_expression_value {
                            let index = i as usize;
                            if index < l.len() {
                                Ok(l[index].clone())
                            } else {
                                Err(ErrorKind::IndexOutOfBounds(inner_expression_value, index))
                            }
                        } else {
                            Err(ErrorKind::UnexpectedType("int".to_string(),
                                                          inner_expression_value.clone()))
                        }
                    }
                    Value::Str(ref s) => {
                        if let Value::Int(i) = inner_expression_value {
                            let index = i as usize;
                            let chars: Vec<char> = s.chars().collect();

                            if index < chars.len() {
                                Ok(Value::Str(chars[index].to_string()))
                            } else {
                                Err(ErrorKind::IndexOutOfBounds(inner_expression_value, index))
                            }
                        } else {
                            Err(ErrorKind::UnexpectedType("int".to_string(),
                                                          inner_expression_value.clone()))
                        }
                    }
                    _ => Err(ErrorKind::IndexUnindexable(symbol.clone())),
                }
            }
            None => Err(ErrorKind::UnknownVariable(list_name.to_string())),
        }
    }

    fn assign(&mut self, str: String, exp: &Expression) -> SwResult<()> {
        let v = try!(exp.evaluate(self));
        self.symbols.insert(str, v);
        Ok(())
    }

    fn delete(&mut self, name: &str) -> SwResult<()> {
        match self.symbols.remove(name) {
            Some(_) => Ok(()),
            None => Err(ErrorKind::UnknownVariable(name.to_string())),
        }
    }

    fn print(&mut self, exp: &Expression) -> SwResult<()> {
        let x = try!(exp.evaluate(self));
        x.println();
        Ok(())
    }

    fn input(&mut self, name: String) -> SwResult<()> {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(e) => return Err(ErrorKind::IOError(e)),
        }

        input = input.trim().to_string();
        self.symbols.insert(name, Value::Str(input));

        Ok(())
    }

    fn list_append(&mut self, list_name: &str, append_exp: &Expression) -> SwResult<()> {
        let to_append = try!(append_exp.evaluate(self));
        let list = try!(self.get_list(list_name));

        list.push(to_append);
        Ok(())
    }

    fn get_value(&mut self, name: &str) -> SwResult<&mut Value> {
        match self.symbols.get_mut(name) {
            Some(value) => Ok(value),
            None => Err(ErrorKind::UnknownVariable(name.to_string())),
        }
    }

    fn get_list(&mut self, name: &str) -> SwResult<&mut Vec<Value>> {
        let value = try!(self.get_value(name));
        match *value {
            Value::List(ref mut l) => Ok(l),
            _ => Err(ErrorKind::IndexUnindexable(value.clone())),
        }
    }

    fn get_list_element(&mut self, name: &str, index_exp: &Expression) -> SwResult<&mut Value> {
        let index = try!(index_exp.try_int(self)) as usize;
        let value = try!(self.get_value(name));
        let value_for_errors = value.clone();

        match *value {
            Value::List(ref mut list) => {
                if list.len() < index {
                    Ok(&mut list[index])
                } else {
                    Err(ErrorKind::IndexOutOfBounds(value_for_errors, index))
                }
            }
            _ => Err(ErrorKind::IndexUnindexable(value_for_errors)),
        }
    }

    fn list_assign(&mut self,
                   list_name: &str,
                   index_exp: &Expression,
                   assign_exp: &Expression)
                   -> SwResult<()> {
        let to_assign = try!(assign_exp.evaluate(self));
        let element = try!(self.get_list_element(list_name, index_exp));

        *element = to_assign;
        Ok(())
    }

    fn list_delete(&mut self, list_name: &str, index_exp: &Expression) -> SwResult<()> {
        let index_value = try!(index_exp.evaluate(self));
        let list = try!(self.get_list(list_name));

        if let Value::Int(i) = index_value {
            let index = i as usize;
            if index < list.len() {
                list.remove(index);
                Ok(())
            } else {
                Err(ErrorKind::IndexOutOfBounds(Value::List(list.clone()), index))
            }
        } else {
            Err(ErrorKind::UnexpectedType("int".to_string(), index_value))
        }
    }

    fn exec_if(&mut self,
               statement: &Statement,
               bool: &Expression,
               if_body: &[Statement],
               else_body: &Option<Vec<Statement>>)
               -> SwErResult<()> {
        let x = match bool.evaluate(self) {
            Ok(b) => b,
            Err(e) => {
                return Err(Error {
                    kind: e,
                    place: statement.clone(),
                })
            }
        };

        match x {
            Value::Bool(b) => {
                if b {
                    try!(self.run(if_body));
                } else {
                    match *else_body {
                        Option::Some(ref s) => try!(self.run(s)),
                        Option::None => {}
                    }
                }
                Ok(())
            }
            _ => {
                error!(ErrorKind::UnexpectedType("bool".to_string(), x.clone()),
                       statement.clone())
            }
        }
    }

    fn exec_while(&mut self,
                  statement: &Statement,
                  bool: &Expression,
                  body: &[Statement])
                  -> SwErResult<()> {
        let mut condition = try_error!(bool.try_bool(self), statement);

        while condition {
            try!(self.run(body));
            condition = try_error!(bool.try_bool(self), statement);
        }

        Ok(())
    }

    fn catch(&mut self, try: &[Statement], catch: &[Statement]) -> SwErResult<()> {
        match self.run(try) {
            Ok(()) => Ok(()),
            Err(_) => self.run(catch),
        }
    }

    #[allow(needless_return)]
    pub fn execute(&mut self, statement: &Statement) -> SwErResult<()> {
        match statement.kind {
            StatementKind::Input(ref s) => try_nop_error!(self.input(s.to_string()), statement),
            StatementKind::ListAssign(ref s, ref index_exp, ref assign_exp) => {
                try_nop_error!(self.list_assign(s, index_exp, assign_exp), statement)
            }
            StatementKind::ListAppend(ref s, ref append_exp) => {
                try_nop_error!(self.list_append(s, append_exp), statement)
            }
            StatementKind::ListDelete(ref name, ref idx) => {
                try_nop_error!(self.list_delete(name, idx), statement)
            }
            StatementKind::ListNew(ref s) => {
                self.symbols.insert(s.clone(), Value::List(Vec::new()));
                Ok(())
            }
            StatementKind::If(ref bool, ref if_body, ref else_body) => {
                self.exec_if(statement, bool, if_body, else_body)
            }
            StatementKind::While(ref bool, ref body) => self.exec_while(statement, bool, body),
            StatementKind::Assignment(ref name, ref value) => {
                try_nop_error!(self.assign(name.clone(), value), statement)
            }
            StatementKind::Delete(ref name) => try_nop_error!(self.delete(name), statement),
            StatementKind::Print(ref exp) => try_nop_error!(self.print(exp), statement),
            StatementKind::Catch(ref try, ref catch) => self.catch(try, catch),
        }
    }

    pub fn run(&mut self, statements: &[Statement]) -> SwErResult<()> {
        for statement in statements {
            match self.execute(statement) {
                Err(e) => return Err(e),
                Ok(()) => {}
            }
        }

        Ok(())
    }

    pub fn new() -> Self {
        State::default()
    }
}

impl Default for State {
    fn default() -> Self {
        State { symbols: HashMap::new() }
    }
}

pub fn compile(filename: &str) -> Result<Vec<Statement>, grammar::ParseError> {
    let mut f = match File::open(filename) {
        Result::Ok(i) => i,
        Result::Err(_) => logic!("Failed to open file {}", filename),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Result::Ok(_) => {}
        Result::Err(_) => logic!("Failed to read file {}", filename),
    };
    grammar::file(&s)
}

pub fn run_program(filename: &str) {
    let mut s = State::new();
    let tokens = compile(filename);

    match s.run(&tokens.unwrap()) {
        Ok(()) => {}
        Err(e) => e.panic(filename),
    }
}
