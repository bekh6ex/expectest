
//! A module contains matchers.

pub use self::partial_eq::{ be_equal_to, BeEqualTo };
pub use self::partial_ord::{
    be_less_than, BeLessThan,
    be_less_or_equal_to, BeLessOrEqualTo,
    be_greater_than, BeGreaterThan,
    be_greater_or_equal_to, BeGreaterOrEqualTo,
};
pub use self::bool::{ be_true, BeTrue, be_false, BeFalse };
pub use self::option::{ be_some, BeSome, be_none, BeNone };
pub use self::result::{ be_ok, BeOk };
pub use self::floats::{ be_close_to, BeCloseTo };
pub use self::empty::{ be_empty, BeEmpty };

mod partial_eq;
mod partial_ord;
mod bool;
mod option;
mod result;
mod floats;
mod empty;
