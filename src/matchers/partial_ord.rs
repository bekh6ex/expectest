
use std::fmt;
use core::{Matcher, Join};

/// A matcher for types that conforms to `PartialOrd` trait.
pub struct PartialOrder<E> {
    expected: E,
    order: Order,
}

impl<E> PartialOrder<E> {
    fn new(expected: E, order: Order) -> PartialOrder<E> {
        PartialOrder { expected: expected, order: order }
    }
}

impl<A, E> Matcher<A, E> for PartialOrder<E>
    where
        A: PartialOrd<E> + fmt::Debug,
        E: fmt::Debug {

    fn failure_message(&self, join: Join, actual: &A) -> String {
        format!("expected {} be {} <{:?}>, got <{:?}>",
            join, self.order, self.expected, actual)
    }

    fn matches(&self, actual: &A) -> bool {
        match self.order {
            Order::LessThan => *actual < self.expected,
            Order::LessOrEqualTo => *actual <= self.expected,
            Order::GreaterThan => *actual > self.expected,
            Order::GreaterOrEqualTo => *actual >= self.expected,
        }
    }
}

/// Returns new `PartialOrder` (less than) matcher.
pub fn be_less_than<E>(expected: E) -> PartialOrder<E> {
    PartialOrder::new(expected, Order::LessThan)
}

/// Returns new `PartialOrder` (less or equal to) matcher.
pub fn be_less_or_equal_to<E>(expected: E) -> PartialOrder<E> {
    PartialOrder::new(expected, Order::LessOrEqualTo)
}

//// Returns new `PartialOrder` (greater than) matcher.
pub fn be_greater_than<E>(expected: E) -> PartialOrder<E> {
    PartialOrder::new(expected, Order::GreaterThan)
}

/// Returns new `PartialOrder` (greater or equal to) matcher.
pub fn be_greater_or_equal_to<E>(expected: E) -> PartialOrder<E> {
    PartialOrder::new(expected, Order::GreaterOrEqualTo)
}

enum Order {
    LessThan,
    LessOrEqualTo,
    GreaterThan,
    GreaterOrEqualTo,
}

impl fmt::Display for Order {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let order = match self {
            &Order::LessThan => "less than",
            &Order::LessOrEqualTo => "less or equal to",
            &Order::GreaterThan => "greater than",
            &Order::GreaterOrEqualTo => "greater or equal to",
        };
        fmt.write_str(order)
    }
}

#[cfg(test)]
mod tests {
    use super::{be_less_than, be_less_or_equal_to, be_greater_than, be_greater_or_equal_to};
    use core::{Matcher, Join};

    #[test]
    fn be_less_than_one_matches() {
        assert!(be_less_than(1).matches(&0));
    }

    #[test]
    #[should_panic]
    fn be_less_than_one_should_panic() {
        assert!(be_less_than(1).matches(&1));
    }

    #[test]
    fn be_less_than_one_failure_message() {
        let m = be_less_than(1).failure_message(Join::To, &1);
        assert!(m == "expected to be less than <1>, got <1>");
    }

    #[test]
    fn be_less_or_equal_to_one_matches() {
        assert!(be_less_or_equal_to(1).matches(&1));
    }

    #[test]
    fn be_less_or_equal_to_one_matches_zero() {
        assert!(be_less_or_equal_to(1).matches(&0));
    }

    #[test]
    #[should_panic]
    fn be_less_or_equal_to_one_should_panic() {
        assert!(be_less_or_equal_to(1).matches(&2));
    }

    #[test]
    fn be_less_or_equal_to_one_failure_message() {
        let m = be_less_or_equal_to(1).failure_message(Join::To, &2);
        assert!(m == "expected to be less or equal to <1>, got <2>");
    }

    #[test]
    fn be_greater_than_zero_matches() {
        assert!(be_greater_than(0).matches(&1));
    }

    #[test]
    #[should_panic]
    fn be_greater_than_zero_should_panic() {
        assert!(be_greater_than(0).matches(&0));
    }

    #[test]
    fn be_greater_than_zero_failure_message() {
        let m = be_greater_than(0).failure_message(Join::To, &0);
        assert!(m == "expected to be greater than <0>, got <0>");
    }

    #[test]
    fn be_greater_or_equal_to_zero_matches() {
        assert!(be_greater_or_equal_to(0).matches(&0));
    }

    #[test]
    fn be_greater_or_equal_to_zero_matches_one() {
        assert!(be_greater_or_equal_to(0).matches(&1));
    }

    #[test]
    #[should_panic]
    fn be_greater_or_equal_to_zero_should_panic() {
        assert!(be_greater_or_equal_to(0).matches(&-1));
    }

    #[test]
    fn be_greater_or_equal_to_zero_failure_message() {
        let m = be_greater_or_equal_to(0).failure_message(Join::To, &-1);
        assert!(m == "expected to be greater or equal to <0>, got <-1>");
    }
}
