use super::{Value, StatementKind, Expression, Operator, Statement};
use super::grammar;
use StatementKind as Kind;

fn statement(kind: Kind) -> Statement {
    Statement {
        kind: kind,
        start: 0,
        end: 1,
    }
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
    assert_eq!(l,
               Kind::ListAssign("foobar".to_string(), 30.into(), 10.into()));
}

#[test]
fn test_printing() {
    let l = grammar::statement_kind("show me what you got \"Hello\"").unwrap();
    assert_eq!(l, Kind::Print("Hello".into()));
}

#[test]
fn test_list_length() {
    let l = grammar::expression("apple squanch").unwrap();
    assert_eq!(l, Expression::ListLength("apple".to_string()));
}

#[test]
fn test_not() {
    let l = grammar::expression("!foo").unwrap();
    assert_eq!(l,
               Expression::Not(Box::new(Expression::Variable("foo".to_string()))));
}

#[test]
fn test_nested_while() {
    let l = grammar::while_loop(r#"while x :<
 while y :<
 show me what you got 30
 >:
 >:"#)
        .unwrap();

    let print = statement(Kind::Print(30.into()));
    let y = Expression::Variable("y".to_string());

    assert_eq!(l,
               Kind::While(Expression::Variable("x".to_string()),
                           vec![statement(Kind::While(y,
                                                      vec![
                print,
               ]))]));
}


#[test]
fn test_while() {
    let l = grammar::while_loop(r#"while x :<
 show me what you got 30
 >:"#)
        .unwrap();
    assert_eq!(l,
               Kind::While(Expression::Variable("x".to_string()),
                           vec![
               statement(Kind::Print(30.into())),
               ]));
}


#[test]
fn test_block() {
    let l = grammar::block(r#":<
 show me what you got 10




 portal gun x


 >:"#)
        .unwrap();

    assert_eq!(l,
               vec![Kind::Print(10.into()), Kind::Input("x".to_string())]);
}


#[test]
fn test_block_starts_with_newline() {
    let l = grammar::block(r#":<

show me what you got 10
    >:"#)
        .unwrap();
    assert_eq!(l.len(), 1);
    assert_eq!(l[0].kind, Kind::Print(10.into()));
}

#[test]
fn test_input() {
    let l = grammar::statement_kind(r"portal gun x").unwrap();
    assert_eq!(l, Kind::Input("x".to_string()));
}

#[test]
fn test_equality() {
    let l = grammar::expression(r"(x == y)").unwrap();
    assert_eq!(l,
               Expression::OperatorExpression(Box::new(Expression::Variable("x".to_string())),
                                              Operator::Equality,
                                              Box::new(Expression::Variable("y".to_string()))));
}

#[test]
fn test_index_and_addition() {
    let l = grammar::expression(r"(x[10] + 30)").unwrap();
    assert_eq!(
        l,
        Expression::OperatorExpression(
            Box::new(Expression::ListIndex(
                "x".to_string(),
                Box::new(10.into())
            )),
            Operator::Add,
            Box::new(30.into())
        )
    );
}

#[test]
fn test_list_deletion() {
    let l = grammar::statement_kind(r"squanch x[10]").unwrap();
    assert_eq!(l, Kind::ListDelete("x".to_string(), 10.into()));
}

#[test]
fn test_while_compound_condition() {
    let l = grammar::while_loop(r#"while (x or y) :<
 show me what you got 30
 >:"#)
        .unwrap();

    let thirty = Kind::Print(30.into());
    let x = Box::new(Expression::Variable("x".to_string()));
    let y = Box::new(Expression::Variable("y".to_string()));

    assert_eq!(l,
               Kind::While(Expression::OperatorExpression(x, Operator::Or, y),
                           vec![statement(thirty)]));
}


#[test]
fn test_or() {
    let l = grammar::expression(r"(x or y)").unwrap();
    assert_eq!(l,
               Expression::OperatorExpression(Box::new(Expression::Variable("x".to_string())),
                                              Operator::Or,
                                              Box::new(Expression::Variable("y".to_string()))));
}

#[test]
fn test_and() {
    let l = grammar::expression(r"(x and y)").unwrap();
    assert_eq!(l,
               Expression::OperatorExpression(Box::new(Expression::Variable("x".to_string())),
                                              Operator::And,
                                              Box::new(Expression::Variable("y".to_string()))));
}

#[test]
fn test_neq() {
    let l = grammar::expression(r"!(x == y)").unwrap();
    assert_eq!(
        l,
        Expression::Not(
            Box::new(Expression::OperatorExpression(
                Box::new(Expression::Variable("x".to_string())),
                Operator::Equality,
                Box::new(Expression::Variable("y".to_string())),
            ))
        )
    );
}

#[test]
fn test_operator_expression_parenthesis() {
    let l = grammar::expression(r"(x + y)").unwrap();
    assert_eq!(l,
               Expression::OperatorExpression(Box::new(Expression::Variable("x".to_string())),
                                              Operator::Add,
                                              Box::new(Expression::Variable("y".to_string()))));
}

#[test]
fn test_expression_parenthesis() {
    let l = grammar::expression(r"(x)").unwrap();
    assert_eq!(l, Expression::Variable("x".to_string()));
}


#[test]
fn test_operator_expression_parenthesis_expression_parenthesis() {
    let l = grammar::expression(r"((x) + y)").unwrap();
    assert_eq!(l,
               Expression::OperatorExpression(Box::new(Expression::Variable("x".to_string())),
                                              Operator::Add,
                                              Box::new(Expression::Variable("y".to_string()))));
}

#[test]
fn test_operator_expressions_no_whitespace() {
    let l = grammar::expression(r"(x+y)").unwrap();
    assert_eq!(l,
               Expression::OperatorExpression(Box::new(Expression::Variable("x".to_string())),
                                              Operator::Add,
                                              Box::new(Expression::Variable("y".to_string()))));
}

#[test]
fn test_multiple_operator_expressions_parenthesis() {
    let l = grammar::expression(r"((x + y) * 5)").unwrap();
    assert_eq!(
        l,
        Expression::OperatorExpression(
            Box::new(Expression::OperatorExpression(
                Box::new(Expression::Variable("x".to_string())),
                Operator::Add,
                Box::new(Expression::Variable("y".to_string())),
            )),
            Operator::Multiply,
            Box::new(5.into()),
        )
    );
}

#[test]
fn test_eval_static_string() {
    let l = grammar::expression(r#"{"(1 + 3)"}"#).unwrap();
    assert_eq!(l, Expression::Eval(Box::new("(1 + 3)".into())));
}

#[test]
fn test_eval_add_strings() {
    let l = grammar::expression(r#"{("(1 " + "+ 3)")}"#).unwrap();
    assert_eq!(l,
               Expression::Eval(Box::new(Expression::OperatorExpression(Box::new("(1 ".into()),
                                                                        Operator::Add,
                                                                        Box::new("+ 3)".into())))));
}

#[test]
fn test_eval_add_string_to_variable() {
    let l = grammar::expression(r#"{("(1 " + x)}"#).unwrap();
    assert_eq!(
        l,
        Expression::Eval(
            Box::new(Expression::OperatorExpression(
                Box::new("(1 ".into()),
                Operator::Add,
                Box::new(Expression::Variable("x".to_string())),
            ))
        )
    );
}

#[test]
fn test_catch() {
    let l = grammar::statement_kind(r#"normal plan :<
        x squanch ("hello" + 10)
    >: plan for failure :<
        show me what you got "ERROR"



    >:"#)
        .unwrap();

    let addition = Expression::OperatorExpression(Box::new("hello".into()),
                                                  Operator::Add,
                                                  Box::new(10.into()));
    let x = statement(Kind::Assignment("x".to_string(), addition));
    let error = statement(Kind::Print("ERROR".into()));

    assert_eq!(l, Kind::Catch(vec![x], vec![error]));
}
