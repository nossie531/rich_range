use crate::for_test::aliases::*;
use crate::for_test::consts::*;
use rich_range::prelude::*;
use std::f32;
use std::ops::Range;

pub fn range() -> Range<usize> {
    START..END
}

pub fn range_wrapper() -> RwNormal<usize> {
    RwNormal::new(START..END)
}

pub fn range_univ() -> RangeUniv<usize> {
    RangeUniv::new(In(START), Ex(END))
}

pub fn range_univ_flt() -> RangeUniv<f32> {
    RangeUniv::new(In(START_FLT), Ex(END_FLT))
}

pub fn unmixables() -> impl Iterator<Item = (RangeUniv<f32>, RangeUniv<f32>)> {
    [
        (ru::new(f32::NAN..END_FLT), range_univ_flt()),
        (ru::new(START_FLT..f32::NAN), range_univ_flt()),
        (range_univ_flt(), ru::new(f32::NAN..END_FLT)),
        (range_univ_flt(), ru::new(END_FLT..f32::NAN)),
    ]
    .into_iter()
}
