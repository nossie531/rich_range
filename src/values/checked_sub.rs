//! Provider of [`CheckedSub`].

use core::ops::Sub;

/// A trait enabling subtraction with overflow checked.
pub trait CheckedSub<Rhs = Self>: Sub<Rhs> {
    /// Checked subtraction.
    ///
    /// Returns [`None`] if overflow occured.
    fn checked_sub(self, rhs: Rhs) -> Option<Self::Output>;
}

/// Implement [`CheckedSub`] for reference of integer.
macro_rules! impl_checked_sub_for_int_ref {
    ($ty:ty) => {
        impl CheckedSub for &$ty {
            fn checked_sub(self, rhs: &$ty) -> Option<Self::Output> {
                (*self).checked_sub(*rhs)
            }
        }
    };
}

/// Implement [`CheckedSub`] for reference of integer.
macro_rules! impl_checked_sub_for_float_ref {
    ($ty:ty) => {
        impl CheckedSub for &$ty {
            fn checked_sub(self, rhs: &$ty) -> Option<Self::Output> {
                Some(self - rhs)
            }
        }
    };
}

impl_checked_sub_for_int_ref!(i8);
impl_checked_sub_for_int_ref!(u8);
impl_checked_sub_for_int_ref!(i16);
impl_checked_sub_for_int_ref!(u16);
impl_checked_sub_for_int_ref!(i32);
impl_checked_sub_for_int_ref!(u32);
impl_checked_sub_for_int_ref!(i64);
impl_checked_sub_for_int_ref!(u64);
impl_checked_sub_for_int_ref!(i128);
impl_checked_sub_for_int_ref!(u128);
impl_checked_sub_for_int_ref!(isize);
impl_checked_sub_for_int_ref!(usize);
impl_checked_sub_for_float_ref!(f32);
impl_checked_sub_for_float_ref!(f64);
