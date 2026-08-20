//! Provider of [`RangeParts`].

use crate::shorthands::aliases::*;
use crate::*;
use core::ops::{Bound, RangeBounds};
use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

/// Range destructuring provider.
pub trait RangeParts<T>: RangeBounds<T> {
    /// Returns range bounds.
    #[must_use]
    fn parts(self) -> (Bound<T>, Bound<T>);
}

impl<T> RangeParts<T> for Range<T> {
    #[inline]
    fn parts(self) -> (Bound<T>, Bound<T>) {
        (In(self.start), Ex(self.end))
    }
}

impl<T> RangeParts<T> for RangeFrom<T> {
    #[inline]
    fn parts(self) -> (Bound<T>, Bound<T>) {
        (In(self.start), Ub)
    }
}

impl<T> RangeParts<T> for RangeTo<T> {
    #[inline]
    fn parts(self) -> (Bound<T>, Bound<T>) {
        (Ub, Ex(self.end))
    }
}

impl<T> RangeParts<T> for RangeInclusive<T> {
    #[inline]
    fn parts(self) -> (Bound<T>, Bound<T>) {
        <[_; _]>::from(self.into_inner()).map(In).into()
    }
}

impl<T> RangeParts<T> for RangeToInclusive<T> {
    #[inline]
    fn parts(self) -> (Bound<T>, Bound<T>) {
        (Ub, In(self.end))
    }
}

impl<T> RangeParts<T> for RangeFull {
    #[inline]
    fn parts(self) -> (Bound<T>, Bound<T>) {
        (Ub, Ub)
    }
}

impl<T> RangeParts<T> for (Bound<T>, Bound<T>) {
    #[inline]
    fn parts(self) -> (Bound<T>, Bound<T>) {
        (self.0, self.1)
    }
}

impl<T> RangeParts<T> for RangeUniv<T> {
    #[inline]
    fn parts(self) -> (Bound<T>, Bound<T>) {
        (self.start, self.end)
    }
}

impl<R, T> RangeParts<T> for RangeView<'_, R, T>
where
    R: ?Sized + RangeBounds<T>,
    T: Clone,
{
    #[inline]
    fn parts(self) -> (Bound<T>, Bound<T>) {
        (self.start_bound().cloned(), self.end_bound().cloned())
    }
}

impl<R, T> RangeParts<T> for RangeWrapper<R, T>
where
    R: RangeParts<T>,
{
    #[inline]
    fn parts(self) -> (Bound<T>, Bound<T>) {
        self.0.parts()
    }
}
