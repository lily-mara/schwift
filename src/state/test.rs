pub use super::State;
pub use super::super::statement::StatementKind as Kind;
pub use super::super::statement::Statement;
pub use super::super::value::Value;
pub use super::super::expression::Expression;
pub use super::super::error::ErrorKind as EKind;

pub fn exp<F>(x: F) -> Expression
    where F: Into<Value>
{
    x.into().into()
}

describe! state {
    before_each {
        let mut state = State::new();
    }

    it "should add symbol to table when processing assignment statement" {
        let statement = Statement::tnew(Kind::assignment("x", 10));
        state.execute(&statement).unwrap();
        assert_eq!(state.symbols.get("x"), Some(&(10.into())));
    }

    it "should remove symbol from table when processing deletion statement" {
        let statement = Statement::tnew(Kind::assignment("x", 10));
        state.execute(&statement).unwrap();
        assert_eq!(state.symbols.get("x"), Some(&(10.into())));

        let delete = Statement::tnew(Kind::delete("x"));
        state.execute(&delete).unwrap();
        assert_eq!(state.symbols.get("x"), None);
    }

    describe! get {
        it "should return value if it is present" {
            state.assign("x".to_string(), &exp(10)).unwrap();
            assert_eq!(state.get("x").unwrap(), 10.into());
        }

        it "should return undefined error if not present" {
            assert_eq!(state.get("x"), Err(EKind::UnknownVariable("x".to_string())));
        }
    }
}
