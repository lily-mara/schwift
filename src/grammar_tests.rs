#[allow(unused_imports)]
use super::statement;
use super::grammar;
use statement::StatementKind as Kind;
use statement::Statement;
use super::expression::Expression as Exp;
use super::value::Value;
use super::Operator as Op;

fn statement(kind: Kind) -> Statement {
    Statement::tnew(kind)
}

#[test]
fn test_int() {
    let l = grammar::value("3").unwrap();
    assert_eq!(l, Value::new(3))
}

#[test]
fn test_string() {
    let l = grammar::value("\"hello!\"").unwrap();
    assert_eq!(l, Value::new("hello!"))
}

#[test]
fn test_expression_string() {
    let l = grammar::expression("\"hello!\"").unwrap();
    assert_eq!(l, Exp::new("hello!"));
}

#[test]
fn test_list_instantiation() {
    let l = grammar::statement_kind("foobar on a cob").unwrap();
    assert_eq!(l, Kind::new_list("foobar"));
}

#[test]
fn test_list_instantiation_statement() {
    let l = grammar::statement_kind("foobar on a cob").unwrap();
    assert_eq!(l, Kind::new_list("foobar"));
}

#[test]
fn test_list_append_statement() {
    let l = grammar::statement_kind("foobar assimilate 10").unwrap();
    assert_eq!(l, Kind::list_append("foobar", 10));

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
    let l = grammar::statement_kind(r#"while x :<
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
    let l = grammar::statement_kind(r#"while x :<
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
    let l = grammar::statement_kind(r#"while (x or y) :<
 show me what you got 30
 >:"#)
        .unwrap();

    let thirty = Kind::print(30);
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

#[test]
fn test_args() {
    let l = grammar::args(r#"(x, "bar")"#).unwrap();

    assert_eq!(l, vec![Exp::variable("x"), Exp::value("bar")]);
}

#[test]
fn test_function_def() {
    let l = grammar::statement_kind(r#"foo (x, y) :<
        show me what you got (x + y)
    >:"#)
        .unwrap();

    let addition = Exp::operator(Exp::variable("x"), Op::Add, Exp::variable("y"));
    let print = vec![Statement::new(Kind::print(addition), 0, 1)];

    let func = Statement::new(Kind::Function("foo".to_string(),
                                             vec!["x".to_string(), "y".to_string()],
                                             print),
                              0,
                              1);

    assert_eq!(func, l);
}

#[test]
fn test_function_no_space_in_name_and_params() {
    let l = grammar::statement_kind(r#"foo(x, y) :<
        show me what you got (x + y)
    >:"#)
        .unwrap();

    let addition = Exp::operator(Exp::variable("x"), Op::Add, Exp::variable("y"));
    let print = vec![Statement::new(Kind::print(addition), 0, 1)];

    let func = Statement::new(Kind::Function("foo".to_string(),
                                             vec!["x".to_string(), "y".to_string()],
                                             print),
                              0,
                              1);

    assert_eq!(func, l);
}

#[test]
fn test_function_no_spaces_in_def() {
    let l = grammar::file(r#"foo(x):<
        show me what you got x
    >:"#)
        .unwrap();

    let print = vec![Statement::new(Kind::print(Exp::variable("x")), 0, 1)];

    let func = Statement::new(Kind::Function("foo".to_string(), vec!["x".to_string()], print),
                              0,
                              1);

    assert_eq!(func, l[0]);
}

#[test]
fn test_modulus() {
    let l = grammar::expression(r#"(5 % 4)"#).unwrap();
    assert_eq!(l, Exp::operator(Exp::value(5), Op::Modulus, Exp::value(4)));
}

#[test]
fn test_empty_after_block() {
    let l = grammar::file(r#"
        if rick :<
            x squanch 100
        >:


    "#)
        .unwrap();
    assert_eq!(l[0], Statement::tnew(Kind::if_block(Exp::value(true), vec![Statement::tnew(Kind::assignment("x", Exp::value(100)))], None)));
}
