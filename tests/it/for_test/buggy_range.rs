use rich_range::conv::*;
use rich_range::*;
use std::ops::{Bound, RangeBounds};

pub struct BuggyRange<T>(Bound<T>, Bound<T>);

impl<T> BuggyRange<T> {
    pub fn new(start: Bound<T>, end: Bound<T>) -> Self {
        Self(start, end)
    }
}

impl<T> RangeBounds<T> for BuggyRange<T> {
    fn start_bound(&self) -> std::ops::Bound<&T> {
        self.0.as_ref()
    }

    fn end_bound(&self) -> std::ops::Bound<&T> {
        self.1.as_ref()
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
