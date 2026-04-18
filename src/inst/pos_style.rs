//! Provider of [`PosStyle`].

/// Number system of position.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum PosStyle {
    /// Continuous.
    Real,
    /// None continuous.
    Step,
}
