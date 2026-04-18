//! Shorthand for [`RangeUniv`].

use crate::conv::*;
use crate::*;

/// Returns a new [`RangeUniv`] from range bounds.
///
/// # Examples
///
/// ```
/// use rich_range::prelude::*;
/// use std::ops::Bound::*;
///
/// let r = ru::new(30..60);
/// assert_eq!(r.start, Included(30));
/// assert_eq!(r.end, Excluded(60));
/// ```
#[inline]
#[must_use]
pub fn new<R, T>(range: R) -> RangeUniv<T>
where
    R: RangeParts<T>,
{
    let (s, e) = RangeParts::parts(range);
    RangeUniv::new(s, e)
}
