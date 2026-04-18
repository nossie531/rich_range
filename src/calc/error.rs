//! Provider of [`Error`].

use crate::conv::BoundsUnfit;

/// Range error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Error {
    /// Position of range bound is overflowed.
    Overflow,
    /// Unordered value like NaN is detected.
    Unorderd,
    /// Start side bound is required but it is unbounded.
    StartUnbounded,
    /// End side bound is required but it is unbounded.
    EndUnbounded,
    /// Bounds are unfit to range type.
    BoundsUnfit(BoundsUnfit),
}

impl From<BoundsUnfit> for Error {
    fn from(value: BoundsUnfit) -> Self {
        Self::BoundsUnfit(value)
    }
}
