use crate::{
    error::{ErrorKind, ErrorKindExt, ErrorWithContext, SwResult},
    expression::Expression,
    grammar,
    statement::{Statement, StatementKind},
    value::{self, Value},
    vec_map::VecMap,
};
use std::{borrow, io, mem};

type Map<K, V> = VecMap<K, V>;

#[cfg(test)]
mod test;

pub struct State {
    symbols: Map<String, Value>,
    last_return: Option<Value>,
    libraries: Vec<libloading::Library>,
}

// macro_rules! error {
//     ( $kind:expr, $place:expr ) => {{
//         Err(crate::error::Error::new($kind, $place))
//     }};
// }

// macro_rules! try_error {
//     ( $error:expr, $statement:expr ) => {{
//         match $error {
//             Ok(val) => val,
//             Err(err) => return Err(crate::error::Error::new(err, $statement.clone())),
//         }
//     }};
// }

// macro_rules! try_nop_error {
//     ( $error:expr, $statement:expr ) => {{
//         match $error {
//             Ok(_) => Ok(()),
//             Err(err) => return Err(crate::error::Error::new(err, $statement.clone())),
//         }
//     }};
// }

impl State {
    pub fn list_index<'a>(
        &'a self,
        list_name: &str,
        exp: &Expression,
    ) -> SwResult<borrow::Cow<'a, Value>> {
        let inner_expression_value = exp.evaluate(self)?.into_owned();
        match self.symbols.get(list_name) {
            Some(symbol) => match *symbol {
                Value::List(ref l) => {
                    if let Value::Int(i) = inner_expression_value {
                        let index = i as usize;
                        if index < l.len() {
                            Ok(borrow::Cow::Borrowed(&l[index]))
                        } else {
                            Err(ErrorKind::IndexOutOfBounds {
                                len: l.len(),
                                index,
                            }
                            .into())
                        }
                    } else {
                        Err(ErrorKind::UnexpectedType {
                            expected: value::Type::Int,
                            actual: inner_expression_value.get_type(),
                        }
                        .into())
                    }
                }
                Value::Str(ref s) => {
                    if let Value::Int(i) = inner_expression_value {
                        let index = i as usize;
                        let chars: Vec<char> = s.chars().collect();

                        if index < chars.len() {
                            Ok(borrow::Cow::Owned(Value::Str(chars[index].to_string())))
                        } else {
                            Err(ErrorKind::IndexOutOfBounds {
                                len: chars.len(),
                                index,
                            }
                            .into())
                        }
                    } else {
                        Err(ErrorKind::UnexpectedType {
                            expected: value::Type::Int,
                            actual: inner_expression_value.get_type(),
                        }
                        .into())
                    }
                }
                _ => Err(ErrorKind::IndexUnindexable(symbol.get_type()).into()),
            },
            None => Err(ErrorKind::UnknownVariable(list_name.to_string()).into()),
        }
    }

    pub fn call_function(&self, name: &str, args: &[Expression]) -> SwResult<Value> {
        let mut call_args = Vec::new();

        for x in args {
            call_args.push(x.evaluate(self)?.into_owned());
        }

        if let Value::NativeFunction(ref funk) = *self.get(name)? {
            return funk.call(&mut call_args);
        }

        match self.get(name)? {
            Value::Function(ref params, ref body) => {
                if args.len() != params.len() {
                    return Err(ErrorKind::InvalidArguments(
                        name.to_string(),
                        args.len(),
                        params.len(),
                    )
                    .into());
                }

                let mut child_state = Self::default();

                for (name, arg) in params.iter().zip(call_args) {
                    child_state.symbols.insert(name.to_string(), arg);
                }

                child_state.run(body)?;

                let last_ret = mem::replace(&mut child_state.last_return, None);

                match last_ret {
                    Some(val) => Ok(val),
                    None => Err(ErrorKind::NoReturn(name.to_string()).into()),
                }
            }
            val => Err(ErrorKind::UnexpectedType {
                expected: value::Type::Function,
                actual: val.get_type(),
            }
            .into()),
        }
    }

    pub fn get(&self, name: &str) -> SwResult<&Value> {
        match self.symbols.get(name) {
            Some(val) => Ok(val),
            None => Err(ErrorKind::UnknownVariable(name.to_string()).into()),
        }
    }

    pub fn assign(&mut self, str: String, exp: &Expression) -> SwResult<()> {
        let v = exp.evaluate(self)?.into_owned();
        self.symbols.insert(str, v);
        Ok(())
    }

    fn delete(&mut self, name: &str) -> SwResult<()> {
        match self.symbols.remove(name) {
            Some(_) => Ok(()),
            None => Err(ErrorKind::UnknownVariable(name.to_string()).into()),
        }
    }

    fn print(&mut self, exp: &Expression) -> SwResult<()> {
        let x = exp.evaluate(self)?;
        x.println();
        Ok(())
    }

    fn print_no_nl(&mut self, exp: &Expression) -> SwResult<()> {
        let x = exp.evaluate(self)?;
        x.print();
        Ok(())
    }

    fn input(&mut self, name: String) -> SwResult<()> {
        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        input = input.trim().to_string();
        self.symbols.insert(name, Value::Str(input));

        Ok(())
    }

    fn list_append(&mut self, list_name: &str, append_exp: &Expression) -> SwResult<()> {
        let to_append = append_exp.evaluate(self)?.into_owned();
        let list = self.get_list(list_name)?;

        list.push(to_append);
        Ok(())
    }

    fn get_mut(&mut self, name: &str) -> SwResult<&mut Value> {
        match self.symbols.get_mut(name) {
            Some(value) => Ok(value),
            None => Err(ErrorKind::UnknownVariable(name.to_string()).into()),
        }
    }

    fn get_list(&mut self, name: &str) -> SwResult<&mut Vec<Value>> {
        let value = self.get_mut(name)?;
        match *value {
            Value::List(ref mut l) => Ok(l),
            _ => Err(ErrorKind::IndexUnindexable(value.get_type()).into()),
        }
    }

    fn get_list_element(&mut self, name: &str, index_exp: &Expression) -> SwResult<&mut Value> {
        let index = index_exp.try_int(self)? as usize;
        let value = self.get_mut(name)?;

        match *value {
            Value::List(ref mut list) => {
                let len = list.len();
                if index < len {
                    Ok(&mut list[index])
                } else {
                    Err(ErrorKind::IndexOutOfBounds { len, index }.into())
                }
            }
            ref val => Err(ErrorKind::IndexUnindexable(val.get_type()).into()),
        }
    }

    fn list_assign(
        &mut self,
        list_name: &str,
        index_exp: &Expression,
        assign_exp: &Expression,
    ) -> SwResult<()> {
        let to_assign = assign_exp.evaluate(self)?.into_owned();
        let element = self.get_list_element(list_name, index_exp)?;

        *element = to_assign;
        Ok(())
    }

    fn list_delete(&mut self, list_name: &str, index_exp: &Expression) -> SwResult<()> {
        let index_value = index_exp.evaluate(self)?;

        if let Value::Int(i) = *index_value {
            let index = i as usize;
            let list = self.get_list(list_name)?;

            if index < list.len() {
                list.remove(index);
                Ok(())
            } else {
                Err(ErrorKind::IndexOutOfBounds {
                    len: list.len(),
                    index,
                }
                .into())
            }
        } else {
            Err(ErrorKind::UnexpectedType {
                expected: value::Type::Int,
                actual: index_value.get_type(),
            }
            .into())
        }
    }

    fn exec_if(
        &mut self,
        bool: &Expression,
        if_body: &[Statement],
        else_body: &Option<Vec<Statement>>,
    ) -> SwResult<()> {
        let x = bool.evaluate(self)?;

        match *x {
            Value::Bool(b) => {
                if b {
                    self.run(if_body)?;
                } else {
                    match *else_body {
                        Option::Some(ref s) => self.run(s)?,
                        Option::None => {}
                    }
                }
            }
            _ => {
                return Err(ErrorKind::UnexpectedType {
                    expected: value::Type::Bool,
                    actual: x.get_type(),
                }
                .into())
            }
        }

        Ok(())
    }

    fn exec_while(
        &mut self,
        statement: &Statement,
        bool: &Expression,
        body: &[Statement],
    ) -> SwResult<()> {
        let mut condition = bool.try_bool(self)?;

        while condition {
            self.run(body)?;
            if self.last_return.is_some() {
                return Ok(());
            }
            condition = bool.try_bool(self).with_error_ctx(statement)?;
        }

        Ok(())
    }

    fn catch(&mut self, try_block: &[Statement], catch: &[Statement]) -> SwResult<()> {
        match self.run(try_block) {
            Err(_) => self.run(catch)?,
            _ => {}
        }

        Ok(())
    }

    pub fn execute(&mut self, statement: &Statement) -> Result<(), ErrorWithContext> {
        match statement.kind {
            StatementKind::Input(ref s) => self.input(s.to_string()),
            StatementKind::ListAssign(ref s, ref index_exp, ref assign_exp) => {
                self.list_assign(s, index_exp, assign_exp)
            }
            StatementKind::ListAppend(ref s, ref append_exp) => self.list_append(s, append_exp),
            StatementKind::ListDelete(ref name, ref idx) => self.list_delete(name, idx),
            StatementKind::ListNew(ref s) => {
                self.symbols.insert(s.clone(), Value::List(Vec::new()));

                Ok(())
            }
            StatementKind::If(ref bool, ref if_body, ref else_body) => {
                self.exec_if(bool, if_body, else_body)
            }
            StatementKind::While(ref bool, ref body) => self.exec_while(statement, bool, body),
            StatementKind::Assignment(ref name, ref value) => self.assign(name.clone(), value),
            StatementKind::Delete(ref name) => self.delete(name),
            StatementKind::Print(ref exp) => self.print(exp),
            StatementKind::PrintNoNl(ref exp) => self.print_no_nl(exp),
            StatementKind::Catch(ref try_block, ref catch) => self.catch(try_block, catch),
            StatementKind::Function(ref name, ref args, ref body) => {
                self.symbols
                    .insert(name.clone(), Value::Function(args.clone(), body.clone()));
                Ok(())
            }
            StatementKind::Return(ref expr) => {
                let val = expr.evaluate(self).with_error_ctx(statement)?;
                self.last_return = Some(val.into_owned());

                Ok(())
            }

            StatementKind::FunctionCall(ref name, ref args) => {
                self.call_function(name, args).map(|_| ())
            }
            StatementKind::DylibLoad(ref lib_path, ref functions) => {
                self.dylib_load(lib_path, functions)
            }
        }
        .with_error_ctx(statement)
    }

    pub fn run(&mut self, statements: &[Statement]) -> Result<(), ErrorWithContext> {
        for statement in statements {
            match self.execute(statement) {
                Err(e) => return Err(e),
                Ok(()) => {}
            }
            if let StatementKind::Return(_) = statement.kind {
                return Ok(());
            }
            if self.last_return.is_some() {
                return Ok(());
            }
        }

        Ok(())
    }

    fn dylib_load(&mut self, lib_path: &str, functions: &[Statement]) -> SwResult<()> {
        unsafe {
            let dylib = libloading::Library::new(lib_path)?;

            let compat: libloading::Symbol<&u32> =
                dylib.get(b"LIBSCHWIFT_ABI_COMPAT").map_err(|error| {
                    ErrorKind::MissingAbiCompat {
                        error,
                        library: lib_path.into(),
                    }
                })?;

            if **compat != crate::LIBSCHWIFT_ABI_COMPAT {
                return Err(ErrorKind::IncompatibleAbi(**compat).into());
            }

            for statement in functions {
                match statement.kind {
                    StatementKind::FunctionCall(ref name, _) => {
                        let wrapped_func: libloading::Symbol<value::_Func> =
                            dylib.get(name.as_bytes())?;

                        let func = wrapped_func.into_raw();
                        self.insert(name.as_str(), value::Func::new(func));
                    }
                    _ => return Err(ErrorKind::NonFunctionCallInDylib(statement.clone()).into()),
                }
            }

            self.libraries.push(dylib);
        }

        Ok(())
    }

    pub fn parse_args(&mut self, args: &[&str]) {
        let mut value_args = Vec::new();

        for arg in args {
            value_args.push(grammar::value(arg).unwrap_or_else(|_| Value::Str((*arg).into())));
        }

        self.symbols.insert("argv".into(), value_args.into());
    }

    pub fn insert<S, V>(&mut self, name: S, value: V)
    where
        S: Into<String>,
        V: Into<Value>,
    {
        self.symbols.insert(name.into(), value.into());
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            symbols: Map::new(),
            last_return: None,
            libraries: Vec::new(),
        }
    }
}
