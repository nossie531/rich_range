//! Aliases.

#![doc(hidden)]

use crate::RangeWrapper;
use core::ops::Bound;
use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

pub(crate) use core::ops::Bound::Excluded as Ex;
pub(crate) use core::ops::Bound::Included as In;
pub(crate) use core::ops::Bound::Unbounded as Ub;

pub(crate) type RangeNormalWrapper<T> = RangeWrapper<Range<T>, T>;
pub(crate) type RangeFromWrapper<T> = RangeWrapper<RangeFrom<T>, T>;
pub(crate) type RangeToWrapper<T> = RangeWrapper<RangeTo<T>, T>;
pub(crate) type RangeInclusiveWrapper<T> = RangeWrapper<RangeInclusive<T>, T>;
pub(crate) type RangeToInclusiveWrapper<T> = RangeWrapper<RangeToInclusive<T>, T>;
pub(crate) type RangeFullWrapper<T> = RangeWrapper<RangeFull, T>;
pub(crate) type RangeBoundsWrapper<T> = RangeWrapper<(Bound<T>, Bound<T>), T>;
