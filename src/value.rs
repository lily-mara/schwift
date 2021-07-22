use crate::{
    error::{ErrorKind, SwResult},
    statement::Statement,
    util, Operator,
};
use lazy_static::*;
use regex::Regex;
use std::{clone, cmp::Ordering, f64 as FloatValueType, fmt};

pub type FloatT = f64;
pub type IntT = i64;

#[cfg(unix)]
use libloading::os::unix::Symbol;

#[cfg(windows)]
use libloading::os::windows::Symbol;

pub type _Func = unsafe extern "C" fn(*mut Vec<Value>) -> *mut SwResult<Value>;
pub type _FuncSymbol = Symbol<_Func>;

pub struct Func {
    f: _FuncSymbol,
}

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Int(IntT),
    Float(FloatT),
    Bool(bool),
    List(Vec<Value>),
    Function(Vec<String>, Vec<Statement>),
    NativeFunction(Func),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Str,
    Int,
    Float,
    Bool,
    List,
    Function,
    NativeFunction,
    Union(Box<Self>, Box<Self>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Str => write!(f, "string"),
            Type::Int => write!(f, "int"),
            Type::Bool => write!(f, "bool"),
            Type::List => write!(f, "list"),
            Type::Float => write!(f, "float"),
            Type::Function => write!(f, "function"),
            Type::NativeFunction => write!(f, "native function"),
            Type::Union(t1, t2) => write!(f, "{} or {}", t1, t2),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;
        match *self {
            Str(ref x) => write!(f, "{}", x),
            Int(x) => write!(f, "{}", x),
            Float(x) => write!(f, "{}", x),
            Bool(x) if x => write!(f, "rick"),
            Bool(_) => write!(f, "morty"),
            List(ref x) => write!(f, "{}", util::slice_value_format(x)),
            Function(ref params, _) => write!(f, "[Function {}]", util::slice_format(params)),
            NativeFunction(_) => write!(f, "[Native Function]"),
        }
    }
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
        Self { f }
    }

    pub fn call(&self, args: &mut Vec<Value>) -> SwResult<Value> {
        let f = &self.f;
        let val = unsafe {
            let result = f(args as *mut Vec<Value>);
            if result.is_null() {
                return Err(ErrorKind::DylibReturnedNil.into());
            }

            Box::from_raw(result)
        };

        *val
    }
}

impl From<_FuncSymbol> for Value {
    fn from(from: _FuncSymbol) -> Self {
        Value::NativeFunction(Func { f: from })
    }
}

impl From<_FuncSymbol> for Func {
    fn from(from: _FuncSymbol) -> Self {
        Self { f: from }
    }
}

impl From<Func> for Value {
    fn from(from: Func) -> Self {
        Value::NativeFunction(from)
    }
}

impl From<IntT> for Value {
    fn from(from: IntT) -> Self {
        Value::Int(from)
    }
}

impl From<FloatT> for Value {
    fn from(from: FloatT) -> Self {
        Value::Float(from)
    }
}

impl From<bool> for Value {
    fn from(from: bool) -> Self {
        Value::Bool(from)
    }
}

impl From<String> for Value {
    fn from(from: String) -> Self {
        Value::Str(from)
    }
}

impl From<&'static str> for Value {
    fn from(from: &'static str) -> Self {
        Value::Str(from.into())
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Self>,
{
    fn from(from: Vec<T>) -> Self {
        let mut buf = vec![];
        for i in from {
            buf.push(i.into());
        }

        Value::List(buf)
    }
}

impl Value {
    pub fn new<T>(val: T) -> Self
    where
        T: Into<Self>,
    {
        val.into()
    }

    pub fn get_type(&self) -> Type {
        use Value::*;
        match self {
            Str(_) => Type::Str,
            Int(_) => Type::Int,
            Float(_) => Type::Float,
            Bool(_) => Type::Bool,
            List(_) => Type::List,
            Function(_, _) => Type::Function,
            NativeFunction(_) => Type::NativeFunction,
        }
    }

    pub fn len(&self) -> SwResult<usize> {
        use self::Value::*;
        match *self {
            Str(ref s) => Ok(s.chars().count()),
            List(ref l) => Ok(l.len()),
            _ => Err(ErrorKind::UnexpectedType {
                expected: Type::Union(Box::new(Type::Str), Box::new(Type::List)),
                actual: self.get_type(),
            }
            .into()),
        }
    }

    pub fn is_empty(&self) -> SwResult<bool> {
        use self::Value::*;
        match *self {
            Str(ref s) => Ok(s.is_empty()),
            List(ref l) => Ok(l.is_empty()),
            _ => Err(ErrorKind::UnexpectedType {
                expected: Type::Union(Box::new(Type::Str), Box::new(Type::List)),
                actual: self.get_type(),
            }
            .into()),
        }
    }

    #[cfg(feature = "debug_printing")]
    pub fn print(&self) {
        print!("{:?}", self);
    }

    #[cfg(not(feature = "debug_printing"))]
    pub fn print(&self) {
        print!("{}", self);
    }

    pub fn println(&self) {
        self.print();
        println!();
    }

    fn assert_f32(&self) -> SwResult<FloatT> {
        match *self {
            Value::Float(f) => Ok(f),
            Value::Int(i) => Ok(i as FloatT),
            _ => Err(ErrorKind::UnexpectedType {
                expected: Type::Float,
                actual: self.get_type(),
            }
            .into()),
        }
    }

    fn assert_bool(&self) -> SwResult<bool> {
        match *self {
            Value::Bool(b) => Ok(b),
            _ => Err(ErrorKind::UnexpectedType {
                expected: Type::Float,
                actual: self.get_type(),
            }
            .into()),
        }
    }

    pub fn not(&self) -> SwResult<Self> {
        match *self {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            _ => Err(ErrorKind::UnexpectedType {
                expected: Type::Bool,
                actual: self.get_type(),
            }
            .into()),
        }
    }

    pub fn less_than(&self, other: &Self) -> SwResult<Self> {
        if let (&Value::Int(i1), &Value::Int(i2)) = (self, other) {
            Ok(Value::Bool(i1 < i2))
        } else {
            Ok(Value::Bool(self.assert_f32()? < other.assert_f32()?))
        }
    }

    pub fn greater_than(&self, other: &Self) -> SwResult<Self> {
        if let (&Value::Int(i1), &Value::Int(i2)) = (self, other) {
            Ok(Value::Bool(i1 > i2))
        } else {
            Ok(Value::Bool(self.assert_f32()? > other.assert_f32()?))
        }
    }

    pub fn greater_than_equal(&self, other: &Self) -> SwResult<Self> {
        if let (&Value::Int(i1), &Value::Int(i2)) = (self, other) {
            Ok(Value::Bool(i1 >= i2))
        } else {
            Ok(Value::Bool(self.assert_f32()? >= other.assert_f32()?))
        }
    }

    pub fn less_than_equal(&self, other: &Self) -> SwResult<Self> {
        if let (&Value::Int(i1), &Value::Int(i2)) = (self, other) {
            Ok(Value::Bool(i1 <= i2))
        } else {
            Ok(Value::Bool(self.assert_f32()? <= other.assert_f32()?))
        }
    }

    pub fn and(&self, other: &Self) -> SwResult<Self> {
        Ok(Value::Bool(self.assert_bool()? && other.assert_bool()?))
    }

    pub fn or(&self, other: &Self) -> SwResult<Self> {
        Ok(Value::Bool(self.assert_bool()? || other.assert_bool()?))
    }

    pub fn equals(&self, other: &Self) -> Self {
        Value::Bool(self.eq(other))
    }

    pub fn modulus(&self, other: &Self) -> SwResult<Self> {
        if let Value::Int(i1) = *self {
            if let Value::Int(i2) = *other {
                Ok((i1 % i2).into())
            } else {
                Err(ErrorKind::UnexpectedType {
                    expected: Type::Int,
                    actual: other.get_type(),
                }
                .into())
            }
        } else {
            Err(ErrorKind::UnexpectedType {
                expected: Type::Int,
                actual: self.get_type(),
            }
            .into())
        }
    }

    pub fn add(&self, other: &Self) -> SwResult<Self> {
        match (self, other) {
            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Float(f1 + f2)),
            (Value::Int(i1), Value::Int(i2)) => Ok(Value::Int(i1 + i2)),
            (Value::Float(f), Value::Int(i)) | (Value::Int(i), Value::Float(f)) => {
                Ok(Value::Float(*i as FloatT + f))
            }
            (Value::Str(ref s1), Value::Str(ref s2)) => {
                let mut new_buf = s1.clone();
                new_buf.push_str(s2);
                Ok(Value::Str(new_buf))
            }
            _ => Err(ErrorKind::InvalidBinaryExpression(
                self.get_type(),
                other.get_type(),
                Operator::Add,
            )
            .into()),
        }
    }

    pub fn subtract(&self, other: &Self) -> SwResult<Self> {
        match (self, other) {
            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Float(f1 - f2)),
            (Value::Int(i1), Value::Int(i2)) => Ok(Value::Int(i1 - i2)),
            (Value::Float(f), Value::Int(i)) => Ok(Value::Float(f - *i as FloatT)),
            (Value::Int(i), Value::Float(f)) => Ok(Value::Float(*i as FloatT - f)),
            _ => Err(ErrorKind::InvalidBinaryExpression(
                self.get_type(),
                other.get_type(),
                Operator::Subtract,
            )
            .into()),
        }
    }

    pub fn multiply(&self, other: &Self) -> SwResult<Self> {
        match (self, other) {
            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Float(f1 * f2)),
            (Value::Int(i1), Value::Int(i2)) => Ok(Value::Int(i1 * i2)),
            (Value::Float(f), Value::Int(i)) | (Value::Int(i), Value::Float(f)) => {
                Ok(Value::Float(*i as FloatT * f))
            }
            (Value::Str(ref s), Value::Int(i)) => {
                let mut new_buf = s.clone();
                for _ in 0..(i - 1) {
                    new_buf.push_str(s);
                }
                Ok(Value::Str(new_buf))
            }
            _ => Err(ErrorKind::InvalidBinaryExpression(
                self.get_type(),
                other.get_type(),
                Operator::Multiply,
            )
            .into()),
        }
    }

    pub fn divide(&self, other: &Self) -> SwResult<Self> {
        match (self, other) {
            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Float(f1 / f2)),
            (Value::Int(i1), Value::Int(i2)) => Ok(Value::Int(i1 / i2)),
            (Value::Float(f), Value::Int(i)) => Ok(Value::Float(f / *i as FloatT)),
            (Value::Int(i), Value::Float(f)) => Ok(Value::Float(*i as FloatT / f)),
            _ => Err(ErrorKind::InvalidBinaryExpression(
                self.get_type(),
                other.get_type(),
                Operator::Divide,
            )
            .into()),
        }
    }

    pub fn shift_left(&self, other: &Self) -> SwResult<Self> {
        match (self, other) {
            (Value::Int(i1), Value::Int(i2)) => Ok(Value::Int(i1 << i2)),
            _ => Err(ErrorKind::InvalidBinaryExpression(
                self.get_type(),
                other.get_type(),
                Operator::ShiftLeft,
            )
            .into()),
        }
    }

    pub fn shift_right(&self, other: &Self) -> SwResult<Self> {
        match (self, other) {
            (Value::Int(i1), Value::Int(i2)) => Ok(Value::Int(i1 >> i2)),
            _ => Err(ErrorKind::InvalidBinaryExpression(
                self.get_type(),
                other.get_type(),
                Operator::ShiftRight,
            )
            .into()),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Option::Some(Ordering::Equal)
        } else {
            let (s, o) = match (self, other) {
                (Value::Int(i), Value::Float(f)) => (*i as FloatT, *f),
                (Value::Float(f), Value::Int(i)) => (*f, *i as FloatT),
                _ => return Option::None,
            };

            s.partial_cmp(&o)
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::Str(ref s1), Value::Str(ref s2)) => s1 == s2,
            (Value::List(ref l1), Value::List(ref l2)) => l1 == l2,
            (Value::Int(i1), Value::Int(i2)) => i1 == i2,
            (Value::Int(i), Value::Float(f)) | (Value::Float(f), Value::Int(i)) => {
                (*i as FloatT - f).abs() < FloatValueType::EPSILON
            }
            (Value::Float(f1), Value::Float(f2)) => (f1 - f2).abs() < FloatValueType::EPSILON,
            _ => false,
        }
    }
}

pub fn string_parse(string: &str) -> String {
    lazy_static! {
        static ref NEWLINE: Regex = Regex::new("[^\\\\]?\\\\n").unwrap();
    }

    NEWLINE.replace_all(string, "\n").into()
}
