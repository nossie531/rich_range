//! Shorthand for [`RangeView`].

use crate::*;
use core::ops::RangeBounds;

/// Returns a new [`RangeView`] from range bounds.
/// 
/// # Examples
///
/// ```
/// use rich_range::prelude::*;
///
/// let r = rv::new(&(30..60));
/// assert_eq!(r.0, &(30..60));
/// ```
pub fn new<R, T>(range: &R) -> RangeView<'_, R, T>
where
    R: ?Sized + RangeBounds<T>,
    T: ?Sized,
{
    RangeView::new(range)
}
