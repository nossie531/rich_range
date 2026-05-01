//! Provider of [`CutMode`].

use crate::{parts::bound, shorthands::aliases::*};
use core::ops::{Bound, RangeBounds};

/// Cut mode.
///
/// This value is used by the [`RichRangeBounds::cut_adv`] method.
/// In the methods, this value specifies how to handle the bound
/// variants of range cut ends.
///
/// [`RichRangeBounds::cut_adv`]: crate::RichRangeBounds::cut_adv
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum CutMode {
    /// Target range bounds first, forward rule fallback.
    ///
    /// Follows target range bounds variants; if they are unbounded,
    /// start bound is included, end bound is excluded.
    FallbackFw,

    /// Target range bounds first, forward rule fallback.
    ///
    /// Follows target range bounds variants; if they are unbounded,
    /// start bound is excluded, end bound is included.
    FallbackBw,

    /// Target range bounds first, forward rule fallback.
    ///
    /// Follows target range bounds variants; if they are unbounded,
    /// start bound and end bound are both included.
    FallbackIn,

    /// Target range bounds first, forward rule fallback.
    ///
    /// Follows target range bounds variants; if they are unbounded,
    /// start bound and end bound are both excluded.
    FallbackEx,

    /// Start bound is included, end bound is excluded.
    AlwaysFw,

    /// Start bound is excluded, end bound is included.
    AlwaysBw,

    /// Start bound and end bound are both included.
    AlwaysIn,

    /// Start bound and end bound are both excluded.
    AlwaysEx,
}

impl CutMode {
    /// Returns bound for start side.
    pub(crate) fn for_start<R, T>(&self, target: &R, pos: T) -> Bound<T>
    where
        R: RangeBounds<T>,
    {
        match self {
            CutMode::FallbackFw => bound(target.start_bound()).map_pos(In, pos),
            CutMode::FallbackBw => bound(target.start_bound()).map_pos(Ex, pos),
            CutMode::FallbackIn => bound(target.start_bound()).map_pos(In, pos),
            CutMode::FallbackEx => bound(target.start_bound()).map_pos(Ex, pos),
            CutMode::AlwaysFw => In(pos),
            CutMode::AlwaysBw => Ex(pos),
            CutMode::AlwaysIn => In(pos),
            CutMode::AlwaysEx => Ex(pos),
        }
    }

    /// Returns bound for end side.
    pub(crate) fn for_end<R, T>(&self, target: &R, pos: T) -> Bound<T>
    where
        R: RangeBounds<T>,
    {
        match self {
            CutMode::FallbackFw => bound(target.end_bound()).map_pos(Ex, pos),
            CutMode::FallbackBw => bound(target.end_bound()).map_pos(In, pos),
            CutMode::FallbackIn => bound(target.end_bound()).map_pos(In, pos),
            CutMode::FallbackEx => bound(target.end_bound()).map_pos(Ex, pos),
            CutMode::AlwaysFw => Ex(pos),
            CutMode::AlwaysBw => In(pos),
            CutMode::AlwaysIn => In(pos),
            CutMode::AlwaysEx => Ex(pos),
        }
    }
}
