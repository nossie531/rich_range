//! Provider of [`RangeWrapper`].

use crate::conv::*;
use crate::parts::*;
use crate::*;
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::marker::PhantomData;
use core::mem;
use core::ops::{Add, Sub};
use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
use core::ops::{Bound, Range, RangeBounds};

/// Range wrapper for type keeping calculation.
///
/// # Examples
///
/// ```
/// use rich_range::prelude::*;
///
/// let r = rw::new(30..60) & rw::new(40..70);
/// assert_eq!(r.0, 40..60);
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct RangeWrapper<R, T>(
    /// Wrapped range.
    pub R,
    /// Phantom data.
    PhantomData<T>,
)
where
    R: RangeBounds<T>;

impl<R, T> RangeWrapper<R, T>
where
    R: RangeBounds<T>,
{
    /// Creates a new instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let r = RangeWrapper::new(30..60);
    /// assert_eq!(r.0, 30..60);
    /// ```
    #[must_use]
    pub fn new(value: R) -> Self {
        Self(value, PhantomData)
    }

    /// Creates a new reference from base reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let r = RangeWrapper::from_ref(&(30..60));
    /// assert_eq!(r.0, 30..60);
    /// ```
    #[must_use]
    pub fn from_ref(base: &R) -> &RangeWrapper<R, T> {
        unsafe { mem::transmute(base) }
    }
}

/// Relay methods to [`RangeBounds`].
///
/// # Notes about Rust
///
/// Type methods take precedence over trait methods in "method call syntax".
/// Therefore, these methods reduces the need to use "Fully qualified syntax".
impl<R, T> RangeWrapper<R, T>
where
    R: RangeBounds<T>,
{
    /// Returns `true` if this range contains given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// let target = rw::new(30..60);
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
impl<R, T> RangeWrapper<R, T>
where
    R: RangeBounds<T>,
{
    #[inline]
    #[must_use]
    #[doc_on_only]
    #[doc = doc_rrb::side::is_empty::top!()]
    #[doc = doc_rrb::side::is_empty::sub::examples::head!()]
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rw::new(30..30).is_empty());
    /// assert!(rw::new(60..30).is_empty());
    /// assert!(!rw::new(30..60).is_empty());
    /// assert!(!rw::new(30..=30).is_empty());
    /// ```
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
    /// assert!(rw::new(60..30).is_broken());
    /// assert!(!rw::new(30..60).is_broken());
    /// assert!(!rw::new(30..30).is_broken());
    /// assert!(!rw::new(30..=30).is_broken());
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
    /// assert!(rw::new(30..30).is_cursor());
    /// assert!(!rw::new(30..60).is_cursor());
    /// assert!(!rw::new(60..30).is_cursor());
    /// assert!(!rw::new(30..=30).is_cursor());
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
    /// let target1 = rw::new((Included(30), Excluded(30)));
    /// let target2 = rw::new((Excluded(30), Included(30)));
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
    /// let target1 = rw::new((Excluded(30), Included(30)));
    /// let target2 = rw::new((Included(30), Excluded(30)));
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
    /// assert!(rw::new(30..=30).is_point());
    /// assert!(!rw::new(30..30).is_point());
    /// assert!(!rw::new(30..60).is_point());
    /// assert!(!rw::new(60..30).is_point());
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
    /// assert!(rw::new::<_, usize>(..).is_wide());
    /// assert!(rw::new(30..).is_wide());
    /// assert!(!rw::new(30..60).is_wide());
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
    /// assert!(rw::new::<_, usize>(..).is_full());
    /// assert!(!rw::new(30..).is_full());
    /// assert!(!rw::new(30..60).is_full());
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
    /// let target = rw::new(30..60);
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
    /// let target = rw::new(30..60);
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
    /// assert_eq!(rw::new(30..).head(), 30);
    /// assert_eq!(rw::new((Excluded(30), Unbounded)).head(), 31);
    /// assert_eq!(rw::new(..60).head(), i32::MIN);
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
    /// assert_eq!(rw::new(30..60).tail(), 59);
    /// assert_eq!(rw::new(30..=60).tail(), 60);
    /// assert_eq!(rw::new(30..).tail(), i32::MAX);
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
    /// assert_eq!(rw::new(30..).prev(), 29);
    /// assert_eq!(rw::new((Excluded(30), Unbounded)).prev(), 30);
    /// assert_eq!(rw::new(..60).prev(), i32::MIN);
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
    /// assert_eq!(rw::new(30..60).next(), 60);
    /// assert_eq!(rw::new(30..=60).next(), 61);
    /// assert_eq!(rw::new(30..).next(), i32::MAX);
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
    /// assert_eq!(rw::new(30..60).cursor(), None);
    /// assert_eq!(rw::new(30..=30).cursor(), None);
    /// assert_eq!(rw::new(30..30).cursor(), Some(&30));
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
    /// assert_eq!(rw::new(30..60).point(), None);
    /// assert_eq!(rw::new(30..30).point(), None);
    /// assert_eq!(rw::new(30..=30).point(), Some(&30));
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
    /// assert_eq!(rw::new(30..).len(), None);
    /// assert_eq!(rw::new(30..60).len(), Some(30));
    /// assert_eq!(rw::new(60..30).len(), Some(0));
    /// assert_eq!(rw::new(30..=60).len(), Some(31));
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
    /// assert_eq!(rw::new(30..).size(), None);
    /// assert_eq!(rw::new(30..60).size(), Some(30));
    /// assert_eq!(rw::new(60..30).size(), Some(0));
    /// assert_eq!(rw::new(30..=60).size(), Some(31));
    /// ```
    pub fn size(&self) -> Option<T>
    where
        T: Step,
        for<'a> &'a T: Sub<&'a T, Output = T>,
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
    /// assert_eq!(rw::new(30.0..).width(), None);
    /// assert_eq!(rw::new(30.0..60.0).width(), Some(30.0));
    /// assert_eq!(rw::new(60.0..30.0).width(), Some(0.0));
    /// assert_eq!(rw::new(30.0..=60.0).width(), Some(30.0));
    /// ```
    pub fn width(&self) -> Option<T>
    where
        T: PartialOrd,
        for<'a> &'a T: Sub<&'a T, Output = T>,
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
    /// let target = rw::new(30..60);
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
    /// let target = rw::new(30..60);
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
    /// let target = rw::new(30..60);
    /// let result = target.as_ref();
    /// assert_eq!(result.0.start_bound(), Included(&30));
    /// assert_eq!(result.0.end_bound(), Excluded(&60));
    /// ```
    pub fn as_ref(&self) -> <Self as RangeSrc<T>>::Range<&T>
    where
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
    /// let src = rw::new::<_, u16>(30..60);
    /// assert_eq!(src.cast::<f32>(), rw::new(30.0..60.0));
    /// ```
    pub fn cast<U>(self) -> <Self as RangeSrc<T>>::Range<U>
    where
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
    /// let src1 = rw::new::<_, i16>(30..60);
    /// let src2 = rw::new::<_, i16>(-30..60);
    /// assert_eq!(src1.try_cast::<u16>(), Some(rw::new(30..60)));
    /// assert_eq!(src2.try_cast::<u16>(), None);
    /// ```
    pub fn try_cast<U>(self) -> Option<<Self as RangeSrc<T>>::Range<U>>
    where
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
    /// let target = rw::new(30..=60);
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
    /// let target1 = rw::new(30..60);
    /// let target2 = rw::new(60..30);
    /// let result1 = target1.clone().into_option();
    /// let result2 = target2.clone().into_option();
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
    /// let target = rw::new(3..6);
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
    /// let r1 = rw::new::<_, usize>(..).flip();
    /// let r2 = rw::new(30..).flip();
    /// let r3 = rw::new(..60).flip();
    /// let r4 = rw::new(30..60).flip();
    /// let r5 = rw::new(30..30).flip();
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
    /// let r1 = rw::new::<_, usize>(..).flip_adv(CursorMode::Off);
    /// let r2 = rw::new(30..).flip_adv(CursorMode::Off);
    /// let r3 = rw::new(..60).flip_adv(CursorMode::Off);
    /// let r4 = rw::new(30..60).flip_adv(CursorMode::Off);
    /// let r5 = rw::new(30..30).flip_adv(CursorMode::Off);
    /// let r6 = rw::new(30..30).flip_adv(CursorMode::On);
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
    /// let target = rw::new(30..60);
    /// let result = target.shl(10);
    /// assert_eq!(result, rw::new(20..50));
    /// ```
    pub fn shl(&self, value: impl Borrow<T>) -> Self
    where
        T: Sized,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: Sub<&'a T, Output = T>,
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
    /// let target = rw::new(30..60);
    /// let result = target.shr(10);
    /// assert_eq!(result, rw::new(40..70));
    /// ```
    pub fn shr(&self, value: impl Borrow<T>) -> Self
    where
        T: Sized,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: Add<&'a T, Output = T>,
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
    /// let target = rw::new(30..60);
    /// let result = target.add_start(10);
    /// assert_eq!(result, rw::new(40..60));
    /// ```
    pub fn add_start(&self, value: impl Borrow<T>) -> Self
    where
        T: Clone,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: Add<&'a T, Output = T>,
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
    /// let target = rw::new(30..60);
    /// let result = target.add_end(10);
    /// assert_eq!(result, rw::new(30..70));
    /// ```
    pub fn add_end(&self, value: impl Borrow<T>) -> Self
    where
        T: Clone,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: Add<&'a T, Output = T>,
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
    /// let target = rw::new(30..60);
    /// let result = target.sub_start(10);
    /// assert_eq!(result, rw::new(20..60));
    /// ```
    pub fn sub_start(&self, value: impl Borrow<T>) -> Self
    where
        T: Clone,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: Sub<&'a T, Output = T>,
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
    /// let target = rw::new(30..60);
    /// let result = target.sub_end(10);
    /// assert_eq!(result, rw::new(30..50));
    /// ```
    pub fn sub_end(&self, value: impl Borrow<T>) -> Self
    where
        T: Clone,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: Sub<&'a T, Output = T>,
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
    /// let target = rw::new(30..60);
    /// let result = target.calc_start(40);
    /// assert_eq!(result, rw::new(20..60));
    /// ```
    pub fn calc_start(&self, width: impl Borrow<T>) -> Self
    where
        T: Clone,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: Sub<&'a T, Output = T>,
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
    /// let target = rw::new(30..60);
    /// let result = target.calc_end(40);
    /// assert_eq!(result, rw::new(30..70));
    /// ```
    pub fn calc_end(&self, width: impl Borrow<T>) -> Self
    where
        T: Clone,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: Add<&'a T, Output = T>,
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
    /// let target = rw::new(30..60);
    /// let result = target.align_start(40);
    /// assert_eq!(result, rw::new(40..70));
    /// ```
    pub fn align_start(&self, value: impl Borrow<T>) -> Self
    where
        T: Clone + PartialOrd,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
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
    /// let target = rw::new(30..60);
    /// let result = target.align_end(70);
    /// assert_eq!(result, rw::new(40..70));
    /// ```
    pub fn align_end(&self, value: impl Borrow<T>) -> Self
    where
        T: Clone + PartialOrd,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
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
    /// let target = rw::new(30..);
    /// let result = target.map(|x| x * 2);
    /// assert_eq!(result, rw::new(60..));
    /// ```
    pub fn map<F, U>(self, f: F) -> <Self as RangeSrc<T>>::Range<U>
    where
        F: FnMut(T) -> U,
        Self: RangeSrc<T> + RangeParts<T>,
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
    /// let target = rw::new::<_, u8>(30..);
    /// let result1 = target.clone().try_map(|x| x.checked_mul(2));
    /// let result2 = target.clone().try_map(|x| x.checked_mul(10));
    /// assert_eq!(result1, Some(rw::new(60..)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn try_map<F, U>(self, f: F) -> Option<<Self as RangeSrc<T>>::Range<U>>
    where
        F: FnMut(T) -> Option<U>,
        Self: RangeSrc<T> + RangeParts<T>,
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
    /// let target = rw::new::<_, u8>(30..60);
    /// let result1 = target.checked_shl(10);
    /// let result2 = target.checked_shl(40);
    /// assert_eq!(result1, Some(rw::new(20..50)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn checked_shl(&self, value: impl Borrow<T>) -> Option<Self>
    where
        T: Sized,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
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
    /// let target = rw::new::<_, u8>(30..60);
    /// let result1 = target.checked_shr(10);
    /// let result2 = target.checked_shr(200);
    /// assert_eq!(result1, Some(rw::new(40..70)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn checked_shr(&self, value: impl Borrow<T>) -> Option<Self>
    where
        T: Sized,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
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
    /// let target = rw::new::<_, u8>(30..60);
    /// let result1 = target.checked_add_start(10);
    /// let result2 = target.checked_add_start(250);
    /// assert_eq!(result1, Some(rw::new(40..60)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn checked_add_start(&self, value: impl Borrow<T>) -> Option<Self>
    where
        T: Clone,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
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
    /// let target = rw::new::<_, u8>(30..60);
    /// let result1 = target.checked_add_end(10);
    /// let result2 = target.checked_add_end(250);
    /// assert_eq!(result1, Some(rw::new(30..70)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn checked_add_end(&self, value: impl Borrow<T>) -> Option<Self>
    where
        T: Clone,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
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
    /// let target = rw::new::<_, u8>(30..60);
    /// let result1 = target.checked_sub_start(10);
    /// let result2 = target.checked_sub_start(40);
    /// assert_eq!(result1, Some(rw::new(20..60)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn checked_sub_start(&self, value: impl Borrow<T>) -> Option<Self>
    where
        T: Clone,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
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
    /// let target = rw::new::<_, u8>(30..60);
    /// let result1 = target.checked_sub_end(10);
    /// let result2 = target.checked_sub_end(70);
    /// assert_eq!(result1, Some(rw::new(30..50)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn checked_sub_end(&self, value: impl Borrow<T>) -> Option<Self>
    where
        T: Clone,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
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
    /// let target = rw::new::<_, u8>(30..60);
    /// let r1 = target.checked_calc_start(40);
    /// let r2 = target.checked_calc_start(70);
    /// assert_eq!(r1, Some(rw::new(20..60)));
    /// assert_eq!(r2, None);
    /// ```
    pub fn checked_calc_start(&self, width: impl Borrow<T>) -> Option<Self>
    where
        T: Clone,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
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
    /// let target = rw::new::<_, u8>(30..60);
    /// let r1 = target.checked_calc_end(40);
    /// let r2 = target.checked_calc_end(230);
    /// assert_eq!(r1, Some(rw::new(30..70)));
    /// assert_eq!(r2, None);
    /// ```
    pub fn checked_calc_end(&self, width: impl Borrow<T>) -> Option<Self>
    where
        T: Clone,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
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
    /// let target = rw::new::<_, u8>(30..60);
    /// let result1 = target.checked_align_start(40);
    /// let result2 = target.checked_align_start(230);
    /// assert_eq!(result1, Some(rw::new(40..70)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn checked_align_start(&self, value: impl Borrow<T>) -> Option<Self>
    where
        T: Clone + PartialOrd,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
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
    /// let target = rw::new::<_, u8>(30..60);
    /// let result1 = target.checked_align_end(70);
    /// let result2 = target.checked_align_end(20);
    /// assert_eq!(result1, Some(rw::new(40..70)));
    /// assert_eq!(result2, None);
    /// ```
    pub fn checked_align_end(&self, value: impl Borrow<T>) -> Option<Self>
    where
        T: Clone + PartialOrd,
        Self: RangeSrc<T, Range<T> = Self>,
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
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
    /// assert!(rw::new(30..60).equiv(&(30..60)));
    /// assert!(!rw::new(30..60).equiv(&(30..65)));
    /// assert!(rw::new(30..30).equiv(&(0..0)));
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
    /// ```
    /// use rich_range::prelude::*;
    ///
    /// assert!(rw::new(30..60).intersects(&(50..70)));
    /// assert!(rw::new(30..60).intersects(&(50..50)));
    /// assert!(!rw::new(30..60).intersects(&(70..80)));
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
    /// assert!(rw::new(30..60).includes(&(40..50)));
    /// assert!(!rw::new(30..60).includes(&(70..80)));
    /// assert!(!rw::new(30..60).includes(&(60..60)));
    /// assert!(rw::new(30..30).includes(&(30..30)));
    /// assert!(!rw::new(30..30).includes(&(40..40)));
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
    /// assert!(rw::new(30..60).included(&(20..70)));
    /// assert!(!rw::new(30..60).included(&(40..70)));
    /// assert!(rw::new(30..30).included(&(30..30)));
    /// assert!(!rw::new(30..30).included(&(40..40)));
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
    /// assert!(rw::new(30..=60).adjoins(&(60..70)));
    /// assert!(!rw::new(30..=60).adjoins(&(70..80)));
    /// assert!(rw::new(30..60).adjoins(&(20..=30)));
    /// assert!(!rw::new(30..60).adjoins(&(10..=20)));
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
    /// assert!(rw::new(30..60).adjoins_prev(&(20..=30)));
    /// assert!(!rw::new(30..60).adjoins_prev(&(10..=20)));
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
    /// assert!(rw::new(30..=60).adjoins_next(&(60..70)));
    /// assert!(!rw::new(30..=60).adjoins_next(&(70..80)));
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
    /// assert!(rw::new(30..60).touches(&(60..70)));
    /// assert!(!rw::new(30..60).touches(&(70..80)));
    /// assert!(rw::new(30..60).touches(&(20..30)));
    /// assert!(!rw::new(30..60).touches(&(10..20)));
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
    /// assert!(rw::new(30..60).touches_prev(&(20..30)));
    /// assert!(!rw::new(30..60).touches_prev(&(10..20)));
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
    /// assert!(rw::new(30..60).touches_next(&(60..70)));
    /// assert!(!rw::new(30..60).touches_next(&(70..80)));
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
    /// let target = rw::new(30..60);
    /// let result = target.rel(&rw::new(20..70), PosStyle::Step);
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
    /// let r = rw::new(30..60);
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
    /// let target = rw::new(30..60);
    /// let r1 = target.interval(&rw::new(50..70));
    /// let r2 = target.interval(&rw::new(60..80));
    /// let r3 = target.interval(&rw::new(70..90));
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
    /// let target = rw::new(30..60);
    /// let r1 = target.interval_adv(&rw::new(50..70), CursorMode::Off);
    /// let r2 = target.interval_adv(&rw::new(60..80), CursorMode::Off);
    /// let r3 = target.interval_adv(&rw::new(60..80), CursorMode::On);
    /// let r4 = target.interval_adv(&rw::new(70..90), CursorMode::Off);
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
    /// let r = rw::new(30..60).prod(&rw::new(40..70));
    /// assert_eq!(r, Some(rw::new(40..60)));
    /// ```
    pub fn prod(&self, other: &Self) -> Option<Self>
    where
        Self: RangeSrc<T, Range<T> = Self>,
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
    /// let r1 = rw::new(20..40).enwrap(&rw::new(30..50));
    /// let r2 = rw::new(10..20).enwrap(&rw::new(40..60));
    /// assert_eq!(r1, Some(rw::new(20..50)));
    /// assert_eq!(r2, Some(rw::new(10..60)));
    /// ```
    pub fn enwrap(&self, other: &Self) -> Option<Self>
    where
        Self: RangeSrc<T, Range<T> = Self>,
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
    /// let r1 = rw::new(30..60).union(&rw::new(40..70));
    /// let r2 = rw::new(30..60).union(&rw::new(70..80));
    /// assert_eq!(r1, (rw::new(30..70), None));
    /// assert_eq!(r2, (rw::new(30..60), Some(rw::new(70..80))));
    /// ```
    pub fn union(&self, other: &Self) -> (Self, Option<Self>)
    where
        Self: RangeSrc<T, Range<T> = Self>,
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
    /// let r1 = rw::new(30..60).diff(&(50..70));
    /// let r2 = rw::new(30..60).diff(&(40..50));
    /// let r3 = rw::new(30..60).diff(&(40..40));
    /// assert_eq!(r1, [Some(ru::new(30..50)), None]);
    /// assert_eq!(r2, [Some(ru::new(30..40)), Some(ru::new(50..60))]);
    /// assert_eq!(r3, [Some(ru::new(30..60)), None]);
    /// ```
    pub fn diff<R2>(&self, other: &R2) -> [Option<RangeUniv<T>>; 2]
    where
        R2: RangeBounds<T>,
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
    /// let r1 = rw::new(30..60).diff_adv(&(50..70), CursorMode::Off);
    /// let r2 = rw::new(30..60).diff_adv(&(40..50), CursorMode::Off);
    /// let r3 = rw::new(30..60).diff_adv(&(40..40), CursorMode::Off);
    /// let r4 = rw::new(30..60).diff_adv(&(40..40), CursorMode::On);
    /// assert_eq!(r1, [Some(ru::new(30..50)), None]);
    /// assert_eq!(r2, [Some(ru::new(30..40)), Some(ru::new(50..60))]);
    /// assert_eq!(r3, [Some(ru::new(30..60)), None]);
    /// assert_eq!(r4, [Some(ru::new(30..40)), Some(ru::new(40..60))]);
    /// ```
    pub fn diff_adv<R2>(&self, other: &R2, mode: CursorMode) -> [Option<RangeUniv<T>>; 2]
    where
        R2: RangeBounds<T>,
        T: Clone + PartialOrd,
    {
        RichRangeBounds::diff_adv(self, other, mode)
    }
}

impl<R, T> RangeBounds<T> for RangeWrapper<R, T>
where
    R: RangeBounds<T>,
{
    #[inline]
    fn start_bound(&self) -> Bound<&T> {
        self.0.start_bound()
    }

    #[inline]
    fn end_bound(&self) -> Bound<&T> {
        self.0.end_bound()
    }
}

impl<R, T> RichRangeBounds<T> for RangeWrapper<R, T>
where
    R: RangeBounds<T>,
{
    // nop.
}

impl<R, T> IntoIterator for RangeWrapper<R, T>
where
    R: RangeBounds<T>,
    T: Step,
{
    type Item = T;
    type IntoIter = IterRichRange<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<R, T> Eq for RangeWrapper<R, T>
where
    R: RangeBounds<T>,
    T: Eq,
{
    // nop.
}

impl<R, T> PartialEq for RangeWrapper<R, T>
where
    R: RangeBounds<T>,
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        calc::is_eq(self, other)
    }
}

impl<R, T> PartialOrd for RangeWrapper<R, T>
where
    R: RangeBounds<T>,
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
    /// let r = rw::new(30..60).partial_cmp(&rw::new(40..70));
    /// assert_eq!(r, None);
    ///
    /// let r = rw::new(30..60).partial_cmp(&rw::new(70..90));
    /// assert_eq!(r, Some(Ordering::Less));
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        calc::cmp(self, other)
    }
}

/// Implement [`Shl`] (with reference or value parameters).
macro_rules! impl_shl {
    (($($lhsRef:tt)?) ($($rhsRef:tt)?) $(, main:$main:meta)? $(, sub:$sub:meta)?) => {
        impl<R, T> Shl<$($rhsRef)?T> for $($lhsRef)?RangeWrapper<R, T>
        where
            R: RangeBounds<T>,
            for<'a> &'a T: Sub<&'a T, Output = T>,
            RangeWrapper<R, T>: RangeSrc<T, Range<T> = RangeWrapper<R, T>>,
        {
            type Output = RangeWrapper<R, T>;

            $(
                #[$main]
                /// Performs the `<<` operation.
                /// <a id="main_shl" style="visibility:hidden;"></a>
                ///
                /// Returns a new range with both ends subtracted by given value.
                ///
                /// # Panics
                ///
                /// Panics if position of the bound is overflowed.
                ///
                /// # Examples
                ///
                /// ```
                /// use rich_range::prelude::*;
                ///
                /// assert_eq!(rw::new(30..60) << 10, rw::new(20..50));
                /// assert_eq!(rw::new(30..) << 10, rw::new(20..));
                /// ```
            )?
            $(
                #[$sub]
                /// See main overload [document](RangeUniv::shl).
            )?
            fn shl(self, rhs: $($rhsRef)?T) -> Self::Output {
                calc::shl(util::to_ref!($($lhsRef)?, self), rhs)
            }
        }
    }
}

/// Implement [`Shr`] (with reference or value parameters).
macro_rules! impl_shr {
    (($($lhsRef:tt)?) ($($rhsRef:tt)?) $(, main:$main:meta)? $(, sub:$sub:meta)?) => {
        impl<R, T> Shr<$($rhsRef)?T> for $($lhsRef)?RangeWrapper<R, T>
        where
            R: RangeBounds<T>,
            for<'a> &'a T: Add<&'a T, Output = T>,
            RangeWrapper<R, T>: RangeSrc<T, Range<T> = RangeWrapper<R, T>>,
        {
            type Output = RangeWrapper<R, T>;

            $(
                #[$main]
                /// Performs the `>>` operation.
                /// <a id="main_shr" style="visibility:hidden;"></a>
                ///
                /// Returns a new range with both ends added by given value.
                ///
                /// # Panics
                ///
                /// Panics if position of the bound is overflowed.
                ///
                /// # Examples
                ///
                /// ```
                /// use rich_range::prelude::*;
                ///
                /// assert_eq!(rw::new(30..60) >> 10, rw::new(40..70));
                /// assert_eq!(rw::new(30..) >> 10, rw::new(40..));
                /// ```
            )?
            $(
                #[$sub]
                /// See main overload [document](RangeUniv::shr).
            )?
            fn shr(self, rhs: $($rhsRef)?T) -> Self::Output {
                calc::shr(util::to_ref!($($lhsRef)?, self), rhs)
            }
        }
    }
}

/// Implement [`BitAnd`] (with reference or value parameters).
macro_rules! impl_bitand {
    (($($lhsRef:tt)?) ($($rhsRef:tt)?) $(, main:$main:meta)? $(, sub:$sub:meta)?) => {
        impl<R, T> BitAnd<$($rhsRef)?RangeWrapper<R, T>> for $($lhsRef)?RangeWrapper<R, T>
        where
            R: RangeBounds<T>,
            T: Clone + PartialOrd + HasLimits,
            RangeWrapper<R, T>: RangeSrc<T, Range<T> = RangeWrapper<R, T>>,
        {
            type Output = RangeWrapper<R, T>;

            $(
                #[$main]
                /// Performs the `&` operation.
                ///
                /// Returns the shared range of two ranges.
                ///
                /// # Notes
                ///
                /// If two range has no intersection, returns [broken empty][eh].
                ///
                /// [eh]: crate::RichRangeBounds#empty-handling
                ///
                /// # Panics
                ///
                /// Panics if range has unordered position like NaN.
                ///
                /// # Examples
                ///
                /// ```
                /// use rich_range::prelude::*;
                ///
                /// assert_eq!(rw::new(30..60) & rw::new(40..70), rw::new(40..60));
                /// assert!((rw::new(30..60) & rw::new(70..80)).is_broken());
                /// ```
            )?
            $(
                #[$sub]
                /// See main overload [document](RangeUniv::bitand).
            )?
            fn bitand(self, rhs: $($rhsRef)?RangeWrapper<R, T>) -> Self::Output {
                let rx = util::to_ref!($($lhsRef)?, self);
                let ry = util::to_ref!($($rhsRef)?, rhs);
                calc::closed_prod(rx, ry)
            }
        }
    }
}

/// Implement [`BitOr`] (with reference or value parameters).
macro_rules! impl_bitor {
    (($($lhsRef:tt)?) ($($rhsRef:tt)?) $(, main:$main:meta)? $(, sub:$sub:meta)?) => {
        impl<R, T> BitOr<$($rhsRef)?RangeWrapper<R, T>> for $($lhsRef)?RangeWrapper<R, T>
        where
            R: RangeBounds<T>,
            T: Clone + PartialOrd,
            RangeWrapper<R, T>: RangeSrc<T, Range<T> = RangeWrapper<R, T>>,
        {
            type Output = RangeWrapper<R, T>;

            $(
                #[$main]
                /// Performs the `|` operation.
                ///
                /// Returns the merged ranges of two ranges.
                ///
                /// # Panics
                ///
                /// Panics if range has unordered position like NaN.
                ///
                /// # Examples
                ///
                /// ```
                /// use rich_range::prelude::*;
                ///
                /// assert_eq!(rw::new(30..60) | rw::new(40..70), rw::new(30..70));
                /// assert_eq!(rw::new(30..60) | rw::new(70..80), rw::new(30..60));
                /// ```
            )?
            $(
                #[$sub]
                /// See main overload [document](RangeUniv::bitor).
            )?
            fn bitor(self, rhs: $($rhsRef)?RangeWrapper<R, T>) -> Self::Output {
                let rx = util::to_ref!($($lhsRef)?, self);
                let ry = util::to_ref!($($rhsRef)?, rhs);
                calc::closed_union(rx, ry)
            }
        }
    }
}

/// Implement [`BitXor`] (with reference or value parameters).
macro_rules! impl_bitxor {
    (($($lhsRef:tt)?) ($($rhsRef:tt)?) $(, main:$main:meta)? $(, sub:$sub:meta)?) => {
        impl<R, T> BitXor<$($rhsRef)?RangeWrapper<R, T>> for $($lhsRef)?RangeWrapper<R, T>
        where
            R: RangeBounds<T>,
            T: Clone + PartialOrd + HasLimits,
            RangeWrapper<R, T>: RangeSrc<T, Range<T> = RangeWrapper<R, T>>,
        {
            type Output = RangeWrapper<R, T>;

            $(
                #[$main]
                /// Performs the `^` operation.
                ///
                /// Returns the super range of two ranges.
                ///
                /// # Note
                ///
                /// - Both range is empty, returns [broken empty][eh].
                /// - One range is empty, returns the other range.
                ///
                /// [eh]: crate::RichRangeBounds#empty-handling
                ///
                /// # Panics
                ///
                /// Panics if range has unordered position like NaN.
                ///
                /// # Examples
                ///
                /// ```
                /// use rich_range::prelude::*;
                /// use std::ops::Range;
                ///
                /// assert_eq!(rw::new(30..40) ^ rw::new(60..70), rw::new(30..70));
                /// assert!((rw::new(30..30) ^ rw::new(60..60)).is_broken());
                /// ```
            )?
            $(
                #[$sub]
                /// See main overload [document](RangeUniv::bitxor).
            )?
            fn bitxor(self, rhs: $($rhsRef)?RangeWrapper<R, T>) -> Self::Output {
                let rx = util::to_ref!($($lhsRef)?, self);
                let ry = util::to_ref!($($rhsRef)?, rhs);
                calc::closed_enwrap(rx, ry)
            }
        }
    }
}

/// Implement [`ShlAssign`] (with reference or value parameters).
macro_rules! impl_shl_assign {
    (($($rhsRef:tt)?) $(, main:$main:meta)? $(, sub:$sub:meta)?) => {
        impl<R, T> ShlAssign<$($rhsRef)?T> for RangeWrapper<R, T>
        where
            R: RangeBounds<T>,
            T: Clone,
            for<'a> &'a T: Sub<&'a T, Output = T>,
            RangeWrapper<R, T>: RangeSrc<T, Range<T> = RangeWrapper<R, T>>,
        {
            $(
                #[$main]
                /// Performs the `<<=` operation.
                /// <a id="main_shl_assign" style="visibility:hidden;"></a>
                ///
                /// Assigns range with both ends subtracted by given value.
                ///
                /// # Panics
                ///
                /// Panics if position of the bound is overflowed.
                ///
                /// # Examples
                ///
                /// ```
                /// use rich_range::prelude::*;
                ///
                /// let mut r = rw::new(30..60);
                /// r <<= 10;
                /// assert_eq!(r, rw::new(20..50));
                /// ```
            )?
            $(
                #[$sub]
                /// See main overload [document](RangeUniv::shl_assign).
            )?
            fn shl_assign(&mut self, rhs: $($rhsRef)?T) {
                *self = calc::shl(self, rhs);
            }
        }
    }
}

/// Implement [`ShrAssign`] (with reference or value parameters).
macro_rules! impl_shr_assign {
    (($($rhsRef:tt)?) $(, main:$main:meta)? $(, sub:$sub:meta)?) => {
        impl<R, T> ShrAssign<$($rhsRef)?T> for RangeWrapper<R, T>
        where
            R: RangeBounds<T>,
            T: Clone,
            for<'a> &'a T: Add<&'a T, Output = T>,
            RangeWrapper<R, T>: RangeSrc<T, Range<T> = RangeWrapper<R, T>>,
        {
            $(
                #[$main]
                /// Performs the `>>=` operation.
                /// <a id="main_shr_assign" style="visibility:hidden;"></a>
                ///
                /// Assigns range with both ends subtracted by given value.
                ///
                /// # Panics
                ///
                /// Panics if position of the bound is overflowed.
                ///
                /// # Examples
                ///
                /// ```
                /// use rich_range::prelude::*;
                ///
                /// let mut r = rw::new(30..60);
                /// r >>= 10;
                /// assert_eq!(r, rw::new(40..70));
                /// ```
            )?
            $(
                #[$sub]
                /// See main overload [document](RangeUniv::shr_assign).
            )?
            fn shr_assign(&mut self, rhs: $($rhsRef)?T) {
                *self = calc::shr(self, rhs);
            }
        }
    }
}

/// Implement [`BitAndAssign`] (with reference or value parameters).
macro_rules! impl_bitand_assign {
    (($($rhsRef:tt)?) $(, main:$main:meta)? $(, sub:$sub:meta)?) => {
        impl<R, T> BitAndAssign<$($rhsRef)?RangeWrapper<R, T>> for RangeWrapper<R, T>
        where
            R: RangeBounds<T>,
            T: Clone + PartialOrd + HasLimits,
            RangeWrapper<R, T>: RangeSrc<T, Range<T> = RangeWrapper<R, T>>,
        {
            $(
                #[$main]
                /// Performs the `&=` operation.
                ///
                /// Assigns shared range of two ranges.
                ///
                /// # Notes
                ///
                /// If two range has no intersection, assigns [broken empty][eh].
                ///
                /// [eh]: crate::RichRangeBounds#empty-handling
                ///
                /// # Panics
                ///
                /// Panics if the range has unordered position like NaN.
                ///
                /// # Examples
                ///
                /// ```
                /// use rich_range::prelude::*;
                ///
                /// let mut r = rw::new(30..60);
                /// r &= rw::new(40..70);
                /// assert_eq!(r, rw::new(40..60));
                ///
                /// let mut r = rw::new(30..60);
                /// r &= rw::new(70..80);
                /// assert!(r.is_broken());
                /// ```
            )?
            $(
                #[$sub]
                /// See main overload [document](RangeUniv::bitand_assign).
            )?
            fn bitand_assign(&mut self, rhs: $($rhsRef)?RangeWrapper<R, T>) {
                *self = calc::closed_prod(self, &rhs);
            }
        }
    }
}

/// Implement [`BitOrAssign`] (with reference or value parameters).
macro_rules! impl_bitor_assign {
    (($($rhsRef:tt)?) $(, main:$main:meta)? $(, sub:$sub:meta)?) => {
        impl<R, T> BitOrAssign<$($rhsRef)?RangeWrapper<R, T>> for RangeWrapper<R, T>
        where
            R: RangeBounds<T>,
            T: Clone + PartialOrd,
            RangeWrapper<R, T>: RangeSrc<T, Range<T> = RangeWrapper<R, T>>,
        {
            $(
                #[$main]
                /// Performs the `|=` operation.
                ///
                /// Assigns merged range of two ranges.
                ///
                /// # Panics
                ///
                /// Panics if the range has unordered position like NaN.
                ///
                /// # Examples
                ///
                /// ```
                /// use rich_range::prelude::*;
                ///
                /// let mut r = rw::new(30..60);
                /// r |= rw::new(40..70);
                /// assert_eq!(r, rw::new(30..70));
                ///
                /// let mut r = rw::new(30..60);
                /// r |= rw::new(70..80);
                /// assert_eq!(r, rw::new(30..60));
                /// ```
            )?
            $(
                #[$sub]
                /// See main overload [document](RangeUniv::bitor_assign).
            )?
            fn bitor_assign(&mut self, rhs: $($rhsRef)?RangeWrapper<R, T>) {
                *self = calc::closed_union(self, &rhs);
            }
        }
    }
}

/// Implement [`BitXorAssign`] (with reference or value parameters).
macro_rules! impl_bitxor_assign {
    (($($rhsRef:tt)?) $(, main:$main:meta)? $(, sub:$sub:meta)?) => {
        impl<R, T> BitXorAssign<$($rhsRef)?RangeWrapper<R, T>> for RangeWrapper<R, T>
        where
            R: RangeBounds<T>,
            T: Clone + PartialOrd + HasLimits,
            RangeWrapper<R, T>: RangeSrc<T, Range<T> = RangeWrapper<R, T>>,
        {
            $(
                #[$main]
                /// Performs the `^=` operation.
                ///
                /// Assigns super range of two ranges.
                ///
                /// # Note
                ///
                /// - Both range is empty, assigns [broken empty][eh].
                /// - One range is empty, assigns the other range.
                ///
                /// [eh]: crate::RichRangeBounds#empty-handling
                ///
                /// # Panics
                ///
                /// Panics if the range has unordered position like NaN.
                ///
                /// # Examples
                ///
                /// ```
                /// use rich_range::prelude::*;
                ///
                /// let mut r = rw::new(30..40);
                /// r ^= rw::new(50..60);
                /// assert_eq!(r, rw::new(30..60));
                ///
                /// let mut r = rw::new(30..30);
                /// r ^= rw::new(60..60);
                /// assert!(r.is_broken());
                /// ```
            )?
            $(
                #[$sub]
                /// See main overload [document](RangeUniv::bitxor_assign).
            )?
            fn bitxor_assign(&mut self, rhs: $($rhsRef)?RangeWrapper<R, T>) {
                *self = calc::closed_enwrap(self, &rhs);
            }
        }
    }
}

impl_shl!(() (), main:doc = "");
impl_shl!(() (&), sub:doc = "");
impl_shl!((&) (), sub:doc = "");
impl_shl!((&) (&), sub:doc = "");
impl_shr!(() (), main:doc = "");
impl_shr!(() (&), sub:doc = "");
impl_shr!((&) (), sub:doc = "");
impl_shr!((&) (&), sub:doc = "");
impl_bitand!(() (), main:doc = "");
impl_bitand!(() (&), sub:doc = "");
impl_bitand!((&) (), sub:doc = "");
impl_bitand!((&) (&), sub:doc = "");
impl_bitor!(() (), main:doc = "");
impl_bitor!(() (&), sub:doc = "");
impl_bitor!((&) (), sub:doc = "");
impl_bitor!((&) (&), sub:doc = "");
impl_bitxor!(() (), main:doc = "");
impl_bitxor!(() (&), sub:doc = "");
impl_bitxor!((&) (), sub:doc = "");
impl_bitxor!((&) (&), sub:doc = "");
impl_shl_assign!((), main:doc = "");
impl_shl_assign!((&), sub:doc = "");
impl_shr_assign!((), main:doc = "");
impl_shr_assign!((&), sub:doc = "");
impl_bitand_assign!((), main:doc = "");
impl_bitand_assign!((&), sub:doc = "");
impl_bitor_assign!((), main:doc = "");
impl_bitor_assign!((&), sub:doc = "");
impl_bitxor_assign!((), main:doc = "");
impl_bitxor_assign!((&), sub:doc = "");
