
use std::fmt;
use core::Matcher;

pub struct BeSome<E> {
    expected: Option<E>,
}

pub fn be_some<E>() -> BeSome<E> {
    BeSome {
        expected: None,
    }
}

impl<E> BeSome<E> {
    pub fn value(mut self, v: E) -> BeSome<E> {
        self.expected = Some(v);
        self
    }
}

impl<A, E> Matcher<Option<A>, Option<E>> for BeSome<E>
    where
        A: PartialEq<E> + fmt::Debug,
        E: fmt::Debug {

    fn failure_message(&self, join: &'static str, actual: &Option<A>) -> String {
        if self.expected.is_none() {
            format!("expected {} be Some, got <{:?}>", join, actual)
        } else {
            format!("expected {} be equal to <{:?}>, got <{:?}>",
                join, self.expected, actual)
        }
    }

    fn matches(&self, actual: &Option<A>) -> bool {
        if let Some(ref expected) = self.expected {
            if let Some(ref a) = *actual {
                a == expected
            } else {
                false
            }
        } else {
            actual.is_some()
        }
    }
}

pub struct BeNone;

pub fn be_none() -> BeNone {
    BeNone
}

impl<A> Matcher<Option<A>, Option<A>> for BeNone
    where
        A: fmt::Debug {

    fn failure_message(&self, join: &'static str, actual: &Option<A>) -> String {
        format!("expected {} be None, got <{:?}>", join, actual)
    }

    fn matches(&self, actual: &Option<A>) -> bool {
        actual.is_none()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::Matcher;

    #[test]
    fn be_some_value_matches_some_value() {
        assert!(be_some().value(5).matches(&Some(5)));
    }

    #[test]
    fn be_some_matches_some_value() {
        assert!(be_some().matches(&Some(5)));
    }

    #[test]
    fn be_some_failure_message() {
        let message = be_some().failure_message("to", &None::<u8>);
        assert!(message == "expected to be Some, got <None>");
    }

    #[test]
    fn be_some_value_failure_message() {
        let message = be_some().value(1).failure_message("to", &None::<u8>);
        assert!(message == "expected to be equal to <Some(1)>, got <None>");
    }

    #[test]
    #[should_panic]
    fn be_some_value_matches_some_value_should_panic() {
        assert!(be_some().value(5).matches(&Some(4)));
    }

    #[test]
    #[should_panic]
    fn be_some_value_matches_none_should_panic() {
        assert!(be_some().value(5).matches(&None::<u8>));
    }

    #[test]
    #[should_panic]
    fn be_some_matches_none_should_panic() {
        assert!(be_some().matches(&None::<u8>));
    }

    #[test]
    fn be_none_matches_none() {
        assert!(be_none().matches(&None::<u8>));
    }

    #[test]
    fn be_none_failure_message() {
        let message = be_none().failure_message("to", &Some(2));
        assert!(message == "expected to be None, got <Some(2)>");
    }

    #[test]
    #[should_panic]
    fn be_none_matches_some_should_panic() {
        assert!(be_none().matches(&Some(6)));
    }
}
