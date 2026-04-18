//! Provider of [`CursorMode`].

use crate::shorthands::*;
use core::ops::RangeBounds;

/// Cursor mode.
///
/// This value is used by following methods and its family.
///
/// - [`RichRangeBounds::diff_adv`]
/// - [`RichRangeBounds::flip_adv`]
/// - [`RichRangeBounds::interval_adv`]
///
/// In those methods, this value specifies whether the [cursor empty][eh]
/// behaves as a pure empty or as a position that can cut the range.
///
/// [eh]: crate::RichRangeBounds#empty-handling
/// [`RichRangeBounds::diff_adv`]: crate::RichRangeBounds::diff_adv
/// [`RichRangeBounds::flip_adv`]: crate::RichRangeBounds::flip_adv
/// [`RichRangeBounds::interval_adv`]: crate::RichRangeBounds::interval_adv
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum CursorMode {
    /// Cursor empty behaves as a pure empty.
    Off,

    /// Cursor empty behaves as a position that can cut the range.
    On,
}

impl CursorMode {
    /// Process the range based on this mode.
    ///
    /// If this mode is [`Off`](Self::Off) and the given range is empty,
    /// return [`None`]. Otherwise, wrap the given range in [`Some`] and
    /// return it.
    pub(crate) fn proc<R, T>(&self, range: R) -> Option<R>
    where
        R: RangeBounds<T>,
        T: PartialOrd,
    {
        match self {
            CursorMode::Off => rw::new(range).into_option().map(|x| x.0),
            CursorMode::On => Some(range),
        }
    }
}
