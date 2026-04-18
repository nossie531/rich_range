//! Provider of [`RangeRel`].

/// Range position relation.
///
/// This value is returned by the [`RichRangeBounds::rel`]
/// method and its family.
///
/// # Notes on Terminology
///
/// Idea of "relation" is come fome [Allen's interval algebra][aia].
/// About it, few caveats exist.
///
/// First, the terminology used in this algebra has meanings that slightly
/// differ from daily language. For example, "Overlaps" does not imply all
/// patterns with overlaps (see original definition).
///
/// Second, [`Undefined`] variant is this crate original. This variant is
/// used when detecting [broken empty][eh] or unordered values like NaN.
/// (For [`Undefined`] vs [`Equal`], The former takes precedence.)
///
/// Additionally, The meaning of "Meets" alters by the argument of [`rel`]
/// with [`PosStyle`] type. If the value is [`Real`], bounds pair must be
/// both "Included" for meets (This is original meaning). If the value is
/// [`Step`], bounds pair must be "Included" and "Excluded" for meets.
///
/// [`Equal`]: Self::Equal
/// [`Undefined`]: Self::Undefined
/// [`PosStyle`]: crate::PosStyle
/// [`Real`]: crate::PosStyle::Real
/// [`Step`]: crate::PosStyle::Step
/// [`rel`]: crate::RichRangeBounds::rel
/// [`RichRangeBounds::rel`]: crate::RichRangeBounds::rel
/// [eh]: crate::RichRangeBounds#empty-handling
/// [aia]: https://en.wikipedia.org/wiki/Allen%27s_interval_algebra
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum RangeRel {
    /// Undefined (See [notes](Self#notes-on-terminology)).
    Undefined,

    /// Equal (See [original definition][aia]).
    ///
    /// [aia]: https://en.wikipedia.org/wiki/Allen%27s_interval_algebra
    Equal,

    /// Before (See [original definition][aia]).
    ///
    /// If flag is `false` inverse mode is ON.
    ///
    /// [aia]: https://en.wikipedia.org/wiki/Allen%27s_interval_algebra
    Before(bool),

    /// Meets (See [original definition][aia]).
    ///
    /// If flag is `false` inverse mode is ON.
    ///
    /// [aia]: https://en.wikipedia.org/wiki/Allen%27s_interval_algebra
    Meets(bool),

    /// Starts (See [original definition][aia]).
    ///
    /// If flag is `false` inverse mode is ON.
    ///
    /// [aia]: https://en.wikipedia.org/wiki/Allen%27s_interval_algebra
    Starts(bool),

    /// Finishes (See [original definition][aia]).
    ///
    /// If flag is `false` inverse mode is ON.
    ///
    /// [aia]: https://en.wikipedia.org/wiki/Allen%27s_interval_algebra
    Finishes(bool),

    /// Overlaps (See [original definition][aia]).
    ///
    /// If flag is `false` inverse mode is ON.
    ///
    /// [aia]: https://en.wikipedia.org/wiki/Allen%27s_interval_algebra
    Overlaps(bool),

    /// During (See [original definition][aia]).
    ///
    /// If flag is `false` inverse mode is ON.
    ///
    /// [aia]: https://en.wikipedia.org/wiki/Allen%27s_interval_algebra
    During(bool),
}

impl RangeRel {
    /// Returns `true` if this is [`Undefined`](Self::Undefined) value.
    pub fn is_undefined(&self) -> bool {
        *self == Self::Undefined
    }

    /// Returns `true` if this is [`Equal`](Self::Equal) value.
    pub fn is_equal(&self) -> bool {
        *self == Self::Equal
    }

    /// Returns `true` if this is [`Before`](Self::Before) value.
    pub fn is_bef_xxx(&self) -> bool {
        matches!(*self, Self::Before(_))
    }

    /// Returns `true` if this is normal [`Before`](Self::Before) value.
    pub fn is_bef_reg(&self) -> bool {
        *self == Self::Before(true)
    }

    /// Returns `true` if this is inverse [`Before`](Self::Before) value.
    pub fn is_bef_inv(&self) -> bool {
        *self == Self::Before(false)
    }

    /// Returns `true` if this is [`Meets`](Self::Meets) value.
    pub fn is_met_xxx(&self) -> bool {
        matches!(*self, Self::Meets(_))
    }

    /// Returns `true` if this is normal [`Meets`](Self::Meets) value.
    pub fn is_met_reg(&self) -> bool {
        *self == Self::Meets(true)
    }

    /// Returns `true` if this is inverse [`Meets`](Self::Meets) value.
    pub fn is_met_inv(&self) -> bool {
        *self == Self::Meets(false)
    }

    /// Returns `true` if this is [`Starts`](Self::Starts) value.
    pub fn is_stt_xxx(&self) -> bool {
        matches!(*self, Self::Starts(_))
    }

    /// Returns `true` if this is normal [`Starts`](Self::Starts) value.
    pub fn is_stt_reg(&self) -> bool {
        *self == Self::Starts(true)
    }

    /// Returns `true` if this is inverse [`Starts`](Self::Starts) value.
    pub fn is_stt_inv(&self) -> bool {
        *self == Self::Starts(false)
    }

    /// Returns `true` if this is [`Finishes`](Self::Finishes) value.
    pub fn is_fin_xxx(&self) -> bool {
        matches!(*self, Self::Finishes(_))
    }

    /// Returns `true` if this is normal [`Finishes`](Self::Finishes) value.
    pub fn is_fin_reg(&self) -> bool {
        *self == Self::Finishes(true)
    }

    /// Returns `true` if this is inverse [`Finishes`](Self::Finishes) value.
    pub fn is_fin_inv(&self) -> bool {
        *self == Self::Finishes(false)
    }

    /// Returns `true` if this is [`Overlaps`](Self::Overlaps) value.
    pub fn is_ovl_xxx(&self) -> bool {
        matches!(*self, Self::Overlaps(_))
    }

    /// Returns `true` if this is normal [`Overlaps`](Self::Overlaps) value.
    pub fn is_ovl_reg(&self) -> bool {
        *self == Self::Overlaps(true)
    }

    /// Returns `true` if this is inverse [`Overlaps`](Self::Overlaps) value.
    pub fn is_ovl_inv(&self) -> bool {
        *self == Self::Overlaps(false)
    }

    /// Returns `true` if this is [`During`](Self::During) value.
    pub fn is_dur_xxx(&self) -> bool {
        matches!(*self, Self::During(_))
    }

    /// Returns `true` if this is normal [`During`](Self::During) value.
    pub fn is_dur_reg(&self) -> bool {
        *self == Self::During(true)
    }

    /// Returns `true` if this is inverse [`During`](Self::During) value.
    pub fn is_dur_inv(&self) -> bool {
        *self == Self::During(false)
    }

    /// Returns `true` if this relation represent intersection state.
    pub fn is_intersects(&self) -> bool {
        !matches!(*self, Self::Undefined | Self::Before(_) | Self::Meets(_))
    }

    /// Returns `true` if this relation represent includes state.
    pub fn is_includes(&self) -> bool {
        matches!(
            *self,
            Self::Equal | Self::During(false) | Self::Starts(false) | Self::Finishes(false)
        )
    }

    /// Returns `true` if this relation represent included state.
    pub fn is_included(&self) -> bool {
        matches!(
            *self,
            Self::Equal | Self::During(true) | Self::Starts(true) | Self::Finishes(true)
        )
    }

    /// Returns inversed relation.
    pub fn inverse(&self) -> Self {
        match *self {
            Self::Undefined => Self::Undefined,
            Self::Equal => Self::Equal,
            Self::Before(x) => Self::Before(!x),
            Self::Meets(x) => Self::Meets(!x),
            Self::Starts(x) => Self::Starts(!x),
            Self::Finishes(x) => Self::Finishes(!x),
            Self::Overlaps(x) => Self::Overlaps(!x),
            Self::During(x) => Self::During(!x),
        }
    }
}
