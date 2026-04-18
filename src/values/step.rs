//! Provider of [`Step`].

use crate::*;

/// Substitute for nightly only [`core::iter::Step`].
///
/// See original trait [document][doc] for details.
///
/// [doc]: core::iter::Step
pub trait Step: Clone + PartialOrd + Sized {
    /// Returns the bounds on the number of *successor* steps required
    /// to get from `start` to `end` like [`Iterator::size_hint()`].
    ///
    /// See original method [document][doc] for details.
    ///
    /// [doc]: core::iter::Step::steps_between
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>);

    /// Returns the value that would be obtained by
    /// taking the *successor* of `self` `count` times.
    ///
    /// See original method [document][doc] for details.
    ///
    /// [doc]: core::iter::Step::forward_checked
    fn forward_checked(start: Self, count: usize) -> Option<Self>;

    /// Returns the value that would be obtained by
    /// taking the *predecessor* of `self` `count` times.
    ///
    /// See original method [document][doc] for details.
    ///
    /// [doc]: core::iter::Step::backward_checked
    fn backward_checked(start: Self, count: usize) -> Option<Self>;

    /// Returns the value that would be obtained by
    /// taking the *successor* of `self` `count` times.
    ///
    /// See original method [document][doc] for details.
    ///
    /// [doc]: core::iter::Step::forward
    fn forward(start: Self, count: usize) -> Self {
        Step::forward_checked(start, count).expect(msg::NO_OVF)
    }

    /// Returns the value that would be obtained by
    /// taking the *predecessor* of `self` `count` times.
    ///
    /// See original method [document][doc] for details.
    ///
    /// [doc]: core::iter::Step::backward
    fn backward(start: Self, count: usize) -> Self {
        Step::backward_checked(start, count).expect(msg::NO_OVF)
    }
}

/// Implement [`Step`] for integer type.
macro_rules! impl_step_for_int {
    ($ty:ty as $mode:ident) => {
        impl Step for $ty {
            fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
                if *start > *end {
                    (0, Some(0))
                } else {
                    let steps = usize::try_from(end.abs_diff(*start)).ok();
                    (steps.unwrap_or(usize::MAX), steps)
                }
            }

            fn forward_checked(start: Self, count: usize) -> Option<Self> {
                forward_checked!(start, count.try_into().ok()?, $mode)
            }

            fn backward_checked(start: Self, count: usize) -> Option<Self> {
                backward_checked!(start, count.try_into().ok()?, $mode)
            }
        }
    };
}

/// Implementation of [`Step::forward_checked`].
macro_rules! forward_checked {
    ($x:expr, $y:expr, signed) => {
        $x.checked_add_unsigned($y)
    };
    ($x:expr, $y:expr, unsigned) => {
        $x.checked_add($y)
    };
}

/// Implementation of [`Step::backward_checked`].
macro_rules! backward_checked {
    ($x:expr, $y:expr, signed) => {
        $x.checked_sub_unsigned($y)
    };
    ($x:expr, $y:expr, unsigned) => {
        $x.checked_sub($y)
    };
}

impl_step_for_int!(i8 as signed);
impl_step_for_int!(u8 as unsigned);
impl_step_for_int!(i16 as signed);
impl_step_for_int!(u16 as unsigned);
impl_step_for_int!(i32 as signed);
impl_step_for_int!(u32 as unsigned);
impl_step_for_int!(i64 as signed);
impl_step_for_int!(u64 as unsigned);
impl_step_for_int!(i128 as signed);
impl_step_for_int!(u128 as unsigned);
impl_step_for_int!(isize as signed);
impl_step_for_int!(usize as unsigned);
