macro_rules! r {
    ($(<$type:ty>,)? $(-)*-, $(-)*-) => {{
        use crate::for_test::aliases::*;
        use rich_range::shorthands::ru;
        use std::ops::Bound;
        ru::new::<(Bound<pt!($($type)?)>, Bound<pt!($($type)?)>), pt!($($type)?)>((Ub, Ub))
    }};
    ($(<$type:ty>,)? $(-)*-, =$e:expr) => {{
        use crate::for_test::aliases::*;
        use rich_range::shorthands::ru;
        ru::new::<_, pt!($($type)?)>((Ub, In($e)))
    }};
    ($(<$type:ty>,)? $(-)*-, ?$e:expr) => {{
        use crate::for_test::aliases::*;
        use rich_range::shorthands::ru;
        ru::new::<_, pt!($($type)?)>((Ub, Ex($e)))
    }};
    ($(<$type:ty>,)? =$s:expr, $(-)*-) => {{
        use crate::for_test::aliases::*;
        use rich_range::shorthands::ru;
        ru::new::<_, pt!($($type)?)>((In($s), Ub))
    }};
    ($(<$type:ty>,)? =$s:expr, =$e:expr) => {{
        use crate::for_test::aliases::*;
        use rich_range::shorthands::ru;
        ru::new::<_, pt!($($type)?)>((In($s), In($e)))
    }};
    ($(<$type:ty>,)? =$s:expr, ?$e:expr) => {{
        use crate::for_test::aliases::*;
        use rich_range::shorthands::ru;
        ru::new::<_, pt!($($type)?)>((In($s), Ex($e)))
    }};
    ($(<$type:ty>,)? ?$s:expr, $(-)*-) => {{
        use crate::for_test::aliases::*;
        use rich_range::shorthands::ru;
        ru::new::<_, pt!($($type)?)>((Ex($s), Ub))
    }};
    ($(<$type:ty>,)? ?$s:expr, =$e:expr) => {{
        use crate::for_test::aliases::*;
        use rich_range::shorthands::ru;
        ru::new::<_, pt!($($type)?)>((Ex($s), In($e)))
    }};
    ($(<$type:ty>,)? ?$s:expr, ?$e:expr) => {{
        use crate::for_test::aliases::*;
        use rich_range::shorthands::ru;
        ru::new::<_, pt!($($type)?)>((Ex($s), Ex($e)))
    }};
}

macro_rules! ro {
    (--$(-)*) => {
        None as Option<RangeUniv<_>>
    };
    (<$type:ty>, --$(-)*) => {
        None as Option<RangeUniv<$type>>
    };
    ($($args:tt)*) => {
        Some(r!($($args)*))
    };
}

macro_rules! pt {
    () => {
        usize
    };
    ($type:ty) => {
        $type
    };
}

pub(crate) use pt;
pub(crate) use r;
pub(crate) use ro;
