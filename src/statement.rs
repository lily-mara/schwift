use super::expression::Expression;

use std::io;
use std::fs::File;
use std::cmp;
use std::io::prelude::*;

#[derive(Debug, Clone)]
pub struct Statement {
    start: usize,
    end: usize,
    pub kind: StatementKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementKind {
    Assignment(String, Expression),
    Delete(String),
    Print(Expression),
    PrintNoNl(Expression),
    ListNew(String),
    ListAppend(String, Expression),
    ListAssign(String, Expression, Expression),
    ListDelete(String, Expression),
    If(Expression, Vec<Statement>, Option<Vec<Statement>>),
    While(Expression, Vec<Statement>),
    Input(String),
    Catch(Vec<Statement>, Vec<Statement>),
    Function(String, Vec<String>, Vec<Statement>),
    Return(Expression),
    FunctionCall(String, Vec<Expression>),
    DylibLoad(String, Vec<Statement>),
}

impl StatementKind {
    pub fn assignment<S, E>(name: S, expr: E) -> StatementKind
        where S: Into<String>,
              E: Into<Expression>
    {
        StatementKind::Assignment(name.into(), expr.into())
    }

    pub fn dylib_load<S, E>(lib_path: S, functions: Vec<Statement>) -> StatementKind
        where S: Into<String>
    {

        StatementKind::DylibLoad(lib_path.into(), functions)
    }

    pub fn return_it<E>(expr: E) -> StatementKind
        where E: Into<Expression>
    {
        StatementKind::Return(expr.into())
    }

    pub fn list_append<S, E>(name: S, expr: E) -> StatementKind
        where S: Into<String>,
              E: Into<Expression>
    {
        StatementKind::ListAppend(name.into(), expr.into())
    }

    pub fn list_delete<S, E>(name: S, expr: E) -> StatementKind
        where S: Into<String>,
              E: Into<Expression>
    {
        StatementKind::ListDelete(name.into(), expr.into())
    }

    pub fn if_block<E>(condition: E,
                       if_body: Vec<Statement>,
                       else_body: Option<Vec<Statement>>)
                       -> StatementKind
        where E: Into<Expression>
    {
        StatementKind::If(condition.into(), if_body, else_body)
    }

    pub fn while_block<E>(condition: E, body: Vec<Statement>) -> StatementKind
        where E: Into<Expression>
    {
        StatementKind::While(condition.into(), body)
    }

    pub fn input<S>(name: S) -> StatementKind
        where S: Into<String>
    {
        StatementKind::Input(name.into())
    }

    pub fn catch(try: Vec<Statement>, catch: Vec<Statement>) -> StatementKind {
        StatementKind::Catch(try, catch)
    }

    pub fn list_assign<S, E, R>(name: S, index: E, assign: R) -> StatementKind
        where S: Into<String>,
              E: Into<Expression>,
              R: Into<Expression>
    {
        StatementKind::ListAssign(name.into(), index.into(), assign.into())
    }

    pub fn delete<S>(name: S) -> StatementKind
        where S: Into<String>
    {
        StatementKind::Delete(name.into())
    }

    pub fn print<E>(expr: E) -> StatementKind
        where E: Into<Expression>
    {
        StatementKind::Print(expr.into())
    }

    pub fn new_list<S>(name: S) -> StatementKind
        where S: Into<String>
    {
        StatementKind::ListNew(name.into())
    }
}

impl Statement {
    pub fn new(kind: StatementKind, start: usize, end: usize) -> Statement {
        Statement {
            kind: kind,
            start: start,
            end: end,
        }
    }

    #[cfg(test)]
    pub fn tnew(kind: StatementKind) -> Statement {
        Statement {
            kind: kind,
            start: 0,
            end: 0,
        }
    }

    pub fn get_source(&self, filename: &str) -> io::Result<String> {
        let mut source = String::new();
        let mut f = File::open(filename)?;
        f.read_to_string(&mut source)?;

        assert!(self.start < self.end);
        assert!(source.is_char_boundary(self.start));
        assert!(source.is_char_boundary(self.end));

        Ok(unsafe { source.slice_unchecked(self.start, self.end) }.to_string())
    }
}

#[cfg(test)]
impl cmp::PartialEq<StatementKind> for Statement {
    fn eq(&self, kind: &StatementKind) -> bool {
        self.kind == *kind
    }
}

impl cmp::PartialEq<Statement> for Statement {
    #[cfg(test)]
    fn eq(&self, other: &Statement) -> bool {
        self.kind == other.kind
    }

    #[cfg(not(test))]
    fn eq(&self, other: &Statement) -> bool {
        self.kind == other.kind && self.start == other.start && self.end == other.end
    }
}
