//! Provider of [`Side`].

/// Range bound side.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Side {
    /// Start side.
    S,

    /// End side.
    E,
}
