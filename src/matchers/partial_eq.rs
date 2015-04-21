
use std::fmt;
use core::Matcher;

pub struct BeEqualTo<E> {
    expected: E,
}

pub fn be_equal_to<E>(expected: E) -> BeEqualTo<E> {
    BeEqualTo {
        expected: expected,
    }
}

impl<A, E> Matcher<A, E> for BeEqualTo<E>
    where
        A: PartialEq<E> + fmt::Debug,
        E: fmt::Debug {

    fn failure_message(&self, join: &'static str, actual: &A) -> String {
        format!("expected {} be equal to <{:?}>, got <{:?}>",
            join, self.expected, actual)
    }

    fn matches(&self, actual: &A) -> bool {
        *actual == self.expected
    }
}

#[cfg(test)]
mod test {
    use super::be_equal_to;
    use core::Matcher;

    #[test]
    fn equality_of_ints() {
        assert!(be_equal_to(1).matches(&1));
    }
}
