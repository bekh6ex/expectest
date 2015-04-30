
use core::Matcher;
use std::fmt;
use num::{ self, Float };

pub struct CloseTo<E> {
    expected: E,
    delta: E,
}

pub fn close_to<E>(expected: E) -> CloseTo<E> where E: Float {
    CloseTo {
        expected: expected,
        delta: num::traits::cast(0.001).unwrap(),
    }
}

impl<E> CloseTo<E> {
    pub fn delta(mut self, v: E) -> CloseTo<E> {
        self.delta = v;
        self
    }
}

impl<E> Matcher<E, E> for CloseTo<E>
    where
        E: Float + fmt::Debug {

    fn failure_message(&self, join: &'static str, actual: &E) -> String {
        format!("expected {} be close to <{:?}> ±{:?}, got <{:?}>",
            join, self.expected, self.delta, actual)
    }

    fn matches(&self, actual: &E) -> bool {
        if *actual == self.expected {
            true
        } else {
            (self.expected - *actual).abs() - self.delta <= E::zero()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::close_to;
    use core::Matcher;
    use num::Float;

    #[test]
    fn zero_matches_zero() {
        assert!(close_to(0.0).matches(&0.0_f32));
    }

    #[test]
    fn small_zero_matches_zero() {
        assert!(close_to(0.001_f32).matches(&0.0));
    }

    #[test]
    fn close_to_one_failure_message() {
        let message = close_to(1.0_f32).failure_message("to", &0.0);
        assert!(message == "expected to be close to <1> ±0.001, got <0>");
    }

    #[test]
    #[should_panic]
    fn big_zero_matches_zero_should_panic() {
        assert!(close_to(0.0011_f32).matches(&0.0));
    }

    #[test]
    fn zero_delta_matches_zero() {
        assert!(close_to(0.0).delta(0.1).matches(&0.0_f32));
    }

    #[test]
    fn small_zero_delta_matches_zero() {
        assert!(close_to(0.1_f32).delta(0.1).matches(&0.0));
    }

    #[test]
    fn close_to_one_delta_failure_message() {
        let message = close_to(1.0_f32).delta(0.1).failure_message("to", &0.0);
        assert!(message == "expected to be close to <1> ±0.1, got <0>");
    }

    #[test]
    #[should_panic]
    fn big_zero_delta_matches_zero_should_panic() {
        assert!(close_to(0.11_f32).delta(0.1).matches(&0.0));
    }

    #[test]
    fn infinity_matches_infinity() {
        let infinity: f32 = Float::infinity();
        assert!(close_to(infinity).matches(&Float::infinity()));
    }

    #[test]
    #[should_panic]
    fn infinity_matches_zero_should_panic() {
        let infinity: f32 = Float::infinity();
        assert!(close_to(infinity).matches(&0.0));
    }
}
