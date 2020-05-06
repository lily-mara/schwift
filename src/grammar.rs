use crate::expression::Expression;
use crate::statement::{Statement, StatementKind};
use crate::value::{string_parse, FloatT, IntT, Value};
use crate::Operator;

pub use self::grammar::*;

pub type ParseError = peg::error::ParseError<peg_runtime::str::LineCol>;

peg::parser! {grammar grammar() for str {

    rule string_inquotes() -> String
        = s:$((!['"'][_])*) { string_parse(s) }

    rule int() -> IntT
        = n:$("-"? ['0'..='9']+) { n.parse::<IntT>().unwrap() }

    rule float() -> FloatT
        = n:$(['0'..='9']+ "." ['0'..='9']+) { n.parse::<FloatT>().unwrap() }

    rule string() -> String
        = "\"" s:string_inquotes() "\"" { s }

    pub rule value() -> Value
        = f:float() { Value::Float(f) }
        / i:int() { Value::Int(i) }
        / s:string() { Value::Str(s) }
        / "rick" { Value::Bool(true) }
        / "morty" { Value::Bool(false) }

    rule identifier() -> String
        = s:$(['a'..='z' | 'A'..='Z' | '_'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { s.to_string() }

    pub rule operator() -> Operator
        = "+" { Operator::Add }
        / "-" { Operator::Subtract }
        / "*" { Operator::Multiply }
        / "/" { Operator::Divide }
        / "==" { Operator::Equality }
        / "%" { Operator::Modulus }
        / "moresquanch" { Operator::GreaterThanEqual }
        / "lesssquanch" { Operator::LessThanEqual }
        / "more" { Operator::GreaterThan }
        / "less" { Operator::LessThan }
        / "schwift>" { Operator::ShiftRight }
        / "<schwift" { Operator::ShiftLeft }
        / "or" { Operator::Or }
        / "and" { Operator::And }

    rule WS()
        = [' ' | '\t']+

    rule ws()
        = [' ' | '\t']*

    rule comma()
        = ws() "," ws()

    pub rule block() -> Vec<Statement>
        = ":<" ws() newline()* l:line()* ws() ">:" { l }

    rule line() -> Statement
        = newline()* ws() s:statement() ws() newline()* { s }

    pub rule file() -> Vec<Statement>
        = l:line()+ ws() { l }

    pub rule newline()
        = ws() "\n" / ws() "\r\n"

    pub rule params() -> Vec<String>
        = "(" is:identifier() ** comma() ")" { is }

    pub rule statement_kind() -> StatementKind
        = "squanch" WS() i:identifier() ws() "[" ws() e:expression() ws() "]" { StatementKind::ListDelete(i, e) }
        / i:identifier() WS() "on a cob" { StatementKind::ListNew(i) }
        / i:identifier() WS() "assimilate" WS() e:expression() { StatementKind::ListAppend(i, e) }
        / i:identifier() ws() "[" ws() v:expression() ws() "]" WS() "squanch" WS() e:expression() { StatementKind::ListAssign(i, v, e) }
        / n:identifier() ws() p:params() ws() b:block() { StatementKind::Function(n, p, b) }
        / "squanch" WS() i:identifier() { StatementKind::Delete(i) }
        / i:identifier() WS() "squanch" WS() e:expression() { StatementKind::Assignment(i, e) }
        / "show me what you got!" WS() e:expression() { StatementKind::PrintNoNl(e) }
        / "show me what you got" WS() e:expression() { StatementKind::Print(e) }
        / "if" WS() e:expression() WS() i_bod:block() ws() "else" WS() e_bod:block() { StatementKind::If(e, i_bod, Option::Some(e_bod)) }
        / "if" WS() e:expression() WS() s:block() { StatementKind::If(e, s, Option::None) }
        / "while" WS() e:expression() WS() b:block() { StatementKind::While(e, b) }
        / "portal gun" WS() i:identifier() { StatementKind::Input(i) }
        / "normal plan" ws() try_block:block() ws() "plan for failure" ws() catch:block() { StatementKind::Catch(try_block, catch) }
        / i:identifier() a:args() { StatementKind::FunctionCall(i, a) }
        / "return" WS() e:expression() { StatementKind::Return(e) }
        / "microverse" WS() lib:string() WS() funcs:block() { StatementKind::DylibLoad(lib, funcs) }

    pub rule statement() -> Statement
        = start:position!() s:statement_kind() end:position!() { Statement::new(s, start, end) }

    pub rule expression() -> Expression
        = "{" ws() e:expression() ws() "}" { Expression::Eval(Box::new(e)) }
        / "(" ws() e1:expression() ws() o:operator() ws() e2:expression() ws() ")" { Expression::OpExp(Box::new(e1), o, Box::new(e2)) }
        / "(" ws() e:expression1() ws() ")" { e }
        / expression1()

    pub rule args() -> Vec<Expression>
        = "(" exprs:expression() ** comma() ")" { exprs }

    rule expression1() -> Expression
        = i:identifier() ws() "[" ws() e:expression() ws() "]" { Expression::ListIndex(i, Box::new(e)) }
        / i:identifier() a:args() { Expression::FunctionCall(i, a) }
        / v:value() { Expression::Value(v) }
        / i:identifier() WS() "squanch" { Expression::ListLength(i) }
        / i:identifier() { Expression::Variable(i) }
        / "!" ws() e:expression() { Expression::Not(Box::new(e)) }

}}
