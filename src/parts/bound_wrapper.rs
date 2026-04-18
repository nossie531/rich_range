//! Newtype for [`Bound`] and its around.

use crate::PosStyle;
use crate::shorthands::aliases::*;
use core::mem;
use core::ops::Bound;

/// Returns a [`BoundWrapper`] value from a [`Bound`] value.
#[inline]
pub(crate) fn bound<T>(bound: Bound<T>) -> BoundWrapper<T> {
    BoundWrapper::new(bound)
}

/// Returns a [`BoundWrapper`] reference from a [`Bound`] reference.
#[inline]
pub(crate) fn bound_ref<T>(bound: &Bound<T>) -> &BoundWrapper<T> {
    unsafe { mem::transmute(bound) }
}

/// Local wrapper of [`Bound`].
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct BoundWrapper<T>(pub Bound<T>);

impl<T> BoundWrapper<T> {
    /// Creates a new instance.
    pub fn new(base: Bound<T>) -> Self {
        Self(base)
    }

    /// Returns the position of the bound.
    ///
    /// If this bound is Unbounded, returns [`None`].
    pub fn pos(self) -> Option<T> {
        match self.0 {
            In(x) => Some(x),
            Ex(x) => Some(x),
            Ub => None,
        }
    }

    /// Returns bound kind as optional bool.
    ///
    /// Each bound variants are converted as follows.
    ///
    /// - [`Included`](In): `Some(true)`
    /// - [`Excluded`](Ex): `Some(false)`
    /// - [`Unbounded`](Ub): `None`
    pub fn kind(self) -> Option<bool> {
        match self.0 {
            In(_) => Some(true),
            Ex(_) => Some(false),
            Ub => None,
        }
    }

    /// Returns new instance with toggled Included/Excluded.
    ///
    /// If this bound is Unbounded, returns itself.
    pub fn toggle_included(self) -> Bound<T> {
        match self.0 {
            Ub => Ub,
            In(x) => Ex(x),
            Ex(x) => In(x),
        }
    }

    /// Returns new instance with given Included/Excluded.
    ///
    /// If this bound is Unbounded, returns itself.
    pub fn with_included(self, value: bool) -> Bound<T> {
        match self.0 {
            Ub => Ub,
            In(x) | Ex(x) => (if value { In } else { Ex })(x),
        }
    }

    /// Returns new instance with given position.
    ///
    /// If this bound is Unbounded, returns itself.
    pub fn with_pos(self, value: T) -> Bound<T> {
        match self.0 {
            Ub => Ub,
            In(_) => In(value),
            Ex(_) => Ex(value),
        }
    }

    /// Returns `true` if two bounds are adjoining.
    pub fn adjoins(&self, other: &Bound<T>) -> bool
    where
        T: PartialEq,
    {
        matches!((&self.0, other), (In(x), In(y)) if x == y)
    }

    /// Returns `true` if two bounds are touching.
    pub fn touches(&self, other: &Bound<T>) -> bool
    where
        T: PartialEq,
    {
        matches!((&self.0, other), (In(x), Ex(y)) | (Ex(x), In(y)) if x == y)
    }

    /// Returns `true` if two bounds are meeting.
    pub fn meets(&self, other: &Bound<T>, ps: PosStyle) -> bool
    where
        T: PartialEq,
    {
        match ps {
            PosStyle::Real => self.adjoins(other),
            PosStyle::Step => self.touches(other),
        }
    }

    /// Returns new instance with mapped position by given function.
    pub fn map<F, U>(self, f: F) -> Bound<U>
    where
        F: FnOnce(T) -> U,
    {
        self.0.map(f)
    }

    /// Retruns new instance with mapped position by given tolerant function.
    pub fn try_map<F, U>(self, f: F) -> Option<Bound<U>>
    where
        F: FnOnce(T) -> Option<U>,
    {
        match self.0 {
            In(x) => Some(In(f(x)?)),
            Ex(x) => Some(Ex(f(x)?)),
            Ub => Some(Ub),
        }
    }
}
