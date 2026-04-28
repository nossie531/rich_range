//! Range computation logics.

use crate::calc::*;
use crate::conv::*;
use crate::parts::*;
use crate::shorthands::aliases::*;
use crate::shorthands::*;
use crate::util::*;
use crate::*;
use core::borrow::Borrow;
use core::ops::{Add, RangeBounds, Sub};

/// Returns two ranges by flipping given range.
pub(crate) fn flip<R, T>(range: &R) -> [Option<RangeUniv<T>>; 2]
where
    R: ?Sized + RangeBounds<T>,
    T: Clone + PartialOrd,
{
    flip_adv(range, CursorMode::Off)
}

/// Returns two ranges by flipping given range with advanced parameters.
pub(crate) fn flip_adv<R, T>(range: &R, mode: CursorMode) -> [Option<RangeUniv<T>>; 2]
where
    R: ?Sized + RangeBounds<T>,
    T: Clone + PartialOrd,
{
    let range = &rv::new(range);

    // Guard for broken empty.
    if range.is_broken() {
        return [Some(RangeUniv::new(Ub, Ub)), None];
    }

    // Guard for cursor empty.
    if range.is_cursor() {
        return if mode == CursorMode::Off {
            [Some(RangeUniv::new(Ub, Ub)), None]
        } else {
            let pos = range.cursor().unwrap();
            let face = if range.is_cursor_fwd() { Ex } else { In }(pos.clone());
            let back = if range.is_cursor_fwd() { In } else { Ex }(pos.clone());
            let fst = Some(RangeUniv::new(Ub, face));
            let snd = Some(RangeUniv::new(back, Ub));
            [fst, snd]
        };
    }

    // Flip non empty range.
    let (s, e) = (bound(range.start_bound()), bound(range.end_bound()));
    let xs = s.toggle_included().cloned();
    let xe = e.toggle_included().cloned();
    match (&xs, &xe) {
        (Ub, Ub) => [None, None],
        (_, Ub) => [Some(RangeUniv::new(Ub, xs)), None],
        (Ub, _) => [Some(RangeUniv::new(xe, Ub)), None],
        (_, _) => [Some(RangeUniv::new(Ub, xs)), Some(RangeUniv::new(xe, Ub))],
    }
}

/// Returns a new range by subtracting given value to both sides.
///
/// # Panics
///
/// Panics if any of the following cases occured.
///
/// - Position of the bound is overflowed.
/// - [Custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn shl<R, T>(range: &R, value: impl Borrow<T>) -> R::Range<T>
where
    R: ?Sized + RangeSrc<T>,
    for<'a> &'a T: Sub<&'a T, Output = T>,
{
    let s = range.start_bound().map(|x| x - value.borrow());
    let e = range.end_bound().map(|x| x - value.borrow());
    <R as RangeSrc<T>>::new((s, e)).unwrap()
}

/// Returns a new range by adding given value to both sides.
///
/// # Panics
///
/// Panics if any of the following cases occured.
///
/// - Position of the bound is overflowed.
/// - [Custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn shr<R, T>(range: &R, value: impl Borrow<T>) -> R::Range<T>
where
    R: ?Sized + RangeSrc<T>,
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    let s = range.start_bound().map(|x| x + value.borrow());
    let e = range.end_bound().map(|x| x + value.borrow());
    <R as RangeSrc<T>>::new((s, e)).unwrap()
}

/// Returns a new range by adding given value to start side.
///
/// # Panics
///
/// Panics if any of the following cases occured.
///
/// - Position of the bound is overflowed.
/// - [Custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn add_start<R, T>(range: &R, value: impl Borrow<T>) -> R::Range<T>
where
    R: ?Sized + RangeSrc<T>,
    T: Clone,
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    let s = range.start_bound().map(|x| x + value.borrow());
    let e = range.end_bound().cloned();
    <R as RangeSrc<T>>::new((s, e)).unwrap()
}

/// Returns a new range by adding given value to end side.
///
/// # Panics
///
/// Panics if any of the following cases occured.
///
/// - Position of the bound is overflowed.
/// - [Custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn add_end<R, T>(range: &R, value: impl Borrow<T>) -> R::Range<T>
where
    R: ?Sized + RangeSrc<T>,
    T: Clone,
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    let s = range.start_bound().cloned();
    let e = range.end_bound().map(|x| x + value.borrow());
    <R as RangeSrc<T>>::new((s, e)).unwrap()
}

/// Returns a new range by subtracting given value to start side.
///
/// # Panics
///
/// Panics if any of the following cases occured.
///
/// - Position of the bound is overflowed.
/// - [Custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn sub_start<R, T>(range: &R, value: impl Borrow<T>) -> R::Range<T>
where
    R: ?Sized + RangeSrc<T>,
    T: Clone,
    for<'a> &'a T: Sub<&'a T, Output = T>,
{
    let s = range.start_bound().map(|x| x - value.borrow());
    let e = range.end_bound().cloned();
    <R as RangeSrc<T>>::new((s, e)).unwrap()
}

/// Returns a new range by subtracting given value to end side.
///
/// # Panics
///
/// Panics if any of the following cases occured.
///
/// - Position of the bound is overflowed.
/// - [Custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn sub_end<R, T>(range: &R, value: impl Borrow<T>) -> R::Range<T>
where
    R: ?Sized + RangeSrc<T>,
    T: Clone,
    for<'a> &'a T: Sub<&'a T, Output = T>,
{
    let s = range.start_bound().cloned();
    let e = range.end_bound().map(|x| x - value.borrow());
    <R as RangeSrc<T>>::new((s, e)).unwrap()
}

/// Returns a new range with start bound
/// based on original end bound and the given width.
///
/// # Panics
///
/// Panics if any of the following cases occured.
///
/// - Start or end bound is unbounded.
/// - Position of the bound is overflowed.
/// - [Custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn calc_start<R, T>(range: &R, width: impl Borrow<T>) -> R::Range<T>
where
    R: ?Sized + RangeSrc<T>,
    T: Clone,
    for<'a> &'a T: Sub<&'a T, Output = T>,
{
    assert!(!matches!(range.start_bound(), Ub), "{}", msg::EXIST_SB);
    assert!(!matches!(range.end_bound(), Ub), "{}", msg::EXIST_EB);
    let (sb, eb) = (range.start_bound(), range.end_bound());
    let sp = bound(eb).pos().unwrap().sub(width.borrow());
    let ret = (sb.map(|_| sp), eb.cloned());
    <R as RangeSrc<T>>::new(ret).unwrap()
}

/// Returns a new range with end bound
/// based on original start bound and the given width.
///
/// # Panics
///
/// Panics if any of the following cases occured.
///
/// - Start or end bound is unbounded.
/// - Position of the bound is overflowed.
/// - [Custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn calc_end<R, T>(range: &R, width: impl Borrow<T>) -> R::Range<T>
where
    R: ?Sized + RangeSrc<T>,
    T: Clone,
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    assert!(!matches!(range.start_bound(), Ub), "{}", msg::EXIST_SB);
    assert!(!matches!(range.end_bound(), Ub), "{}", msg::EXIST_EB);
    let (sb, eb) = (range.start_bound(), range.end_bound());
    let ep = bound(sb).pos().unwrap().add(width.borrow());
    let ret = (sb.cloned(), eb.map(|_| ep));
    <R as RangeSrc<T>>::new(ret).unwrap()
}

/// Returns a new range with start bound aligned at given value.
///
/// # Errors
///
/// This function returns errors in the following cases.
///
/// - The Start bound is unbounded.
/// - Position of the bound is overflowed.
/// - The range or the value have unordered position like NaN.
/// - [Custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn align_start<R, T>(range: &R, value: impl Borrow<T>) -> Result<R::Range<T>, Error>
where
    R: ?Sized + RangeSrc<T>,
    T: Clone + PartialOrd,
    for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
    for<'a> &'a T: CheckedSub<&'a T, Output = T>,
{
    // Arguments preparation.
    let range = &rv::new(range);
    let value = value.borrow();
    let [sb, eb] = <[_; _]>::from(range.bounds()).map(bound);

    // Guard for start unbounds.
    let Some(sp) = sb.pos() else {
        return Err(Error::StartUnbounded);
    };

    // Guard for end unbounds.
    let Some(ep) = eb.pos() else {
        return Ok(<R as RangeSrc<T>>::new((sb.map(|_| value).cloned(), Ub))?);
    };

    // Guard for unordered values.
    if !util::is_ordered(&[Some(value), sb.pos(), eb.pos()]) {
        return Err(Error::Unorderd);
    }

    // Use offset if it is not overflowed.
    if let Some(offset) = Signed::checked_distance(sp, value) {
        let s = sb.map(|_| value).cloned();
        let e = eb.try_map(|x| Signed::checked_add(x, &offset));
        return Ok(<R as RangeSrc<T>>::new((s, e.ok_or(Error::Overflow)?))?);
    }

    // Use width if it is not overflowed.
    if let Some(width) = Signed::checked_distance(sp, ep) {
        let s = sb.map(|_| value).cloned();
        let e = eb.try_map(|_| Signed::checked_add(value, &width));
        return Ok(<R as RangeSrc<T>>::new((s, e.ok_or(Error::Overflow)?))?);
    }

    Err(Error::Overflow)
}

/// Returns a new range with end bound aligned at given value.
///
/// # Errors
///
/// This function returns errors in the following cases.
///
/// - The end bound is unbounded.
/// - Position of the bound is overflowed.
/// - The range or the value have unordered position like NaN.
/// - [Custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn align_end<R, T>(range: &R, value: impl Borrow<T>) -> Result<R::Range<T>, Error>
where
    R: ?Sized + RangeSrc<T>,
    T: Clone + PartialOrd,
    for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
    for<'a> &'a T: CheckedSub<&'a T, Output = T>,
{
    // Arguments preparation.
    let range = &rv::new(range);
    let value = value.borrow();
    let [sb, eb] = <[_; _]>::from(range.bounds()).map(bound);

    // Guard for end unbounds.
    let Some(ep) = eb.pos() else {
        return Err(Error::EndUnbounded);
    };

    // Guard for start unbounds.
    let Some(sp) = sb.pos() else {
        return Ok(<R as RangeSrc<T>>::new((Ub, eb.map(|_| value).cloned()))?);
    };

    // Guard for unordered values.
    if !util::is_ordered(&[Some(value), sb.pos(), eb.pos()]) {
        return Err(Error::Unorderd);
    }

    // Use offset if it is not overflowed.
    if let Some(offset) = Signed::checked_distance(ep, value) {
        let s = sb.try_map(|x| Signed::checked_add(x, &offset));
        let e = eb.map(|_| value).cloned();
        return Ok(<R as RangeSrc<T>>::new((s.ok_or(Error::Overflow)?, e))?);
    }

    // Use width if it is not overflowed.
    if let Some(width) = Signed::checked_distance(sp, ep) {
        let s = sb.try_map(|_| Signed::checked_sub(value, &width));
        let e = eb.map(|_| value).cloned();
        return Ok(<R as RangeSrc<T>>::new((s.ok_or(Error::Overflow)?, e))?);
    }

    Err(Error::Overflow)
}

/// Overflow checked version of [`shl`].
///
/// # Panics
///
/// Panics if [custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn checked_shl<R, T>(range: &R, value: impl Borrow<T>) -> Option<R::Range<T>>
where
    R: ?Sized + RangeSrc<T>,
    for<'a> &'a T: CheckedSub<&'a T, Output = T>,
{
    let (s, e) = (range.start_bound(), range.end_bound());
    let s = bound(s).try_map(|x| CheckedSub::checked_sub(x, value.borrow()))?;
    let e = bound(e).try_map(|x| CheckedSub::checked_sub(x, value.borrow()))?;
    Some(<R as RangeSrc<T>>::new((s, e)).unwrap())
}

/// Overflow checked version of [`shr`].
///
/// # Panics
///
/// Panics if [custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn checked_shr<R, T>(range: &R, value: impl Borrow<T>) -> Option<R::Range<T>>
where
    R: ?Sized + RangeSrc<T>,
    for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
{
    let (s, e) = (range.start_bound(), range.end_bound());
    let s = bound(s).try_map(|x| CheckedAdd::checked_add(x, value.borrow()))?;
    let e = bound(e).try_map(|x| CheckedAdd::checked_add(x, value.borrow()))?;
    Some(<R as RangeSrc<T>>::new((s, e)).unwrap())
}

/// Overflow checked version of [`add_start`].
///
/// # Panics
///
/// Panics if [custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn checked_add_start<R, T>(range: &R, value: impl Borrow<T>) -> Option<R::Range<T>>
where
    R: ?Sized + RangeSrc<T>,
    T: Clone,
    for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
{
    let (s, e) = (range.start_bound(), range.end_bound());
    let s = bound(s).try_map(|x| CheckedAdd::checked_add(x, value.borrow()))?;
    Some(<R as RangeSrc<T>>::new((s, e.cloned())).unwrap())
}

/// Overflow checked version of [`add_end`].
///
/// # Panics
///
/// Panics if [custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn checked_add_end<R, T>(range: &R, value: impl Borrow<T>) -> Option<R::Range<T>>
where
    R: ?Sized + RangeSrc<T>,
    T: Clone,
    for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
{
    let (s, e) = (range.start_bound(), range.end_bound());
    let e = bound(e).try_map(|x| CheckedAdd::checked_add(x, value.borrow()))?;
    Some(<R as RangeSrc<T>>::new((s.cloned(), e)).unwrap())
}

/// Overflow checked version of [`sub_start`].
///
/// # Panics
///
/// Panics if [custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn checked_sub_start<R, T>(range: &R, value: impl Borrow<T>) -> Option<R::Range<T>>
where
    R: ?Sized + RangeSrc<T>,
    T: Clone,
    for<'a> &'a T: CheckedSub<&'a T, Output = T>,
{
    let (s, e) = (range.start_bound(), range.end_bound());
    let s = bound(s).try_map(|x| CheckedSub::checked_sub(x, value.borrow()))?;
    Some(<R as RangeSrc<T>>::new((s, e.cloned())).unwrap())
}

/// Overflow checked version of [`sub_end`].
///
/// # Panics
///
/// Panics if [custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn checked_sub_end<R, T>(range: &R, value: impl Borrow<T>) -> Option<R::Range<T>>
where
    R: ?Sized + RangeSrc<T>,
    T: Clone,
    for<'a> &'a T: CheckedSub<&'a T, Output = T>,
{
    let (s, e) = (range.start_bound(), range.end_bound());
    let e = bound(e).try_map(|x| CheckedSub::checked_sub(x, value.borrow()))?;
    Some(<R as RangeSrc<T>>::new((s.cloned(), e)).unwrap())
}

/// Overflow checked version of [`calc_start`].
///
/// # Panics
///
/// Panics if any of the following cases occured.
///
/// - Start or end bound is unbounded.
/// - [Custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn checked_calc_start<R, T>(range: &R, width: impl Borrow<T>) -> Option<R::Range<T>>
where
    R: ?Sized + RangeSrc<T>,
    T: Clone,
    for<'a> &'a T: CheckedSub<&'a T, Output = T>,
{
    assert!(!matches!(range.start_bound(), Ub), "{}", msg::EXIST_SB);
    assert!(!matches!(range.end_bound(), Ub), "{}", msg::EXIST_EB);
    let (sb, eb) = (range.start_bound(), range.end_bound());
    let sp = bound(eb).pos().unwrap().checked_sub(width.borrow())?;
    let ret = (sb.map(|_| sp), eb.cloned());
    Some(<R as RangeSrc<T>>::new(ret).unwrap())
}

/// Overflow checked version of [`calc_end`].
///
/// # Panics
///
/// Panics if any of the following cases occured.
///
/// - Start or end bound is unbounded.
/// - [Custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn checked_calc_end<R, T>(range: &R, width: impl Borrow<T>) -> Option<R::Range<T>>
where
    R: ?Sized + RangeSrc<T>,
    T: Clone,
    for<'a> &'a T: CheckedAdd<&'a T, Output = T>,
{
    assert!(!matches!(range.start_bound(), Ub), "{}", msg::EXIST_SB);
    assert!(!matches!(range.end_bound(), Ub), "{}", msg::EXIST_EB);
    let (sb, eb) = (range.start_bound(), range.end_bound());
    let ep = bound(sb).pos().unwrap().checked_add(width.borrow())?;
    let ret = (sb.cloned(), eb.map(|_| ep));
    Some(<R as RangeSrc<T>>::new(ret).unwrap())
}

/// Return two ranges by cutting a range at given position.
pub(crate) fn cut<R, T>(range: &R, pos: &T, mode: CutMode) -> [Option<RangeUniv<T>>; 2]
where
    R: ?Sized + RangeBounds<T>,
    T: Clone + PartialOrd,
{
    let range = &rv::new(range);

    if range.is_broken() {
        return [None, None];
    }

    if bound(range.start_bound()).pos().is_some_and(|s| pos <= s) {
        return [None, Some(range.to_univ())];
    }

    if bound(range.end_bound()).pos().is_some_and(|e| e <= pos) {
        return [Some(range.to_univ()), None];
    }

    let fst_stt = range.start_bound().cloned();
    let fst_end = mode.for_end(range, pos.clone());
    let snd_stt = mode.for_start(range, pos.clone());
    let snd_end = range.end_bound().cloned();
    let fst = RangeUniv::new(fst_stt, fst_end);
    let snd = RangeUniv::new(snd_stt, snd_end);
    [Some(fst), Some(snd)]
}

/// Returns the range between two ranges.
///
/// # Panics
///
/// Panics if ranges have unordered position like NaN.
pub(crate) fn interval<RX, RY, T>(rx: &RX, ry: &RY) -> Option<RangeUniv<T>>
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: Clone + PartialOrd,
{
    debug_assert!(calc::is_mixable(rx, ry));
    interval_adv(rx, ry, CursorMode::Off)
}

/// Returns the range between two ranges with advanced parameters.
///
/// # Panics
///
/// Panics if ranges have unordered position like NaN.
pub(crate) fn interval_adv<RX, RY, T>(rx: &RX, ry: &RY, mode: CursorMode) -> Option<RangeUniv<T>>
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: Clone + PartialOrd,
{
    debug_assert!(calc::is_mixable(rx, ry));
    let (rx, ry) = (&rv::new(rx), &rv::new(ry));
    let (bx, by) = (rx.bounds(), ry.bounds());
    if rx.is_full() || ry.is_full() {
        return None;
    }

    let ord = calc::cmp(rx, ry)?;
    let fst = if ord.is_le() { bx } else { by };
    let snd = if ord.is_le() { by } else { bx };
    let s = bound(fst.1.cloned()).toggle_included();
    let e = bound(snd.0.cloned()).toggle_included();
    mode.proc(RangeUniv::new(s, e))
}

/// Returns the product of two ranges.
///
/// # Panics
///
/// Panics if ranges have unordered position like NaN.
pub(crate) fn prod<RX, RY, T>(rx: &RX, ry: &RY) -> Option<RangeUniv<T>>
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: Clone + PartialOrd,
{
    debug_assert!(calc::is_mixable(rx, ry));

    // Prepare edges.
    let (rx, ry) = (&rv::new(rx), &rv::new(ry));
    let (ex, ey) = (rx.edges(), ry.edges());

    // Guard for no intersection.
    if !rx.intersects(ry) {
        return None;
    }

    // Guard for both cursor pattern.
    if let (Some(x), Some(y)) = (rx.cursor(), ry.cursor()) {
        let rx_win = rx.is_cursor_fwd() || ry.is_cursor_bwd();
        let (rx, ry) = (rx.to_univ(), ry.to_univ());
        return x.eq(y).then_some(if rx_win { rx } else { ry });
    }

    // Calculate return range.
    let s = MixMode::Normal.max_bound(ex.0, ey.0).cloned();
    let e = MixMode::Normal.min_bound(ex.1, ey.1).cloned();
    Some(RangeUniv::new(s, e))
}

/// Returns the superset of two ranges.
///
/// # Panics
///
/// Panics if ranges have unordered position like NaN.
pub(crate) fn enwrap<RX, RY, T>(rx: &RX, ry: &RY) -> Option<RangeUniv<T>>
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: Clone + PartialOrd,
{
    debug_assert!(calc::is_mixable(rx, ry));

    // Prepare edges.
    let (rx, ry) = (&rv::new(rx), &rv::new(ry));
    let (ex, ey) = (rx.edges(), ry.edges());

    // Guard for empties.
    if rx.is_empty() && ry.is_empty() {
        return None;
    } else if rx.is_empty() {
        return Some(ry.to_univ());
    } else if ry.is_empty() {
        return Some(rx.to_univ());
    }

    // Calculate return range.
    let s = MixMode::Normal.min_bound(ex.0, ey.0).cloned();
    let e = MixMode::Normal.max_bound(ex.1, ey.1).cloned();
    Some(RangeUniv::new(s, e))
}

/// Returns the union of two ranges.
///
/// # Panics
///
/// Panics if ranges have unordered position like NaN.
pub(crate) fn union<RX, RY, T>(rx: &RX, ry: &RY) -> (RangeUniv<T>, Option<RangeUniv<T>>)
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: Clone + PartialOrd,
{
    debug_assert!(calc::is_mixable(rx, ry));

    // Prepare edges.
    let (rx, ry) = (&rv::new(rx), &rv::new(ry));
    let (ex, ey) = (rx.edges(), ry.edges());

    // Guard for broken.
    if rx.is_broken() || ry.is_broken() {
        let (rx, ry) = (rx.to_univ(), ry.to_univ());
        return (rx, Some(ry));
    }

    // Guard for same position cursors.
    if rx.is_cursor() && rx.cursor() == ry.cursor() {
        let (rx, ry) = (rx.to_univ(), ry.to_univ());
        let rx_win = rx.is_cursor_fwd() || ry.is_cursor_bwd();
        return (if rx_win { rx } else { ry }, None);
    }

    // Calculate return range.
    if rx.intersects(ry) || rx.touches(ry) {
        let s = MixMode::Union.min_bound(ex.0, ey.0).cloned();
        let e = MixMode::Union.max_bound(ex.1, ey.1).cloned();
        let ret = RangeUniv::new(s, e);
        (ret, None)
    } else {
        let (rx, ry) = (rx.to_univ(), ry.to_univ());
        let rx_win = ex.0 <= ey.0;
        let pair = util::swap_if(!rx_win, (rx, ry));
        (pair.0, Some(pair.1))
    }
}

/// Returns the difference of two ranges.
///
/// # Panics
///
/// Panics if ranges have unordered position like NaN.
pub(crate) fn diff<RX, RY, T>(rx: &RX, ry: &RY) -> [Option<RangeUniv<T>>; 2]
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: Clone + PartialOrd,
{
    debug_assert!(calc::is_mixable(rx, ry));
    diff_adv(rx, ry, CursorMode::Off)
}

/// Returns the difference of two ranges with advanced parameters.
///
/// # Panics
///
/// Panics if ranges have unordered position like NaN.
pub(crate) fn diff_adv<RX, RY, T>(rx: &RX, ry: &RY, mode: CursorMode) -> [Option<RangeUniv<T>>; 2]
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: Clone + PartialOrd,
{
    debug_assert!(calc::is_mixable(rx, ry));

    // Prepare edges.
    let (rx, ry) = (&rv::new(rx), &rv::new(ry));
    let (ex, ey) = (rx.edges(), ry.edges());

    // Pattern for same position cursors.
    if matches!((rx.cursor(), ry.cursor()), (Some(x), Some(y)) if x == y) {
        return [None, None];
    }

    // Pattern for 1st argument start edge is 2nd argument cursor.
    if matches!((ex.0.pos(), ry.cursor()), (Some(x), Some(y)) if x == y) {
        return [None, Some(rx.to_univ())];
    }

    // Pattern for 1st argument end edge is 2nd argument cursor.
    if matches!((ex.1.pos(), ry.cursor()), (Some(x), Some(y)) if x == y) {
        return [Some(rx.to_univ()), None];
    }

    // Pattern for no intersection.
    if !rx.intersects(ry) || mode == CursorMode::Off && ry.is_cursor() {
        let fwd = calc::cmp(rx, ry).is_none_or(|x| x.is_le());
        let fst = if fwd { Some(rx.to_univ()) } else { None };
        let snd = if fwd { None } else { Some(rx.to_univ()) };
        return [fst, snd];
    }

    // Calculate return range.
    let fst_exists = !ex.0.is_contained(&ry.bounds());
    let snd_exists = !ex.1.is_contained(&ry.bounds());
    let fst_end = MixMode::Diff.min_bound(ex.1, ey.0);
    let snd_stt = MixMode::Diff.max_bound(ex.0, ey.1);
    let fst = RangeUniv::new(rx.start_bound().cloned(), fst_end.cloned());
    let snd = RangeUniv::new(snd_stt.cloned(), rx.end_bound().cloned());
    let fst = (fst_exists && !rv::new(&fst).is_broken()).then_some(fst);
    let snd = (snd_exists && !rv::new(&snd).is_broken()).then_some(snd);
    [fst, snd]
}

/// Closed version of [`prod`].
///
/// # Panics
///
/// Panics if any of the following cases occured.
///
/// - Ranges have unordered position like NaN.
/// - [Custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn closed_prod<R, T>(range: &R, value: &R) -> R
where
    R: RangeSrc<T, Range<T> = R>,
    T: Clone + PartialOrd + HasLimits,
{
    self::prod(range, value)
        .map_or_else(<R as RangeSrc<T>>::new_broken, <R as RangeSrc<T>>::new_from)
        .unwrap()
}

/// Closed version of [`union`].
///
/// # Panics
///
/// Panics if any of the following cases occured.
///
/// - Ranges have unordered position like NaN.
/// - [Custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn closed_union<R, T>(range: &R, value: &R) -> R
where
    R: RangeSrc<T, Range<T> = R>,
    T: Clone + PartialOrd,
{
    let unions = self::union(range, value);
    let snd = unions.1.as_ref();
    let snd = snd.is_some_and(|x| x.bounds() == rv::new(range).bounds());
    let ret = if !snd { unions.0 } else { unions.1.unwrap() };
    <R as RangeSrc<_>>::new_from(ret).unwrap()
}

/// Closed version of [`enwrap`].
///
/// # Panics
///
/// Panics if any of the following cases occured.
///
/// - Ranges have unordered position like NaN.
/// - [Custom range type][crt] conversion is failed.
///
/// [crt]: crate::conv::RangeSrc#about-custom-range-type
pub(crate) fn closed_enwrap<R, T>(range: &R, value: &R) -> R
where
    R: RangeSrc<T, Range<T> = R>,
    T: Clone + PartialOrd + HasLimits,
{
    self::enwrap(range, value)
        .map_or_else(<R as RangeSrc<T>>::new_broken, <R as RangeSrc<T>>::new_from)
        .unwrap()
}
