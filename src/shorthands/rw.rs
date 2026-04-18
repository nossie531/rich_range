//! Shorthand for [`RangeWrapper`].

use crate::*;
use core::ops::RangeBounds;

/// Returns a new [`RangeWrapper`] from range bounds value.
///
/// # Examples
///
/// ```
/// use rich_range::prelude::*;
/// use std::ops::Bound;
///
/// let r = rw::new(30..60);
/// assert_eq!(r.0.start, 30);
/// assert_eq!(r.0.end, 60);
/// ```
#[inline]
#[must_use]
pub fn new<R, T>(range: R) -> RangeWrapper<R, T>
where
    R: RangeBounds<T>,
{
    RangeWrapper::new(range)
}

/// Returns a new [`RangeWrapper`] from range bounds reference.
///
/// # Examples
///
/// ```
/// use rich_range::prelude::*;
/// use std::ops::Bound;
///
/// let r = rw::refr(&(30..60));
/// assert_eq!(&r.0.start, &30);
/// assert_eq!(&r.0.end, &60);
/// ```
#[inline]
#[must_use]
pub fn refr<R, T>(range: &R) -> &RangeWrapper<R, T>
where
    R: RangeBounds<T>,
{
    RangeWrapper::from_ref(range)
}
