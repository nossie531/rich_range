//! Provider of [`RangeView`].

use crate::conv::*;
use crate::parts::*;
use crate::*;
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::marker::PhantomData;
use core::ops::{Add, Bound, Range, RangeBounds, Sub};
use rustdoc_copy::prelude::*;

/// Range viewer for abstraction.
///
/// # Examples
///
/// ```
/// use rich_range::prelude::*;
///
/// let r1 = rv::new(&(..60));
/// let r2 = rv::new(&(50..));
/// assert!(r1.intersects(&r2));
/// ```
///
/// # Vs `RangeWrapper`
///
/// This type is similar to [`RangeWrapper<R, T>`]. Both type can adapt
/// range types to [`RichRangeBounds`] from [`RangeBounds`]. But this type
/// is not required `R` and `T` to be [`Sized`]. Instead, the result of
/// range operations in this type are all abstracted into [`RangeUniv`].
#[derive(Debug)]
pub struct RangeView<'a, R, T>(
    /// Reference to range.
    pub &'a R,
    /// Phantom data.
    PhantomData<T>,
)
where
    T: ?Sized,
    R: ?Sized + RangeBounds<T>;

impl<'a, R, T> RangeView<'a, R, T>
where
    T: ?Sized,
    R: ?Sized + RangeBounds<T>,
{
    /// Creates a new instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let r = RangeView::new(&(30..60));
    /// assert_eq!(r.0, &(30..60));
    /// ```
    pub fn new(base: &'a R) -> Self {
        Self(base, PhantomData)
    }

    /// Returns universal range.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let r = rv::new(&(30..60)).to_univ();
    /// assert_eq!(r, ru::new(30..60));
    /// ```
    pub fn to_univ(&self) -> RangeUniv<T>
    where
        T: Clone,
    {
        let s = self.0.start_bound().cloned();
        let e = self.0.end_bound().cloned();
        RangeUniv::new(s, e)
    }
}

/// Relay methods to [`RangeBounds`].
///
/// # Notes about Rust
///
/// Type methods take precedence over trait methods in "method call syntax".
/// Therefore, these methods reduces the need to use "Fully qualified syntax".
impl<'a, R, T> RangeView<'a, R, T>
where
    R: ?Sized + RangeBounds<T>,
    T: ?Sized,
{
    /// Returns `true` if this range contains given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new(&(30..60));
    /// assert!(target.contains(&40));
    /// assert!(!target.contains(&70));
    /// ```
    #[inline]
    #[must_use]
    pub fn contains<U>(&self, value: &U) -> bool
    where
        T: PartialOrd<U>,
        U: ?Sized + PartialOrd<T>,
    {
        RangeBounds::contains(self, value)
    }
}

/// Relay methods to [`RichRangeBounds`].
///
/// # Notes about Rust
///
/// Type methods take precedence over trait methods in "method call syntax".
/// Therefore, these methods reduces the need to use "Fully qualified syntax".
impl<'a, R, T> RangeView<'a, R, T>
where
    R: ?Sized + RangeBounds<T>,
    T: ?Sized,
{
    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::is_empty::top!()]
    #[doc = doc_rrb::side::is_empty::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rv::new(&(30..30)).is_empty());
    /// assert!(rv::new(&(60..30)).is_empty());
    /// assert!(!rv::new(&(30..60)).is_empty());
    /// assert!(!rv::new(&(30..=30)).is_empty());
    pub fn is_empty(&self) -> bool
    where
        T: PartialOrd,
    {
        RichRangeBounds::is_empty(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::is_broken::top!()]
    #[doc = doc_rrb::side::is_broken::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rv::new(&(60..30)).is_broken());
    /// assert!(!rv::new(&(30..60)).is_broken());
    /// assert!(!rv::new(&(30..30)).is_broken());
    /// assert!(!rv::new(&(30..=30)).is_broken());
    /// ```
    pub fn is_broken(&self) -> bool
    where
        T: PartialOrd,
    {
        RichRangeBounds::is_broken(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::is_cursor::top!()]
    #[doc = doc_rrb::side::is_cursor::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rv::new(&(30..30)).is_cursor());
    /// assert!(!rv::new(&(30..60)).is_cursor());
    /// assert!(!rv::new(&(60..30)).is_cursor());
    /// assert!(!rv::new(&(30..=30)).is_cursor());
    /// ```
    pub fn is_cursor(&self) -> bool
    where
        T: PartialEq,
    {
        RichRangeBounds::is_cursor(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::is_cursor_fwd::top!()]
    #[doc = doc_rrb::side::is_cursor_fwd::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    /// use std::ops::Bound::*;
    ///
    /// let target1 = rv::new(&(Included(30), Excluded(30)));
    /// let target2 = rv::new(&(Excluded(30), Included(30)));
    /// assert!(target1.is_cursor_fwd());
    /// assert!(!target2.is_cursor_fwd());
    /// ```
    pub fn is_cursor_fwd(&self) -> bool
    where
        T: PartialEq,
    {
        RichRangeBounds::is_cursor_fwd(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::is_cursor_bwd::top!()]
    #[doc = doc_rrb::side::is_cursor_bwd::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    /// use std::ops::Bound::*;
    ///
    /// let target1 = rv::new(&(Excluded(30), Included(30)));
    /// let target2 = rv::new(&(Included(30), Excluded(30)));
    /// assert!(target1.is_cursor_bwd());
    /// assert!(!target2.is_cursor_bwd());
    /// ```
    pub fn is_cursor_bwd(&self) -> bool
    where
        T: PartialEq,
    {
        RichRangeBounds::is_cursor_bwd(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::is_point::top!()]
    #[doc = doc_rrb::side::is_point::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rv::new(&(30..=30)).is_point());
    /// assert!(!rv::new(&(30..30)).is_point());
    /// assert!(!rv::new(&(30..60)).is_point());
    /// assert!(!rv::new(&(60..30)).is_point());
    /// ```
    pub fn is_point(&self) -> bool
    where
        T: PartialEq,
    {
        RichRangeBounds::is_point(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::is_wide::top!()]
    #[doc = doc_rrb::side::is_wide::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rv::new::<_, usize>(&(..)).is_wide());
    /// assert!(rv::new(&(30..)).is_wide());
    /// assert!(!rv::new(&(30..60)).is_wide());
    /// ```
    pub fn is_wide(&self) -> bool
    where
        T: PartialOrd,
    {
        RichRangeBounds::is_wide(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::is_full::top!()]
    #[doc = doc_rrb::side::is_full::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rv::new::<_, usize>(&(..)).is_full());
    /// assert!(!rv::new(&(30..)).is_full());
    /// assert!(!rv::new(&(30..60)).is_full());
    /// ```
    pub fn is_full(&self) -> bool
    where
        T: PartialOrd,
    {
        RichRangeBounds::is_full(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::start_edge::top!()]
    #[doc = doc_rrb::side::start_edge::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let result = target.start_edge();
    /// assert_eq!(result, Edge::new(Side::S, Included(&30)));
    /// ```
    pub fn start_edge(&self) -> Edge<&T> {
        RichRangeBounds::start_edge(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::end_edge::top!()]
    #[doc = doc_rrb::side::end_edge::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let result = target.end_edge();
    /// assert_eq!(result, Edge::new(Side::E, Excluded(&60)));
    /// ```
    pub fn end_edge(&self) -> Edge<&T> {
        RichRangeBounds::end_edge(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::head::top!()]
    #[doc = doc_rrb::side::head::sub::panics::all!()]
    #[doc = doc_rrb::side::head::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    /// use std::ops::Bound::*;
    ///
    /// assert_eq!(rv::new(&(30..)).head(), 30);
    /// assert_eq!(rv::new(&(Excluded(30), Unbounded)).head(), 31);
    /// assert_eq!(rv::new(&(..60)).head(), i32::MIN);
    /// ```
    pub fn head(&self) -> T
    where
        T: Clone + HasLimits + HasNexts,
    {
        RichRangeBounds::head(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::tail::top!()]
    #[doc = doc_rrb::side::tail::sub::panics::all!()]
    #[doc = doc_rrb::side::tail::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    /// use std::ops::Bound;
    ///
    /// assert_eq!(rv::new(&(30..60)).tail(), 59);
    /// assert_eq!(rv::new(&(30..=60)).tail(), 60);
    /// assert_eq!(rv::new(&(30..)).tail(), i32::MAX);
    /// ```
    pub fn tail(&self) -> T
    where
        T: Clone + HasLimits + HasNexts,
    {
        RichRangeBounds::tail(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::prev::top!()]
    #[doc = doc_rrb::side::prev::sub::panics::all!()]
    #[doc = doc_rrb::side::prev::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    /// use std::ops::Bound::*;
    ///
    /// assert_eq!(rv::new(&(30..)).prev(), 29);
    /// assert_eq!(rv::new(&(Excluded(30), Unbounded)).prev(), 30);
    /// assert_eq!(rv::new(&(..60)).prev(), i32::MIN);
    /// ```
    pub fn prev(&self) -> T
    where
        T: Clone + HasLimits + HasNexts,
    {
        RichRangeBounds::prev(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::next::top!()]
    #[doc = doc_rrb::side::next::sub::panics::all!()]
    #[doc = doc_rrb::side::next::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert_eq!(rv::new(&(30..60)).next(), 60);
    /// assert_eq!(rv::new(&(30..=60)).next(), 61);
    /// assert_eq!(rv::new(&(30..)).next(), i32::MAX);
    /// ```
    pub fn next(&self) -> T
    where
        T: Clone + HasLimits + HasNexts,
    {
        RichRangeBounds::next(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::cursor::top!()]
    #[doc = doc_rrb::side::cursor::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert_eq!(rv::new(&(30..60)).cursor(), None);
    /// assert_eq!(rv::new(&(30..=30)).cursor(), None);
    /// assert_eq!(rv::new(&(30..30)).cursor(), Some(&30));
    /// ```
    pub fn cursor(&self) -> Option<&T>
    where
        T: PartialEq,
    {
        RichRangeBounds::cursor(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::point::top!()]
    #[doc = doc_rrb::side::point::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert_eq!(rv::new(&(30..60)).point(), None);
    /// assert_eq!(rv::new(&(30..30)).point(), None);
    /// assert_eq!(rv::new(&(30..=30)).point(), Some(&30));
    /// ```
    pub fn point(&self) -> Option<&T>
    where
        T: PartialEq,
    {
        RichRangeBounds::point(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::len::top!()]
    #[doc = doc_rrb::side::len::sub::groups::all!()]
    #[doc = doc_rrb::side::len::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert_eq!(rv::new(&(30..)).len(), None);
    /// assert_eq!(rv::new(&(30..60)).len(), Some(30));
    /// assert_eq!(rv::new(&(60..30)).len(), Some(0));
    /// assert_eq!(rv::new(&(30..=60)).len(), Some(31));
    /// ```
    pub fn len(&self) -> Option<usize>
    where
        T: Step,
    {
        RichRangeBounds::len(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::size::top!()]
    #[doc = doc_rrb::side::size::sub::panics::all!()]
    #[doc = doc_rrb::side::size::sub::groups::all!()]
    #[doc = doc_rrb::side::size::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert_eq!(rv::new(&(30..)).size(), None);
    /// assert_eq!(rv::new(&(30..60)).size(), Some(30));
    /// assert_eq!(rv::new(&(60..30)).size(), Some(0));
    /// assert_eq!(rv::new(&(30..=60)).size(), Some(31));
    /// ```
    pub fn size(&self) -> Option<T>
    where
        T: Step,
        for<'x> &'x T: Sub<&'x T, Output = T>,
    {
        RichRangeBounds::size(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::width::top!()]
    #[doc = doc_rrb::side::width::sub::notes::all!()]
    #[doc = doc_rrb::side::width::sub::panics::all!()]
    #[doc = doc_rrb::side::width::sub::groups::all!()]
    #[doc = doc_rrb::side::width::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert_eq!(rv::new(&(30.0..)).width(), None);
    /// assert_eq!(rv::new(&(30.0..60.0)).width(), Some(30.0));
    /// assert_eq!(rv::new(&(60.0..30.0)).width(), Some(0.0));
    /// assert_eq!(rv::new(&(30.0..=60.0)).width(), Some(30.0));
    /// ```
    pub fn width(&self) -> Option<T>
    where
        T: Sized + PartialOrd,
        for<'x> &'x T: Sub<&'x T, Output = T>,
    {
        RichRangeBounds::width(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::bounds::top!()]
    #[doc = doc_rrb::side::bounds::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let result = target.bounds();
    /// assert_eq!(result.0, Included(&30));
    /// assert_eq!(result.1, Excluded(&60));
    /// ```
    pub fn bounds(&self) -> (Bound<&T>, Bound<&T>) {
        RichRangeBounds::bounds(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::edges::top!()]
    #[doc = doc_rrb::side::edges::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let result = target.edges();
    /// assert_eq!(result.0, Edge::new(Side::S, Included(&30)));
    /// assert_eq!(result.1, Edge::new(Side::E, Excluded(&60)));
    /// ```
    pub fn edges(&self) -> (Edge<&T>, Edge<&T>) {
        RichRangeBounds::edges(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::as_ref::top!()]
    #[doc = doc_rrb::side::as_ref::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    /// use std::ops::{Bound::*, RangeBounds};
    ///
    /// let target = rv::new(&(30..60));
    /// let result = target.as_ref();
    /// assert_eq!(result.start_bound(), Included(&30));
    /// assert_eq!(result.end_bound(), Excluded(&60));
    /// ```
    pub fn as_ref(&self) -> <Self as RangeSrc<T>>::Range<&T>
    where
        T: Sized,
        Self: RangeSrc<T>,
    {
        RichRangeBounds::as_ref(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::cast::top!()]
    #[doc = doc_rrb::side::cast::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let src = rv::new::<_, u16>(&(30..60));
    /// assert_eq!(src.cast::<f32>(), ru::new(30.0..60.0));
    /// ```
    pub fn cast<U>(self) -> <Self as RangeSrc<T>>::Range<U>
    where
        T: Sized,
        U: From<T>,
        Self: RangeSrc<T> + RangeParts<T>,
    {
        RichRangeBounds::cast(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::try_cast::top!()]
    #[doc = doc_rrb::side::try_cast::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let src1 = rv::new::<_, i16>(&(30..60));
    /// let src2 = rv::new::<_, i16>(&(-30..60));
    /// assert_eq!(src1.try_cast::<u16>(), Some(ru::new(30..60)));
    /// assert_eq!(src2.try_cast::<u16>(), None);
    /// ```
    pub fn try_cast<U>(self) -> Option<<Self as RangeSrc<T>>::Range<U>>
    where
        T: Sized,
        U: TryFrom<T>,
        Self: RangeSrc<T> + RangeParts<T>,
    {
        RichRangeBounds::try_cast(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::to_range::top!()]
    #[doc = doc_rrb::side::to_range::sub::panics::all!()]
    #[doc = doc_rrb::side::to_range::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new(&(30..=60));
    /// let result = target.to_range();
    /// assert_eq!(result, 30..61);
    /// ```
    pub fn to_range(&self) -> Range<T>
    where
        T: Clone,
        T: HasLimits + HasNexts,
    {
        RichRangeBounds::to_range(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::into_option::top!()]
    #[doc = doc_rrb::side::into_option::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target1 = rv::new(&(30..60));
    /// let target2 = rv::new(&(60..30));
    /// let result1 = target1.into_option();
    /// let result2 = target2.into_option();
    /// assert_eq!(result1, Some(target1));
    /// assert_eq!(result2, None);
    /// ```
    pub fn into_option(self) -> Option<Self>
    where
        Self: Sized,
        T: PartialOrd,
    {
        RichRangeBounds::into_option(self)
    }

    #[inline]
    #[doc_on_only]
    #[doc = doc_rrb::side::iter::top!()]
    #[doc = doc_rrb::side::iter::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new(&(3..6));
    /// let result = target.iter();
    /// assert!(result.eq([3, 4, 5].into_iter()));
    /// ```
    pub fn iter(&self) -> IterRichRange<T>
    where
        T: Step,
    {
        RichRangeBounds::iter(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::flip::top!()]
    #[doc = doc_rrb::side::flip::sub::notes::all!()]
    #[doc = doc_rrb::side::flip::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let r1 = rv::new::<_, usize>(&(..)).flip();
    /// let r2 = rv::new(&(30..)).flip();
    /// let r3 = rv::new(&(..60)).flip();
    /// let r4 = rv::new(&(30..60)).flip();
    /// let r5 = rv::new(&(30..30)).flip();
    /// assert_eq!(r1, [None, None]);
    /// assert_eq!(r2, [Some(ru::new(..30)), None]);
    /// assert_eq!(r3, [Some(ru::new(60..)), None]);
    /// assert_eq!(r4, [Some(ru::new(..30)), Some(ru::new(60..))]);
    /// assert_eq!(r5, [Some(ru::new(..)), None]);
    /// ```
    pub fn flip(&self) -> [Option<RangeUniv<T>>; 2]
    where
        T: Clone + PartialOrd,
    {
        RichRangeBounds::flip(self)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::flip_adv::top!()]
    #[doc = doc_rrb::side::flip_adv::sub::notes::all!()]
    #[doc = doc_rrb::side::flip_adv::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let r1 = rv::new::<_, usize>(&(..)).flip_adv(CursorMode::Off);
    /// let r2 = rv::new(&(30..)).flip_adv(CursorMode::Off);
    /// let r3 = rv::new(&(..60)).flip_adv(CursorMode::Off);
    /// let r4 = rv::new(&(30..60)).flip_adv(CursorMode::Off);
    /// let r5 = rv::new(&(30..30)).flip_adv(CursorMode::Off);
    /// let r6 = rv::new(&(30..30)).flip_adv(CursorMode::On);
    /// assert_eq!(r1, [None, None]);
    /// assert_eq!(r2, [Some(ru::new(..30)), None]);
    /// assert_eq!(r3, [Some(ru::new(60..)), None]);
    /// assert_eq!(r4, [Some(ru::new(..30)), Some(ru::new(60..))]);
    /// assert_eq!(r5, [Some(ru::new(..)), None]);
    /// assert_eq!(r6, [Some(ru::new(..30)), Some(ru::new(30..))]);
    /// ```
    pub fn flip_adv(&self, mode: CursorMode) -> [Option<RangeUniv<T>>; 2]
    where
        T: Clone + PartialOrd,
    {
        RichRangeBounds::flip_adv(self, mode)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::shl::top!()]
    #[doc = doc_rrb::side::shl::sub::panics::all!()]
    #[doc = doc_rrb::side::shl::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let result = target.shl(10);
    /// assert_eq!(result, ru::new(20..50));
    /// ```
    pub fn shl(&self, value: impl Borrow<T>) -> RangeUniv<T>
    where
        T: Sized,
        for<'x> &'x T: Sub<&'x T, Output = T>,
    {
        RichRangeBounds::shl(self, value)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::shr::top!()]
    #[doc = doc_rrb::side::shr::sub::panics::all!()]
    #[doc = doc_rrb::side::shr::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let result = target.shr(10);
    /// assert_eq!(result, ru::new(40..70));
    /// ```
    pub fn shr(&self, value: impl Borrow<T>) -> RangeUniv<T>
    where
        T: Sized,
        for<'x> &'x T: Add<&'x T, Output = T>,
    {
        RichRangeBounds::shr(self, value)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::add_start::top!()]
    #[doc = doc_rrb::side::add_start::sub::panics::all!()]
    #[doc = doc_rrb::side::add_start::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let result = target.add_start(10);
    /// assert_eq!(result, ru::new(40..60));
    /// ```
    pub fn add_start(&self, value: impl Borrow<T>) -> RangeUniv<T>
    where
        T: Clone,
        for<'x> &'x T: Add<&'x T, Output = T>,
    {
        RichRangeBounds::add_start(self, value)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::add_end::top!()]
    #[doc = doc_rrb::side::add_end::sub::panics::all!()]
    #[doc = doc_rrb::side::add_end::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let result = target.add_end(10);
    /// assert_eq!(result, ru::new(30..70));
    /// ```
    pub fn add_end(&self, value: impl Borrow<T>) -> RangeUniv<T>
    where
        T: Clone,
        for<'x> &'x T: Add<&'x T, Output = T>,
    {
        RichRangeBounds::add_end(self, value)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::sub_start::top!()]
    #[doc = doc_rrb::side::sub_start::sub::panics::all!()]
    #[doc = doc_rrb::side::sub_start::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let result = target.sub_start(10);
    /// assert_eq!(result, ru::new(20..60));
    /// ```
    pub fn sub_start(&self, value: impl Borrow<T>) -> RangeUniv<T>
    where
        T: Clone,
        for<'x> &'x T: Sub<&'x T, Output = T>,
    {
        RichRangeBounds::sub_start(self, value)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::sub_end::top!()]
    #[doc = doc_rrb::side::sub_end::sub::panics::all!()]
    #[doc = doc_rrb::side::sub_end::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let result = target.sub_end(10);
    /// assert_eq!(result, ru::new(30..50));
    /// ```
    pub fn sub_end(&self, value: impl Borrow<T>) -> RangeUniv<T>
    where
        T: Clone,
        for<'x> &'x T: Sub<&'x T, Output = T>,
    {
        RichRangeBounds::sub_end(self, value)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::calc_start::top!()]
    #[doc = doc_rrb::side::calc_start::sub::panics::all!()]
    #[doc = doc_rrb::side::calc_start::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let result = target.calc_start(40);
    /// assert_eq!(result, ru::new(20..60));
    /// ```
    pub fn calc_start(&self, width: impl Borrow<T>) -> RangeUniv<T>
    where
        T: Clone,
        for<'x> &'x T: Sub<&'x T, Output = T>,
    {
        RichRangeBounds::calc_start(self, width)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::calc_end::top!()]
    #[doc = doc_rrb::side::calc_end::sub::panics::all!()]
    #[doc = doc_rrb::side::calc_end::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let result = target.calc_end(40);
    /// assert_eq!(result, ru::new(30..70));
    /// ```
    pub fn calc_end(&self, width: impl Borrow<T>) -> RangeUniv<T>
    where
        T: Clone,
        for<'x> &'x T: Add<&'x T, Output = T>,
    {
        RichRangeBounds::calc_end(self, width)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::align_start::top!()]
    #[doc = doc_rrb::side::align_start::sub::panics::all!()]
    #[doc = doc_rrb::side::align_start::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let result = target.align_start(40);
    /// assert_eq!(result, ru::new(40..70));
    /// ```
    pub fn align_start(&self, value: impl Borrow<T>) -> RangeUniv<T>
    where
        T: Clone + PartialOrd,
        for<'x> &'x T: CheckedAdd<&'x T, Output = T>,
        for<'x> &'x T: CheckedSub<&'x T, Output = T>,
    {
        RichRangeBounds::align_start(self, value)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::align_end::top!()]
    #[doc = doc_rrb::side::align_end::sub::panics::all!()]
    #[doc = doc_rrb::side::align_end::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let result = target.align_end(70);
    /// assert_eq!(result, ru::new(40..70));
    /// ```
    pub fn align_end(&self, value: impl Borrow<T>) -> RangeUniv<T>
    where
        T: Clone + PartialOrd,
        for<'x> &'x T: CheckedAdd<&'x T, Output = T>,
        for<'x> &'x T: CheckedSub<&'x T, Output = T>,
    {
        RichRangeBounds::align_end(self, value)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::map::top!()]
    #[doc = doc_rrb::side::map::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new(&(30..));
    /// let result = target.map(|x| x * 2);
    /// assert_eq!(result, ru::new(60..));
    /// ```
    pub fn map<F, U>(self, f: F) -> RangeUniv<U>
    where
        T: Clone,
        F: FnMut(T) -> U,
    {
        RichRangeBounds::map(self, f)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::try_map::top!()]
    #[doc = doc_rrb::side::try_map::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new::<_, u8>(&(30..));
    /// let result1 = target.clone().try_map(|x| x.checked_mul(2));
    /// let result2 = target.clone().try_map(|x| x.checked_mul(10));
    /// assert_eq!(result1, Some(ru::new(60..)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn try_map<F, U>(self, f: F) -> Option<RangeUniv<U>>
    where
        T: Clone,
        F: FnMut(T) -> Option<U>,
    {
        RichRangeBounds::try_map(self, f)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::checked_shl::top!()]
    #[doc = doc_rrb::side::checked_shl::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new::<_, u8>(&(30..60));
    /// let result1 = target.checked_shl(10);
    /// let result2 = target.checked_shl(40);
    /// assert_eq!(result1, Some(ru::new(20..50)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn checked_shl(&self, value: impl Borrow<T>) -> Option<RangeUniv<T>>
    where
        T: Sized,
        for<'x> &'x T: CheckedSub<&'x T, Output = T>,
    {
        RichRangeBounds::checked_shl(self, value)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::checked_shr::top!()]
    #[doc = doc_rrb::side::checked_shr::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new::<_, u8>(&(30..60));
    /// let result1 = target.checked_shr(10);
    /// let result2 = target.checked_shr(200);
    /// assert_eq!(result1, Some(ru::new(40..70)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn checked_shr(&self, value: impl Borrow<T>) -> Option<RangeUniv<T>>
    where
        T: Sized,
        for<'x> &'x T: CheckedAdd<&'x T, Output = T>,
    {
        RichRangeBounds::checked_shr(self, value)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::checked_add_start::top!()]
    #[doc = doc_rrb::side::checked_add_start::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new::<_, u8>(&(30..60));
    /// let result1 = target.checked_add_start(10);
    /// let result2 = target.checked_add_start(250);
    /// assert_eq!(result1, Some(ru::new(40..60)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn checked_add_start(&self, value: impl Borrow<T>) -> Option<RangeUniv<T>>
    where
        T: Clone,
        for<'x> &'x T: CheckedAdd<&'x T, Output = T>,
    {
        RichRangeBounds::checked_add_start(self, value)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::checked_add_end::top!()]
    #[doc = doc_rrb::side::checked_add_end::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new::<_, u8>(&(30..60));
    /// let result1 = target.checked_add_end(10);
    /// let result2 = target.checked_add_end(250);
    /// assert_eq!(result1, Some(ru::new(30..70)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn checked_add_end(&self, value: impl Borrow<T>) -> Option<RangeUniv<T>>
    where
        T: Clone,
        for<'x> &'x T: CheckedAdd<&'x T, Output = T>,
    {
        RichRangeBounds::checked_add_end(self, value)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::checked_sub_start::top!()]
    #[doc = doc_rrb::side::checked_sub_start::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new::<_, u8>(&(30..60));
    /// let result1 = target.checked_sub_start(10);
    /// let result2 = target.checked_sub_start(40);
    /// assert_eq!(result1, Some(ru::new(20..60)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn checked_sub_start(&self, value: impl Borrow<T>) -> Option<RangeUniv<T>>
    where
        T: Clone,
        for<'x> &'x T: CheckedSub<&'x T, Output = T>,
    {
        RichRangeBounds::checked_sub_start(self, value)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::checked_sub_end::top!()]
    #[doc = doc_rrb::side::checked_sub_end::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new::<_, u8>(&(30..60));
    /// let result1 = target.checked_sub_end(10);
    /// let result2 = target.checked_sub_end(70);
    /// assert_eq!(result1, Some(ru::new(30..50)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn checked_sub_end(&self, value: impl Borrow<T>) -> Option<RangeUniv<T>>
    where
        T: Clone,
        for<'x> &'x T: CheckedSub<&'x T, Output = T>,
    {
        RichRangeBounds::checked_sub_end(self, value)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::checked_calc_start::top!()]
    #[doc = doc_rrb::side::checked_calc_start::sub::panics::all!()]
    #[doc = doc_rrb::side::checked_calc_start::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new::<_, u8>(&(30..60));
    /// let r1 = target.checked_calc_start(40);
    /// let r2 = target.checked_calc_start(70);
    /// assert_eq!(r1, Some(ru::new(20..60)));
    /// assert_eq!(r2, None);
    /// ```
    pub fn checked_calc_start(&self, width: impl Borrow<T>) -> Option<RangeUniv<T>>
    where
        T: Clone,
        for<'x> &'x T: CheckedSub<&'x T, Output = T>,
    {
        RichRangeBounds::checked_calc_start(self, width)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::checked_calc_end::top!()]
    #[doc = doc_rrb::side::checked_calc_end::sub::panics::all!()]
    #[doc = doc_rrb::side::checked_calc_end::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new::<_, u8>(&(30..60));
    /// let r1 = target.checked_calc_end(40);
    /// let r2 = target.checked_calc_end(230);
    /// assert_eq!(r1, Some(ru::new(30..70)));
    /// assert_eq!(r2, None);
    /// ```
    pub fn checked_calc_end(&self, width: impl Borrow<T>) -> Option<RangeUniv<T>>
    where
        T: Clone,
        for<'x> &'x T: CheckedAdd<&'x T, Output = T>,
    {
        RichRangeBounds::checked_calc_end(self, width)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::checked_align_start::top!()]
    #[doc = doc_rrb::side::checked_align_start::sub::panics::all!()]
    #[doc = doc_rrb::side::checked_align_start::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new::<_, u8>(&(30..60));
    /// let result1 = target.checked_align_start(40);
    /// let result2 = target.checked_align_start(230);
    /// assert_eq!(result1, Some(ru::new(40..70)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn checked_align_start(
        &self,
        value: impl Borrow<T>,
    ) -> Option<<Self as RangeSrc<T>>::Range<T>>
    where
        T: Clone + PartialOrd,
        Self: RangeSrc<T>,
        for<'x> &'x T: CheckedAdd<&'x T, Output = T>,
        for<'x> &'x T: CheckedSub<&'x T, Output = T>,
    {
        RichRangeBounds::checked_align_start(self, value)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::checked_align_end::top!()]
    #[doc = doc_rrb::side::checked_align_end::sub::panics::all!()]
    #[doc = doc_rrb::side::checked_align_end::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new::<_, u8>(&(30..60));
    /// let result1 = target.checked_align_end(70);
    /// let result2 = target.checked_align_end(20);
    /// assert_eq!(result1, Some(ru::new(40..70)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn checked_align_end(
        &self,
        value: impl Borrow<T>,
    ) -> Option<<Self as RangeSrc<T>>::Range<T>>
    where
        T: Clone + PartialOrd,
        Self: RangeSrc<T>,
        for<'x> &'x T: CheckedAdd<&'x T, Output = T>,
        for<'x> &'x T: CheckedSub<&'x T, Output = T>,
    {
        RichRangeBounds::checked_align_end(self, value)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::equiv::top!()]
    #[doc = doc_rrb::side::equiv::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rv::new(&(30..60)).equiv(&(30..60)));
    /// assert!(!rv::new(&(30..60)).equiv(&(30..65)));
    /// assert!(rv::new(&(30..30)).equiv(&(0..0)));
    /// ```
    pub fn equiv<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        RichRangeBounds::equiv(self, other)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::intersects::top!()]
    #[doc = doc_rrb::side::intersects::sub::notes::all!()]
    #[doc = doc_rrb::side::intersects::sub::examples::head!()]
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rv::new(&(30..60)).intersects(&(50..70)));
    /// assert!(rv::new(&(30..60)).intersects(&(50..50)));
    /// assert!(!rv::new(&(30..60)).intersects(&(70..80)));
    /// ```
    pub fn intersects<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        RichRangeBounds::intersects(self, other)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::includes::top!()]
    #[doc = doc_rrb::side::includes::sub::notes::all!()]
    #[doc = doc_rrb::side::includes::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rv::new(&(30..60)).includes(&(40..50)));
    /// assert!(!rv::new(&(30..60)).includes(&(70..80)));
    /// assert!(!rv::new(&(30..60)).includes(&(60..60)));
    /// assert!(rv::new(&(30..30)).includes(&(30..30)));
    /// assert!(!rv::new(&(30..30)).includes(&(40..40)));
    /// ```
    pub fn includes<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        RichRangeBounds::includes(self, other)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::included::top!()]
    #[doc = doc_rrb::side::included::sub::notes::all!()]
    #[doc = doc_rrb::side::included::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rv::new(&(30..60)).included(&(20..70)));
    /// assert!(!rv::new(&(30..60)).included(&(40..70)));
    /// assert!(rv::new(&(30..30)).included(&(30..30)));
    /// assert!(!rv::new(&(30..30)).included(&(40..40)));
    /// ```    
    pub fn included<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        RichRangeBounds::included(self, other)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::adjoins::top!()]
    #[doc = doc_rrb::side::adjoins::sub::notes::all!()]
    #[doc = doc_rrb::side::adjoins::sub::groups::all!()]
    #[doc = doc_rrb::side::adjoins::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rv::new(&(30..=60)).adjoins(&(60..70)));
    /// assert!(!rv::new(&(30..=60)).adjoins(&(70..80)));
    /// assert!(rv::new(&(30..60)).adjoins(&(20..=30)));
    /// assert!(!rv::new(&(30..60)).adjoins(&(10..=20)));
    /// ```
    pub fn adjoins<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        RichRangeBounds::adjoins(self, other)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::adjoins_prev::top!()]
    #[doc = doc_rrb::side::adjoins_prev::sub::notes::all!()]
    #[doc = doc_rrb::side::adjoins_prev::sub::groups::all!()]
    #[doc = doc_rrb::side::adjoins_prev::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rv::new(&(30..60)).adjoins_prev(&(20..=30)));
    /// assert!(!rv::new(&(30..60)).adjoins_prev(&(10..=20)));
    /// ```
    pub fn adjoins_prev<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        RichRangeBounds::adjoins_prev(self, other)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::adjoins_next::top!()]
    #[doc = doc_rrb::side::adjoins_next::sub::notes::all!()]
    #[doc = doc_rrb::side::adjoins_next::sub::groups::all!()]
    #[doc = doc_rrb::side::adjoins_next::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rv::new(&(30..=60)).adjoins_next(&(60..70)));
    /// assert!(!rv::new(&(30..=60)).adjoins_next(&(70..80)));
    /// ```
    pub fn adjoins_next<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        RichRangeBounds::adjoins_next(self, other)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::touches::top!()]
    #[doc = doc_rrb::side::touches::sub::notes::all!()]
    #[doc = doc_rrb::side::touches::sub::groups::all!()]
    #[doc = doc_rrb::side::touches::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rv::new(&(30..60)).touches(&(60..70)));
    /// assert!(!rv::new(&(30..60)).touches(&(70..80)));
    /// assert!(rv::new(&(30..60)).touches(&(20..30)));
    /// assert!(!rv::new(&(30..60)).touches(&(10..20)));
    /// ```
    pub fn touches<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        RichRangeBounds::touches(self, other)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::touches_prev::top!()]
    #[doc = doc_rrb::side::touches_prev::sub::notes::all!()]
    #[doc = doc_rrb::side::touches_prev::sub::groups::all!()]
    #[doc = doc_rrb::side::touches_prev::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rv::new(&(30..60)).touches_prev(&(20..30)));
    /// assert!(!rv::new(&(30..60)).touches_prev(&(10..20)));
    /// ```
    pub fn touches_prev<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        RichRangeBounds::touches_prev(self, other)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::touches_next::top!()]
    #[doc = doc_rrb::side::touches_next::sub::notes::all!()]
    #[doc = doc_rrb::side::touches_next::sub::groups::all!()]
    #[doc = doc_rrb::side::touches_next::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rv::new(&(30..60)).touches_next(&(60..70)));
    /// assert!(!rv::new(&(30..60)).touches_next(&(70..80)));
    /// ```
    pub fn touches_next<R2>(&self, other: &R2) -> bool
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        RichRangeBounds::touches_next(self, other)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::rel::top!()]
    #[doc = doc_rrb::side::rel::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let result = target.rel(&rv::new(&(20..70)), PosStyle::Step);
    /// assert_eq!(result, RangeRel::During(true));
    /// ```
    pub fn rel<R2>(&self, other: &R2, ps: PosStyle) -> RangeRel
    where
        R2: ?Sized + RangeBounds<T>,
        T: PartialOrd,
    {
        RichRangeBounds::rel(self, other, ps)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::cut::top!()]
    #[doc = doc_rrb::side::cut::sub::notes::all!()]
    #[doc = doc_rrb::side::cut::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let r = rv::new(&(30..60));
    /// let [fst, snd] = r.cut(&40, CutMode::Standard);
    /// assert_eq!(fst, Some(ru::new(30..40)));
    /// assert_eq!(snd, Some(ru::new(40..60)));
    /// ```
    pub fn cut(&self, pos: &T, mode: CutMode) -> [Option<RangeUniv<T>>; 2]
    where
        T: Clone + PartialOrd,
    {
        RichRangeBounds::cut(self, pos, mode)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::interval::top!()]
    #[doc = doc_rrb::side::interval::sub::panics::all!()]
    #[doc = doc_rrb::side::interval::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let r1 = target.interval(&rv::new(&(50..70)));
    /// let r2 = target.interval(&rv::new(&(60..80)));
    /// let r3 = target.interval(&rv::new(&(70..90)));
    /// assert_eq!(r1, None);
    /// assert_eq!(r2, None);
    /// assert_eq!(r3, Some(ru::new(60..70)));
    /// ```
    pub fn interval(&self, other: &Self) -> Option<RangeUniv<T>>
    where
        T: Clone + PartialOrd,
    {
        RichRangeBounds::interval(self, other)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::interval_adv::top!()]
    #[doc = doc_rrb::side::interval_adv::sub::panics::all!()]
    #[doc = doc_rrb::side::interval_adv::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let target = rv::new(&(30..60));
    /// let r1 = target.interval_adv(&rv::new(&(50..70)), CursorMode::Off);
    /// let r2 = target.interval_adv(&rv::new(&(60..80)), CursorMode::Off);
    /// let r3 = target.interval_adv(&rv::new(&(60..80)), CursorMode::On);
    /// let r4 = target.interval_adv(&rv::new(&(70..90)), CursorMode::Off);
    /// assert_eq!(r1, None);
    /// assert_eq!(r2, None);
    /// assert_eq!(r3, Some(ru::new(60..60)));    
    /// assert_eq!(r4, Some(ru::new(60..70)));
    /// ```
    pub fn interval_adv(&self, other: &Self, mode: CursorMode) -> Option<RangeUniv<T>>
    where
        T: Clone + PartialOrd,
    {
        RichRangeBounds::interval_adv(self, other, mode)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::prod::top!()]
    #[doc = doc_rrb::side::prod::sub::notes::all!()]
    #[doc = doc_rrb::side::prod::sub::panics::all!()]
    #[doc = doc_rrb::side::prod::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let r = rv::new(&(30..60)).prod(&rv::new(&(40..70)));
    /// assert_eq!(r, Some(ru::new(40..60)));
    /// ```
    pub fn prod(&self, other: &Self) -> Option<RangeUniv<T>>
    where
        T: Clone + PartialOrd,
    {
        RichRangeBounds::prod(self, other)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::enwrap::top!()]
    #[doc = doc_rrb::side::enwrap::sub::notes::all!()]
    #[doc = doc_rrb::side::enwrap::sub::panics::all!()]
    #[doc = doc_rrb::side::enwrap::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let r1 = rv::new(&(20..40)).enwrap(&rv::new(&(30..50)));
    /// let r2 = rv::new(&(10..20)).enwrap(&rv::new(&(40..60)));
    /// assert_eq!(r1, Some(ru::new(20..50)));
    /// assert_eq!(r2, Some(ru::new(10..60)));
    /// ```
    pub fn enwrap(&self, other: &Self) -> Option<RangeUniv<T>>
    where
        T: Clone + PartialOrd,
    {
        RichRangeBounds::enwrap(self, other)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::union::top!()]
    #[doc = doc_rrb::side::union::sub::notes::all!()]
    #[doc = doc_rrb::side::union::sub::panics::all!()]
    #[doc = doc_rrb::side::union::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let r1 = rv::new(&(30..60)).union(&rv::new(&(40..70)));
    /// let r2 = rv::new(&(30..60)).union(&rv::new(&(70..80)));
    /// assert_eq!(r1, (ru::new(30..70), None));
    /// assert_eq!(r2, (ru::new(30..60), Some(ru::new(70..80))));
    /// ```
    pub fn union(&self, other: &Self) -> (RangeUniv<T>, Option<RangeUniv<T>>)
    where
        T: Clone + PartialOrd,
    {
        RichRangeBounds::union(self, other)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::diff::top!()]
    #[doc = doc_rrb::side::diff::sub::panics::all!()]
    #[doc = doc_rrb::side::diff::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let r1 = rv::new(&(30..60)).diff(&(50..70));
    /// let r2 = rv::new(&(30..60)).diff(&(40..50));
    /// let r3 = rv::new(&(30..60)).diff(&(40..40));
    /// assert_eq!(r1, [Some(ru::new(30..50)), None]);
    /// assert_eq!(r2, [Some(ru::new(30..40)), Some(ru::new(50..60))]);
    /// assert_eq!(r3, [Some(ru::new(30..60)), None]);
    /// ```
    pub fn diff<R2>(&self, other: &R2) -> [Option<RangeUniv<T>>; 2]
    where
        R2: ?Sized + RangeBounds<T>,
        T: Clone + PartialOrd,
    {
        RichRangeBounds::diff(self, other)
    }

    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::diff_adv::top!()]
    #[doc = doc_rrb::side::diff_adv::sub::panics::all!()]
    #[doc = doc_rrb::side::diff_adv::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    /// use rich_range::*;
    ///
    /// let r1 = rv::new(&(30..60)).diff_adv(&(50..70), CursorMode::Off);
    /// let r2 = rv::new(&(30..60)).diff_adv(&(40..50), CursorMode::Off);
    /// let r3 = rv::new(&(30..60)).diff_adv(&(40..40), CursorMode::Off);
    /// let r4 = rv::new(&(30..60)).diff_adv(&(40..40), CursorMode::On);
    /// assert_eq!(r1, [Some(ru::new(30..50)), None]);
    /// assert_eq!(r2, [Some(ru::new(30..40)), Some(ru::new(50..60))]);
    /// assert_eq!(r3, [Some(ru::new(30..60)), None]);
    /// assert_eq!(r4, [Some(ru::new(30..40)), Some(ru::new(40..60))]);
    /// ```
    pub fn diff_adv<R2>(&self, other: &R2, mode: CursorMode) -> [Option<RangeUniv<T>>; 2]
    where
        R2: ?Sized + RangeBounds<T>,
        T: Clone + PartialOrd,
    {
        RichRangeBounds::diff_adv(self, other, mode)
    }
}

impl<R, T> RangeBounds<T> for RangeView<'_, R, T>
where
    T: ?Sized,
    R: ?Sized + RangeBounds<T>,
{
    fn start_bound(&self) -> Bound<&T> {
        self.0.start_bound()
    }

    fn end_bound(&self) -> Bound<&T> {
        self.0.end_bound()
    }
}

impl<R, T> RichRangeBounds<T> for RangeView<'_, R, T>
where
    T: ?Sized,
    R: ?Sized + RangeBounds<T>,
{
    // nop.
}

impl<R, T> IntoIterator for RangeView<'_, R, T>
where
    R: ?Sized + RangeBounds<T>,
    T: Step,
{
    type Item = T;
    type IntoIter = IterRichRange<T>;

    fn into_iter(self) -> Self::IntoIter {
        let s = self.0.start_bound().cloned();
        let e = self.0.end_bound().cloned();
        IterRichRange::new(s, e)
    }
}

impl<R, T> Clone for RangeView<'_, R, T>
where
    T: ?Sized,
    R: ?Sized + RangeBounds<T>,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<R, T> Copy for RangeView<'_, R, T>
where
    T: ?Sized,
    R: ?Sized + RangeBounds<T>,
{
    // nop
}

impl<R, T> Eq for RangeView<'_, R, T>
where
    R: ?Sized + RangeBounds<T>,
    T: Eq,
{
    // nop.
}

impl<R, T> PartialEq for RangeView<'_, R, T>
where
    R: ?Sized + RangeBounds<T>,
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        calc::is_eq(self, other)
    }
}

impl<R, T> PartialOrd for RangeView<'_, R, T>
where
    R: ?Sized + RangeBounds<T>,
    T: PartialOrd,
{
    /// Returns an ordering between `self` and `other`.
    ///
    /// If all points in the range are either greater than or less than
    /// all point in the other range, returns some order. Otherwise
    /// returns [`None`]. Also, if `self` or `other` has unordered
    /// position like NaN, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    /// use std::cmp::Ordering;
    ///
    /// let r = rv::new(&(30..60)).partial_cmp(&rv::new(&(40..70)));
    /// assert_eq!(r, None);
    ///
    /// let r = rv::new(&(30..60)).partial_cmp(&rv::new(&(70..90)));
    /// assert_eq!(r, Some(Ordering::Less));
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        calc::cmp(self, other)
    }
}
