#[allow(unused_imports)]
use super::{Value, statement};
use super::grammar;
use statement::StatementKind as Kind;
use statement::Statement;
use super::expression::Expression as Exp;
use super::Operator as Op;

fn statement(kind: Kind) -> Statement {
    Statement::tnew(kind)
}

#[test]
fn test_raw_int() {
    let l = grammar::int("3").unwrap();
    assert_eq!(l, 3.into())
}

#[test]
fn test_raw_string() {
    let l = grammar::string("\"hello!\"").unwrap();
    assert_eq!(l, "hello!".into())
}

#[test]
fn test_expression_string() {
    let l = grammar::expression("\"hello!\"").unwrap();
    assert_eq!(l, "hello!".into());
}

#[test]
fn test_list_instantiation() {
    let l = grammar::list_instantiation("foobar on a cob").unwrap();
    assert_eq!(l, Kind::ListNew("foobar".to_string()));
}

#[test]
fn test_list_instantiation_statement() {
    let l = grammar::statement_kind("foobar on a cob").unwrap();
    assert_eq!(l, Kind::ListNew("foobar".to_string()));
}

#[test]
fn test_list_append_statement() {
    let l = grammar::statement_kind("foobar assimilate 10").unwrap();
    assert_eq!(l, Kind::ListAppend("foobar".to_string(), 10.into()));

}

#[test]
fn test_list_assign() {
    let l = grammar::statement_kind("foobar[30] squanch 10").unwrap();
    assert_eq!(l, Kind::list_assign("foobar", 30, 10));
}

#[test]
fn test_printing() {
    let l = grammar::statement_kind("show me what you got \"Hello\"").unwrap();
    assert_eq!(l, Kind::print("Hello"));
}

#[test]
fn test_list_length() {
    let l = grammar::expression("apple squanch").unwrap();
    assert_eq!(l, Exp::list_length("apple"));
}

#[test]
fn test_not() {
    let l = grammar::expression("!foo").unwrap();
    assert_eq!(l, Exp::not(Exp::variable("foo")));
}

#[test]
fn test_nested_while() {
    let l = grammar::while_loop(r#"while x :<
 while y :<
 show me what you got 30
 >:
 >:"#)
        .unwrap();

    assert_eq!(l,
               Kind::while_block(Exp::variable("x"),
                                 vec![statement(Kind::while_block(Exp::variable("y"),
                                                                  vec![
                statement(Kind::print(30)),
               ]))]));
}


#[test]
fn test_while() {
    let l = grammar::while_loop(r#"while x :<
 show me what you got 30
 >:"#)
        .unwrap();
    assert_eq!(l,
               Kind::while_block(Exp::variable("x"),
                                 vec![
               statement(Kind::print(30)),
               ]));
}


#[test]
fn test_block() {
    let l = grammar::block(r#":<
 show me what you got 10




 portal gun x


 >:"#)
        .unwrap();

    assert_eq!(l, vec![Kind::print(10), Kind::input("x")]);
}


#[test]
fn test_block_starts_with_newline() {
    let l = grammar::block(r#":<

show me what you got 10
    >:"#)
        .unwrap();
    assert_eq!(l, vec![Kind::print(10)]);
}

#[test]
fn test_input() {
    let l = grammar::statement_kind(r"portal gun x").unwrap();
    assert_eq!(l, Kind::input("x"));
}

#[test]
fn test_equality() {
    let l = grammar::expression(r"(x == y)").unwrap();
    assert_eq!(l,
               Exp::operator(Exp::variable("x"), Op::Equality, Exp::variable("y")));
}

#[test]
fn test_index_and_addition() {
    let l = grammar::expression(r"(x[10] + 30)").unwrap();
    assert_eq!(l, Exp::operator(Exp::list_index("x", 10), Op::Add, 30));
}

#[test]
fn test_list_deletion() {
    let l = grammar::statement_kind(r"squanch x[10]").unwrap();
    assert_eq!(l, Kind::list_delete("x", 10));
}

#[test]
fn test_while_compound_condition() {
    let l = grammar::while_loop(r#"while (x or y) :<
 show me what you got 30
 >:"#)
        .unwrap();

    let thirty = Kind::Print(30.into());
    let x = Exp::variable("x");
    let y = Exp::variable("y");

    assert_eq!(l,
               Kind::While(Exp::operator(x, Op::Or, y), vec![statement(thirty)]));
}


#[test]
fn test_or() {
    let l = grammar::expression(r"(x or y)").unwrap();
    assert_eq!(l,
               Exp::operator(Exp::variable("x"), Op::Or, Exp::variable("y")));
}

#[test]
fn test_and() {
    let l = grammar::expression(r"(x and y)").unwrap();
    assert_eq!(l,
               Exp::operator(Exp::variable("x"), Op::And, Exp::variable("y")));
}

#[test]
fn test_neq() {
    let l = grammar::expression(r"!(x == y)").unwrap();
    assert_eq!(l,
               Exp::not(Exp::operator(Exp::variable("x"), Op::Equality, Exp::variable("y"))));
}

#[test]
fn test_operator_expression_parenthesis() {
    let l = grammar::expression(r"(x + y)").unwrap();
    assert_eq!(l,
               Exp::operator(Exp::variable("x"), Op::Add, Exp::variable("y")));
}

#[test]
fn test_expression_parenthesis() {
    let l = grammar::expression(r"(x)").unwrap();
    assert_eq!(l, Exp::variable("x"));
}


#[test]
fn test_operator_expression_parenthesis_expression_parenthesis() {
    let l = grammar::expression(r"((x) + y)").unwrap();
    assert_eq!(l,
               Exp::operator(Exp::variable("x"), Op::Add, Exp::variable("y")));
}

#[test]
fn test_operator_expressions_no_whitespace() {
    let l = grammar::expression(r"(x+y)").unwrap();
    assert_eq!(l,
               Exp::operator(Exp::variable("x"), Op::Add, Exp::variable("y")));
}

#[test]
fn test_multiple_operator_expressions_parenthesis() {
    let l = grammar::expression(r"((x + y) * 5)").unwrap();
    assert_eq!(l,
               Exp::operator(Exp::operator(Exp::variable("x"), Op::Add, Exp::variable("y")),
                             Op::Multiply,
                             5));
}

#[test]
fn test_eval_static_string() {
    let l = grammar::expression(r#"{"(1 + 3)"}"#).unwrap();
    assert_eq!(l, Exp::eval("(1 + 3)"));
}

#[test]
fn test_eval_add_strings() {
    let l = grammar::expression(r#"{("(1 " + "+ 3)")}"#).unwrap();
    assert_eq!(l, Exp::eval(Exp::operator("(1 ", Op::Add, "+ 3)")));
}

#[test]
fn test_eval_add_string_to_variable() {
    let l = grammar::expression(r#"{("(1 " + x)}"#).unwrap();
    assert_eq!(l,
               Exp::eval(Exp::operator("(1 ", Op::Add, Exp::variable("x"))));
}

#[test]
fn test_catch() {
    let l = grammar::statement_kind(r#"normal plan :<
        x squanch ("hello" + 10)
    >: plan for failure :<
        show me what you got "ERROR"



    >:"#)
        .unwrap();

    let addition = Exp::operator("hello", Op::Add, 10);
    let x = statement(Kind::assignment("x", addition));
    let error = statement(Kind::print("ERROR"));

    assert_eq!(l, Kind::catch(vec![x], vec![error]));
}
