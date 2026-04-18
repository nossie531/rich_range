use rich_range::conv::*;
use rich_range::*;
use std::ops::{Bound, Range, RangeBounds};

pub struct BuggyRange<T>(Range<T>);

impl<T> BuggyRange<T> {
    pub fn new(start: T, end: T) -> Self {
        Self(start..end)
    }
}

impl<T> RangeBounds<T> for BuggyRange<T> {
    fn start_bound(&self) -> std::ops::Bound<&T> {
        self.0.start_bound()
    }

    fn end_bound(&self) -> std::ops::Bound<&T> {
        self.0.end_bound()
    }
}

impl<T> RichRangeBounds<T> for BuggyRange<T> {
    // nop.
}

impl<T> RangeSrc<T> for BuggyRange<T> {
    type Range<U> = BuggyRange<U>;

    fn new<U>(bounds: (Bound<U>, Bound<U>)) -> Result<Self::Range<U>, BoundsUnfit> {
        Err(BoundsUnfit::new::<Self, U>(&bounds))
    }

    fn new_broken() -> Result<Self::Range<T>, rich_range::conv::BoundsUnfit>
    where
        T: rich_range::HasLimits,
    {
        Err(BoundsUnfit::new_broken::<Self>())
    }
}
