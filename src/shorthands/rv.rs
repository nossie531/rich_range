//! Shorthand for [`RangeView`].

use crate::*;
use core::ops::RangeBounds;

/// Returns a new [`RangeView`] from range bounds.
pub fn new<R, T>(range: &R) -> RangeView<'_, R, T>
where
    R: ?Sized + RangeBounds<T>,
    T: ?Sized,
{
    RangeView::new(range)
}
