//! Provider of [`BoundsUnfit`].

use crate::parts::*;
use core::any;
use core::ops::Bound;

/// Bounds to range conversion error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoundsUnfit {
    /// Error at new range.
    NewRange(NewRangeErr),
    /// Error at new broken range.
    NewBroken(NewBrokenErr),
}

impl BoundsUnfit {
    /// Create a new error with range creation info.
    pub fn new<R, T>(bounds: &(Bound<T>, Bound<T>)) -> Self {
        Self::NewRange(NewRangeErr {
            to_type: any::type_name::<R>(),
            from_sb: bound(bounds.0.as_ref()).kind(),
            from_eb: bound(bounds.1.as_ref()).kind(),
        })
    }

    /// Create a new error with broken range creation info.
    pub fn new_broken<R>() -> Self {
        Self::NewBroken(NewBrokenErr {
            to_type: any::type_name::<R>(),
        })
    }
}

/// Error at new range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NewRangeErr {
    /// The name of the target type.
    to_type: &'static str,
    /// Start bound state of source range.
    from_sb: Option<bool>,
    /// End bound state of source range.
    from_eb: Option<bool>,
}

/// Error at new broken range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NewBrokenErr {
    /// The name of the target type.
    to_type: &'static str,
}
