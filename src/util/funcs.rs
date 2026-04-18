//! Utility functions.

use crate::shorthands::aliases::*;
use crate::util::*;
use crate::*;
use core::ops::{Bound, Sub};

/// Returns a zero value by using subtraction operation.
#[inline]
#[allow(clippy::eq_op)]
pub(crate) fn zero_from<T>(x: &T) -> T
where
    for<'a> &'a T: Sub<&'a T, Output = T>,
{
    x - x
}

/// Returns given value with checking.
///
/// # Panics
///
/// Panics if inside option is [`None`] with given message.
#[track_caller]
pub(crate) fn expect_inside<T>(x: Option<Option<T>>, msg: &str) -> Option<T> {
    match x {
        None => None,
        Some(None) => panic!("{}", msg),
        Some(Some(x)) => Some(x),
    }
}

/// Returns `true` if all values have total order.
pub(crate) fn is_ordered<T, const N: usize>(values: &[T; N]) -> bool
where
    T: PartialOrd,
{
    for (i, e1) in values.iter().enumerate().take(values.len() - 1) {
        for e2 in values.iter().skip(i + 1) {
            if e1.partial_cmp(e2).is_none() {
                return false;
            }
        }
    }

    true
}

/// Returns min of two partial ordered values.
///
/// # Panics
///
/// Panics if `x` and `y` is unordered.
pub(crate) fn choose_min<T>(x: T, y: T) -> T
where
    T: PartialOrd,
{
    let x_win = x.partial_cmp(&y).expect(msg::ORDERED).is_le();
    if x_win { x } else { y }
}

/// Returns max of two partial ordered values.
///
/// # Panics
///
/// Panics if `x` and `y` is unordered.
pub(crate) fn choose_max<T>(x: T, y: T) -> T
where
    T: PartialOrd,
{
    let x_win = x.partial_cmp(&y).expect(msg::ORDERED).is_ge();
    if x_win { x } else { y }
}

/// Returns length between two bounds with type `T`.
///
/// If result is overflowed, returns [`None`].
pub(crate) fn len_between<T>(start: Bound<&T>, end: Bound<&T>) -> Option<T>
where
    T: Step,
    for<'x> &'x T: Sub<&'x T, Output = T>,
{
    match (start, end) {
        (_, Ub) | (Ub, _) => None,
        (In(s), Ex(e)) => Some(if s <= e { e } else { s } - s),
        (Ex(s), In(e)) => Some(if s <= e { e } else { s } - s),
        (In(s), In(e)) if s > e => Some(funcs::zero_from(s)),
        (Ex(s), Ex(e)) if s >= e => Some(funcs::zero_from(s)),
        (In(s), In(e)) => HasNexts::next(&(e - s)),
        (Ex(s), Ex(e)) => HasNexts::prev(&(e - s)),
    }
}

/// Returns length between two bounds with type [`usize`].
///
/// If result is overflowed, returns [`None`].
pub(crate) fn len_usize_between<T>(start: Bound<&T>, end: Bound<&T>) -> Option<usize>
where
    T: Step,
{
    let sb = <T as Step>::steps_between;
    match (start, end) {
        (_, Ub) | (Ub, _) => None,
        (In(s), Ex(e)) => sb(s, e).1,
        (Ex(s), In(e)) => sb(s, e).1,
        (In(s), In(e)) if s > e => Some(0),
        (Ex(s), Ex(e)) if s >= e => Some(0),
        (In(s), In(e)) => sb(s, e).1.as_ref().and_then(HasNexts::next),
        (Ex(s), Ex(e)) => sb(s, e).1.as_ref().and_then(HasNexts::prev),
    }
}

/// Switch pair values by given flag.
pub(crate) fn swap_if<T>(flag: bool, pair: (T, T)) -> (T, T) {
    if flag { (pair.1, pair.0) } else { pair }
}

/// Switch [`checked_add`] and [`checked_sub`] by given flag.
///
/// [`checked_add`]: crate::values::CheckedAdd::checked_add
/// [`checked_sub`]: crate::values::CheckedSub::checked_sub
pub(crate) fn switch_checked_add_or_sub<'x, T>(fwd: bool) -> fn(&'x T, &'x T) -> Option<T>
where
    for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
    for<'a> &'a T: CheckedSub<&'a T, Output = T>,
{
    match fwd {
        true => <&T as CheckedAdd<&T>>::checked_add,
        false => <&T as CheckedSub<&T>>::checked_sub,
    }
}

/// Switch [`forward_checked`] and [`backward_checked`] by given flag.
///
/// [`forward_checked`]: crate::values::Step::forward_checked
/// [`backward_checked`]: crate::values::Step::backward_checked
pub(crate) fn switch_checked_fwd_or_bwd<T>(fwd: bool) -> fn(T, usize) -> Option<T>
where
    T: Step,
{
    match fwd {
        true => Step::forward_checked,
        false => Step::backward_checked,
    }
}

/// Switch [`with_start_bound`] and [`with_end_bound`] by given falg.
///
/// [`with_start_bound`]: crate::RangeUniv::with_start_bound
/// [`with_end_bound`]: crate::RangeUniv::with_end_bound
pub(crate) fn switch_with_start_or_end<T>(fwd: bool) -> fn(&RangeUniv<T>, Bound<T>) -> RangeUniv<T>
where
    T: Clone,
{
    match fwd {
        true => RangeUniv::with_start_bound,
        false => RangeUniv::with_end_bound,
    }
}
