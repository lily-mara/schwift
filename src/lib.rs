#![feature(plugin)]
#![plugin(peg_syntax_ext, clippy)]

extern crate rand;

use std::collections::HashMap;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::cmp::Ordering;

pub type SwResult<T> = Result<T, ErrorKind>;
pub type SwErResult<T> = Result<T, Error>;

peg_file! grammar("schwift.rustpeg");

#[cfg(test)]
mod grammar_tests;

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    List(Vec<Value>),
}

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

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Variable(String),
    OperatorExpression(Box<Expression>, Operator, Box<Expression>),
    Value(Value),
    ListIndex(String, Box<Expression>),
    ListLength(String),
    Not(Box<Expression>),
    Eval(Box<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
    start: usize,
    end: usize,
    kind: StatementKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementKind {
    Assignment(String, Expression),
    Delete(String),
    Print(Expression),
    ListNew(String),
    ListAppend(String, Expression),
    ListAssign(String, Expression, Expression),
    ListDelete(String, Expression),
    If(Expression, Vec<Statement>, Option<Vec<Statement>>),
    While(Expression, Vec<Statement>),
    Input(String),
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

        let mut source = String::new();
        let mut f = File::open(filename).unwrap();
        f.read_to_string(&mut source).unwrap();

        assert!(self.place.start < self.place.end);
        assert!(source.is_char_boundary(self.place.start));
        assert!(source.is_char_boundary(self.place.end));

        let source_part = unsafe { source.slice_unchecked(self.place.start, self.place.end) };

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

impl Expression {
    pub fn eval(&self, state: &State) -> SwResult<Value> {
        match *self {
            Expression::Variable(ref var_name) => {
                match state.symbols.get(var_name) {
                    Some(value) => Ok(value.clone()),
                    None => Err(ErrorKind::UnknownVariable(var_name.clone())),
                }
            }
            Expression::OperatorExpression(ref left_exp, ref operator, ref right_exp) => {
                let left = try!(left_exp.eval(state));
                let right = try!(right_exp.eval(state));
                match *operator {
                    Operator::Add => left.add(&right),
                    Operator::Subtract => left.subtract(&right),
                    Operator::Multiply => left.multiply(&right),
                    Operator::Divide => left.divide(&right),
                    Operator::Equality => Ok(left.equals(&right)),
                    Operator::LessThan => left.less_than(&right),
                    Operator::GreaterThan => left.greater_than(&right),
                    Operator::LessThanEqual => left.less_than_equal(&right),
                    Operator::GreaterThanEqual => left.greater_than_equal(&right),
                    Operator::ShiftLeft => left.shift_left(&right),
                    Operator::ShiftRight => left.shift_right(&right),
                    Operator::And => left.and(&right),
                    Operator::Or => left.or(&right),
                }
            }
            Expression::Value(ref v) => Ok(v.clone()),
            Expression::ListIndex(ref var_name, ref e) => state.list_index(var_name, e),
            Expression::Not(ref e) => try!(e.eval(state)).not(),
            Expression::ListLength(ref var_name) => {
                match state.symbols.get(var_name) {
                    Some(value) => {
                        match *value {
                            Value::List(ref list) => Ok(Value::Int(list.len() as i32)),
                            Value::Str(ref s) => Ok(Value::Int(s.len() as i32)),
                            _ => Err(ErrorKind::IndexUnindexable(value.clone())),
                        }
                    }
                    None => Err(ErrorKind::UnknownVariable(var_name.clone())),
                }
            }
            Expression::Eval(ref exp) => {
                let inner_val = try!(exp.eval(state));
                if let Value::Str(ref inner) = inner_val {
                    match grammar::expression(inner) {
                        Ok(inner_evaled) => inner_evaled.eval(state),
                        Err(s) => Err(ErrorKind::SyntaxError(s)),
                    }
                } else {
                    Err(ErrorKind::UnexpectedType("string".to_string(), inner_val))
                }
            }
        }
    }

    pub fn try_bool(&self, state: &State) -> SwResult<bool> {
        let value = try!(self.eval(state));
        if let Value::Bool(x) = value {
            Ok(x)
        } else {
            Err(ErrorKind::UnexpectedType("bool".to_string(), value))
        }
    }

    pub fn try_int(&self, state: &State) -> SwResult<i32> {
        let value = try!(self.eval(state));
        if let Value::Int(x) = value {
            Ok(x)
        } else {
            Err(ErrorKind::UnexpectedType("int".to_string(), value))
        }
    }
}

impl State {
    fn list_index(&self, list_name: &str, exp: &Expression) -> SwResult<Value> {
        let inner_expression_value = try!(exp.eval(self));
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
        let v = try!(exp.eval(self));
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
        let x = try!(exp.eval(self));
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
        let to_append = try!(append_exp.eval(self));
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
        let to_assign = try!(assign_exp.eval(self));
        let element = try!(self.get_list_element(list_name, index_exp));

        *element = to_assign;
        Ok(())
    }

    fn list_delete(&mut self, list_name: &str, index_exp: &Expression) -> SwResult<()> {
        let index_value = try!(index_exp.eval(self));
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
        let x = match bool.eval(self) {
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

impl Value {
    pub fn print(&self) {
        match *self {
            Value::Int(i) => print!("{}", i),
            Value::Float(i) => print!("{}", i),
            Value::Bool(i) => print!("{}", i),
            Value::Str(ref i) => print!("{}", i),
            Value::List(ref i) => print!("{:?}", i),
        }
    }

    pub fn println(&self) {
        self.print();
        println!("");
    }

    fn assert_f32(&self) -> SwResult<f32> {
        match *self {
            Value::Float(f) => Ok(f),
            Value::Int(i) => Ok(i as f32),
            _ => Err(ErrorKind::UnexpectedType("float".to_string(), self.clone())),
        }
    }

    fn assert_bool(&self) -> SwResult<bool> {
        match *self {
            Value::Bool(b) => Ok(b),
            _ => Err(ErrorKind::UnexpectedType("bool".to_string(), self.clone())),
        }
    }

    pub fn not(&mut self) -> SwResult<Value> {
        match *self {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            _ => Err(ErrorKind::UnexpectedType("bool".to_string(), self.clone())),
        }
    }

    pub fn less_than(&self, other: &Value) -> SwResult<Value> {
        Ok(Value::Bool(try!(self.assert_f32()) < try!(other.assert_f32())))
    }

    pub fn greater_than(&self, other: &Value) -> SwResult<Value> {
        Ok(Value::Bool(try!(self.assert_f32()) > try!(other.assert_f32())))
    }

    pub fn greater_than_equal(&self, other: &Value) -> SwResult<Value> {
        Ok(Value::Bool(try!(self.assert_f32()) >= try!(other.assert_f32())))
    }

    pub fn less_than_equal(&self, other: &Value) -> SwResult<Value> {
        Ok(Value::Bool(try!(self.assert_f32()) <= try!(other.assert_f32())))
    }

    pub fn and(&self, other: &Value) -> SwResult<Value> {
        Ok(Value::Bool(try!(self.assert_bool()) && try!(other.assert_bool())))
    }

    pub fn or(&self, other: &Value) -> SwResult<Value> {
        Ok(Value::Bool(try!(self.assert_bool()) || try!(other.assert_bool())))
    }

    pub fn equals(&self, other: &Value) -> Value {
        Value::Bool(self.eq(other))
    }

    pub fn add(&self, other: &Value) -> SwResult<Value> {
        match (self, other) {
            (&Value::Float(f1), &Value::Float(f2)) => Ok(Value::Float(f1 + f2)),
            (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(i1 + i2)),
            (&Value::Float(f), &Value::Int(i)) |
            (&Value::Int(i), &Value::Float(f)) => Ok(Value::Float(i as f32 + f)),
            (&Value::Str(ref s1), &Value::Str(ref s2)) => {
                let mut new_buf = s1.clone();
                new_buf.push_str(s2);
                Ok(Value::Str(new_buf))
            }
            _ => {
                Err(ErrorKind::InvalidBinaryExpression(self.clone(), other.clone(), Operator::Add))
            }
        }
    }

    pub fn subtract(&self, other: &Value) -> SwResult<Value> {
        match (self, other) {
            (&Value::Float(f1), &Value::Float(f2)) => Ok(Value::Float(f1 - f2)),
            (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(i1 - i2)),
            (&Value::Float(f), &Value::Int(i)) => Ok(Value::Float(f - i as f32)),
            (&Value::Int(i), &Value::Float(f)) => Ok(Value::Float(i as f32 - f)),
            _ => {
                Err(ErrorKind::InvalidBinaryExpression(self.clone(),
                                                       other.clone(),
                                                       Operator::Subtract))
            }
        }
    }

    pub fn multiply(&self, other: &Value) -> SwResult<Value> {
        match (self, other) {
            (&Value::Float(f1), &Value::Float(f2)) => Ok(Value::Float(f1 * f2)),
            (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(i1 * i2)),
            (&Value::Float(f), &Value::Int(i)) |
            (&Value::Int(i), &Value::Float(f)) => Ok(Value::Float(i as f32 * f)),
            (&Value::Str(ref s), &Value::Int(i)) => {
                let mut new_buf = s.clone();
                for _ in 0..(i - 1) {
                    new_buf.push_str(s);
                }
                Ok(Value::Str(new_buf))
            }
            _ => {
                Err(ErrorKind::InvalidBinaryExpression(self.clone(),
                                                       other.clone(),
                                                       Operator::Multiply))
            }
        }
    }

    pub fn divide(&self, other: &Value) -> SwResult<Value> {
        match (self, other) {
            (&Value::Float(f1), &Value::Float(f2)) => Ok(Value::Float(f1 / f2)),
            (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(i1 / i2)),
            (&Value::Float(f), &Value::Int(i)) => Ok(Value::Float(f / i as f32)),
            (&Value::Int(i), &Value::Float(f)) => Ok(Value::Float(i as f32 / f)),
            _ => {
                Err(ErrorKind::InvalidBinaryExpression(self.clone(),
                                                       other.clone(),
                                                       Operator::Divide))
            }
        }
    }

    pub fn shift_left(&self, other: &Value) -> SwResult<Value> {
        match (self, other) {
            (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(i1 << i2)),
            _ => {
                Err(ErrorKind::InvalidBinaryExpression(self.clone(),
                                                       other.clone(),
                                                       Operator::ShiftLeft))
            }
        }
    }

    pub fn shift_right(&self, other: &Value) -> SwResult<Value> {
        match (self, other) {
            (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(i1 >> i2)),
            _ => {
                Err(ErrorKind::InvalidBinaryExpression(self.clone(),
                                                       other.clone(),
                                                       Operator::ShiftRight))
            }
        }
    }

    pub fn type_str<'a>(&self) -> &'a str {
        match *self {
            Value::Str(_) => "string",
            Value::Int(_) => "int",
            Value::Bool(_) => "bool",
            Value::List(_) => "list",
            Value::Float(_) => "float",
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        if self == other {
            Option::Some(Ordering::Equal)
        } else {
            let (s, o) = match (self, other) {
                (&Value::Int(i), &Value::Float(f)) => (i as f32, f),
                (&Value::Float(f), &Value::Int(i)) => (f, i as f32),
                _ => return Option::None,
            };

            s.partial_cmp(&o)
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (&Value::Bool(b1), &Value::Bool(b2)) => b1 == b2,
            (&Value::Str(ref s1), &Value::Str(ref s2)) => s1 == s2,
            (&Value::List(ref l1), &Value::List(ref l2)) => l1 == l2,
            (&Value::Int(i1), &Value::Int(i2)) => i1 == i2,
            (&Value::Int(i), &Value::Float(f)) |
            (&Value::Float(f), &Value::Int(i)) => (i as f32 - f).abs() < std::f32::EPSILON,
            (&Value::Float(f1), &Value::Float(f2)) => (f1 - f2).abs() < std::f32::EPSILON,
            _ => false,
        }
    }

    fn ne(&self, other: &Value) -> bool {
        !self.eq(other)
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
