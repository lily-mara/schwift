use super::Operator;
use std::cmp::Ordering;
use super::error::{ErrorKind, SwResult};
use super::statement::Statement;
use std;
use std::{fmt, clone};

#[cfg(unix)]
use super::lib::os::unix::Symbol;

#[cfg(windows)]
use super::lib::os::windows::Symbol;

pub type _Func = fn(&[Value]) -> SwResult<Value>;
pub type _FuncSymbol = Symbol<_Func>;

pub struct Func {
    f: _FuncSymbol,
}

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    List(Vec<Value>),
    Function(Vec<String>, Vec<Statement>),
    NativeFunction(Func),
}

impl fmt::Debug for Func {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Native function]")
    }
}

impl clone::Clone for Func {
    fn clone(&self) -> Self {
        panic!("You cannot clone a native function.")
    }
}

impl Func {
    pub fn new(f: _FuncSymbol) -> Self {
        Func { f: f }
    }

    pub fn call(&self, args: &[Value]) -> SwResult<Value> {
        let f = &self.f;
        f(args)
    }
}

impl From<_FuncSymbol> for Value {
    fn from(from: _FuncSymbol) -> Value {
        Value::NativeFunction(Func { f: from })
    }
}

impl From<_FuncSymbol> for Func {
    fn from(from: _FuncSymbol) -> Func {
        Func { f: from }
    }
}

impl From<Func> for Value {
    fn from(from: Func) -> Value {
        Value::NativeFunction(from)
    }
}

impl From<i32> for Value {
    fn from(from: i32) -> Value {
        Value::Int(from)
    }
}

impl From<f32> for Value {
    fn from(from: f32) -> Value {
        Value::Float(from)
    }
}

impl From<bool> for Value {
    fn from(from: bool) -> Value {
        Value::Bool(from)
    }
}

impl From<String> for Value {
    fn from(from: String) -> Value {
        Value::Str(from)
    }
}

impl From<&'static str> for Value {
    fn from(from: &'static str) -> Value {
        Value::Str(from.into())
    }
}

impl<T> From<Vec<T>> for Value
    where T: Into<Value>
{
    fn from(from: Vec<T>) -> Value {
        let mut buf = vec![];
        for i in from {
            buf.push(i.into());
        }

        Value::List(buf)
    }
}

impl Value {
    pub fn new<T>(val: T) -> Self
        where T: Into<Value>
    {
        val.into()
    }

    pub fn len(&self) -> SwResult<usize> {
        use self::Value::*;
        match *self {
            Str(ref s) => Ok(s.chars().count()),
            List(ref l) => Ok(l.len()),
            _ => Err(ErrorKind::UnexpectedType("string or list".to_string(), self.clone())),
        }
    }

    pub fn is_empty(&self) -> SwResult<bool> {
        use self::Value::*;
        match *self {
            Str(ref s) => Ok(s.is_empty()),
            List(ref l) => Ok(l.is_empty()),
            _ => Err(ErrorKind::UnexpectedType("string or list".to_string(), self.clone())),
        }
    }

    pub fn print(&self) {
        match *self {
            Value::Int(i) => print!("{}", i),
            Value::Float(i) => print!("{}", i),
            Value::Bool(i) => print!("{}", i),
            Value::Str(ref i) => print!("{}", i),
            Value::List(ref i) => print!("{:?}", i),
            Value::Function(_, ref body) => print!("{:?}", body),
            Value::NativeFunction(_) => print!("[Native function]"),
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
            Value::Function(_, _) => "function",
            Value::NativeFunction(_) => "native function",
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
}
