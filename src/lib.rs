extern crate rand;
extern crate rustc_serialize;

use std::collections::HashMap;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::io;

pub mod grammar;

#[derive(RustcEncodable, RustcDecodable, Debug,Clone)]
pub enum Value {
	Str(String),
	Int(i32),
	Float(f32),
    Bool(bool),
	List(Vec<Value>),
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Variable {
    value: Value,
    constant: bool,
}

pub struct State {
    symbols: HashMap<String, Variable>
}

#[derive(Debug)]
pub enum Op<T> {
    Ok(T),
    TypeError(Value, Value),
}

#[derive(RustcEncodable, RustcDecodable, Debug,Clone,PartialEq)]
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

#[derive(RustcEncodable, RustcDecodable, Debug,Clone,PartialEq)]
pub enum Expression {
    Variable(String),
    OperatorExpression(Box<Expression>, Operator, Box<Expression>),
    Value(Value),
    ListIndex(String, Box<Expression>),
    ListLength(String),
    Not(Box<Expression>),
}

#[derive(RustcEncodable, RustcDecodable, Debug, Clone,PartialEq)]
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

impl <T>Op<T> {
    fn unwrap(self) -> T {
        match self {
            Op::Ok(x) => x,
            Op::TypeError(x, y) => { logic!("Cannot combine {:?} and {:?}", x, y); }
        }
    }
}

impl State {
    fn list_index(&self, list_name: &str, exp: &Expression) -> Variable {
        let inner_expression_value = self.expression_to_variable(exp).value;
        match self.symbols.get(list_name) {
            Some(symbol) => {
                match symbol.value {
                    Value::List(ref l) => {
                        if let Value::Int(i) = inner_expression_value {
                            let index = i as usize;
                            if index < l.len() {
                                Variable::new_variable(l[index].clone())
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
                                Variable::new_variable(Value::Str(s.as_str()[index..(index + 1)].to_string()))
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

    fn expression_to_variable(&self, exp: &Expression) -> Variable {
        match *exp {
            Expression::Variable(ref s) => {
                match self.symbols.get(s) {
                    Some(variable) => variable.clone(),
                    None => logic!("Tried to use variable {} before assignment", s)
                }
            }
            Expression::OperatorExpression(ref a, ref o, ref b) => {
                let x = self.expression_to_variable(a);
                let y = self.expression_to_variable(b);
                Variable::new_variable(match *o {
                    Operator::Add => x.add(y.value).unwrap(),
                    Operator::Subtract => x.subtract(y.value).unwrap(),
                    Operator::Multiply => x.multiply(y.value).unwrap(),
                    Operator::Divide => x.divide(y.value).unwrap(),
                    Operator::Equality => x.equals(&y.value),
                    Operator::LessThan => x.less_than(&y.value),
                    Operator::GreaterThan => x.greater_than(&y.value),
                    Operator::LessThanEqual => x.less_than_equal(&y.value),
                    Operator::GreaterThanEqual => x.greater_than_equal(&y.value),
                    Operator::ShiftLeft => x.shift_left(y.value).unwrap(),
                    Operator::ShiftRight => x.shift_right(y.value).unwrap(),
                    Operator::And => x.and(&y.value),
                    Operator::Or => x.or(&y.value),
                })
            }
            Expression::Value(ref v) => Variable::new_variable(v.clone()),
            Expression::ListIndex(ref s, ref e) => self.list_index(s, e),
            Expression::Not(ref e) => {
                let mut x = self.expression_to_variable(e);
                x.not();
                x
            },
            Expression::ListLength(ref s) => {
                match self.symbols.get(s) {
                    Some(symbol) => {
                        match symbol.value {
                            Value::List(ref l) => Variable::new_variable(Value::Int(l.len() as i32)),
                            Value::Str(ref s) => Variable::new_variable(Value::Int(s.len() as i32)),
                            _ => logic!("You tried to index variable {}, which is not indexable", s),
                        }
                    },
                    None => logic!("There is no variable named {}", s),
                }
            }
        }
    }

    fn assign(&mut self, str: String, exp: &Expression) {
        let v = self.expression_to_variable(exp);
        self.symbols.insert(str, v);
    }

    fn delete(&mut self, str: &str) {
        self.symbols.remove(str);
    }

    fn print(&mut self, exp: &Expression) {
        let x = self.expression_to_variable(exp);
        x.println();
    }

    pub fn run(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            match statement {
                Statement::Input(ref s) => {
                    let mut input = String::new();

                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {}
                        Err(e) => logic!("Failed to read from standard input: {}", e),
                    }

                    input = input.trim().to_string();
                    self.symbols.insert(s.to_string(), Variable::new_variable(Value::Str(input)));
                },
                Statement::ListNew(s) => {
                    self.symbols.insert(s, Variable::new_variable(Value::List(Vec::new())));
                },
                Statement::ListAppend(ref s, ref e) => {
                    if self.symbols.contains_key(s) {
                        let val = self.expression_to_variable(e).value;
                        if let Value::List(ref mut l) = self.symbols.get_mut(s).unwrap().value {
                            l.push(val);
                        } else {
                            logic!("You tried to index variable {}, which is not indexable", s);
                        }
                    } else {
                        logic!("There is no variable named {}", s);
                    }

                },
                Statement::ListAssign(ref s, ref index_expression, ref assign_expression) => {
                    if self.symbols.contains_key(s) {
                        let val = self.expression_to_variable(assign_expression).value;
                        let x = self.expression_to_variable(index_expression).value;
                        if let Value::List(ref mut l) = self.symbols.get_mut(s).unwrap().value {
                            if let Value::Int(i) = x {
                                let index = i as usize;
                                if index < l.len() {
                                    l[index] = val;
                                } else {
                                    logic!("Cob index out of bounds for cob {}", s);
                                }
                            } else {
                                logic!("You tried to index cob {} with a non-int value {:?}", s, x);
                            }
                        } else {
                            logic!("You tried to index variable {}, which is not indexable", s);
                        }
                    } else {
                        logic!("There is no variable named {}", s);
                    }

                },
                Statement::ListDelete(ref s, ref index_expression) => {
                    if self.symbols.contains_key(s) {
                        let x = self.expression_to_variable(index_expression).value;
                        if let Value::List(ref mut l) = self.symbols.get_mut(s).unwrap().value {
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
                    } else {
                        logic!("There is no variable named {}", s);
                    }

                },
                Statement::If(bool_expression, if_body, else_body) => {
                    let x = self.expression_to_variable(&bool_expression).value;
                    match x {
                        Value::Bool(b) => {
                            if b {
                                self.run(if_body);
                            } else {
                                match else_body {
                                    Option::Some(s) => self.run(s),
                                    Option::None => {},
                                }
                            }
                        }
                        _ => logic!("Tried to use non-bool value {:?} as a bool", bool_expression),
                    }
                },
                Statement::While(ref bool_expression, ref body) => {
                    let mut b = self.eval_bool(bool_expression);
                    while b {
                        self.run(body.clone());
                        b = self.eval_bool(bool_expression);
                    }
                }
                Statement::Assignment(i, j) => self.assign(i, &j),
                Statement::Delete(ref i) => self.delete(i),
                Statement::Print(ref i) => self.print(i),
            }
        }
    }

    pub fn eval_bool(&self, bool_expression: &Expression) -> bool {
        let b = self.expression_to_variable(bool_expression).value;
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

impl Variable {
    pub fn new_variable(value: Value) -> Variable {
        Variable{ value: value, constant: false }
    }

    pub fn new_constant(value: Value) -> Variable {
        Variable{ value: value, constant: true }
    }

    pub fn print(&self) {
        match self.value {
            Value::Int(i) => print!("{}", i),
            Value::Float(i) => print!("{}", i),
            Value::Bool(i) => print!("{}", i),
            Value::Str(ref i) => print!("{}", i),
            Value::List(ref i) => print!("{:?}", i),
        }
    }

    pub fn not(&mut self) {
        match self.value {
            Value::Bool(b) => self.value = Value::Bool(!b),
            _ => logic!("Tried to negate non-bool value {:?}", self.value),
        }
    }

    pub fn println(&self) {
        self.print();
        println!("");
    }

    pub fn assign(&mut self, value: &Value) {
        if self.constant {
            logic!("Tried to assign to constant value {:?}", self.value);
        }
        self.value = value.clone();
    }

    pub fn add(&self, value: Value) -> Op<Value> {
        match self.value {
            Value::Int(i) => {
                if let Value::Int(j) = value {
                    Op::Ok(Value::Int(i + j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            Value::Float(i) => {
                if let Value::Float(j) = value {
                    Op::Ok(Value::Float(i + j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            Value::Str(ref i) => {
                if let Value::Str(j) = value {
                    let mut new_buf = i.clone();
                    new_buf.push_str(&j);
                    Op::Ok(Value::Str(new_buf))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            _ => logic!("Tried to add {:?} and {:?} which have incompatable types", self.value, value),
        }
    }

    pub fn subtract(&self, value: Value) -> Op<Value> {
        match self.value {
            Value::Int(i) => {
                if let Value::Int(j) = value {
                    Op::Ok(Value::Int(i - j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            Value::Float(i) => {
                if let Value::Float(j) = value {
                    Op::Ok(Value::Float(i - j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            _ => logic!("Tried to subtract {:?} and {:?} which have incompatable types", self.value, value),
        }
    }

    pub fn multiply(&self, value: Value) -> Op<Value> {
        match self.value {
            Value::Int(i) => {
                if let Value::Int(j) = value {
                    Op::Ok(Value::Int(i * j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            Value::Float(i) => {
                if let Value::Float(j) = value {
                    Op::Ok(Value::Float(i * j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            Value::Str(ref i) => {
                if let Value::Int(j) = value {
                    let mut new_buf = i.clone();
                    for _ in 0..(j - 1) {
                        new_buf.push_str(i);
                    }
                    Op::Ok(Value::Str(new_buf))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            _ => logic!("Tried to multiply {:?} and {:?} which have incompatable types", self.value, value),
        }
    }

    pub fn divide(&self, value: Value) -> Op<Value> {
        match self.value {
            Value::Int(i) => {
                if let Value::Int(j) = value {
                    Op::Ok(Value::Int(i / j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            Value::Float(i) => {
                if let Value::Float(j) = value {
                    Op::Ok(Value::Float(i / j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            _ => logic!("Tried to divide {:?} and {:?} which have incompatable types", self.value, value),
        }
    }

    pub fn shift_left(&self, value: Value) -> Op<Value> {
        match self.value {
            Value::Int(i) => {
                if let Value::Int(j) = value {
                    Op::Ok(Value::Int(i << j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            _ => logic!("Tried to bit shift non-int value {:?} << {:?}", self.value, value),
        }
    }

    pub fn shift_right(&self, value: Value) -> Op<Value> {
        match self.value {
            Value::Int(i) => {
                if let Value::Int(j) = value {
                    Op::Ok(Value::Int(i >> j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            _ => logic!("Tried to bit shift non-int value {:?} >> {:?}", self.value, value),
        }
    }

    pub fn equals(&self, value: &Value) -> Value {
        match self.value {
            Value::Str(_) => self.str_eq(value),
            _ => Value::Bool(self.value.eq(value))
        }
    }

    pub fn greater_than(&self, value: &Value) -> Value {
        Value::Bool(self.value.greater_than(value))
    }

    pub fn greater_than_equal(&self, value: &Value) -> Value {
        Value::Bool(self.value.greater_than_equal(value))
    }

    pub fn less_than(&self, value: &Value) -> Value {
        Value::Bool(self.value.less_than(value))
    }

    pub fn less_than_equal(&self, value: &Value) -> Value {
        Value::Bool(self.value.less_than_equal(value))
    }

    pub fn and(&self, value: &Value) -> Value {
        Value::Bool(self.value.and(value))
    }

    pub fn or(&self, value: &Value) -> Value {
        Value::Bool(self.value.or(value))
    }

    pub fn str_eq(&self, value: &Value) -> Value {
        let val = &self.value;
        if let (&Value::Str(ref x), &Value::Str(ref y)) = (val, value) {
            return Value::Bool(x == y);
        }
        Value::Bool(false)
    }
}

impl Value {
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

    pub fn less_than(&self, other: &Value) -> bool {
        self.assert_f32() < other.assert_f32()
    }

    pub fn greater_than(&self, other: &Value) -> bool {
        self.assert_f32() > other.assert_f32()
    }

    pub fn greater_than_equal(&self, other: &Value) -> bool {
        self.assert_f32() >= other.assert_f32()
    }

    pub fn less_than_equal(&self, other: &Value) -> bool {
        self.assert_f32() <= other.assert_f32()
    }

    pub fn and(&self, other: &Value) -> bool {
        self.assert_bool() && other.assert_bool()
    }

    pub fn or(&self, other: &Value) -> bool {
        self.assert_bool() || other.assert_bool()
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
    s.run(tokens.unwrap());
}
