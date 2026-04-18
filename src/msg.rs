//! Messages.

/// Message for `must_use` attribute of iterator.
macro_rules! iter_must_use {
    () => {
        "Iterators are lazy and do nothing unless consumed."
    };
}

pub(crate) use iter_must_use;

/// Message that expects range bounds to be in oreder.
pub(crate) const BOUNDS_ORDERED: &str = "Range bounds should be ordered.";

/// Message that expects no overflow.
pub(crate) const NO_OVF: &str = "Overflow.";

/// Message that expects values have total order.
pub(crate) const ORDERED: &str = "Values should be ordered.";

/// Message that expects start bound.
pub(crate) const EXIST_SB: &str = "Start bound should exist.";

/// Message that expects end bound.
pub(crate) const EXIST_EB: &str = "End bound should exist.";

/// Message that expects given bound side.
pub(crate) fn exist_bound(fwd: bool) -> &'static str {
    if fwd { EXIST_SB } else { EXIST_EB }
}
