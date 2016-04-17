extern crate rand;
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::prelude::*;

pub mod schwift_grammar;

#[derive(Debug,Clone)]
pub enum Value {
	Str(String),
	Int(i32),
	Float(f32),
    Bool(bool),
	List(Vec<Value>),
}

#[derive(Clone)]
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

#[derive(Debug,Clone)]
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
}

#[derive(Debug,Clone)]
pub enum Expression {
    Variable(String),
    OperatorExpression(Box<Expression>, Operator, Box<Expression>),
    Value(Value),
    ListIndex(String, Box<Expression>),
    ListLength(String),
    Not(Box<Expression>),
}

#[derive(Debug)]
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

pub fn logic_error(s: &str) {
    let mut rng = thread_rng();
    let choice: &str = rng.choose(&QUOTES).unwrap();
    panic!("\n\n\tYou made a Rickdiculous mistake\n\tError:{}\n\t{}\n\n",s, choice);
}

impl <T>Op<T> {
    fn unwrap(self) -> T {
        match self {
            Op::Ok(x) => x,
            Op::TypeError(x, y) => { logic_error(&format!("Cannot combine {:?} and {:?}", x, y)); unreachable!(); }
        }
    }
}

impl State {
    fn expression_to_variable(&self, exp: Expression) -> Variable {
        match exp {
            Expression::Variable(ref s) => {
                if self.symbols.contains_key(s) {
                    let y = &(self.symbols);
                    let x = y.get(s).unwrap();
                    x.clone()
                } else {
                    logic_error(&format!("Tried to use variable {} before assignment", s));
                    unreachable!();
                }
            }
            Expression::OperatorExpression(a, o, b) => {
                let x = self.expression_to_variable(*a);
                let y = self.expression_to_variable(*b);
                Variable::new_variable(match o {
                    Operator::Add => x.add(y.value).unwrap(),
                    Operator::Subtract => x.subtract(y.value).unwrap(),
                    Operator::Multiply => x.multiply(y.value).unwrap(),
                    Operator::Divide => x.divide(y.value).unwrap(),
                    Operator::Equality => x.equals(y.value).unwrap(),
                    Operator::LessThan => x.less_than(y.value).unwrap(),
                    Operator::GreaterThan => x.greater_than(y.value).unwrap(),
                    Operator::LessThanEqual => x.less_than_equal(y.value).unwrap(),
                    Operator::GreaterThanEqual => x.greater_than_equal(y.value).unwrap(),
                    Operator::ShiftLeft => x.shift_left(y.value).unwrap(),
                    Operator::ShiftRight => x.shift_right(y.value).unwrap(),
                })
            }
            Expression::Value(v) => Variable::new_variable(v),
            Expression::ListIndex(ref s, ref e) => {
                if self.symbols.contains_key(s) {
                    let x = self.expression_to_variable(*e.clone()).value;
                    let val = &(self.symbols).get(s).unwrap().value;
                    if let &Value::List(ref l) = val {
                        if let Value::Int(i) = x {
                            let index = i as usize;
                            if index < l.len() {
                                Variable::new_variable(l[index].clone())
                            } else {
                                logic_error("You don't have that many kernels on your cob, idiot.");
                                unreachable!();
                            }
                        } else {
                            logic_error("You can only index with an int");
                            unreachable!();
                        }
                    } else {
                        if let &Value::Str(ref s) = val {
                            if let Value::Int(i) = x {
                                let index = i as usize;
                                if index < s.len() {
                                    Variable::new_variable(Value::Str(s.as_str()[index..(index)].to_string()))
                                } else {
                                    logic_error("You don't have that many kernels on your cob, idiot.");
                                    unreachable!();
                                }
                            } else {
                                logic_error("You can only index with an int");
                                unreachable!();
                            }
                        } else {
                            logic_error("Type error, you are trying index something other than a cob.");
                            unreachable!();
                        }
                    }
                } else {
                    logic_error("OOOweeee you squanched it, that cob doesn't exist.");
                    unreachable!();
                }
            },
            Expression::Not(e) => {
                let mut x = self.expression_to_variable(*e);
                x.not();
                x
            },
            Expression::ListLength(ref s) => {
                if self.symbols.contains_key(s) {
                    let val = &(self.symbols).get(s).unwrap().value;
                    if let &Value::List(ref l) = val {
                        Variable::new_variable(Value::Int(l.len() as i32))
                    } else {
                        if let &Value::Str(ref s) = val {
                            Variable::new_variable(Value::Int(s.len() as i32))
                        } else {
                            logic_error("Type error, you are trying index something other than a cob.");
                            unreachable!();
                        }
                    }
                } else {
                    logic_error("OOOweeee you squanched it, that cob doesn't exist.");
                    unreachable!();
                }
            }
        }
    }

    fn assign(&mut self, str: String, exp: Expression) {
        let v = self.expression_to_variable(exp);
        self.symbols.insert(str, v);
    }

    fn delete(&mut self, str: String) {
        self.symbols.remove(&str);
    }

    fn print(&mut self, exp: Expression) {
        let x = self.expression_to_variable(exp);
        x.println();
    }

    pub fn run(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            match statement {
                Statement::ListNew(s) => {
                    self.symbols.insert(s, Variable::new_variable(Value::List(Vec::new())));
                },
                Statement::ListAppend(ref s, ref e) => {
                    if self.symbols.contains_key(s) {
                        let val = self.expression_to_variable(e.clone()).value;
                        if let Value::List(ref mut l) = self.symbols.get_mut(s).unwrap().value {
                            l.push(val);
                        } else {
                            logic_error("Type error, you are trying index something other than a cob.")
                        }
                    } else {
                        logic_error("OOOweeee you squanched it, that cob doesn't exist.")
                    }

                },
                Statement::ListAssign(ref s, ref index_expression, ref assign_expression) => {
                    if self.symbols.contains_key(s) {
                        let val = self.expression_to_variable(assign_expression.clone()).value;
                        let x = self.expression_to_variable(index_expression.clone()).value;
                        if let Value::List(ref mut l) = self.symbols.get_mut(s).unwrap().value {
                            if let Value::Int(i) = x {
                                let index = i as usize;
                                if index < l.len() {
                                    l[index] = val;
                                } else {
                                    logic_error("You don't have that many kernels on your cob.")
                                }
                            } else {
                                logic_error("You can only index with an int");
                            }
                        } else {
                            logic_error("Type error, you are trying index something other than a cob.")
                        }
                    } else {
                        logic_error("OOOweeee you squanched it, that cob doesn't exist.")
                    }

                },
                Statement::ListDelete(ref s, ref index_expression) => {
                    if self.symbols.contains_key(s) {
                        let x = self.expression_to_variable(index_expression.clone()).value;
                        if let Value::List(ref mut l) = self.symbols.get_mut(s).unwrap().value {
                            if let Value::Int(i) = x {
                                let index = i as usize;
                                if index < l.len() {
                                    l.remove(index);
                                } else {
                                    logic_error("You don't have that many kernels on your cob, idiot.")
                                }
                            } else {
                                logic_error("You can only index with an int");
                            }
                        } else {
                            logic_error("Type error, you are trying index something other than a cob.")
                        }
                    } else {
                        logic_error("OOOweeee you squanched it, that cob doesn't exist.")
                    }

                },
                Statement::If(bool_expression, if_body, else_body) => {
                    let x = self.expression_to_variable(bool_expression).value;
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
                        _=> logic_error("Ah geez, you you used a non-bool for a bool")

                    }
                },
                Statement::Assignment(i, j) => self.assign(i, j),
                Statement::Delete(i) => self.delete(i),
                Statement::Print(i) => self.print(i),
            }
        }
    }

    pub fn new() -> State {
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
            _ => {
                logic_error("Can only negate boolean values");
                unreachable!();
            }
        }
    }

    pub fn println(&self) {
        self.print();
        println!("");
    }

    pub fn assign(&mut self, value: Value) {
        if self.constant {
            logic_error("Tried to assign to a constant value");
        }
        self.value = value;
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
            _ => {
                logic_error("Tried to add incompatable types");
                unreachable!();
            },
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
            _ => {
                logic_error("Tried to subtract incompatable types");
                unreachable!();
            },
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
                        new_buf.push_str(&i);
                    }
                    Op::Ok(Value::Str(new_buf))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            }
            _ => {
                logic_error("Tried to multiply incompatable types");
                unreachable!();
            },
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
            _ => {
                logic_error("Tried to divide incompatable types");
                unreachable!();
            },
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
            _ => {
                logic_error("Only ints can be bit-shifted.");
                unreachable!();
            },
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
            _ => {
                logic_error("Only ints can be bit-shifted.");
                unreachable!();
            },
        }
    }

    pub fn equals(&self, value: Value) -> Op<Value> {
        Op::Ok(Value::Bool(self.value.equals(value)))
    }

    pub fn greater_than(&self, value: Value) -> Op<Value> {
        Op::Ok(Value::Bool(self.value.greater_than(value)))
    }

    pub fn greater_than_equal(&self, value: Value) -> Op<Value> {
        Op::Ok(Value::Bool(self.value.greater_than_equal(value)))
    }

    pub fn less_than(&self, value: Value) -> Op<Value> {
        Op::Ok(Value::Bool(self.value.less_than(value)))
    }

    pub fn less_than_equal(&self, value: Value) -> Op<Value> {
        Op::Ok(Value::Bool(self.value.less_than_equal(value)))
    }
}

fn equals(x: f32, y: f32) -> bool {
    x == y
}

fn less_than(x: f32, y: f32) -> bool {
    x < y
}

fn less_than_equal(x: f32, y: f32) -> bool {
    x <= y
}

fn greater_than(x: f32, y: f32) -> bool {
    x > y
}

fn greater_than_equal(x: f32, y: f32) -> bool {
    x >= y
}

impl Value {
    pub fn number_comparisson(&self, other: Value, f: &Fn(f32, f32) -> bool) -> bool {
        match *self {
            Value::Int(i) => {
                if let Value::Int(j) = other {
                    f(i as f32, j as f32)
                } else {
                    if let Value::Float(j) = other {
                        f(i as f32, j)
                    } else {
                        false
                    }
                }
            },
            Value::Float(i) => {
                if let Value::Float(j) = other {
                    f(i, j)
                } else {
                    if let Value::Int(j) = other {
                        f(i, j as f32)
                    } else {
                        false
                    }
                }
            },
            _ => {
                logic_error("Tried to compare incompatable types");
                unreachable!();
            },
        }
    }

    pub fn equals(&self, other: Value) -> bool {
        self.number_comparisson(other, &equals)
    }

    pub fn less_than(&self, other: Value) -> bool {
        self.number_comparisson(other, &less_than)
    }

    pub fn greater_than(&self, other: Value) -> bool {
        self.number_comparisson(other, &greater_than)
    }

    pub fn greater_than_equal(&self, other: Value) -> bool {
        self.number_comparisson(other, &greater_than_equal)
    }

    pub fn less_than_equal(&self, other: Value) -> bool {
        self.number_comparisson(other, &less_than_equal)
    }
}

pub fn parse_file(filename: &str) ->  Result<Vec<Statement>, schwift_grammar::ParseError> {
    let mut f = match File::open(filename){
        Result::Ok(i) => i,
        Result::Err(_) => { logic_error("failed to open file"); unreachable!() },
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Result::Ok(_) => {},
        Result::Err(_) => { logic_error("failed to read file"); unreachable!() },
    };
    schwift_grammar::file(&s)
}

pub fn run_program(filename: &str) {
    let mut s = State::new();
    let tokens = parse_file(filename);
    s.run(tokens.unwrap());
}
