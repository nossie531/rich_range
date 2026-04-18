//! Provider of [`Signed`].

use crate::*;

/// Value with sign flag.
pub(crate) struct Signed<T> {
    /// Sign falg.
    sign: bool,

    /// Value.
    value: T,
}

impl<T> Signed<T> {
    /// Create a new instance.
    pub fn new(sign: bool, value: T) -> Self {
        Self { sign, value }
    }

    /// Returns distance between two points.
    pub fn checked_distance(px: &T, py: &T) -> Option<Self>
    where
        T: PartialOrd,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
    {
        let sign = px.partial_cmp(py)?.is_gt();
        let pair = util::swap_if(sign, (px, py));
        let abs = pair.1.checked_sub(pair.0)?;
        Some(Self::new(sign, abs))
    }

    /// Returns the result of adding the distance to the value.
    pub fn checked_add(x: &T, y: &Signed<T>) -> Option<T>
    where
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
    {
        let f = util::switch_checked_add_or_sub(!y.sign);
        f(x, &y.value)
    }

    /// Returns the result of subtracting the distance from the value.
    pub fn checked_sub(x: &T, y: &Signed<T>) -> Option<T>
    where
        for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
        for<'a> &'a T: CheckedSub<&'a T, Output = T>,
    {
        let f = util::switch_checked_add_or_sub(y.sign);
        f(x, &y.value)
    }
}
