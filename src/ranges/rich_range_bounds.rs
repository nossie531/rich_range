//! Provider of [`RichRangeBounds`].

use crate::calc::*;
use crate::conv::*;
use crate::parts::*;
use crate::shorthands::aliases::*;
use crate::shorthands::*;
use crate::util::*;
use crate::*;
use core::borrow::Borrow;
use core::ops::{Add, Bound, Range, RangeBounds, Sub};
use rustdoc_copy::prelude::*;

/// Rich version of [`RangeBounds`].
///
/// # Empty handling
///
/// This crate categorize empties range into three types.
///
/// - **Forward cursor empty**:
///   Included start and Excluded end with same position
/// - **Backward cursor empty**:
///   Excluded start and Included end with same position
/// - **Broken empty**:
///   All other empties
///
/// Cursor emptys' positions are used by some functions. For example,
/// [`includes`] method with some range and cursor empty returns `true`,
/// if the first range includes the cursor position.
///
/// Broken empties' informations (variants and positions) are not used
/// anymore in this crate. For example, [`includes`] method with some
/// range and broken empty returns always `false`, even if both ends
/// of the broken empty are included in the first range.
///
/// [`includes`]: doc_share::Self::includes
#[doc_on_only]
#[doc_share(doc_rrb)]
pub trait RichRangeBounds<T>: RangeBounds<T>
where
    T: ?Sized,
{
    /// Returns `true` if this range is empty.
    ///
    /// This is substitute for nightly only [`RangeBounds::is_empty`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert!(RichRangeBounds::is_empty(&ru::new(30..30)));
    /// assert!(RichRangeBounds::is_empty(&ru::new(60..30)));
    /// assert!(!RichRangeBounds::is_empty(&ru::new(30..60)));
    /// assert!(!RichRangeBounds::is_empty(&ru::new(30..=30)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn is_empty(&self) -> bool
    where
        T: PartialOrd,
    {
        match (self.start_bound(), self.end_bound()) {
            (Ub, _) => false,
            (_, Ub) => false,
            (In(s), Ex(e)) => s >= e,
            (Ex(s), In(e)) => s >= e,
            (Ex(s), Ex(e)) => s >= e,
            (In(s), In(e)) => s > e,
        }
    }

    /// Returns `true` if this range is [broken empty][eh].
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert!(RichRangeBounds::is_broken(&ru::new(60..30)));
    /// assert!(!RichRangeBounds::is_broken(&ru::new(30..60)));
    /// assert!(!RichRangeBounds::is_broken(&ru::new(30..30)));
    /// assert!(!RichRangeBounds::is_broken(&ru::new(30..=30)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn is_broken(&self) -> bool
    where
        T: PartialOrd,
    {
        rv::new(self).is_empty() && !self.is_cursor()
    }

    /// Returns `true` if this range is [cursor empty][eh].
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert!(RichRangeBounds::is_cursor(&ru::new(30..30)));
    /// assert!(!RichRangeBounds::is_cursor(&ru::new(30..60)));
    /// assert!(!RichRangeBounds::is_cursor(&ru::new(60..30)));
    /// assert!(!RichRangeBounds::is_cursor(&ru::new(30..=30)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn is_cursor(&self) -> bool
    where
        T: PartialEq,
    {
        self.is_cursor_fwd() || self.is_cursor_bwd()
    }

    /// Returns `true` if this range is [forward cursor empty][eh].
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    /// use std::ops::Bound::*;
    ///
    /// let target1 = ru::new((Included(30), Excluded(30)));
    /// let target2 = ru::new((Excluded(30), Included(30)));
    /// assert!(RichRangeBounds::is_cursor_fwd(&target1));
    /// assert!(!RichRangeBounds::is_cursor_fwd(&target2));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn is_cursor_fwd(&self) -> bool
    where
        T: PartialEq,
    {
        matches!((self.start_bound(), self.end_bound()), (In(s), Ex(e)) if s == e)
    }

    /// Returns `true` if this range is [backward cursor empty][eh].
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    /// use std::ops::Bound::*;
    ///
    /// let target1 = ru::new((Excluded(30), Included(30)));
    /// let target2 = ru::new((Included(30), Excluded(30)));
    /// assert!(RichRangeBounds::is_cursor_bwd(&target1));
    /// assert!(!RichRangeBounds::is_cursor_bwd(&target2));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn is_cursor_bwd(&self) -> bool
    where
        T: PartialEq,
    {
        matches!((self.start_bound(), self.end_bound()), (Ex(s), In(e)) if s == e)
    }

    /// Returns `true` if this range is point.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert!(RichRangeBounds::is_point(&ru::new(30..=30)));
    /// assert!(!RichRangeBounds::is_point(&ru::new(30..30)));
    /// assert!(!RichRangeBounds::is_point(&ru::new(30..60)));
    /// assert!(!RichRangeBounds::is_point(&ru::new(60..30)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn is_point(&self) -> bool
    where
        T: PartialEq,
    {
        matches!((self.start_bound(), self.end_bound()), (In(s), In(e)) if s == e)
    }

    /// Returns `true` if this range is wide range.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert!(RichRangeBounds::is_wide(&ru::new::<_, usize>(..)));
    /// assert!(RichRangeBounds::is_wide(&ru::new(30..)));
    /// assert!(!RichRangeBounds::is_wide(&ru::new(30..60)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn is_wide(&self) -> bool
    where
        T: PartialOrd,
    {
        matches!((self.start_bound(), self.end_bound()), (Ub, _) | (_, Ub))
    }

    /// Returns `true` if this range is full range.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert!(RichRangeBounds::is_full(&ru::new::<_, usize>(..)));
    /// assert!(!RichRangeBounds::is_full(&ru::new(30..)));
    /// assert!(!RichRangeBounds::is_full(&ru::new(30..60)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn is_full(&self) -> bool
    where
        T: PartialOrd,
    {
        matches!((self.start_bound(), self.end_bound()), (Ub, Ub))
    }

    /// Returns start edge of this range.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    /// use std::ops::Bound::*;
    ///
    /// let target = ru::new(30..60);
    /// let result = RichRangeBounds::start_edge(&target);
    /// assert_eq!(result, Edge::new(Side::S, Included(&30)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn start_edge(&self) -> Edge<&T> {
        Edge::new(Side::S, self.start_bound())
    }

    /// Returns end edge of this range.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    /// use std::ops::Bound::*;
    ///
    /// let target = ru::new(30..60);
    /// let result = RichRangeBounds::end_edge(&target);
    /// assert_eq!(result, Edge::new(Side::E, Excluded(&60)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn end_edge(&self) -> Edge<&T> {
        Edge::new(Side::E, self.end_bound())
    }

    /// Returns the head value of this range.
    ///
    /// The head value is calculated by start bound [normalization][nm].
    ///
    /// [nm]: crate::norm#normalization-and-unnormalization
    ///
    /// # Panics
    ///
    /// Panics if overflow occured.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    /// use std::ops::Bound::*;
    ///
    /// assert_eq!(RichRangeBounds::head(&ru::new(30..)), 30);
    /// assert_eq!(RichRangeBounds::head(&ru::new((Excluded(30), Unbounded))), 31);
    /// assert_eq!(RichRangeBounds::head(&ru::new(..60)), i32::MIN);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn head(&self) -> T
    where
        T: Clone + HasLimits + HasNexts,
    {
        norm::sb_to_head(self.start_bound()).expect(msg::NO_OVF)
    }

    /// Returns the tail value of this range.
    ///
    /// The tail value is calculated by end bound [unnormalization][nm].
    ///
    /// [nm]: crate::norm#normalization-and-unnormalization
    ///
    /// # Panics
    ///
    /// Panics if overflow occured.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    /// use std::ops::Bound;
    ///
    /// assert_eq!(RichRangeBounds::tail(&ru::new(30..60)), 59);
    /// assert_eq!(RichRangeBounds::tail(&ru::new(30..=60)), 60);
    /// assert_eq!(RichRangeBounds::tail(&ru::new(30..)), i32::MAX);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn tail(&self) -> T
    where
        T: Clone + HasLimits + HasNexts,
    {
        norm::eb_to_tail(self.end_bound()).expect(msg::NO_OVF)
    }

    /// Returns the previous value of this range.
    ///
    /// The previous value is calculated by start bound [unnormalization][nm].
    ///
    /// [nm]: crate::norm#normalization-and-unnormalization
    ///
    /// # Panics
    ///
    /// Panics if overflow occured.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    /// use std::ops::Bound::*;
    ///
    /// assert_eq!(RichRangeBounds::prev(&ru::new(30..)), 29);
    /// assert_eq!(RichRangeBounds::prev(&ru::new((Excluded(30), Unbounded))), 30);
    /// assert_eq!(RichRangeBounds::prev(&ru::new(..60)), i32::MIN);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn prev(&self) -> T
    where
        T: Clone + HasLimits + HasNexts,
    {
        norm::sb_to_prev(self.start_bound()).expect(msg::NO_OVF)
    }

    /// Returns the next value of this range.
    ///
    /// The next value is calculated by end bound [normalization][nm].
    ///
    /// [nm]: crate::norm#normalization-and-unnormalization
    ///
    /// # Panics
    ///
    /// Panics if overflow occured.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert_eq!(RichRangeBounds::next(&ru::new(30..60)), 60);
    /// assert_eq!(RichRangeBounds::next(&ru::new(30..=60)), 61);
    /// assert_eq!(RichRangeBounds::next(&ru::new(30..)), i32::MAX);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn next(&self) -> T
    where
        T: Clone + HasLimits + HasNexts,
    {
        norm::eb_to_next(self.end_bound()).expect(msg::NO_OVF)
    }

    /// Returns the [cursor empty][eh] position.
    ///
    /// If this range is not cursor empty, returns [`None`].
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert_eq!(RichRangeBounds::cursor(&ru::new(30..60)), None);
    /// assert_eq!(RichRangeBounds::cursor(&ru::new(30..=30)), None);
    /// assert_eq!(RichRangeBounds::cursor(&ru::new(30..30)), Some(&30));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn cursor(&self) -> Option<&T>
    where
        T: PartialEq,
    {
        if !self.is_cursor() {
            return None;
        }

        match (self.start_bound(), self.end_bound()) {
            (In(s), Ex(_)) => Some(s),
            (Ex(_), In(e)) => Some(e),
            _ => unreachable!(),
        }
    }

    /// Returns the point.
    ///
    /// If this range is not point, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert_eq!(RichRangeBounds::point(&ru::new(30..60)), None);
    /// assert_eq!(RichRangeBounds::point(&ru::new(30..30)), None);
    /// assert_eq!(RichRangeBounds::point(&ru::new(30..=30)), Some(&30));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn point(&self) -> Option<&T>
    where
        T: PartialEq,
    {
        self.is_point()
            .then(|| bound(self.start_bound()).pos().unwrap())
    }

    /// Returns the range length.
    ///
    /// If start or end is unbounded, or result length is
    /// too large for [`usize`], returns [`None`].
    ///
    /// If this range is empty, returns value like zero
    /// (This value is created end position minus end position).
    ///
    /// # Groups
    ///
    /// Following methods have similar purpose.
    ///
    /// - [`len`](Self::len) - for memory size type (ex: [`isize`], [`usize`]).
    /// - [`size`](Self::size) - for digital type (ex: [`i32`], [`u32`]).
    /// - [`width`](Self::width) - for analog type (ex: [`f32`], [`f64`]).
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert_eq!(RichRangeBounds::len(&ru::new(30..)), None);
    /// assert_eq!(RichRangeBounds::len(&ru::new(30..60)), Some(30));
    /// assert_eq!(RichRangeBounds::len(&ru::new(60..30)), Some(0));
    /// assert_eq!(RichRangeBounds::len(&ru::new(30..=60)), Some(31));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn len(&self) -> Option<usize>
    where
        T: Step,
    {
        util::len_usize_between(self.start_bound(), self.end_bound())
    }

    /// Returns the range size.
    ///
    /// If start or end is unbounded, returns [`None`].
    ///
    /// If this range is empty, returns value like zero
    /// (This value is created end position minus end position).
    ///
    /// # Panics
    ///
    /// Panics if overflow occured.
    ///
    /// # Groups
    ///
    /// Following methods have similar purpose.
    ///
    /// - [`len`](Self::len) - for memory size type (ex: [`isize`], [`usize`]).
    /// - [`size`](Self::size) - for digital type (ex: [`i32`], [`u32`]).
    /// - [`width`](Self::width) - for analog type (ex: [`f32`], [`f64`]).
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert_eq!(RichRangeBounds::size(&ru::new(30..)), None);
    /// assert_eq!(RichRangeBounds::size(&ru::new(30..60)), Some(30));
    /// assert_eq!(RichRangeBounds::size(&ru::new(60..30)), Some(0));
    /// assert_eq!(RichRangeBounds::size(&ru::new(30..=60)), Some(31));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn size(&self) -> Option<T>
    where
        T: Step,
        for<'a> &'a T: Sub<&'a T, Output = T>,
    {
        util::len_between(self.start_bound(), self.end_bound())
    }

    /// Returns the range width.
    ///
    /// If start or end is unbounded, returns [`None`].
    ///
    /// If this range is empty, returns value like zero
    /// (This value is created end position minus end position).
    ///
    /// # Notes
    ///
    /// The bound variants (Included or Excluded) are **not** considered.
    ///
    /// # Panics
    ///
    /// Panics if overflow occured.
    ///
    /// # Groups
    ///
    /// Following methods have similar purpose.
    ///
    /// - [`len`](Self::len) - for memory size type (ex: `isize`, `usize`).
    /// - [`size`](Self::size) - for digital type (ex: `i32`, `u32`).
    /// - [`width`](Self::width) - for analog type (ex: `f32`, `f64`).
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert_eq!(RichRangeBounds::width(&ru::new(30.0..)), None);
    /// assert_eq!(RichRangeBounds::width(&ru::new(30.0..60.0)), Some(30.0));
    /// assert_eq!(RichRangeBounds::width(&ru::new(60.0..30.0)), Some(0.0));
    /// assert_eq!(RichRangeBounds::width(&ru::new(30.0..=60.0)), Some(30.0));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn width(&self) -> Option<T>
    where
        T: Sized + PartialOrd,
        for<'a> &'a T: Sub<&'a T, Output = T>,
    {
        let empty = rv::new(self).is_empty();
        let s = bound(self.start_bound()).pos()?;
        let e = bound(self.end_bound()).pos()?;
        Some(e - (if empty { e } else { s }))
    }

    /// Returns both bounds of this range.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    /// use std::ops::Bound::*;
    ///
    /// let target = ru::new(30..60);
    /// let result = RichRangeBounds::bounds(&target);
    /// assert_eq!(result.0, Included(&30));
    /// assert_eq!(result.1, Excluded(&60));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn bounds(&self) -> Pair<Bound<&T>> {
        (self.start_bound(), self.end_bound())
    }

    /// Returns both edges of this range.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    /// use std::ops::Bound::*;
    ///
    /// let target = ru::new(30..60);
    /// let result = RichRangeBounds::edges(&target);
    /// assert_eq!(result.0, Edge::new(Side::S, Included(&30)));
    /// assert_eq!(result.1, Edge::new(Side::E, Excluded(&60)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn edges(&self) -> Pair<Edge<&T>> {
        (self.start_edge(), self.end_edge())
    }

    /// Converts this range to a range with reference.
    ///
    /// # Panics
    ///
    /// Panics if [custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    /// use std::ops::{Bound::*, RangeBounds};
    ///
    /// let target = ru::new(30..60);
    /// let result = RichRangeBounds::as_ref(&target);
    /// assert_eq!(result.start_bound(), Included(&30));
    /// assert_eq!(result.end_bound(), Excluded(&60));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn as_ref(&self) -> Self::Range<&T>
    where
        T: Sized,
        Self: RangeSrc<T>,
    {
        let s = self.start_bound();
        let e = self.end_bound();
        <Self as RangeSrc<_>>::new((s, e)).unwrap()
    }

    /// Casts range component type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let src = ru::new::<_, u16>(30..60);
    /// let dst = RichRangeBounds::cast::<f32>(src);
    /// assert_eq!(dst, ru::new(30.0..60.0));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn cast<U>(self) -> Self::Range<U>
    where
        T: Sized,
        U: From<T>,
        Self: Sized + RangeSrc<T> + RangeParts<T>,
    {
        self.map(U::from)
    }

    /// Tries to cast range component type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let src1 = ru::new::<_, i16>(30..60);
    /// let src2 = ru::new::<_, i16>(-30..60);
    /// let dst1 = RichRangeBounds::try_cast::<u16>(src1);
    /// let dst2 = RichRangeBounds::try_cast::<u16>(src2);
    /// assert_eq!(dst1, Some(ru::new(30..60)));
    /// assert_eq!(dst2, None);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn try_cast<U>(self) -> Option<Self::Range<U>>
    where
        T: Sized,
        U: TryFrom<T>,
        Self: Sized + RangeSrc<T> + RangeParts<T>,
    {
        self.try_map(|x| U::try_from(x).ok())
    }

    /// Returns [normalized][nm] range representation.
    ///
    /// [nm]: crate::norm#normalization-and-unnormalization
    ///
    /// # Panics
    ///
    /// Panics if any bounds of the result are overflowed.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new(30..=60);
    /// let result = RichRangeBounds::to_range(&target);
    /// assert_eq!(result, 30..61);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn to_range(&self) -> Range<T>
    where
        T: Clone,
        T: HasLimits + HasNexts,
    {
        let s = norm::sb_to_head(self.start_bound()).expect(msg::NO_OVF);
        let e = norm::eb_to_next(self.end_bound()).expect(msg::NO_OVF);
        s..e
    }

    /// Converts empty to [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target1 = ru::new(30..60);
    /// let target2 = ru::new(60..30);
    /// let result1 = RichRangeBounds::into_option(target1);
    /// let result2 = RichRangeBounds::into_option(target2);
    /// assert_eq!(result1, Some(target1));
    /// assert_eq!(result2, None);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn into_option(self) -> Option<Self>
    where
        Self: Sized,
        T: PartialOrd,
    {
        (!rv::new(&self).is_empty()).then_some(self)
    }

    /// Returns an iterator over the elements within this range.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = rw::new(3..6);
    /// let result = RichRangeBounds::iter(&target);
    /// assert!(result.eq([3, 4, 5].into_iter()));
    /// ```
    #[doc_on_only]
    fn iter(&self) -> IterRichRange<T>
    where
        T: Step,
    {
        let s = self.start_bound().cloned();
        let e = self.end_bound().cloned();
        IterRichRange::new(s, e)
    }

    /// Returns two ranges by flipping this range.
    ///
    /// This is equivalent to [`flip_adv`] with [`CursorMode::Off`].
    /// 
    /// [`flip_adv`]: Self::flip_adv
    /// 
    /// # Notes
    ///
    /// If range is empty, returns one full range.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let r1 = RichRangeBounds::flip(&ru::new::<_, usize>(..));
    /// let r2 = RichRangeBounds::flip(&ru::new(30..));
    /// let r3 = RichRangeBounds::flip(&ru::new(..60));
    /// let r4 = RichRangeBounds::flip(&ru::new(30..60));
    /// let r5 = RichRangeBounds::flip(&ru::new(30..30));
    /// assert_eq!(r1, (None, None));
    /// assert_eq!(r2, (Some(ru::new(..30)), None));
    /// assert_eq!(r3, (Some(ru::new(60..)), None));
    /// assert_eq!(r4, (Some(ru::new(..30)), Some(ru::new(60..))));
    /// assert_eq!(r5, (Some(ru::new(..)), None));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn flip(&self) -> Pair<Option<RangeUniv<T>>>
    where
        T: Clone + PartialOrd,
    {
        calc::flip(self)
    }

    /// Returns two ranges by flipping this range with advanced parameters.
    ///
    /// # Notes
    ///
    /// - If range is [broken empty][eh], returns one full range.
    /// - If range is [cursor empty][eh], result depends `mode` value.
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let r1 = RichRangeBounds::flip_adv(&ru::new::<_, usize>(..), CursorMode::Off);
    /// let r2 = RichRangeBounds::flip_adv(&ru::new(30..), CursorMode::Off);
    /// let r3 = RichRangeBounds::flip_adv(&ru::new(..60), CursorMode::Off);
    /// let r4 = RichRangeBounds::flip_adv(&ru::new(30..60), CursorMode::Off);
    /// let r5 = RichRangeBounds::flip_adv(&ru::new(30..30), CursorMode::Off);
    /// let r6 = RichRangeBounds::flip_adv(&ru::new(30..30), CursorMode::On);
    /// assert_eq!(r1, (None, None));
    /// assert_eq!(r2, (Some(ru::new(..30)), None));
    /// assert_eq!(r3, (Some(ru::new(60..)), None));
    /// assert_eq!(r4, (Some(ru::new(..30)), Some(ru::new(60..))));
    /// assert_eq!(r5, (Some(ru::new(..)), None));
    /// assert_eq!(r6, (Some(ru::new(..30)), Some(ru::new(30..))));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn flip_adv(&self, mode: CursorMode) -> Pair<Option<RangeUniv<T>>>
    where
        T: Clone + PartialOrd,
    {
        calc::flip_adv(self, mode)
    }

    /// Returns a new range with both ends subtracted by given value.
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - Position of the bound is overflowed.
    /// - [Custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new(30..60);
    /// let result = RichRangeBounds::shl(&target, 10);
    /// assert_eq!(result, ru::new(20..50));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn shl(&self, value: impl Borrow<T>) -> Self::Range<T>
    where
        T: Sized,
        Self: RangeSrc<T>,
        for<'a> &'a T: Sub<&'a T, Output = T>,
    {
        calc::shl(self, value)
    }

    /// Returns a new range with both ends subtracted by given value.
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - Position of the bound is overflowed.
    /// - [Custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new(30..60);
    /// let result = RichRangeBounds::shr(&target, 10);
    /// assert_eq!(result, ru::new(40..70));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn shr(&self, value: impl Borrow<T>) -> Self::Range<T>
    where
        T: Sized,
        Self: RangeSrc<T>,
        for<'a> &'a T: Add<&'a T, Output = T>,
    {
        calc::shr(self, value)
    }

    /// Returns a new range by adding given value to start side.
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - Position of the bound is overflowed.
    /// - [Custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new(30..60);
    /// let result = RichRangeBounds::add_start(&target, 10);
    /// assert_eq!(result, ru::new(40..60));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn add_start(&self, value: impl Borrow<T>) -> Self::Range<T>
    where
        T: Clone,
        Self: RangeSrc<T>,
        for<'a> &'a T: Add<&'a T, Output = T>,
    {
        calc::add_start(self, value)
    }

    /// Returns a new range by adding given value to end side.
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - Position of the bound is overflowed.
    /// - [Custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new(30..60);
    /// let result = RichRangeBounds::add_end(&target, 10);
    /// assert_eq!(result, ru::new(30..70));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn add_end(&self, value: impl Borrow<T>) -> Self::Range<T>
    where
        T: Clone,
        Self: RangeSrc<T>,
        for<'a> &'a T: Add<&'a T, Output = T>,
    {
        calc::add_end(self, value)
    }

    /// Returns a new range by subtracting given value to start side.
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - Position of the bound is overflowed.
    /// - [Custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new(30..60);
    /// let result = RichRangeBounds::sub_start(&target, 10);
    /// assert_eq!(result, ru::new(20..60));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn sub_start(&self, value: impl Borrow<T>) -> Self::Range<T>
    where
        T: Clone,
        Self: RangeSrc<T>,
        for<'a> &'a T: Sub<&'a T, Output = T>,
    {
        calc::sub_start(self, value)
    }

    /// Returns a new range by subtracting given value to end side.
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - Position of the bound is overflowed.
    /// - [Custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new(30..60);
    /// let result = RichRangeBounds::sub_end(&target, 10);
    /// assert_eq!(result, ru::new(30..50));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn sub_end(&self, value: impl Borrow<T>) -> Self::Range<T>
    where
        T: Clone,
        Self: RangeSrc<T>,
        for<'a> &'a T: Sub<&'a T, Output = T>,
    {
        calc::sub_end(self, value)
    }

    /// Returns a new range with start bound based on
    /// original end bound and the given width.
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - Start or end bound is unbounded.
    /// - Position of the bound is overflowed.
    /// - [Custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new(30..60);
    /// let result = RichRangeBounds::calc_start(&target, 40);
    /// assert_eq!(result, ru::new(20..60));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn calc_start(&self, width: impl Borrow<T>) -> Self::Range<T>
    where
        T: Clone,
        Self: RangeSrc<T>,
        for<'a> &'a T: Sub<&'a T, Output = T>,
    {
        calc::calc_start(self, width)
    }

    /// Returns a new range with end bound
    /// based on original start bound and the given width.
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - Start or end bound is unbounded.
    /// - Position of the bound is overflowed.
    /// - [Custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new(30..60);
    /// let result = RichRangeBounds::calc_end(&target, 40);
    /// assert_eq!(result, ru::new(30..70));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn calc_end(&self, width: impl Borrow<T>) -> Self::Range<T>
    where
        T: Clone,
        Self: RangeSrc<T>,
        for<'a> &'a T: Add<&'a T, Output = T>,
    {
        calc::calc_end(self, width)
    }

    /// Returns a new range with start bound aligned at given value.
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - The Start bound is unbounded.
    /// - Position of the bound is overflowed.
    /// - The range or the value have unordered position like NaN.
    /// - [Custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new(30..60);
    /// let result = RichRangeBounds::align_start(&target, 40);
    /// assert_eq!(result, ru::new(40..70));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn align_start(&self, value: impl Borrow<T>) -> Self::Range<T>
    where
        T: Clone + PartialOrd,
        Self: RangeSrc<T>,
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
    {
        calc::align_start(self, value).unwrap()
    }

    /// Returns a new range with end bound aligned at given value.
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - The end bound is unbounded.
    /// - Position of the bound is overflowed.
    /// - The range or the value have unordered position like NaN.
    /// - [Custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new(30..60);
    /// let result = RichRangeBounds::align_end(&target, 70);
    /// assert_eq!(result, ru::new(40..70));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn align_end(&self, value: impl Borrow<T>) -> Self::Range<T>
    where
        T: Clone + PartialOrd,
        Self: RangeSrc<T>,
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
    {
        calc::align_end(self, value).unwrap()
    }

    /// Returns a new instance with mapped position of bounds.
    ///
    /// # Panics
    ///
    /// Panics if [custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new(30..);
    /// let result = RichRangeBounds::map(target, |x| x * 2);
    /// assert_eq!(result, ru::new(60..));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn map<F, U>(self, mut f: F) -> Self::Range<U>
    where
        T: Sized,
        F: FnMut(T) -> U,
        Self: Sized + RangeSrc<T> + RangeParts<T>,
    {
        let bounds = RangeParts::parts(self);
        let s = bounds.0.map(&mut f);
        let e = bounds.1.map(&mut f);
        <Self as RangeSrc<_>>::new((s, e)).unwrap()
    }

    /// Try to returns a new instance with mapped position of bounds.
    ///
    /// # Panics
    ///
    /// Panics if [custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = || ru::new::<_, u8>(30..).clone();
    /// let result1 = RichRangeBounds::try_map(target(), |x| x.checked_mul(2));
    /// let result2 = RichRangeBounds::try_map(target(), |x| x.checked_mul(10));
    /// assert_eq!(result1, Some(ru::new(60..)));
    /// assert_eq!(result2, None);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn try_map<F, U>(self, mut f: F) -> Option<Self::Range<U>>
    where
        T: Sized,
        F: FnMut(T) -> Option<U>,
        Self: Sized + RangeSrc<T> + RangeParts<T>,
    {
        let bounds = RangeParts::parts(self);
        let s = bound(bounds.0).try_map(&mut f)?;
        let e = bound(bounds.1).try_map(&mut f)?;
        Some(<Self as RangeSrc<_>>::new((s, e)).unwrap())
    }

    /// Overflow checked version of [`shl`](Self::shl).
    ///
    /// # Panics
    ///
    /// Panics if [custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new::<_, u8>(30..60);
    /// let result1 = RichRangeBounds::checked_shl(&target, 10);
    /// let result2 = RichRangeBounds::checked_shl(&target, 40);
    /// assert_eq!(result1, Some(ru::new(20..50)));
    /// assert_eq!(result2, None);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn checked_shl(&self, value: impl Borrow<T>) -> Option<Self::Range<T>>
    where
        T: Sized,
        Self: RangeSrc<T>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
    {
        calc::checked_shl(self, value)
    }

    /// Overflow checked version of [`shr`](Self::shr).
    ///
    /// # Panics
    ///
    /// Panics if [custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new::<_, u8>(30..60);
    /// let result1 = RichRangeBounds::checked_shr(&target, 10);
    /// let result2 = RichRangeBounds::checked_shr(&target, 200);
    /// assert_eq!(result1, Some(ru::new(40..70)));
    /// assert_eq!(result2, None);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn checked_shr(&self, value: impl Borrow<T>) -> Option<Self::Range<T>>
    where
        T: Sized,
        Self: RangeSrc<T>,
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
    {
        calc::checked_shr(self, value)
    }

    /// Overflow checked version of [`add_start`](Self::add_start).
    ///
    /// # Panics
    ///
    /// Panics if [custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new::<_, u8>(30..60);
    /// let result1 = RichRangeBounds::checked_add_start(&target, 10);
    /// let result2 = RichRangeBounds::checked_add_start(&target, 250);
    /// assert_eq!(result1, Some(ru::new(40..60)));
    /// assert_eq!(result2, None);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn checked_add_start(&self, value: impl Borrow<T>) -> Option<Self::Range<T>>
    where
        T: Clone,
        Self: RangeSrc<T>,
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
    {
        calc::checked_add_start(self, value)
    }

    /// Overflow checked version of [`add_end`](Self::add_end).
    ///
    /// # Panics
    ///
    /// Panics if [custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new::<_, u8>(30..60);
    /// let result1 = RichRangeBounds::checked_add_end(&target, 10);
    /// let result2 = RichRangeBounds::checked_add_end(&target, 250);
    /// assert_eq!(result1, Some(ru::new(30..70)));
    /// assert_eq!(result2, None);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn checked_add_end(&self, value: impl Borrow<T>) -> Option<Self::Range<T>>
    where
        T: Clone,
        Self: RangeSrc<T>,
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
    {
        calc::checked_add_end(self, value)
    }

    /// Overflow checked version of [`sub_start`](Self::sub_start).
    ///
    /// # Panics
    ///
    /// Panics if [custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new::<_, u8>(30..60);
    /// let result1 = RichRangeBounds::checked_sub_start(&target, 10);
    /// let result2 = RichRangeBounds::checked_sub_start(&target, 40);
    /// assert_eq!(result1, Some(ru::new(20..60)));
    /// assert_eq!(result2, None);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn checked_sub_start(&self, value: impl Borrow<T>) -> Option<Self::Range<T>>
    where
        T: Clone,
        Self: RangeSrc<T>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
    {
        calc::checked_sub_start(self, value)
    }

    /// Overflow checked version of [`sub_end`](Self::sub_end).
    ///
    /// # Panics
    ///
    /// Panics if [custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new::<_, u8>(30..60);
    /// let result1 = RichRangeBounds::checked_sub_end(&target, 10);
    /// let result2 = RichRangeBounds::checked_sub_end(&target, 70);
    /// assert_eq!(result1, Some(ru::new(30..50)));
    /// assert_eq!(result2, None);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn checked_sub_end(&self, value: impl Borrow<T>) -> Option<Self::Range<T>>
    where
        T: Clone,
        Self: RangeSrc<T>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
    {
        calc::checked_sub_end(self, value)
    }

    /// Overflow checked version of [`calc_start`](Self::calc_start).
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - Start or end bound is unbounded.
    /// - [Custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new::<_, u8>(30..60);
    /// let r1 = RichRangeBounds::checked_calc_start(&target, 40);
    /// let r2 = RichRangeBounds::checked_calc_start(&target, 70);
    /// assert_eq!(r1, Some(ru::new(20..60)));
    /// assert_eq!(r2, None);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn checked_calc_start(&self, width: impl Borrow<T>) -> Option<Self::Range<T>>
    where
        T: Clone,
        Self: RangeSrc<T>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
    {
        calc::checked_calc_start(self, width)
    }

    /// Overflow checked version of [`calc_end`](Self::calc_end).
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - Start or end bound is unbounded.
    /// - [Custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new::<_, u8>(30..60);
    /// let r1 = RichRangeBounds::checked_calc_end(&target, 40);
    /// let r2 = RichRangeBounds::checked_calc_end(&target, 230);
    /// assert_eq!(r1, Some(ru::new(30..70)));
    /// assert_eq!(r2, None);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn checked_calc_end(&self, width: impl Borrow<T>) -> Option<Self::Range<T>>
    where
        T: Clone,
        Self: RangeSrc<T>,
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
    {
        calc::checked_calc_end(self, width)
    }

    /// Overflow checked version of [`align_start`](Self::align_start).
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - The start bound is unbounded.
    /// - The range or the value have unordered position like NaN.
    /// - [Custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new::<_, u8>(30..60);
    /// let result1 = RichRangeBounds::checked_align_start(&target, 40);
    /// let result2 = RichRangeBounds::checked_align_start(&target, 230);
    /// assert_eq!(result1, Some(ru::new(40..70)));
    /// assert_eq!(result2, None);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn checked_align_start(&self, value: impl Borrow<T>) -> Option<Self::Range<T>>
    where
        T: Clone + PartialOrd,
        Self: RangeSrc<T>,
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
    {
        let ret = calc::align_start(self, value);
        match ret {
            Err(Error::Overflow) => None,
            x => x.ok(),
        }
    }

    /// Overflow checked version of [`align_end`](Self::align_end).
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - The end bound is unbounded.
    /// - The range or the value have unordered position like NaN.
    /// - [Custom range type][crt] conversion is failed.
    ///
    /// [crt]: crate::conv::RangeSrc#about-custom-range-type
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new::<_, u8>(30..60);
    /// let result1 = RichRangeBounds::checked_align_end(&target, 70);
    /// let result2 = RichRangeBounds::checked_align_end(&target, 20);
    /// assert_eq!(result1, Some(ru::new(40..70)));
    /// assert_eq!(result2, None);
    /// ```
    #[must_use]
    #[doc_on_only]
    fn checked_align_end(&self, value: impl Borrow<T>) -> Option<Self::Range<T>>
    where
        T: Clone + PartialOrd,
        Self: RangeSrc<T>,
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
    {
        let ret = calc::align_end(self, value);
        match ret {
            Err(Error::Overflow) => None,
            x => x.ok(),
        }
    }

    /// Returns `true` if two ranges are equivalent.
    ///
    /// This method treat all empty ranges as identical
    /// (This is difference from [`eq`] method).
    ///
    /// [`eq`]: core::cmp::PartialEq::eq
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert!(RichRangeBounds::equiv(&ru::new(30..60), &(30..60)));
    /// assert!(!RichRangeBounds::equiv(&ru::new(30..60), &(30..65)));
    /// assert!(RichRangeBounds::equiv(&ru::new(30..30), &(0..0)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn equiv<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        calc::is_equiv(self, other)
    }

    /// Returns `true` if two ranges are intersect.
    ///
    /// # Notes
    ///
    /// - If ranges have unordered position like NaN, returns `false`.
    /// - If either of two ranges is [broken empty][eh], returns `false`.
    /// - If either of two ranges is [cursor empty][eh] and included
    ///   in the other, returns `true`.
    /// - If two ranges are both [cursor empties][eh] and same position,
    ///   returns `true`.
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert!(RichRangeBounds::intersects(&ru::new(30..60), &(50..70)));
    /// assert!(RichRangeBounds::intersects(&ru::new(30..60), &(50..50)));
    /// assert!(!RichRangeBounds::intersects(&ru::new(30..60), &(70..80)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn intersects<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        calc::is_intersects(self, other)
    }

    /// Returns `true` if `self` include `other`.
    ///
    /// # Notes
    ///
    /// - If ranges have unordered position like NaN, returns `false`.
    /// - If either of two ranges is [broken empty][eh], returns `false`.
    /// - If second ranges is [cursor empty][eh] and included in the
    ///   first range, returns `true`.
    /// - If two ranges are both [cursor empties][eh] and same position,
    ///   returns `true`.
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert!(RichRangeBounds::includes(&ru::new(30..60), &(40..50)));
    /// assert!(!RichRangeBounds::includes(&ru::new(30..60), &(70..80)));
    /// assert!(!RichRangeBounds::includes(&ru::new(30..60), &(60..60)));
    /// assert!(RichRangeBounds::includes(&ru::new(30..30), &(30..30)));
    /// assert!(!RichRangeBounds::includes(&ru::new(30..30), &(40..40)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn includes<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        calc::is_includes(self, other)
    }

    /// Returns `true` if `self` is included by `other`.
    ///
    /// # Notes
    ///
    /// - If ranges have unordered position like NaN, returns `false`.
    /// - If either of two ranges is [broken empty][eh], returns `false`.
    /// - If first ranges is [cursor empty][eh] and included in the
    ///   second range, returns `true`.
    /// - If two ranges are both [cursor empties][eh] and same position,
    ///   returns `true`.
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert!(RichRangeBounds::included(&ru::new(30..60), &(20..70)));
    /// assert!(!RichRangeBounds::included(&ru::new(30..60), &(40..70)));
    /// assert!(RichRangeBounds::included(&ru::new(30..30), &(30..30)));
    /// assert!(!RichRangeBounds::included(&ru::new(30..30), &(40..40)));
    /// ```    
    #[must_use]
    #[doc_on_only]
    fn included<R>(&self, other: &R) -> bool
    where
        R: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        rv::new(other).includes(self)
    }

    /// Returns `true` if two ranges are adjoining.
    ///
    /// # Notes
    ///
    /// - If ranges have unordered position like NaN, returns `false`.
    /// - If either of two ranges is [broken empty][eh], returns `false`.
    /// - Two bounds adjoins only if both are Included.
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Groups
    ///
    /// Following methods have similar purpose.
    ///
    /// - [`adjoins`](Self::adjoins) - for continuous type.
    /// - [`touches`](Self::touches) - for none continuous type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert!(RichRangeBounds::adjoins(&ru::new(30..=60), &(60..70)));
    /// assert!(!RichRangeBounds::adjoins(&ru::new(30..=60), &(70..80)));
    /// assert!(RichRangeBounds::adjoins(&ru::new(30..60), &(20..=30)));
    /// assert!(!RichRangeBounds::adjoins(&ru::new(30..60), &(10..=20)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn adjoins<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        calc::is_adjoins(self, other)
    }

    /// Returns `true` if the start bound adjoins the end bound of given range.
    ///
    /// # Notes
    ///
    /// - If ranges have unordered position like NaN, returns `false`.
    /// - If either of two ranges is [broken empty][eh], returns `false`.
    /// - Two bounds adjoins only if both are Included.
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Groups
    ///
    /// Following methods have similar purpose.
    ///
    /// - [`adjoins_prev`](Self::adjoins_prev) - for continuous type.
    /// - [`touches_prev`](Self::touches_prev) - for none continuous type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert!(RichRangeBounds::adjoins_prev(&ru::new(30..60), &(20..=30)));
    /// assert!(!RichRangeBounds::adjoins_prev(&ru::new(30..60), &(10..=20)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn adjoins_prev<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        calc::is_adjoins_prev(self, other)
    }

    /// Returns `true` if the end bound adjoins the start bound of given range.
    ///
    /// # Notes
    ///
    /// - If ranges have unordered position like NaN, returns `false`.
    /// - If either of two ranges is [broken empty][eh], returns `false`.
    /// - Two bounds adjoins only if both are Included.
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Groups
    ///
    /// Following methods have similar purpose.
    ///
    /// - [`adjoins_next`](Self::adjoins_next) - for continuous type.
    /// - [`touches_next`](Self::touches_next) - for none continuous type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert!(RichRangeBounds::adjoins_next(&ru::new(30..=60), &(60..70)));
    /// assert!(!RichRangeBounds::adjoins_next(&ru::new(30..=60), &(70..80)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn adjoins_next<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        calc::is_adjoins_next(self, other)
    }

    /// Returns `true` if two ranges are toucheing.
    ///
    /// # Notes
    ///
    /// - If ranges have unordered position like NaN, returns `false`.
    /// - If either of two ranges is [broken empty][eh], returns `false`.
    /// - Two bounds touches only if Included / Excluded pair.
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Groups
    ///
    /// Following methods have similar purpose.
    ///
    /// - [`adjoins`](Self::adjoins) - for continuous type.
    /// - [`touches`](Self::touches) - for none continuous type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert!(RichRangeBounds::touches(&ru::new(30..60), &(60..70)));
    /// assert!(!RichRangeBounds::touches(&ru::new(30..60), &(70..80)));
    /// assert!(RichRangeBounds::touches(&ru::new(30..60), &(20..30)));
    /// assert!(!RichRangeBounds::touches(&ru::new(30..60), &(10..20)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn touches<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        calc::is_touches(self, other)
    }

    /// Returns `true` if the start bound touches the end bound of given range.
    ///
    /// # Notes
    ///
    /// - If ranges have unordered position like NaN, returns `false`.
    /// - If either of two ranges is [broken empty][eh], returns `false`.
    /// - Two bounds touches only if Included / Excluded pair.
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Groups
    ///
    /// Following methods have similar purpose.
    ///
    /// - [`adjoins_prev`](Self::adjoins_prev) - for continuous type.
    /// - [`touches_prev`](Self::touches_prev) - for none continuous type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert!(RichRangeBounds::touches_prev(&ru::new(30..60), &(20..30)));
    /// assert!(!RichRangeBounds::touches_prev(&ru::new(30..60), &(10..20)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn touches_prev<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        calc::is_touches_prev(self, other)
    }

    /// Returns `true` if the end bound touches the start bound of given range.
    ///
    /// # Notes
    ///
    /// - If ranges have unordered position like NaN, returns `false`.
    /// - If either of two ranges is [broken empty][eh], returns `false`.
    /// - Two bounds touches only if Included / Excluded pair.
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Groups
    ///
    /// Following methods have similar purpose.
    ///
    /// - [`adjoins_next`](Self::adjoins_next) - for continuous type.
    /// - [`touches_next`](Self::touches_next) - for none continuous type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// assert!(RichRangeBounds::touches_next(&ru::new(30..60), &(60..70)));
    /// assert!(!RichRangeBounds::touches_next(&ru::new(30..60), &(70..80)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn touches_next<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        calc::is_touches_next(self, other)
    }

    /// Returns relation of two ranges.
    ///
    /// For more details, see [`RangeRel`] document.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new(30..60);
    /// let result = RichRangeBounds::rel(&target, &ru::new(20..70), PosStyle::Step);
    /// assert_eq!(result, RangeRel::During(true));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn rel<R2>(&self, other: &R2, ps: PosStyle) -> RangeRel
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        calc::rel(self, other, ps)
    }

    /// Return two ranges by cutting self at given position.
    ///
    /// This is equivalent to [`cut_adv`] with [`CutMode::FallbackFw`].
    /// 
    /// [`cut_adv`]: Self::cut_adv
    /// 
    /// # Notes
    ///
    /// - If this range is broken empty, returns two [`None`].
    /// - Returned 1st range is less or equal than the given position.
    /// - Returned 2nd range is greater or equal than the given position.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new(30..60);
    /// let (fst, snd) = RichRangeBounds::cut(&target, &40);
    /// assert_eq!(fst, Some(ru::new(30..40)));
    /// assert_eq!(snd, Some(ru::new(40..60)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn cut(&self, pos: &T) -> Pair<Option<RangeUniv<T>>>
    where
        T: Clone + PartialOrd,
    {
        calc::cut(self, pos, CutMode::FallbackFw)
    }

    /// Return two ranges by cutting self at given position with advanced parameter.
    ///
    /// # Notes
    ///
    /// - If this range is broken empty, returns two [`None`].
    /// - Returned 1st range is less or equal than the given position.
    /// - Returned 2nd range is greater or equal than the given position.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new(30..60);
    /// let (fst, snd) = RichRangeBounds::cut_adv(&target, &40, CutMode::FallbackFw);
    /// assert_eq!(fst, Some(ru::new(30..40)));
    /// assert_eq!(snd, Some(ru::new(40..60)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn cut_adv(&self, pos: &T, mode: CutMode) -> Pair<Option<RangeUniv<T>>>
    where
        T: Clone + PartialOrd,
    {
        calc::cut(self, pos, mode)
    }

    /// Returns the range between two ranges.
    ///
    /// # Panics
    ///
    /// Panics if ranges have unordered position like NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = ru::new(30..60);
    /// let r1 = RichRangeBounds::interval(&target, &ru::new(50..70));
    /// let r2 = RichRangeBounds::interval(&target, &ru::new(60..80));
    /// let r3 = RichRangeBounds::interval(&target, &ru::new(70..90));
    /// assert_eq!(r1, None);
    /// assert_eq!(r2, None);
    /// assert_eq!(r3, Some(ru::new(60..70)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn interval(&self, other: &Self) -> Option<RangeUniv<T>>
    where
        T: Clone + PartialOrd,
    {
        assert!(calc::is_mixable(self, other), "{}", msg::BOUNDS_ORDERED);
        let ret = calc::interval(self, other)?;
        Some(<RangeUniv<T> as RangeSrc<T>>::new_from(ret).unwrap())
    }

    /// Returns the range between two ranges with advanced parameters.
    ///
    /// # Panics
    ///
    /// Panics if ranges have unordered position like NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = rw::new(30..60);
    /// let r1 = RichRangeBounds::interval_adv(&target, &rw::new(50..70), CursorMode::Off);
    /// let r2 = RichRangeBounds::interval_adv(&target, &rw::new(60..80), CursorMode::Off);
    /// let r3 = RichRangeBounds::interval_adv(&target, &rw::new(60..80), CursorMode::On);
    /// let r4 = RichRangeBounds::interval_adv(&target, &rw::new(70..90), CursorMode::Off);
    /// assert_eq!(r1, None);
    /// assert_eq!(r2, None);
    /// assert_eq!(r3, Some(ru::new(60..60)));    
    /// assert_eq!(r4, Some(ru::new(60..70)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn interval_adv(&self, other: &Self, mode: CursorMode) -> Option<RangeUniv<T>>
    where
        T: Clone + PartialOrd,
    {
        assert!(calc::is_mixable(self, other), "{}", msg::BOUNDS_ORDERED);
        let ret = calc::interval_adv(self, other, mode)?;
        Some(<RangeUniv<T> as RangeSrc<T>>::new_from(ret).unwrap())
    }

    /// Returns the product of two ranges.
    ///
    /// # Notes
    ///
    /// - If two ranges have no intersection, returns [`None`].
    /// - If two ranges are both [cursor empties][eh] and same position,
    ///   returns the cursor empty. Here, if one is forward cursor empty
    ///   and one is backward cursor empty, The former takes precedence.
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Panics
    ///
    /// Panics if ranges have unordered position like NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let r = RichRangeBounds::prod(&ru::new(30..60), &ru::new(40..70));
    /// assert_eq!(r, Some(ru::new(40..60)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn prod(&self, other: &Self) -> Option<Self::Range<T>>
    where
        Self: RangeSrc<T>,
        T: Clone + PartialOrd,
    {
        assert!(calc::is_mixable(self, other), "{}", msg::BOUNDS_ORDERED);
        let ret = calc::prod(self, other)?;
        Some(<Self as RangeSrc<T>>::new_from(ret).unwrap())
    }

    /// Returns the superset of two ranges.
    ///
    /// # Notes
    ///
    /// - Both range is empty, returns [`None`].
    /// - One range is empty, returns the other range.
    ///
    /// # Panics
    ///
    /// Panics if ranges have unordered position like NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let r1 = RichRangeBounds::enwrap(&ru::new(20..40), &ru::new(30..50));
    /// let r2 = RichRangeBounds::enwrap(&ru::new(10..20), &ru::new(40..60));
    /// assert_eq!(r1, Some(ru::new(20..50)));
    /// assert_eq!(r2, Some(ru::new(10..60)));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn enwrap(&self, other: &Self) -> Option<Self::Range<T>>
    where
        Self: RangeSrc<T>,
        T: Clone + PartialOrd,
    {
        assert!(calc::is_mixable(self, other), "{}", msg::BOUNDS_ORDERED);
        let ret = calc::enwrap(self, other)?;
        Some(<Self as RangeSrc<T>>::new_from(ret).unwrap())
    }

    /// Returns the union of two ranges.
    ///
    /// # Notes
    ///
    /// - If either of two ranges is [broken empty][eh], returns the other.
    /// - If two ranges can not merge, returns outputs with ascending order.
    /// - If Two ranges can merge, returns the merged range
    ///   Here, one range is [forward cursor empty][eh] and the other is
    ///   [backward cursor empty][eh], the former takes precedence.
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Panics
    ///
    /// Panics if ranges have unordered position like NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let r1 = RichRangeBounds::union(&ru::new(30..60), &ru::new(40..70));
    /// let r2 = RichRangeBounds::union(&ru::new(30..60), &ru::new(70..80));
    /// assert_eq!(r1, (ru::new(30..70), None));
    /// assert_eq!(r2, (ru::new(30..60), Some(ru::new(70..80))));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn union(&self, other: &Self) -> (Self::Range<T>, Option<Self::Range<T>>)
    where
        Self: RangeSrc<T>,
        T: Clone + PartialOrd,
    {
        assert!(calc::is_mixable(self, other), "{}", msg::BOUNDS_ORDERED);
        let (fst, snd) = calc::union(self, other);
        let fst = <Self as RangeSrc<T>>::new_from(fst).unwrap();
        let snd = snd.map(|snd| <Self as RangeSrc<T>>::new_from(snd).unwrap());
        (fst, snd)
    }

    /// Returns the difference of two ranges.
    ///
    /// This is equivalent to [`diff_adv`] with [`CursorMode::Off`].
    /// 
    /// [`diff_adv`]: Self::diff_adv
    /// 
    /// # Panics
    ///
    /// Panics if ranges have unordered position like NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let r1 = RichRangeBounds::diff(&ru::new(30..60), &ru::new(50..70));
    /// let r2 = RichRangeBounds::diff(&ru::new(30..60), &ru::new(40..50));
    /// let r3 = RichRangeBounds::diff(&ru::new(30..60), &ru::new(40..40));
    /// assert_eq!(r1, (Some(ru::new(30..50)), None));
    /// assert_eq!(r2, (Some(ru::new(30..40)), Some(ru::new(50..60))));
    /// assert_eq!(r3, (Some(ru::new(30..60)), None));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn diff<R>(&self, other: &R) -> Pair<Option<RangeUniv<T>>>
    where
        R: ?Sized + RangeBounds<T>,
        T: Clone + PartialOrd,
    {
        assert!(calc::is_mixable(self, other), "{}", msg::BOUNDS_ORDERED);
        calc::diff(self, other)
    }

    /// Returns the difference of two ranges with advanced parameters.
    ///
    /// [eh]: doc_share::Self#empty-handling
    ///
    /// # Panics
    ///
    /// Panics if ranges have unordered position like NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let r1 = RichRangeBounds::diff_adv(&ru::new(30..60), &(50..70), CursorMode::Off);
    /// let r2 = RichRangeBounds::diff_adv(&ru::new(30..60), &(40..50), CursorMode::Off);
    /// let r3 = RichRangeBounds::diff_adv(&ru::new(30..60), &(40..40), CursorMode::Off);
    /// let r4 = RichRangeBounds::diff_adv(&ru::new(30..60), &(40..40), CursorMode::On);
    /// assert_eq!(r1, (Some(ru::new(30..50)), None));
    /// assert_eq!(r2, (Some(ru::new(30..40)), Some(ru::new(50..60))));
    /// assert_eq!(r3, (Some(ru::new(30..60)), None));
    /// assert_eq!(r4, (Some(ru::new(30..40)), Some(ru::new(40..60))));
    /// ```
    #[must_use]
    #[doc_on_only]
    fn diff_adv<R>(&self, other: &R, mode: CursorMode) -> Pair<Option<RangeUniv<T>>>
    where
        R: ?Sized + RangeBounds<T>,
        T: Clone + PartialOrd,
    {
        assert!(calc::is_mixable(self, other), "{}", msg::BOUNDS_ORDERED);
        calc::diff_adv(self, other, mode)
    }
}
