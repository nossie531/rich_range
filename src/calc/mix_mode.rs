//! Provider of [`MixMode`].

use crate::parts::*;
use crate::*;
use core::ops::Bound;

/// Mix edge mode.
///
/// This enum is used at min / max operation of two edges. At these operation,
/// The position of the result can be defined by single rule. But about bound
/// variant Included / Excluded is depends on this mode.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum MixMode {
    /// For normal case.
    ///
    /// Result variant becomes that of the selected boundary.
    Normal,

    /// For range difference.
    ///
    /// If second edge position is selected, result is fliped its
    /// Included or Excluded. Otherwise first edge bound is selected.
    Diff,

    /// For range union.
    ///
    /// If both edges have same position and both bounds variants
    /// are either Included or Excluded, Included takes precedence.
    Union,
}

impl MixMode {
    /// Returns the minimum bound of two edges.
    pub fn min_bound<'a, T>(&self, x: Edge<&'a T>, y: Edge<&'a T>) -> Bound<&'a T>
    where
        T: PartialOrd,
    {
        let cmp = x.partial_cmp(&y).expect(msg::ORDERED);
        let val = if cmp.is_le() { x } else { y };
        let inc = match self {
            MixMode::Diff if cmp.is_ge() => !y.is_included(),
            MixMode::Union if cmp.is_eq() => x.is_included() || y.is_included(),
            _ => val.is_included(),
        };

        val.with_included(inc).bound()
    }

    /// Returns the maximum bound of two edges.
    pub fn max_bound<'a, T>(&self, x: Edge<&'a T>, y: Edge<&'a T>) -> Bound<&'a T>
    where
        T: PartialOrd,
    {
        let cmp = x.partial_cmp(&y).expect(msg::ORDERED);
        let val = if cmp.is_ge() { x } else { y };
        let inc = match self {
            MixMode::Diff if cmp.is_le() => !y.is_included(),
            MixMode::Union if cmp.is_eq() => x.is_included() || y.is_included(),
            _ => val.is_included(),
        };

        val.with_included(inc).bound()
    }
}
