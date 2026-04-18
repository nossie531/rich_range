//! Provider of [`CutMode`].

use crate::shorthands::aliases::*;
use core::ops::Bound;

/// Cut mode.
///
/// This value is used by the [`RichRangeBounds::cut`] method and
/// its family. In the methods, this value specifies how to handle
/// the bound variants of range cut ends.
///
/// [`RichRangeBounds::cut`]: crate::RichRangeBounds::cut
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum CutMode {
    /// Start bound is Included, End bound is Excluded.
    Standard,

    /// Start bound is Excluded, End bound is Included.
    Backward,

    /// Start and end bound is both Included.
    Included,

    /// Start and end bound is both Excluded.
    Excluded,
}

impl CutMode {
    /// Returns bound for start side.
    #[must_use]
    pub fn for_start<T>(&self, pos: T) -> Bound<T> {
        match self {
            CutMode::Standard => In(pos),
            CutMode::Backward => Ex(pos),
            CutMode::Included => In(pos),
            CutMode::Excluded => Ex(pos),
        }
    }

    /// Returns bound for end side.
    #[must_use]
    pub fn for_end<T>(&self, pos: T) -> Bound<T> {
        match self {
            CutMode::Standard => Ex(pos),
            CutMode::Backward => In(pos),
            CutMode::Included => In(pos),
            CutMode::Excluded => Ex(pos),
        }
    }
}
