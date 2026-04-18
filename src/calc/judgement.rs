//! Range judgement logics.

use crate::parts::*;
use crate::shorthands::*;
use crate::*;
use core::cmp::Ordering;
use core::ops::RangeBounds;

/// Returns `true` if two ranges are mixable.
///
/// To be mixable, all positions in two ranges must be compareable.
pub(crate) fn is_mixable<RX, RY, T>(rx: &RX, ry: &RY) -> bool
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: ?Sized + PartialOrd,
{
    let (rx, ry) = (rv::new(rx), rv::new(ry));
    let (bx, by) = (rx.bounds(), ry.bounds());
    let positions = [bx.0, bx.1, by.0, by.1].map(|x| bound(x).pos());
    util::is_ordered(&positions)
}

/// Returns `true` if two ranges are equal.
pub(crate) fn is_eq<RX, RY, T>(x: &RX, y: &RY) -> bool
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: ?Sized + PartialEq,
{
    let eq_s = x.start_bound() == y.start_bound();
    let eq_e = x.end_bound() == y.end_bound();
    eq_s && eq_e
}

/// Returns `true` if two ranges are equivalent (all empty is equivalent).
pub(crate) fn is_equiv<RX, RY, T>(rx: &RX, ry: &RY) -> bool
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: ?Sized + PartialOrd,
{
    let (rx, ry) = (rv::new(rx), rv::new(ry));
    let (px, py) = (rx.edges(), ry.edges());
    let both_empty = rx.is_empty() && ry.is_empty();
    if both_empty { true } else { px == py }
}

/// Returns `true` if two ranges are intersect.
pub(crate) fn is_intersects<RX, RY, T>(rx: &RX, ry: &RY) -> bool
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: ?Sized + PartialOrd,
{
    if !calc::is_mixable(rx, ry) {
        return false;
    }

    let (rx, ry) = (rv::new(rx), rv::new(ry));
    let (sx, sy) = (rx.edges(), ry.edges());
    let broken = rx.is_broken() || ry.is_broken();
    let hit_cur = rx.is_cursor() && rx.cursor() == ry.cursor();
    let hit_x_min = sx.0.is_contained(&ry.bounds());
    let hit_x_max = sx.1.is_contained(&ry.bounds());
    let hit_y_min = sy.0.is_contained(&rx.bounds());
    let hit_y_max = sy.1.is_contained(&rx.bounds());
    !broken && (hit_cur || hit_x_min || hit_x_max || hit_y_min || hit_y_max)
}

/// Returns `true` if 1st range includes 2nd range.
pub(crate) fn is_includes<RX, RY, T>(rx: &RX, ry: &RY) -> bool
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: ?Sized + PartialOrd,
{
    if !calc::is_mixable(rx, ry) {
        return false;
    }

    let (rx, ry) = (rv::new(rx), rv::new(ry));
    let (ex, ey) = (rx.edges(), ry.edges());
    let broken = rx.is_broken() || ry.is_broken();
    let ptn_cvc = rx.is_cursor() && ry.is_cursor();
    let ptn_rvc = !rx.is_cursor() && ry.is_cursor();
    let ptn_xvr = !ry.is_cursor();
    let cvc = ptn_cvc && rx.cursor() == ry.cursor();
    let rvc = ptn_rvc && rx.contains(ry.cursor().unwrap());
    let xvr = ptn_xvr && ex.0 <= ey.0 && ey.1 <= ex.1;
    !broken && (cvc || rvc || xvr)
}

/// Returns `true` if 1st range adjoins to 2nd range.
pub(crate) fn is_adjoins<RX, RY, T>(rx: &RX, ry: &RY) -> bool
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: ?Sized + PartialOrd,
{
    is_adjoins_prev(rx, ry) || is_adjoins_next(rx, ry)
}

/// Returns `true` if 1st range start adjoins to 2nd range end.
pub(crate) fn is_adjoins_prev<RX, RY, T>(rx: &RX, ry: &RY) -> bool
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: ?Sized + PartialOrd,
{
    if !calc::is_mixable(rx, ry) {
        return false;
    }

    let (rx, ry) = (rv::new(rx), rv::new(ry));
    let broken = rx.is_broken() || ry.is_broken();
    let adj = bound_ref(&ry.end_bound()).adjoins(&rx.start_bound());
    !broken && adj
}

/// Returns `true` if 1st range end adjoins to 2nd range start.
pub(crate) fn is_adjoins_next<RX, RY, T>(rx: &RX, ry: &RY) -> bool
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: ?Sized + PartialOrd,
{
    if !calc::is_mixable(rx, ry) {
        return false;
    }

    let (rx, ry) = (rv::new(rx), rv::new(ry));
    let broken = rx.is_broken() || ry.is_broken();
    let adj = bound_ref(&rx.end_bound()).adjoins(&ry.start_bound());
    !broken && adj
}

/// Returns `true` if 1st range touches to 2nd range.
pub(crate) fn is_touches<RX, RY, T>(rx: &RX, ry: &RY) -> bool
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: ?Sized + PartialOrd,
{
    is_touches_prev(rx, ry) || is_touches_next(rx, ry)
}

/// Returns `true` if 1st range start touches to 2nd range end.
pub(crate) fn is_touches_prev<RX, RY, T>(rx: &RX, ry: &RY) -> bool
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: ?Sized + PartialOrd,
{
    if !calc::is_mixable(rx, ry) {
        return false;
    }

    let (rx, ry) = (rv::new(rx), rv::new(ry));
    let broken = rx.is_broken() || ry.is_broken();
    let adj = bound_ref(&ry.end_bound()).touches(&rx.start_bound());
    !broken && adj
}

/// Returns `true` if 1st range end touches to 2nd range start.
pub(crate) fn is_touches_next<RX, RY, T>(rx: &RX, ry: &RY) -> bool
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: ?Sized + PartialOrd,
{
    if !calc::is_mixable(rx, ry) {
        return false;
    }

    let (rx, ry) = (rv::new(rx), rv::new(ry));
    let broken = rx.is_broken() || ry.is_broken();
    let adj = bound_ref(&rx.end_bound()).touches(&ry.start_bound());
    !broken && adj
}

/// Returns the ordering of two ranges.
pub(crate) fn cmp<RX, RY, T>(rx: &RX, ry: &RY) -> Option<Ordering>
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: ?Sized + PartialOrd,
{
    if !is_mixable(rx, ry) {
        return None;
    }

    let (x, y) = (&rv::new(rx), &rv::new(ry));
    let broken = x.is_broken() || y.is_broken();
    let x_min = util::choose_min(x.edges().0, x.edges().1);
    let y_min = util::choose_min(y.edges().0, y.edges().1);
    let x_max = util::choose_max(x.edges().0, x.edges().1);
    let y_max = util::choose_max(y.edges().0, y.edges().1);

    if broken {
        None
    } else if calc::is_eq(x, y) {
        Some(Ordering::Equal)
    } else if x_max < y_min || x_max == y_min && y.is_cursor() {
        Some(Ordering::Less)
    } else if y_max < x_min || y_max == x_min && x.is_cursor() {
        Some(Ordering::Greater)
    } else {
        None
    }
}

/// Returns relation of two ranges.
pub(crate) fn rel<RX, RY, T>(rx: &RX, ry: &RY, ps: PosStyle) -> RangeRel
where
    RX: ?Sized + RangeBounds<T>,
    RY: ?Sized + RangeBounds<T>,
    T: ?Sized + PartialOrd,
{
    if !calc::is_mixable(rx, ry) {
        return RangeRel::Undefined;
    }

    let (rx, ry) = (rv::new(rx), rv::new(ry));
    let (ex, ey) = (rx.edges(), ry.edges());
    let xs_vs_ys = ex.0.partial_cmp(&ey.0).unwrap();
    let xs_vs_ye = ex.0.partial_cmp(&ey.1).unwrap();
    let xe_vs_ys = ex.1.partial_cmp(&ey.0).unwrap();
    let xe_vs_ye = ex.1.partial_cmp(&ey.1).unwrap();
    let xe_meet_ys = bound_ref(&ex.1.bound()).meets(&ey.0.bound(), ps);
    let xs_meet_ye = bound_ref(&ex.0.bound()).meets(&ey.1.bound(), ps);

    if rx.is_broken() || ry.is_broken() {
        RangeRel::Undefined
    } else if xs_vs_ys.is_eq() && xe_vs_ye.is_eq() {
        RangeRel::Equal
    } else if xe_meet_ys || xs_meet_ye {
        RangeRel::Meets(xe_meet_ys)
    } else if xs_vs_ys.is_eq() {
        RangeRel::Starts(xe_vs_ye.is_lt())
    } else if xe_vs_ye.is_eq() {
        RangeRel::Finishes(xs_vs_ys.is_gt())
    } else if xe_vs_ys.is_lt() || xs_vs_ye.is_gt() {
        RangeRel::Before(xe_vs_ys.is_lt())
    } else if xs_vs_ys == xe_vs_ye {
        RangeRel::Overlaps(xs_vs_ys.is_lt())
    } else {
        RangeRel::During(xs_vs_ys.is_gt())
    }
}
