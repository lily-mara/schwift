#![feature(plugin)]
#![plugin(peg_syntax_ext, clippy)]

extern crate rand;

use std::collections::HashMap;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::cmp::Ordering;

pub type SwResult<T> = Result<T, Error>;

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

#[derive(Debug, PartialEq)]
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
pub enum Error {
    UnknownVariable(String),
    IndexUnindexable(Value),
    SyntaxError(grammar::ParseError),
    IndexOutOfBounds(Value, usize),
    IOError(io::Error),
    UnexpectedType(String, Value),
    InvalidBinaryExpression(Value, Value, Operator),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Variable(String),
    OperatorExpression(Box<Expression>, Operator, Box<Expression>),
    Value(Value),
    ListIndex(String, Box<Expression>),
    ListLength(String),
    Not(Box<Expression>),
    Eval(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
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

impl Error {
    pub fn panic(&self) {
        match *self {
            Error::UnknownVariable(ref name) => {
                logic!("There's no {} in this universe, Morty!", name)
            }
            Error::IndexUnindexable(ref value) => {
                logic!("I'll try and say this slowly Morty. You can't index that. It's a {}",
                       value.type_str())
            }
            Error::SyntaxError(ref err) => {
                logic!("If you're going to start trying to construct sub-programs in your \
                        programs Morty, you'd better make sure you're careful! {:?}",
                       err)
            }
            Error::IndexOutOfBounds(ref value, ref index) => {
                logic!("This isn't your mom's wine bottle Morty, you can't just keep asking for \
                        more, there's not that much here! You want {}, but you're dealing with \
                        {:?}!",
                       index,
                       value)
            }
            Error::IOError(ref err) => {
                logic!("Looks like we're having a comm-burp-unications problem Morty: {:?}",
                       err)
            }
            Error::UnexpectedType(ref expected, ref value) => {
                logic!("I asked for a {}, not a {} Morty.",
                       expected,
                       value.type_str())
            }
            Error::InvalidBinaryExpression(ref lhs, ref rhs, ref op) => {
                logic!("It's like apples and space worms Morty! You can't {:?} a {} and a {}!",
                       op,
                       lhs.type_str(),
                       rhs.type_str())
            }
        }
    }
}

impl Expression {
    pub fn eval(&self, state: &State) -> SwResult<Value> {
        match *self {
            Expression::Variable(ref var_name) => {
                match state.symbols.get(var_name) {
                    Some(value) => Ok(value.clone()),
                    None => Err(Error::UnknownVariable(var_name.clone())),
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
                            _ => Err(Error::IndexUnindexable(value.clone())),
                        }
                    }
                    None => Err(Error::UnknownVariable(var_name.clone())),
                }
            }
            Expression::Eval(ref exp) => {
                let inner_val = try!(exp.eval(state));
                if let Value::Str(ref inner) = inner_val {
                    match grammar::expression(inner) {
                        Ok(inner_evaled) => inner_evaled.eval(state),
                        Err(s) => Err(Error::SyntaxError(s)),
                    }
                } else {
                    Err(Error::UnexpectedType("string".to_string(), inner_val))
                }
            }
        }
    }

    pub fn try_bool(&self, state: &State) -> SwResult<bool> {
        let value = try!(self.eval(state));
        if let Value::Bool(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType("bool".to_string(), value))
        }
    }

    pub fn try_int(&self, state: &State) -> SwResult<i32> {
        let value = try!(self.eval(state));
        if let Value::Int(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType("int".to_string(), value))
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
                                Err(Error::IndexOutOfBounds(inner_expression_value, index))
                            }
                        } else {
                            Err(Error::UnexpectedType("int".to_string(),
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
                                Err(Error::IndexOutOfBounds(inner_expression_value, index))
                            }
                        } else {
                            Err(Error::UnexpectedType("int".to_string(),
                                                      inner_expression_value.clone()))
                        }
                    }
                    _ => Err(Error::IndexUnindexable(symbol.clone())),
                }
            }
            None => Err(Error::UnknownVariable(list_name.to_string())),
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
            None => Err(Error::UnknownVariable(name.to_string())),
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
            Err(e) => return Err(Error::IOError(e)),
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
            None => Err(Error::UnknownVariable(name.to_string())),
        }
    }

    fn get_list(&mut self, name: &str) -> SwResult<&mut Vec<Value>> {
        let value = try!(self.get_value(name));
        match *value {
            Value::List(ref mut l) => Ok(l),
            _ => Err(Error::IndexUnindexable(value.clone())),
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
                    Err(Error::IndexOutOfBounds(value_for_errors, index))
                }
            }
            _ => Err(Error::IndexUnindexable(value_for_errors)),
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
                Err(Error::IndexOutOfBounds(Value::List(list.clone()), index))
            }
        } else {
            Err(Error::UnexpectedType("int".to_string(), index_value))
        }
    }

    fn exec_if(&mut self,
               bool: &Expression,
               if_body: &[Statement],
               else_body: &Option<Vec<Statement>>)
               -> SwResult<()> {
        let x = try!(bool.eval(self));
        match x {
            Value::Bool(b) => {
                if b {
                    self.run(if_body);
                } else {
                    match *else_body {
                        Option::Some(ref s) => self.run(s),
                        Option::None => {}
                    }
                }
                Ok(())
            }
            _ => Err(Error::UnexpectedType("bool".to_string(), x.clone())),
        }
    }

    fn exec_while(&mut self, bool: &Expression, body: &[Statement]) -> SwResult<()> {
        let mut condition = try!(bool.try_bool(self));

        while condition {
            self.run(body);
            condition = try!(bool.try_bool(self));
        }

        Ok(())
    }

    pub fn execute(&mut self, statement: &Statement) -> SwResult<()> {
        match *statement {
            Statement::Input(ref s) => self.input(s.to_string()),
            Statement::ListAssign(ref s, ref index_exp, ref assign_exp) => {
                self.list_assign(s, index_exp, assign_exp)
            }
            Statement::ListAppend(ref s, ref append_exp) => self.list_append(s, append_exp),
            Statement::ListDelete(ref name, ref idx) => self.list_delete(name, idx),
            Statement::ListNew(ref s) => {
                self.symbols.insert(s.clone(), Value::List(Vec::new()));
                Ok(())
            }
            Statement::If(ref bool, ref if_body, ref else_body) => {
                self.exec_if(bool, if_body, else_body)
            }
            Statement::While(ref bool, ref body) => self.exec_while(bool, body),
            Statement::Assignment(ref name, ref value) => self.assign(name.clone(), value),
            Statement::Delete(ref name) => self.delete(name),
            Statement::Print(ref exp) => self.print(exp),
        }
    }

    pub fn run(&mut self, statements: &[Statement]) {
        for statement in statements {
            match self.execute(statement) {
                Ok(_) => {}
                Err(e) => e.panic(),
            }
        }
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
            _ => Err(Error::UnexpectedType("float".to_string(), self.clone())),
        }
    }

    fn assert_bool(&self) -> SwResult<bool> {
        match *self {
            Value::Bool(b) => Ok(b),
            _ => Err(Error::UnexpectedType("bool".to_string(), self.clone())),
        }
    }

    pub fn not(&mut self) -> SwResult<Value> {
        match *self {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            _ => Err(Error::UnexpectedType("bool".to_string(), self.clone())),
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
            _ => Err(Error::InvalidBinaryExpression(self.clone(), other.clone(), Operator::Add)),
        }
    }

    pub fn subtract(&self, other: &Value) -> SwResult<Value> {
        match (self, other) {
            (&Value::Float(f1), &Value::Float(f2)) => Ok(Value::Float(f1 - f2)),
            (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(i1 - i2)),
            (&Value::Float(f), &Value::Int(i)) => Ok(Value::Float(f - i as f32)),
            (&Value::Int(i), &Value::Float(f)) => Ok(Value::Float(i as f32 - f)),
            _ => {
                Err(Error::InvalidBinaryExpression(self.clone(), other.clone(), Operator::Subtract))
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
                Err(Error::InvalidBinaryExpression(self.clone(), other.clone(), Operator::Multiply))
            }
        }
    }

    pub fn divide(&self, other: &Value) -> SwResult<Value> {
        match (self, other) {
            (&Value::Float(f1), &Value::Float(f2)) => Ok(Value::Float(f1 / f2)),
            (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(i1 / i2)),
            (&Value::Float(f), &Value::Int(i)) => Ok(Value::Float(f / i as f32)),
            (&Value::Int(i), &Value::Float(f)) => Ok(Value::Float(i as f32 / f)),
            _ => Err(Error::InvalidBinaryExpression(self.clone(), other.clone(), Operator::Divide)),
        }
    }

    pub fn shift_left(&self, other: &Value) -> SwResult<Value> {
        match (self, other) {
            (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(i1 << i2)),
            _ => {
                Err(Error::InvalidBinaryExpression(self.clone(),
                                                   other.clone(),
                                                   Operator::ShiftLeft))
            }
        }
    }

    pub fn shift_right(&self, other: &Value) -> SwResult<Value> {
        match (self, other) {
            (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(i1 >> i2)),
            _ => {
                Err(Error::InvalidBinaryExpression(self.clone(),
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
    s.run(&tokens.unwrap());
}
