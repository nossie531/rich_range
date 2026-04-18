//! Provider of [`IterRichRange`].

use crate::shorthands::*;
use crate::*;
use core::iter::FusedIterator;
use core::ops::Bound;

/// An Iterator over the values on range.
///
/// This `struct` is created by the [`RangeUniv::iter`] method.
/// See its documentat for more detail.
///
/// # Notes
///
/// This iterator will panic in following situations.
///
/// - Internal value overflow
/// - Unbounded start side with forward iteration
/// - Unbounded end side with backward iteration
#[derive(Clone, Debug)]
#[must_use = msg::iter_must_use!()]
pub struct IterRichRange<T> {
    /// Target range.
    range: RangeUniv<T>,

    /// Whether the next target will overflow.
    overflowed: bool,
}

impl<T> IterRichRange<T> {
    /// Creates a new instance.
    pub(crate) fn new(start: Bound<T>, end: Bound<T>) -> Self
    where
        T: Step,
    {
        let range = ru::new((start, end));
        let overflowed = range.is_empty();
        Self { range, overflowed }
    }
}

impl<T> Iterator for IterRichRange<T>
where
    T: Step,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.progress(0, true)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<T> {
        self.progress(n, true)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.overflowed {
            (0, Some(0))
        } else if self.range.is_wide() {
            (usize::MAX, None)
        } else {
            match self.range.len() {
                Some(x) => (x, Some(x)),
                None => (usize::MAX, None),
            }
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.size_hint().1.expect(msg::NO_OVF)
    }

    #[inline]
    fn last(mut self) -> Option<T> {
        assert!(!self.range.is_wide(), "{}", msg::NO_OVF);
        self.next_back()
    }

    #[inline]
    fn min(mut self) -> Option<T> {
        self.next()
    }

    #[inline]
    fn max(mut self) -> Option<T> {
        self.next_back()
    }

    #[inline]
    fn is_sorted(self) -> bool {
        true
    }
}

impl<T> DoubleEndedIterator for IterRichRange<T>
where
    T: Step,
{
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.progress(0, false)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<T> {
        self.progress(n, false)
    }
}

impl<T> FusedIterator for IterRichRange<T>
where
    T: Step,
{
    // nop.
}

impl<T> ExactSizeIterator for IterRichRange<T>
where
    T: Step,
{
    // nop.
}

/// Private tools.
mod pv {
    use crate::parts::*;
    use crate::shorthands::aliases::*;
    use crate::*;

    /// Returns `true` if iteration is done.
    pub fn is_done<T>(fwd: bool, curr: Edge<&T>, sentinel: Edge<&T>) -> bool
    where
        T: PartialOrd,
    {
        let ord = curr.partial_cmp(&sentinel);
        ord.is_none_or(|x| if fwd { x.is_gt() } else { x.is_lt() })
    }

    /// Get iteration value from edge.
    pub fn get_iter_value<T>(edge: Edge<&T>) -> Option<T>
    where
        T: Step,
    {
        let Ex(x) = edge.bound() else {
            return edge.pos().cloned();
        };

        Some(match edge.side() {
            Side::S => x.next()?,
            Side::E => x.prev()?,
        })
    }
}

/// Private methods.
impl<T> IterRichRange<T>
where
    T: Step,
{
    /// Progress this iterator.
    fn progress(&mut self, n: usize, fwd: bool) -> Option<T> {
        if self.size_hint().0 == 0 {
            return None;
        }

        // Prepare edges.
        let (s, e) = util::swap_if(!fwd, self.range.edges());
        assert!(!s.is_unbounded(), "{}", msg::exist_bound(fwd));

        // Calculate positions.
        let mov = util::switch_checked_fwd_or_bwd(fwd);
        let n0 = s.try_map(|x| mov(x.clone(), n));
        let n1 = s.try_map(|x| mov(x.clone(), n + 1));
        let overflowed = n1.is_none();
        let curr_edge = n0.expect(msg::NO_OVF);
        let next_bound = n1.map_or(Bound::Unbounded, |x| x.bound());
        let done = pv::is_done(fwd, curr_edge.as_ref(), e);

        // Collect update values.
        let upd = util::switch_with_start_or_end(fwd);
        let range = upd(&self.range, next_bound);
        let ret = (!done).then(|| pv::get_iter_value(curr_edge.as_ref()));
        let ret = util::expect_inside(ret, msg::NO_OVF);

        // Update and return.
        self.overflowed = overflowed;
        self.range = range;
        ret
    }
}
