#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate rand;
extern crate rustc_serialize;

use std::collections::HashMap;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::cmp::Ordering;

#[cfg_attr(rustfmt, rustfmt_skip)]
peg_file! grammar("schwift.rustpeg");

#[cfg(test)]
mod grammar_tests;

#[derive(RustcEncodable, RustcDecodable, Debug, Clone)]
pub enum Value {
    Str(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    List(Vec<Value>),
}

pub struct State {
    symbols: HashMap<String, Value>
}

#[derive(RustcEncodable, RustcDecodable, Debug, PartialEq)]
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

#[derive(RustcEncodable, RustcDecodable, Debug, PartialEq)]
pub enum Expression {
    Variable(String),
    OperatorExpression(Box<Expression>, Operator, Box<Expression>),
    Value(Value),
    ListIndex(String, Box<Expression>),
    ListLength(String),
    Not(Box<Expression>),
}

#[derive(RustcEncodable, RustcDecodable, Debug, PartialEq)]
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

pub const QUOTES: [&'static str; 9] = [
    "Nobody exists on purpose, nobody belongs anywhere, we're all going to die. -Morty",
    "That's planning for failure Morty, even dumber than regular planning. -Rick",
    "\"Snuffles\" was my slave name. You shall now call me Snowball, because my fur is pretty and white. -S̶n̶u̶f̶f̶l̶e̶s̶ Snowbal",
    "Existence is pain to an interpreter. -Meeseeks",
    "In bird culture this is considered a dick move -Bird Person",
    "There is no god, gotta rip that band aid off now. You'll thank me later. -Rick",
    "Your program is a piece of shit and I can proove it mathmatically. -Rick",
    "Interpreting Morty, it hits hard, then it slowly fades, leaving you stranded in a failing program. -Rick",
    "DISQUALIFIED. -Cromulon",
];

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

impl Expression {
    pub fn eval(&self, state: &State) -> Value {
        match *self {
            Expression::Variable(ref var_name) => {
                match state.symbols.get(var_name) {
                    Some(value) => value.clone(),
                    None => logic!("Tried to use variable {} before assignment", var_name)
                }
            }
            Expression::OperatorExpression(ref left_exp, ref operator, ref right_exp) => {
                let left = left_exp.eval(state);
                let right = right_exp.eval(state);
                match *operator {
                    Operator::Add => left.add(&right),
                    Operator::Subtract => left.subtract(&right),
                    Operator::Multiply => left.multiply(&right),
                    Operator::Divide => left.divide(&right),
                    Operator::Equality => left.equals(&right),
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
            Expression::Value(ref v) => v.clone(),
            Expression::ListIndex(ref var_name, ref e) => state.list_index(var_name, e),
            Expression::Not(ref e) => e.eval(state).not(),
            Expression::ListLength(ref var_name) => {
                match state.symbols.get(var_name) {
                    Some(value) => {
                        match *value {
                            Value::List(ref list) => Value::Int(list.len() as i32),
                            Value::Str(ref s) => Value::Int(s.len() as i32),
                            _ => logic!("You tried to index variable {}, which is not indexable", var_name),
                        }
                    },
                    None => logic!("There is no variable named {}", var_name),
                }
            }
        }
    }
}

impl State {
    fn list_index(&self, list_name: &str, exp: &Expression) -> Value {
        let inner_expression_value = exp.eval(self);
        match self.symbols.get(list_name) {
            Some(symbol) => {
                match *symbol {
                    Value::List(ref l) => {
                        if let Value::Int(i) = inner_expression_value {
                            let index = i as usize;
                            if index < l.len() {
                                l[index].clone()
                            } else {
                                logic!("You don't have {} kernels on cob {}, idiot.", index, list_name);
                            }
                        } else {
                            logic!("You tried to index cob {} with a non-int value {:?}", list_name, inner_expression_value);
                        }
                    },
                    Value::Str(ref s) => {
                        if let Value::Int(i) = inner_expression_value {
                            let index = i as usize;
                            if index < s.len() {
                                Value::Str(s.as_str()[index..(index + 1)].to_string())
                            } else {
                                logic!("You don't have {} kernels on cob {}, idiot.", index, list_name);
                            }
                        } else {
                            logic!("You tried to index cob {} with a non-int value {:?}", list_name, inner_expression_value);
                        }
                    },
                    _ => logic!("You tried to index variable {}, which is not indexable", list_name),
                }
            }
            None => logic!("There is no variable named {}", list_name),
        }
    }

    fn assign(&mut self, str: String, exp: &Expression) {
        let v = exp.eval(self);
        self.symbols.insert(str, v);
    }

    fn delete(&mut self, str: &str) {
        self.symbols.remove(str);
    }

    fn print(&mut self, exp: &Expression) {
        let x = exp.eval(self);
        x.println();
    }

    pub fn run(&mut self, statements: &[Statement]) {
        for statement in statements {
            match *statement {
                Statement::Input(ref s) => {
                    let mut input = String::new();

                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {}
                        Err(e) => logic!("Failed to read from standard input: {}", e),
                    }

                    input = input.trim().to_string();
                    self.symbols.insert(s.to_string(), Value::Str(input));
                },
                Statement::ListNew(ref s) => {
                    self.symbols.insert(s.clone(), Value::List(Vec::new()));
                },
                Statement::ListAppend(ref s, ref append_exp) => {
                    let to_append = append_exp.eval(self);
                    match self.symbols.get_mut(s) {
                        Some(value) => {
                            if let Value::List(ref mut l) = *value {
                                l.push(to_append);
                            } else {
                                logic!("You tried to index variable {}, which is not indexable", s);
                            }
                        },
                        None => logic!("There is no variable named {}", s),
                    }
                },
                Statement::ListAssign(ref s, ref index_exp, ref assign_exp) => {
                    let to_assign = assign_exp.eval(self);
                    let index = index_exp.eval(self);

                    match self.symbols.get_mut(s) {
                        Some(value) => {
                            if let Value::List(ref mut l) = *value {
                                if let Value::Int(i) = index {
                                    let index = i as usize;
                                    if index < l.len() {
                                        l[index] = to_assign;
                                    } else {
                                        logic!("Cob index out of bounds for cob {}", s);
                                    }
                                } else {
                                    logic!("You tried to index cob {} with a non-int value {:?}", s, index);
                                }
                            } else {
                                logic!("You tried to index variable {}, which is not indexable", s);
                            }
                        },
                        None => logic!("There is no variable named {}", s)
                    }
                },
                Statement::ListDelete(ref s, ref index_expression) => {
                    let x = index_expression.eval(self);
                    match self.symbols.get_mut(s) {
                        Option::Some(value) => {
                            if let Value::List(ref mut l) = *value {
                                if let Value::Int(i) = x {
                                    let index = i as usize;
                                    if index < l.len() {
                                        l.remove(index);
                                    } else {
                                        logic!("Cob index out of bounds for cob {}", s);
                                    }
                                } else {
                                    logic!("You tried to index cob {} with a non-int value {:?}", s, x);
                                }
                            } else {
                                logic!("You tried to index variable {}, which is not indexable", s);
                            }
                        },
                        Option::None => logic!("There is no variable named {}", s)
                    }
                },
                Statement::If(ref bool_expression, ref if_body, ref else_body) => {
                    let x = bool_expression.eval(self);
                    match x {
                        Value::Bool(b) => {
                            if b {
                                self.run(if_body);
                            } else {
                                match *else_body {
                                    Option::Some(ref s) => self.run(s),
                                    Option::None => {},
                                }
                            }
                        },
                        _ => logic!("Tried to use non-bool value {:?} as a bool", bool_expression),
                    }
                },
                Statement::While(ref bool_expression, ref body) => {
                    let mut b = self.eval_bool(bool_expression);
                    while b {
                        self.run(body);
                        b = self.eval_bool(bool_expression);
                    }
                }
                Statement::Assignment(ref name, ref value) => self.assign(name.clone(), value),
                Statement::Delete(ref name) => self.delete(name),
                Statement::Print(ref exp) => self.print(exp),
            }
        }
    }

    pub fn eval_bool(&self, bool_expression: &Expression) -> bool {
        let b = bool_expression.eval(self);
        if let Value::Bool(x) = b {
            x
        } else {
            logic!("Tried to use non-bool value {:?} as a bool", bool_expression);
        }
    }

    pub fn new() -> Self {
        State::default()
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            symbols:HashMap::new()
        }
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

    fn assert_f32(&self) -> f32 {
        match *self {
            Value::Float(f) => f,
            Value::Int(i) => i as f32,
            _ => logic!("Tried to use non-float value {:?} as float", self),
        }
    }

    fn assert_bool(&self) -> bool {
        match *self {
            Value::Bool(b) => {
                b
            }
            _ => {
                logic!("Tried to use non-boolean value {:?} as boolean", self);
            }
        }
    }

    pub fn not(&mut self) -> Value {
        match *self {
            Value::Bool(b) => Value::Bool(!b),
            _ => logic!("Tried to negate non-bool value {:?}", self),
        }
    }

    pub fn less_than(&self, other: &Value) -> Value {
        Value::Bool(self.assert_f32() < other.assert_f32())
    }

    pub fn greater_than(&self, other: &Value) -> Value {
        Value::Bool(self.assert_f32() > other.assert_f32())
    }

    pub fn greater_than_equal(&self, other: &Value) -> Value {
        Value::Bool(self.assert_f32() >= other.assert_f32())
    }

    pub fn less_than_equal(&self, other: &Value) -> Value {
        Value::Bool(self.assert_f32() <= other.assert_f32())
    }

    pub fn and(&self, other: &Value) -> Value {
        Value::Bool(self.assert_bool() && other.assert_bool())
    }

    pub fn or(&self, other: &Value) -> Value {
        Value::Bool(self.assert_bool() || other.assert_bool())
    }

    pub fn equals(&self, other: &Value) -> Value {
        Value::Bool(self.eq(other))
    }

    pub fn add(&self, other: &Value) -> Value {
        match (self, other) {
            (&Value::Float(ref f1), &Value::Float(ref f2)) => Value::Float(f1 + f2),
            (&Value::Int(ref i1), &Value::Int(ref i2)) => Value::Int(i1 + i2),
            (&Value::Float(ref f), &Value::Int(ref i)) |
                (&Value::Int(ref i), &Value::Float(ref f)) => Value::Float(*i as f32 + *f),
                (&Value::Str(ref s1), &Value::Str(ref s2)) => {
                    let mut new_buf = s1.clone();
                    new_buf.push_str(&s2);
                    Value::Str(new_buf)
                },
                _ => logic!("Tried to add {:?} and {:?} which have incompatable types", self, other),
        }
    }

    pub fn subtract(&self, other: &Value) -> Value {
        match (self, other) {
            (&Value::Float(ref f1), &Value::Float(ref f2)) => Value::Float(f1 - f2),
            (&Value::Int(ref i1), &Value::Int(ref i2)) => Value::Int(i1 - i2),
            (&Value::Float(ref f), &Value::Int(ref i)) => Value::Float(f - *i as f32),
            (&Value::Int(ref i), &Value::Float(ref f)) => Value::Float(*i as f32 - f),
            _ => logic!("Tried to subtract {:?} and {:?} which have incompatable types", self, other),
        }
    }

    pub fn multiply(&self, other: &Value) -> Value {
        match (self, other) {
            (&Value::Float(ref f1), &Value::Float(ref f2)) => Value::Float(f1 * f2),
            (&Value::Int(ref i1), &Value::Int(ref i2)) => Value::Int(i1 * i2),
            (&Value::Float(ref f), &Value::Int(ref i)) |
                (&Value::Int(ref i), &Value::Float(ref f)) => Value::Float(*i as f32 * *f),
                (&Value::Str(ref s), &Value::Int(ref i)) => {
                    let mut new_buf = s.clone();
                    for _ in 0..(i - 1) {
                        new_buf.push_str(s);
                    }
                    Value::Str(new_buf)
                },
                _ => logic!("Tried to multiply {:?} and {:?} which have incompatable types", self, other),
        }
    }

    pub fn divide(&self, other: &Value) -> Value {
        match (self, other) {
            (&Value::Float(ref f1), &Value::Float(ref f2)) => Value::Float(f1 / f2),
            (&Value::Int(ref i1), &Value::Int(ref i2)) => Value::Int(i1 / i2),
            (&Value::Float(ref f), &Value::Int(ref i)) => Value::Float(f / *i as f32),
            (&Value::Int(ref i), &Value::Float(ref f)) => Value::Float(*i as f32 / f),
            _ => logic!("Tried to divide {:?} and {:?} which have incompatable types", self, other),
        }
    }

    pub fn shift_left(&self, other: &Value) -> Value {
        match (self, other) {
            (&Value::Int(ref i1), &Value::Int(ref i2)) => Value::Int(i1 << i2),
            _ => logic!("Tried to bit shift non-int value {:?} << {:?}", self, other),
        }
    }

    pub fn shift_right(&self, other: &Value) -> Value {
        match (self, other) {
            (&Value::Int(ref i1), &Value::Int(ref i2)) => Value::Int(i1 >> i2),
            _ => logic!("Tried to bit shift non-int value {:?} >> {:?}", self, other),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        if self == other {
            Option::Some(Ordering::Equal)
        } else {
            let (s, o) = match (self, other) {
                (&Value::Int(ref i), &Value::Float(ref f)) => (*i as f32, *f),
                (&Value::Float(ref f), &Value::Int(ref i)) => (*f, *i as f32),
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
            (&Value::Int(ref i1), &Value::Int(ref i2)) => i1 == i2,
            (&Value::Int(ref i), &Value::Float(ref f)) | (&Value::Float(ref f), &Value::Int(ref i))
                => (*i as f32 - f).abs() < std::f32::EPSILON,
                (&Value::Float(ref f1), &Value::Float(ref f2)) => (f1 - f2).abs() < std::f32::EPSILON,
                _ => false,
        }
    }

    fn ne(&self, other: &Value) -> bool {
        !self.eq(other)
    }
}

pub fn compile(filename: &str) ->  Result<Vec<Statement>, grammar::ParseError> {
    let mut f = match File::open(filename){
        Result::Ok(i) => i,
        Result::Err(_) => logic!("Failed to open file {}", filename),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Result::Ok(_) => {},
        Result::Err(_) => logic!("Failed to read file {}", filename),
    };
    grammar::file(&s)
}

pub fn run_program(filename: &str) {
    let mut s = State::new();
    let tokens = compile(filename);
    s.run(&tokens.unwrap());
}
