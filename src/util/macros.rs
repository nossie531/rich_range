//! Utility macros.

/// Convert the expression to a reference.
///
/// if it's already a reference, pass the reference symbol as the
/// first argument. Otherwise pass the empty as the first argument.
macro_rules! to_ref {
    (, $expr:expr) => {
        &$expr
    };
    (&, $expr:expr) => {
        $expr
    };
}

pub(crate) use to_ref;
