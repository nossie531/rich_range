//! Provider of [`HasLimits`].

/// A trait enabling getting minimum and maximum values.
pub trait HasLimits: PartialOrd + Sized {
    /// Minimum value.
    const MIN: Self;

    /// Maximum value.
    const MAX: Self;
}

/// Implement [`HasLimits`] for integer type.
macro_rules! impl_has_limits_for_int {
    ($ty:ty) => {
        impl HasLimits for $ty {
            const MIN: Self = Self::MIN;
            const MAX: Self = Self::MAX;
        }
    };
}

/// Implement [`HasLimits`] for floating point type.
macro_rules! impl_has_limits_for_float {
    ($ty:ty) => {
        impl HasLimits for $ty {
            const MIN: Self = Self::NEG_INFINITY;
            const MAX: Self = Self::INFINITY;
        }
    };
}

impl_has_limits_for_int!(i8);
impl_has_limits_for_int!(u8);
impl_has_limits_for_int!(i16);
impl_has_limits_for_int!(u16);
impl_has_limits_for_int!(i32);
impl_has_limits_for_int!(u32);
impl_has_limits_for_int!(i64);
impl_has_limits_for_int!(u64);
impl_has_limits_for_int!(i128);
impl_has_limits_for_int!(u128);
impl_has_limits_for_int!(isize);
impl_has_limits_for_int!(usize);
impl_has_limits_for_float!(f32);
impl_has_limits_for_float!(f64);
