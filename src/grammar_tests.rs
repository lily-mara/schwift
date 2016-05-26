use super::{ Value, Statement, Expression, Operator };
use super::grammar;

#[test]
fn test_raw_int() {
    let l = grammar::int("3").unwrap();
    assert_eq!(l, 3)
}

#[test]
fn test_raw_string() {
    let l = grammar::string("\"hello!\"").unwrap();
    assert_eq!(l, Value::Str("hello!".to_string()))
}

#[test]
fn test_expression_string() {
    let l = grammar::expression("\"hello!\"").unwrap();
    assert_eq!(l, Expression::Value(Value::Str("hello!".to_string())));
}

#[test]
fn test_list_instantiation() {
    let l = grammar::list_instantiation("foobar on a cob").unwrap();
    assert_eq!(l, Statement::ListNew("foobar".to_string()));
}

#[test]
fn test_list_instantiation_statement() {
    let l = grammar::statement("foobar on a cob").unwrap();
    assert_eq!(l, Statement::ListNew("foobar".to_string()));
}

#[test]
fn test_list_append_statement() {
    let l = grammar::statement("foobar assimilate 10").unwrap();
    assert_eq!(l, Statement::ListAppend("foobar".to_string(), Expression::Value(Value::Int(10))));

}

#[test]
fn test_list_assign() {
    let l = grammar::statement("foobar[30] squanch 10").unwrap();
    assert_eq!(l, Statement::ListAssign("foobar".to_string(), Expression::Value(Value::Int(30)),  Expression::Value(Value::Int(10))));
}

#[test]
fn test_printing() {
    let l = grammar::statement("show me what you got \"Hello\"").unwrap();
    assert_eq!(l, Statement::Print(Expression::Value(Value::Str("Hello".to_string()))));
}

#[test]
fn test_list_length() {
    let l = grammar::expression("apple squanch").unwrap();
    assert_eq!(l, Expression::ListLength("apple".to_string()));
}

#[test]
fn test_not() {
    let l = grammar::expression("!foo").unwrap();
    assert_eq!(l, Expression::Not(Box::new(Expression::Variable("foo".to_string()))));
}

#[test]
fn test_nested_while() {
    let l = grammar::while_loop(r#"while x :<
        while y :<
            show me what you got 30
        >:
    >:"#).unwrap();
    assert_eq!(
        l,
        Statement::While(
            Expression::Variable("x".to_string()),
            vec![
                Statement::While(
                    Expression::Variable("y".to_string()),
                    vec![
                        Statement::Print(Expression::Value(Value::Int(30))),
                    ],
                )
            ],
        )
    );
}

#[test]
fn test_while() {
    let l = grammar::while_loop(r#"while x :<
    show me what you got 30
    >:"#).unwrap();
    assert_eq!(
        l,
        Statement::While(
            Expression::Variable("x".to_string()),
            vec![
                Statement::Print(Expression::Value(Value::Int(30))),
            ],
        )
    );
}

#[test]
fn test_block() {
    let l = grammar::block(r#":<
show me what you got 10




portal gun x


    >:"#).unwrap();
    assert_eq!(
        l,
        vec![
            Statement::Print(Expression::Value(Value::Int(10))),
            Statement::Input("x".to_string()),
        ]
    );
}

#[test]
fn test_block_starts_with_newline() {
    let l = grammar::block(r#":<

show me what you got 10
    >:"#).unwrap();
    assert_eq!(
        l,
        vec![
            Statement::Print(Expression::Value(Value::Int(10))),
        ]
    );
}

#[test]
fn test_input() {
    let l = grammar::statement(r"portal gun x").unwrap();
    assert_eq!(
        l,
        Statement::Input("x".to_string())
    );
}

#[test]
fn test_equality() {
    let l = grammar::expression(r"(x == y)").unwrap();
    assert_eq!(
        l,
        Expression::OperatorExpression(
            Box::new(Expression::Variable("x".to_string())),
            Operator::Equality,
            Box::new(Expression::Variable("y".to_string())),
        )
    );
}

#[test]
fn test_index_and_addition() {
    let l = grammar::expression(r"(x[10] + 30)").unwrap();
    assert_eq!(
        l,
        Expression::OperatorExpression(
            Box::new(Expression::ListIndex(
                "x".to_string(),
                Box::new(Expression::Value(Value::Int(10)))
            )),
            Operator::Add,
            Box::new(Expression::Value(Value::Int(30)))
        )
    );
}

#[test]
fn test_list_deletion() {
    let l = grammar::statement(r"squanch x[10]").unwrap();
    assert_eq!(
        l,
        Statement::ListDelete(
            "x".to_string(),
            Expression::Value(Value::Int(10))
        )
    );
}

#[test]
fn test_while_compound_condition() {
    let l = grammar::while_loop(r#"while (x or y) :<
    show me what you got 30
    >:"#).unwrap();
    assert_eq!(
        l,
        Statement::While(
            Expression::OperatorExpression(
                Box::new(Expression::Variable("x".to_string())),
                Operator::Or,
                Box::new(Expression::Variable("y".to_string())),
            ),
            vec![
                Statement::Print(Expression::Value(Value::Int(30))),
            ],
        )
    );
}

#[test]
fn test_or() {
    let l = grammar::expression(r"(x or y)").unwrap();
    assert_eq!(
        l,
        Expression::OperatorExpression(
            Box::new(Expression::Variable("x".to_string())),
            Operator::Or,
            Box::new(Expression::Variable("y".to_string())),
        )
    );
}

#[test]
fn test_and() {
    let l = grammar::expression(r"(x and y)").unwrap();
    assert_eq!(
        l,
        Expression::OperatorExpression(
            Box::new(Expression::Variable("x".to_string())),
            Operator::And,
            Box::new(Expression::Variable("y".to_string())),
        )
    );
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
    assert_eq!(
        l,
        Expression::OperatorExpression(
            Box::new(Expression::Variable("x".to_string())),
            Operator::Add,
            Box::new(Expression::Variable("y".to_string())),
        )
    );
}

#[test]
fn test_expression_parenthesis() {
    let l = grammar::expression(r"(x)").unwrap();
    assert_eq!(
        l,
        Expression::Variable("x".to_string())
    );
}


#[test]
fn test_operator_expression_parenthesis_expression_parenthesis() {
    let l = grammar::expression(r"((x) + y)").unwrap();
    assert_eq!(
        l,
        Expression::OperatorExpression(
            Box::new(Expression::Variable("x".to_string())),
            Operator::Add,
            Box::new(Expression::Variable("y".to_string())),
        )
    );
}

#[test]
fn test_operator_expressions_no_whitespace() {
    let l = grammar::expression(r"(x+y)").unwrap();
    assert_eq!(
        l,
        Expression::OperatorExpression(
            Box::new(Expression::Variable("x".to_string())),
            Operator::Add,
            Box::new(Expression::Variable("y".to_string())),
        )
    );
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
            Box::new(Expression::Value(Value::Int(5))),
        )
    );
}

#[test]
fn test_eval_static_string() {
    let l = grammar::expression(r#"{"(1 + 3)"}"#).unwrap();
    assert_eq!(
        l,
        Expression::Eval(
            Box::new(Expression::Value(Value::Str(r"(1 + 3)".to_string())))
        )
    );
}

#[test]
fn test_eval_add_strings() {
    let l = grammar::expression(r#"{("(1 " + "+ 3)")}"#).unwrap();
    assert_eq!(
        l,
        Expression::Eval(
            Box::new(Expression::OperatorExpression(
                Box::new(Expression::Value(Value::Str("(1 ".to_string()))),
                Operator::Add,
                Box::new(Expression::Value(Value::Str("+ 3)".to_string()))),
            ))
        )
    );
}

#[test]
fn test_eval_add_string_to_variable() {
    let l = grammar::expression(r#"{("(1 " + x)}"#).unwrap();
    assert_eq!(
        l,
        Expression::Eval(
            Box::new(Expression::OperatorExpression(
                Box::new(Expression::Value(Value::Str("(1 ".to_string()))),
                Operator::Add,
                Box::new(Expression::Variable("x".to_string())),
            ))
        )
    );
}
