//! Provider of [`CheckedAdd`].

use core::ops::Add;

/// A trait enabling addion with overflow checked.
pub trait CheckedAdd<Rhs = Self>: Add<Rhs> {
    /// Checked addition.
    ///
    /// Returns [`None`] if overflow occured.
    fn checked_add(self, rhs: Rhs) -> Option<Self::Output>;
}

/// Implement [`CheckedAdd`] for reference of integer.
macro_rules! impl_checked_add_for_int_ref {
    ($ty:ty) => {
        impl CheckedAdd for &$ty {
            #[inline]
            fn checked_add(self, rhs: &$ty) -> Option<Self::Output> {
                (*self).checked_add(*rhs)
            }
        }
    };
}

/// Implement [`CheckedAdd`] for reference of integer.
macro_rules! impl_checked_add_for_float_ref {
    ($ty:ty) => {
        impl CheckedAdd for &$ty {
            #[inline]
            fn checked_add(self, rhs: &$ty) -> Option<Self::Output> {
                Some(self + rhs)
            }
        }
    };
}

impl_checked_add_for_int_ref!(i8);
impl_checked_add_for_int_ref!(u8);
impl_checked_add_for_int_ref!(i16);
impl_checked_add_for_int_ref!(u16);
impl_checked_add_for_int_ref!(i32);
impl_checked_add_for_int_ref!(u32);
impl_checked_add_for_int_ref!(i64);
impl_checked_add_for_int_ref!(u64);
impl_checked_add_for_int_ref!(i128);
impl_checked_add_for_int_ref!(u128);
impl_checked_add_for_int_ref!(isize);
impl_checked_add_for_int_ref!(usize);
impl_checked_add_for_float_ref!(f32);
impl_checked_add_for_float_ref!(f64);
