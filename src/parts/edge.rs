//! Provider of [`Edge`].

use crate::parts::*;
use crate::shorthands::aliases::*;
use core::cmp::Ordering;
use core::ops::{Bound, RangeBounds};

/// Edge value.
///
/// Value is formed by range side (start/end) and range bound.
///
/// # Ordering rule
///
/// Values are ordered in the following sequence.
///
/// | No. | Bound variant | Side  | Position |
/// | :-: | ------------- | ----- | -------- |
/// |  1  | Unbounded     | Start | -        |
/// |  2  | Excluded      | End   | Small    |
/// |  3  | Included      | -     | Small    |
/// |  4  | Included      | -     | Large    |
/// |  5  | Excluded      | Start | Large    |
/// |  6  | Unbounded     | End   | -        |
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Edge<T> {
    /// Side of this edge.
    pub side: Side,

    /// Bound of this edge.
    pub bound: Bound<T>,
}

impl<T> Edge<T> {
    /// Creates a new instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let edge = Edge::new(Side::S, Included(42));
    /// assert_eq!(edge.side(), Side::S);
    /// assert_eq!(edge.bound(), Included(42));
    /// ```
    #[inline]
    pub fn new(side: Side, bound: Bound<T>) -> Self {
        Self { side, bound }
    }

    /// Returns `true` if this edge bound variant is [`Unbounded`](Ub).
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let t1 = Edge::<i32>::new(Side::S, Unbounded);
    /// let t2 = Edge::new(Side::S, Included(42));
    /// assert!(t1.is_unbounded());
    /// assert!(!t2.is_unbounded());
    /// ```
    #[inline]
    pub fn is_unbounded(&self) -> bool {
        matches!(self.bound.as_ref(), Ub)
    }

    /// Returns `true` if this edge bound variant is [`Included`](In).
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let t1 = Edge::new(Side::S, Included(42));
    /// let t2 = Edge::new(Side::S, Excluded(42));
    /// assert!(t1.is_included());
    /// assert!(!t2.is_included());
    /// ```
    #[inline]
    pub fn is_included(&self) -> bool {
        matches!(self.bound.as_ref(), In(_))
    }

    /// Returns `true` if this edge bound variant is [`Excluded`](Ex).
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let t1 = Edge::new(Side::S, Excluded(42));
    /// let t2 = Edge::new(Side::S, Included(42));
    /// assert!(t1.is_excluded());
    /// assert!(!t2.is_excluded());
    /// ```
    #[inline]
    pub fn is_excluded(&self) -> bool {
        matches!(self.bound.as_ref(), Ex(_))
    }

    /// Returns side of this edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let edge = Edge::new(Side::S, Included(42));
    /// assert_eq!(edge.side(), Side::S);
    /// ```
    #[inline]
    pub fn side(self) -> Side {
        self.side
    }

    /// Returns bound of this edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let edge = Edge::new(Side::S, Included(42));
    /// assert_eq!(edge.bound(), Included(42));
    /// ```
    #[inline]
    pub fn bound(self) -> Bound<T> {
        self.bound
    }

    /// Returns position of this edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let t1 = Edge::new(Side::S, Included(42));
    /// let t2 = Edge::new(Side::S, Excluded(42));
    /// let t3 = Edge::<i32>::new(Side::S, Unbounded);
    /// assert_eq!(t1.pos(), Some(42));
    /// assert_eq!(t2.pos(), Some(42));
    /// assert_eq!(t3.pos(), None);
    /// ```
    #[inline]
    pub fn pos(self) -> Option<T> {
        bound(self.bound).pos()
    }

    /// Converts from a reference to a value with reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let edge = Edge::new(Side::S, Included(42));
    /// let result = edge.as_ref();
    /// assert_eq!(result.bound(), Included(&42));
    /// ```
    #[inline]
    pub fn as_ref(&self) -> Edge<&T> {
        Edge::new(self.side, self.bound.as_ref())
    }

    /// Returns new instance with given bound.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let edge = Edge::new(Side::S, Included(42));
    /// let result = edge.with_bound(Excluded(43));
    /// assert_eq!(result, Edge::new(Side::S, Excluded(43)));
    /// ```
    #[inline]
    pub fn with_bound(self, value: Bound<T>) -> Self {
        Self::new(self.side, value)
    }

    /// Returns new instance with given Included/Excluded.
    ///
    /// If this edge bound is unbounded, returns self.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let t1 = Edge::new(Side::S, Included(42));
    /// let t2 = Edge::<i32>::new(Side::S, Unbounded);
    /// assert_eq!(t1.with_included(false), Edge::new(Side::S, Excluded(42)));
    /// assert_eq!(t2.with_included(false), Edge::new(Side::S, Unbounded));
    /// ```
    #[inline]
    pub fn with_included(self, included: bool) -> Self {
        Self::new(self.side, bound(self.bound).with_included(included))
    }

    /// Returns new instance with given position.
    ///
    /// If this edge bound is unbounded, returns self.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let t1 = Edge::new(Side::S, Included(42));
    /// let t2 = Edge::new(Side::S, Excluded(42));
    /// let t3 = Edge::new(Side::S, Unbounded);
    /// assert_eq!(t1.with_pos(43), Edge::new(Side::S, Included(43)));
    /// assert_eq!(t2.with_pos(43), Edge::new(Side::S, Excluded(43)));
    /// assert_eq!(t3.with_pos(43), Edge::new(Side::S, Unbounded));
    /// ```
    #[inline]
    pub fn with_pos(self, value: T) -> Self {
        Self::new(self.side, bound(self.bound).with_pos(value))
    }

    /// Returns a new instance with mapped position.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let edge = Edge::new(Side::S, Included(42));
    /// let result = edge.map(|x| x + 3);
    /// assert_eq!(result, Edge::new(Side::S, Included(45)));
    /// ```
    #[inline]
    pub fn map<F, U>(self, f: F) -> Edge<U>
    where
        F: FnMut(T) -> U,
    {
        Edge::new(self.side, self.bound.map(f))
    }

    /// Try to return a new instance with mapped position.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let edge = Edge::new(Side::S, Included(42_u8));
    /// let r1 = edge.try_map(|x| x.checked_add(200));
    /// let r2 = edge.try_map(|x| x.checked_add(220));
    /// assert_eq!(r1, Some(Edge::new(Side::S, Included(242))));
    /// assert_eq!(r2, None);
    /// ```
    #[inline]
    pub fn try_map<F, U>(self, f: F) -> Option<Edge<U>>
    where
        F: FnMut(T) -> Option<U>,
    {
        Some(Edge::new(self.side, bound(self.bound).try_map(f)?))
    }
}

impl<T> Edge<&T> {
    /// Returns a new instance that clones the reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use rich_range::parts::*;
    /// use std::ops::Bound::*;
    ///
    /// let edge = Edge::new(Side::S, Included(&42));
    /// let result = edge.cloned();
    /// assert_eq!(result, Edge::new(Side::S, Included(42)));
    /// ```
    #[inline]
    pub fn cloned(&self) -> Edge<T>
    where
        T: Clone,
    {
        Edge::new(self.side, self.bound.cloned())
    }
}

/// Crate private methods.
impl<T> Edge<T> {
    /// Returns `true` if this edge is contained in given range.
    pub(crate) fn is_contained<R>(&self, range: &R) -> bool
    where
        T: PartialOrd,
        R: ?Sized + RangeBounds<T>,
    {
        let rs = Edge::new(Side::S, range.start_bound());
        let re = Edge::new(Side::E, range.end_bound());
        rs <= self.as_ref() && self.as_ref() <= re
    }
}

/// Private methods.
impl<T> Edge<T> {
    /// Returns compare parts.
    fn parts(&self) -> (i8, Option<&T>, Option<i8>) {
        (self.part_inf(), self.part_pos(), self.part_bound_variant())
    }

    /// Returns compare part of infinity sign.
    fn part_inf(&self) -> i8 {
        match (self.is_unbounded(), self.side) {
            (false, _) => 0,
            (true, Side::S) => -1,
            (true, Side::E) => 1,
        }
    }

    /// Returns compare part of position.
    fn part_pos(&self) -> Option<&T> {
        self.as_ref().pos()
    }

    /// Returns compare part of bound variant.
    fn part_bound_variant(&self) -> Option<i8> {
        match (&self.bound, self.side) {
            (Ub, _) => None,
            (In(_), _) => Some(0),
            (Ex(_), Side::S) => Some(1),
            (Ex(_), Side::E) => Some(-1),
        }
    }
}

impl<T> Ord for Edge<T>
where
    T: Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.parts().cmp(&other.parts())
    }
}

impl<T> PartialOrd for Edge<T>
where
    T: PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.parts().partial_cmp(&other.parts())
    }
}
