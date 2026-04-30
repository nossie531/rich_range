//! Provider of [`RangeSrc`].

use crate::conv::*;
use crate::shorthands::aliases::*;
use crate::shorthands::*;
use crate::*;
use core::ops::{Bound, RangeBounds};

/// Range construction provider.
///
/// # About custom range type
///
/// The combination of existing range types and the operations provided by this
/// crate is designed to avoid triggering [`BoundsUnfit`] errors and resulting
/// panics. However, this does not apply if you implement [`RangeSrc`] for your
/// custom range type.
pub trait RangeSrc<T>: RangeBounds<T> {
    /// Range type.
    type Range<U>: RangeSrc<U>;

    /// Creates a new range.
    ///
    /// # Panics
    ///
    /// Panics if `Self` does not support given bounds.
    fn new<U>(bounds: (Bound<U>, Bound<U>)) -> Result<Self::Range<U>, BoundsUnfit>;

    /// Creates a new [broken empty](crate::RichRangeBounds#empty-handling).
    ///
    /// # Panics
    ///
    /// Panics if `Self` does not support broken empty.
    fn new_broken() -> Result<Self::Range<T>, BoundsUnfit>
    where
        T: HasLimits;

    /// Returns new range from given range bounds.
    ///
    /// # Panics
    ///
    /// Panics if `Self` is incompatible with given range.
    fn new_from<R>(range: R) -> Result<Self::Range<T>, BoundsUnfit>
    where
        R: RangeParts<T>,
    {
        <Self as RangeSrc<_>>::new(RangeParts::parts(range))
    }
}

impl<T> RangeSrc<T> for RangeUniv<T> {
    type Range<U> = RangeUniv<U>;

    fn new<U>(bounds: (Bound<U>, Bound<U>)) -> Result<Self::Range<U>, BoundsUnfit> {
        Ok(RangeUniv::from(bounds))
    }

    fn new_broken() -> Result<Self::Range<T>, BoundsUnfit>
    where
        T: HasLimits,
    {
        Ok(RangeUniv::new_broken())
    }
}

impl<R, T> RangeSrc<T> for RangeView<'_, R, T>
where
    R: ?Sized + RangeBounds<T>,
{
    type Range<U> = RangeUniv<U>;

    fn new<U>(bounds: (Bound<U>, Bound<U>)) -> Result<Self::Range<U>, BoundsUnfit> {
        Ok(RangeUniv::from(bounds))
    }

    fn new_broken() -> Result<Self::Range<T>, BoundsUnfit>
    where
        T: HasLimits,
    {
        Ok(RangeUniv::new_broken())
    }
}

impl<T> RangeSrc<T> for RangeNormalWrapper<T> {
    type Range<U> = RangeNormalWrapper<U>;

    fn new<U>(bounds: (Bound<U>, Bound<U>)) -> Result<Self::Range<U>, BoundsUnfit> {
        match bounds {
            (In(s), Ex(e)) => Ok(rw::new(s..e)),
            _ => Err(BoundsUnfit::new::<Self, _>(&bounds)),
        }
    }

    fn new_broken() -> Result<Self::Range<T>, BoundsUnfit>
    where
        T: HasLimits,
    {
        Ok(rw::new(T::MAX..T::MIN))
    }
}

impl<T> RangeSrc<T> for RangeFromWrapper<T> {
    type Range<U> = RangeFromWrapper<U>;

    fn new<U>(bounds: (Bound<U>, Bound<U>)) -> Result<Self::Range<U>, BoundsUnfit> {
        match bounds {
            (In(s), Ub) => Ok(rw::new(s..)),
            _ => Err(BoundsUnfit::new::<Self, _>(&bounds)),
        }
    }

    fn new_broken() -> Result<Self::Range<T>, BoundsUnfit>
    where
        T: HasLimits,
    {
        Err(BoundsUnfit::new_broken::<Self>())
    }
}

impl<T> RangeSrc<T> for RangeToWrapper<T> {
    type Range<U> = RangeToWrapper<U>;

    fn new<U>(bounds: (Bound<U>, Bound<U>)) -> Result<Self::Range<U>, BoundsUnfit> {
        match bounds {
            (Ub, Ex(e)) => Ok(rw::new(..e)),
            _ => Err(BoundsUnfit::new::<Self, _>(&bounds)),
        }
    }

    fn new_broken() -> Result<Self::Range<T>, BoundsUnfit>
    where
        T: HasLimits,
    {
        Err(BoundsUnfit::new_broken::<Self>())
    }
}

impl<T> RangeSrc<T> for RangeInclusiveWrapper<T> {
    type Range<U> = RangeInclusiveWrapper<U>;

    fn new<U>(bounds: (Bound<U>, Bound<U>)) -> Result<Self::Range<U>, BoundsUnfit> {
        match bounds {
            (In(s), In(e)) => Ok(rw::new(s..=e)),
            _ => Err(BoundsUnfit::new::<Self, _>(&bounds)),
        }
    }

    fn new_broken() -> Result<Self::Range<T>, BoundsUnfit>
    where
        T: HasLimits,
    {
        Ok(rw::new(T::MAX..=T::MIN))
    }
}

impl<T> RangeSrc<T> for RangeToInclusiveWrapper<T> {
    type Range<U> = RangeToInclusiveWrapper<U>;

    fn new<U>(bounds: (Bound<U>, Bound<U>)) -> Result<Self::Range<U>, BoundsUnfit> {
        match bounds {
            (Ub, In(e)) => Ok(rw::new(..=e)),
            _ => Err(BoundsUnfit::new::<Self, _>(&bounds)),
        }
    }

    fn new_broken() -> Result<Self::Range<T>, BoundsUnfit>
    where
        T: HasLimits,
    {
        Err(BoundsUnfit::new_broken::<Self>())
    }
}

impl<T> RangeSrc<T> for RangeFullWrapper<T> {
    type Range<U> = RangeFullWrapper<U>;

    fn new<U>(bounds: (Bound<U>, Bound<U>)) -> Result<Self::Range<U>, BoundsUnfit> {
        match bounds {
            (Ub, Ub) => Ok(rw::new(..)),
            _ => Err(BoundsUnfit::new::<Self, _>(&bounds)),
        }
    }

    fn new_broken() -> Result<Self::Range<T>, BoundsUnfit>
    where
        T: HasLimits,
    {
        Err(BoundsUnfit::new_broken::<Self>())
    }
}

impl<T> RangeSrc<T> for RangeBoundsWrapper<T> {
    type Range<U> = RangeBoundsWrapper<U>;

    fn new<U>(bounds: (Bound<U>, Bound<U>)) -> Result<Self::Range<U>, BoundsUnfit> {
        Ok(RangeBoundsWrapper::new(bounds))
    }

    fn new_broken() -> Result<Self::Range<T>, BoundsUnfit>
    where
        T: HasLimits,
    {
        Ok(Self::new((Ex(T::MAX), Ex(T::MIN))))
    }
}
