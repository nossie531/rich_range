//! Provider of [`HasNexts`].

use crate::*;

/// A trait enabling getting previous and next values.
pub trait HasNexts: PartialOrd + Sized {
    /// Returns next value.
    ///
    /// If next value does not exists, returns [`None`].
    fn next(&self) -> Option<Self>;

    /// Returns previous value.
    ///
    /// If previous value does not exists, returns [`None`].
    fn prev(&self) -> Option<Self>;
}

impl<T> HasNexts for T
where
    T: Step,
{
    #[inline]
    fn next(&self) -> Option<Self> {
        T::forward_checked(self.clone(), 1)
    }

    #[inline]
    fn prev(&self) -> Option<Self> {
        T::backward_checked(self.clone(), 1)
    }
}

/// Implement [`HasNexts`] for floating point type.
macro_rules! impl_has_nexts_for_float {
    ($ty:ty) => {
        impl HasNexts for $ty {
            #[inline]
            fn next(&self) -> Option<Self> {
                (self != &Self::INFINITY && !self.is_nan()).then_some(self.next_up())
            }

            #[inline]
            fn prev(&self) -> Option<Self> {
                (self != &Self::NEG_INFINITY && !self.is_nan()).then_some(self.next_down())
            }
        }
    };
}

impl_has_nexts_for_float!(f32);
impl_has_nexts_for_float!(f64);
