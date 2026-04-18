use rich_range::prelude::*;
use std::ops::Bound;
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

pub use std::ops::Bound::Excluded as Ex;
pub use std::ops::Bound::Included as In;
pub use std::ops::Bound::Unbounded as Ub;

pub type RvNormal<'a, T> = RangeView<'a, Range<T>, T>;
pub type RwNormal<T> = RangeWrapper<Range<T>, T>;
pub type RwFrom<T> = RangeWrapper<RangeFrom<T>, T>;
pub type RwTo<T> = RangeWrapper<RangeTo<T>, T>;
pub type RwInclusive<T> = RangeWrapper<RangeInclusive<T>, T>;
pub type RwToInclusive<T> = RangeWrapper<RangeToInclusive<T>, T>;
pub type RwFull<T> = RangeWrapper<RangeFull, T>;
pub type RwBounds<T> = RangeWrapper<(Bound<T>, Bound<T>), T>;
