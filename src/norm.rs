//! Bounds normalizer.
//!
//! # Normalization and unnormalization
//!
//! At start and end [`Bound<&T>`]
//! (where `T` requires [`HasLimits`] and [`HasNexts`]).
//!
//! Each bounds are converted with following methods.
//!
//! |                     | Normalize          | Unnormalize        |
//! | --                  | --                 | --                 |
//! | **Unbounded start** | [`HasLimits::MIN`] | [`HasLimits::MIN`] |
//! | **Unbounded end**   | [`HasLimits::MAX`] | [`HasLimits::MAX`] |
//! | **Included start**  | original value     | [`HasNexts::prev`] |
//! | **Included end**    | [`HasNexts::next`] | original value     |
//! | **Excluded start**  | [`HasNexts::next`] | original value     |
//! | **Excluded end**    | original value     | [`HasNexts::prev`] |

use crate::shorthands::aliases::*;
use crate::*;
use core::ops::Bound;

/// [Normalize][nu] start bound.
///
/// If result is overflowed, returns [`None`].
/// 
/// [nu]: crate::norm#normalization-and-unnormalization
///
/// # Examples
///
/// ```
/// use std::ops::Bound::*;
/// use rich_range::norm::sb_to_head;
///
/// let r = sb_to_head(Excluded(&10));
/// assert_eq!(r, Some(11));
/// ```
pub fn sb_to_head<T>(value: Bound<&T>) -> Option<T>
where
    T: Clone + HasLimits + HasNexts,
{
    Some(match value {
        Ub => T::MIN.clone(),
        In(x) => x.clone(),
        Ex(x) => T::next(x)?,
    })
}

/// [Unnormalize][nu] start bound.
///
/// If result is overflowed, returns [`None`].
/// 
/// [nu]: crate::norm#normalization-and-unnormalization
///
/// # Examples
///
/// ```
/// use std::ops::Bound::*;
/// use rich_range::norm::sb_to_prev;
///
/// let r = sb_to_prev(Included(&10));
/// assert_eq!(r, Some(9));
/// ```
pub fn sb_to_prev<T>(value: Bound<&T>) -> Option<T>
where
    T: Clone + HasLimits + HasNexts,
{
    Some(match value {
        Ub => T::MIN.clone(),
        In(x) => T::prev(x)?,
        Ex(x) => x.clone(),
    })
}

/// [Normalize][nu] end bound.
///
/// If result is overflowed, returns [`None`].
/// 
/// [nu]: crate::norm#normalization-and-unnormalization
///
/// # Examples
///
/// ```
/// use std::ops::Bound::*;
/// use rich_range::norm::eb_to_next;
///
/// let r = eb_to_next(Included(&10));
/// assert_eq!(r, Some(11));
/// ```
pub fn eb_to_next<T>(value: Bound<&T>) -> Option<T>
where
    T: Clone + HasLimits + HasNexts,
{
    Some(match value {
        Ub => T::MAX.clone(),
        In(x) => T::next(x)?,
        Ex(x) => x.clone(),
    })
}

/// [Unnormalize][nu] end bound.
///
/// If result is overflowed, returns [`None`].
///
/// [nu]: crate::norm#normalization-and-unnormalization
/// 
/// # Examples
///
/// ```
/// use std::ops::Bound::*;
/// use rich_range::norm::eb_to_tail;
///
/// let r = eb_to_tail(Excluded(&10));
/// assert_eq!(r, Some(9));
/// ```
pub fn eb_to_tail<T>(value: Bound<&T>) -> Option<T>
where
    T: Clone + HasLimits + HasNexts,
{
    Some(match value {
        Ub => T::MAX.clone(),
        In(x) => x.clone(),
        Ex(x) => T::prev(x)?,
    })
}
