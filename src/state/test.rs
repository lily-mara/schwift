pub use super::State;
pub use super::super::statement::StatementKind as Kind;
pub use super::super::statement::Statement;
pub use super::super::value::Value;
pub use super::super::expression::Expression as Exp;
pub use super::super::error::ErrorKind as EKind;

describe! state {
    before_each {
        let mut state = State::new();
    }

    it "should add symbol to table when processing assignment statement" {
        let statement = Statement::tnew(Kind::assignment("x", 10));
        state.execute(&statement).unwrap();
        assert_eq!(state.symbols.get("x"), Some(&(Value::new(10))));
    }

    it "should remove symbol from table when processing deletion statement" {
        let statement = Statement::tnew(Kind::assignment("x", 10));
        state.execute(&statement).unwrap();
        assert_eq!(state.symbols.get("x"), Some(&(Value::new(10))));

        let delete = Statement::tnew(Kind::delete("x"));
        state.execute(&delete).unwrap();
        assert_eq!(state.symbols.get("x"), None);
    }

    describe! get {
        it "should return value if it is present" {
            state.assign("x".to_string(), &Exp::new(10)).unwrap();
            assert_eq!(*state.get("x").unwrap(), Value::new(10));
        }

        it "should return undefined error if not present" {
            assert_eq!(state.get("x"), Err(EKind::UnknownVariable("x".to_string())));
        }
    }
}
