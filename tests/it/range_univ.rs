use crate::for_test::aliases::*;
use crate::for_test::consts::*;
use crate::for_test::macros::*;
use crate::for_test::*;
use rich_range::parts::*;
use rich_range::prelude::*;
use rich_range::*;
use std::cmp::Ordering;
use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
use std::ops::{Index, IndexMut, RangeBounds};
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use test_panic::prelude::*;

#[test]
fn new() {
    let start = In(START);
    let end = Ex(END);
    let result = RangeUniv::new(start, end);
    assert_eq!(result.start, In(START));
    assert_eq!(result.end, Ex(END));
}

#[test]
fn new_cursor() {
    let result = RangeUniv::new_cursor(42);
    assert_eq!(result, r!(=42, ?42));
}

#[test]
fn new_broken() {
    let result = RangeUniv::new_broken();
    assert_eq!(result, r!(=MAX, ?0));
}

#[test]
fn new_point() {
    let result = RangeUniv::new_point(42);
    assert_eq!(result, r!(=42, =42));
}

#[test]
fn with_start_bound() {
    let target = ts::range_univ();
    let result = target.with_start_bound(In(START - 1));
    assert_eq!(result.start, In(START - 1));
    assert_eq!(result.end, Ex(END));
}

#[test]
fn with_end_bound() {
    let target = ts::range_univ();
    let result = target.with_end_bound(Ex(END + 1));
    assert_eq!(result.start, In(START));
    assert_eq!(result.end, Ex(END + 1));
}

#[test]
fn contains() {
    let target = ru::new(30..60);
    assert!(target.contains(&40));
    assert!(!target.contains(&70));
}

#[test]
fn is_empty() {
    assert!(ru::new(30..30).is_empty());
    assert!(ru::new(60..30).is_empty());
    assert!(!ru::new(30..60).is_empty());
    assert!(!ru::new(30..=30).is_empty());
}

#[test]
fn is_broken() {
    assert!(ru::new(60..30).is_broken());
    assert!(!ru::new(30..60).is_broken());
    assert!(!ru::new(30..30).is_broken());
    assert!(!ru::new(30..=30).is_broken());
}

#[test]
fn is_cursor() {
    assert!(ru::new(30..30).is_cursor());
    assert!(!ru::new(30..60).is_cursor());
    assert!(!ru::new(60..30).is_cursor());
    assert!(!ru::new(30..=30).is_cursor());
}

#[test]
fn is_cursor_fwd() {
    let target1 = ru::new((In(30), Ex(30)));
    let target2 = ru::new((Ex(30), In(30)));
    assert!(target1.is_cursor_fwd());
    assert!(!target2.is_cursor_fwd());
}

#[test]
fn is_cursor_bwd() {
    let target1 = ru::new((Ex(30), In(30)));
    let target2 = ru::new((In(30), Ex(30)));
    assert!(target1.is_cursor_bwd());
    assert!(!target2.is_cursor_bwd());
}

#[test]
fn is_point() {
    assert!(ru::new(30..=30).is_point());
    assert!(!ru::new(30..30).is_point());
    assert!(!ru::new(30..60).is_point());
    assert!(!ru::new(60..30).is_point());
}

#[test]
fn is_wide() {
    assert!(ru::new::<_, usize>(..).is_wide());
    assert!(ru::new(30..).is_wide());
    assert!(!ru::new(30..60).is_wide());
}

#[test]
fn is_full() {
    assert!(ru::new::<_, usize>(..).is_full());
    assert!(!ru::new(30..).is_full());
    assert!(!ru::new(30..60).is_full());
}

#[test]
fn start_edge() {
    let target = ru::new(30..60);
    let result = target.start_edge();
    assert_eq!(result, Edge::new(Side::S, In(&30)));
}

#[test]
fn end_edge() {
    let target = ru::new(30..60);
    let result = target.end_edge();
    assert_eq!(result, Edge::new(Side::E, Ex(&60)));
}

#[test]
fn head() {
    assert_eq!(ru::new(30..).head(), 30);
    assert_eq!(ru::new((Ex(30), Ub)).head(), 31);
    assert_eq!(ru::new(..60).head(), i32::MIN);
}

#[test]
fn tail() {
    assert_eq!(ru::new(30..60).tail(), 59);
    assert_eq!(ru::new(30..=60).tail(), 60);
    assert_eq!(ru::new(30..).tail(), i32::MAX);
}

#[test]
fn prev() {
    assert_eq!(ru::new(30..).prev(), 29);
    assert_eq!(ru::new((Ex(30), Ub)).prev(), 30);
    assert_eq!(ru::new(..60).prev(), i32::MIN);
}

#[test]
fn next() {
    assert_eq!(ru::new(30..60).next(), 60);
    assert_eq!(ru::new(30..=60).next(), 61);
    assert_eq!(ru::new(30..).next(), i32::MAX);
}

#[test]
fn cursor() {
    assert_eq!(ru::new(30..60).cursor(), None);
    assert_eq!(ru::new(30..=30).cursor(), None);
    assert_eq!(ru::new(30..30).cursor(), Some(&30));
}

#[test]
fn point() {
    assert_eq!(ru::new(30..60).point(), None);
    assert_eq!(ru::new(30..30).point(), None);
    assert_eq!(ru::new(30..=30).point(), Some(&30));
}

#[test]
fn len() {
    assert_eq!(ru::new(30..).len(), None);
    assert_eq!(ru::new(30..60).len(), Some(30));
    assert_eq!(ru::new(60..30).len(), Some(0));
    assert_eq!(ru::new(30..=60).len(), Some(31));
}

#[test]
fn size() {
    assert_eq!(ru::new(30..).size(), None);
    assert_eq!(ru::new(30..60).size(), Some(30));
    assert_eq!(ru::new(60..30).size(), Some(0));
    assert_eq!(ru::new(30..=60).size(), Some(31));
}

#[test]
fn width() {
    assert_eq!(ru::new(30.0..).width(), None);
    assert_eq!(ru::new(30.0..60.0).width(), Some(30.0));
    assert_eq!(ru::new(60.0..30.0).width(), Some(0.0));
    assert_eq!(ru::new(30.0..=60.0).width(), Some(30.0));
}

#[test]
fn bounds() {
    let target = ru::new(30..60);
    let result = target.bounds();
    assert_eq!(result.0, In(&30));
    assert_eq!(result.1, Ex(&60));
}

#[test]
fn edges() {
    let target = ru::new(30..60);
    let result = target.edges();
    assert_eq!(result.0, Edge::new(Side::S, In(&30)));
    assert_eq!(result.1, Edge::new(Side::E, Ex(&60)));
}

#[test]
fn as_ref() {
    let r = ru::new(30..60);
    let r = r.as_ref();
    assert_eq!(r.start_bound(), In(&30));
}

#[test]
fn cast() {
    let src = ru::new::<_, u16>(30..60);
    assert_eq!(src.cast::<f32>(), ru::new(30.0..60.0));
}

#[test]
fn try_cast() {
    let src1 = ru::new::<_, i16>(30..60);
    let src2 = ru::new::<_, i16>(-30..60);
    assert_eq!(src1.try_cast::<u16>(), Some(ru::new(30..60)));
    assert_eq!(src2.try_cast::<u16>(), None);
}

#[test]
fn to_range() {
    let target = ru::new(30..=60);
    let result = target.to_range();
    assert_eq!(result, 30..61);
}

#[test]
fn iter() {
    let target = ru::new(3..6);
    let result = target.iter();
    assert!(result.eq([3, 4, 5].into_iter()));
}

#[test]
#[rustfmt::skip]
fn flip() {
    let r1 = ru::new::<_, usize>(..).flip();
    let r2 = ru::new(30..).flip();
    let r3 = ru::new(..60).flip();
    let r4 = ru::new(30..60).flip();
    let r5 = ru::new(30..30).flip();
    assert_eq!(r1, (None, None));
    assert_eq!(r2, (Some(ru::new(..30)), None));
    assert_eq!(r3, (Some(ru::new(60..)), None));
    assert_eq!(r4, (Some(ru::new(..30)), Some(ru::new(60..))));
    assert_eq!(r5, (Some(ru::new(..)), None));
}

#[test]
#[rustfmt::skip]
fn flip_adv() {
    let r1 = ru::new::<_, usize>(..).flip_adv(CursorMode::Off);
    let r2 = ru::new(30..).flip_adv(CursorMode::Off);
    let r3 = ru::new(..60).flip_adv(CursorMode::Off);
    let r4 = ru::new(30..60).flip_adv(CursorMode::Off);
    let r5 = ru::new(30..30).flip_adv(CursorMode::Off);
    let r6 = ru::new(30..30).flip_adv(CursorMode::On);
    assert_eq!(r1, (None, None));
    assert_eq!(r2, (Some(ru::new(..30)), None));
    assert_eq!(r3, (Some(ru::new(60..)), None));
    assert_eq!(r4, (Some(ru::new(..30)), Some(ru::new(60..))));
    assert_eq!(r5, (Some(ru::new(..)), None));
    assert_eq!(r6, (Some(ru::new(..30)), Some(ru::new(30..))));
}

#[test]
fn shift() {
    let target = ru::new(30..60);
    let result = target.shift(10, false);
    assert_eq!(result, ru::new(20..50));
}

#[test]
fn add_start() {
    let target = ru::new(30..60);
    let result = target.add_start(10);
    assert_eq!(result, ru::new(40..60));
}

#[test]
fn add_end() {
    let target = ru::new(30..60);
    let result = target.add_end(10);
    assert_eq!(result, ru::new(30..70));
}

#[test]
fn sub_start() {
    let target = ru::new(30..60);
    let result = target.sub_start(10);
    assert_eq!(result, ru::new(20..60));
}

#[test]
fn calc_start() {
    let target = ru::new(30..60);
    let result = target.calc_start(40);
    assert_eq!(result, ru::new(20..60));
}

#[test]
fn calc_end() {
    let target = ru::new(30..60);
    let result = target.calc_end(40);
    assert_eq!(result, ru::new(30..70));
}

#[test]
fn sub_end() {
    let target = ru::new(30..60);
    let result = target.sub_end(10);
    assert_eq!(result, ru::new(30..50));
}

#[test]
fn align_start() {
    let target = ru::new(30..60);
    let result = target.align_start(40);
    assert_eq!(result, ru::new(40..70));
}

#[test]
fn align_end() {
    let target = ru::new(30..60);
    let result = target.align_end(70);
    assert_eq!(result, ru::new(40..70));
}

#[test]
fn map() {
    let target = ru::new(30..);
    let result = target.map(|x| x * 2);
    assert_eq!(result, ru::new(60..));
}

#[test]
fn try_map() {
    let target = ru::new::<_, u8>(30..);
    let result1 = target.clone().try_map(|x| x.checked_mul(2));
    let result2 = target.clone().try_map(|x| x.checked_mul(10));
    assert_eq!(result1, Some(ru::new(60..)));
    assert_eq!(result2, None);
}

#[test]
fn checked_shift() {
    let target = ru::new::<_, u8>(30..60);
    let result1 = target.checked_shift(10, false);
    let result2 = target.checked_shift(40, false);
    assert_eq!(result1, Some(ru::new(20..50)));
    assert_eq!(result2, None);
}

#[test]
fn checked_shl() {
    let target = ru::new::<_, u8>(30..60);
    let result1 = target.checked_shl(10);
    let result2 = target.checked_shl(40);
    assert_eq!(result1, Some(ru::new(20..50)));
    assert_eq!(result2, None);
}

#[test]
fn checked_shr() {
    let target = ru::new::<_, u8>(30..60);
    let result1 = target.checked_shr(10);
    let result2 = target.checked_shr(200);
    assert_eq!(result1, Some(ru::new(40..70)));
    assert_eq!(result2, None);
}

#[test]
fn checked_add_start() {
    let target = ru::new::<_, u8>(30..60);
    let result1 = target.checked_add_start(10);
    let result2 = target.checked_add_start(250);
    assert_eq!(result1, Some(ru::new(40..60)));
    assert_eq!(result2, None);
}

#[test]
fn checked_add_end() {
    let target = ru::new::<_, u8>(30..60);
    let result1 = target.checked_add_end(10);
    let result2 = target.checked_add_end(250);
    assert_eq!(result1, Some(ru::new(30..70)));
    assert_eq!(result2, None);
}

#[test]
fn checked_sub_start() {
    let target = ru::new::<_, u8>(30..60);
    let result1 = target.checked_sub_start(10);
    let result2 = target.checked_sub_start(40);
    assert_eq!(result1, Some(ru::new(20..60)));
    assert_eq!(result2, None);
}

#[test]
fn checked_sub_end() {
    let target = ru::new::<_, u8>(30..60);
    let result1 = target.checked_sub_end(10);
    let result2 = target.checked_sub_end(70);
    assert_eq!(result1, Some(ru::new(30..50)));
    assert_eq!(result2, None);
}

#[test]
fn checked_calc_start() {
    let target = ru::new::<_, u8>(30..60);
    let r1 = target.checked_calc_start(40);
    let r2 = target.checked_calc_start(70);
    assert_eq!(r1, Some(ru::new(20..60)));
    assert_eq!(r2, None);
}

#[test]
fn checked_calc_end() {
    let target = ru::new::<_, u8>(30..60);
    let r1 = target.checked_calc_end(40);
    let r2 = target.checked_calc_end(230);
    assert_eq!(r1, Some(ru::new(30..70)));
    assert_eq!(r2, None);
}

#[test]
fn checked_align_start() {
    let target = ru::new::<_, u8>(30..60);
    let result1 = target.checked_align_start(40);
    let result2 = target.checked_align_start(230);
    assert_eq!(result1, Some(ru::new(40..70)));
    assert_eq!(result2, None);
}

#[test]
fn checked_align_end() {
    let target = ru::new::<_, u8>(30..60);
    let result1 = target.checked_align_end(70);
    let result2 = target.checked_align_end(20);
    assert_eq!(result1, Some(ru::new(40..70)));
    assert_eq!(result2, None);
}

#[test]
fn equiv() {
    assert!(ru::new(30..60).equiv(&(30..60)));
    assert!(!ru::new(30..60).equiv(&(30..65)));
    assert!(ru::new(30..30).equiv(&(0..0)));
}

#[test]
fn intersects() {
    assert!(ru::new(30..60).intersects(&(50..70)));
    assert!(ru::new(30..60).intersects(&(50..50)));
    assert!(!ru::new(30..60).intersects(&(70..80)));
}

#[test]
fn includes() {
    assert!(ru::new(30..60).includes(&(40..50)));
    assert!(!ru::new(30..60).includes(&(70..80)));
    assert!(!ru::new(30..60).includes(&(60..60)));
    assert!(ru::new(30..30).includes(&(30..30)));
    assert!(!ru::new(30..30).includes(&(40..40)));
}

#[test]
fn included() {
    assert!(ru::new(30..60).included(&(20..70)));
    assert!(!ru::new(30..60).included(&(40..70)));
    assert!(ru::new(30..30).included(&(30..30)));
    assert!(!ru::new(30..30).included(&(40..40)));
}

#[test]
fn adjoins() {
    assert!(ru::new(30..=60).adjoins(&(60..70)));
    assert!(!ru::new(30..=60).adjoins(&(70..80)));
    assert!(ru::new(30..60).adjoins(&(20..=30)));
    assert!(!ru::new(30..60).adjoins(&(10..=20)));
}

#[test]
fn adjoins_prev() {
    assert!(ru::new(30..60).adjoins_prev(&(20..=30)));
    assert!(!ru::new(30..60).adjoins_prev(&(10..=20)));
}

#[test]
fn adjoins_next() {
    assert!(ru::new(30..=60).adjoins_next(&(60..70)));
    assert!(!ru::new(30..=60).adjoins_next(&(70..80)));
}

#[test]
fn touches() {
    assert!(ru::new(30..60).touches(&(60..70)));
    assert!(!ru::new(30..60).touches(&(70..80)));
    assert!(ru::new(30..60).touches(&(20..30)));
    assert!(!ru::new(30..60).touches(&(10..20)));
}

#[test]
fn touches_prev() {
    assert!(ru::new(30..60).touches_prev(&(20..30)));
    assert!(!ru::new(30..60).touches_prev(&(10..20)));
}

#[test]
fn touches_next() {
    assert!(ru::new(30..60).touches_next(&(60..70)));
    assert!(!ru::new(30..60).touches_next(&(70..80)));
}

#[test]
fn rel() {
    let target = ru::new(30..60);
    let result = target.rel(&ru::new(20..70), PosStyle::Step);
    assert_eq!(result, RangeRel::During(true));
}

#[test]
fn cut() {
    let r = ru::new(30..60);
    let (fst, snd) = r.cut(&40);
    assert_eq!(fst, Some(ru::new(30..40)));
    assert_eq!(snd, Some(ru::new(40..60)));
}

#[test]
fn cut_adv() {
    let r = ru::new(30..60);
    let (fst, snd) = r.cut_adv(&40, CutMode::FallbackFw);
    assert_eq!(fst, Some(ru::new(30..40)));
    assert_eq!(snd, Some(ru::new(40..60)));
}

#[test]
fn interval() {
    let target = ru::new(30..60);
    let r1 = target.interval(&ru::new(50..70));
    let r2 = target.interval(&ru::new(60..80));
    let r3 = target.interval(&ru::new(70..90));
    assert_eq!(r1, None);
    assert_eq!(r2, None);
    assert_eq!(r3, Some(ru::new(60..70)));
}

#[test]
fn interval_adv() {
    let target = ru::new(30..60);
    let r1 = target.interval_adv(&ru::new(50..70), CursorMode::Off);
    let r2 = target.interval_adv(&ru::new(60..80), CursorMode::Off);
    let r3 = target.interval_adv(&ru::new(60..80), CursorMode::On);
    let r4 = target.interval_adv(&ru::new(70..90), CursorMode::Off);
    assert_eq!(r1, None);
    assert_eq!(r2, None);
    assert_eq!(r3, Some(ru::new(60..60)));
    assert_eq!(r4, Some(ru::new(60..70)));
}

#[test]
fn prod() {
    let r = ru::new(30..60).prod(&ru::new(40..70));
    assert_eq!(r, Some(ru::new(40..60)));
}

#[test]
fn enwrap() {
    let r1 = ru::new(20..40).enwrap(&ru::new(30..50));
    let r2 = ru::new(10..20).enwrap(&ru::new(40..60));
    assert_eq!(r1, Some(ru::new(20..50)));
    assert_eq!(r2, Some(ru::new(10..60)));
}

#[test]
fn union() {
    let r1 = ru::new(30..60).union(&ru::new(40..70));
    let r2 = ru::new(30..60).union(&ru::new(70..80));
    assert_eq!(r1, (ru::new(30..70), None));
    assert_eq!(r2, (ru::new(30..60), Some(ru::new(70..80))));
}

#[test]
fn diff() {
    let r1 = ru::new(30..60).diff(&(50..70));
    let r2 = ru::new(30..60).diff(&(40..50));
    let r3 = ru::new(30..60).diff(&(40..40));
    assert_eq!(r1, (Some(ru::new(30..50)), None));
    assert_eq!(r2, (Some(ru::new(30..40)), Some(ru::new(50..60))));
    assert_eq!(r3, (Some(ru::new(30..60)), None));
}

#[test]
fn diff_adv() {
    let r1 = ru::new(30..60).diff_adv(&(50..70), CursorMode::Off);
    let r2 = ru::new(30..60).diff_adv(&(40..50), CursorMode::Off);
    let r3 = ru::new(30..60).diff_adv(&(40..40), CursorMode::Off);
    let r4 = ru::new(30..60).diff_adv(&(40..40), CursorMode::On);
    assert_eq!(r1, (Some(ru::new(30..50)), None));
    assert_eq!(r2, (Some(ru::new(30..40)), Some(ru::new(50..60))));
    assert_eq!(r3, (Some(ru::new(30..60)), None));
    assert_eq!(r4, (Some(ru::new(30..40)), Some(ru::new(40..60))));
}

#[test]
fn default() {
    let result = RangeUniv::<usize>::default();
    assert_eq!(result, ru::new(usize::default()..usize::default()));
}

#[test]
fn from() {
    assert_eq!(RangeUniv::<usize>::from(30..60), ru::new(30..60));
    assert_eq!(RangeUniv::<usize>::from(30..), ru::new(30..));
    assert_eq!(RangeUniv::<usize>::from(..60), ru::new(..60));
    assert_eq!(RangeUniv::<usize>::from(30..=60), ru::new(30..=60));
    assert_eq!(RangeUniv::<usize>::from(..=60), ru::new(..=60));
    assert_eq!(RangeUniv::<usize>::from(..), ru::new(..));
    assert_eq!(
        RangeUniv::<usize>::from((Ex(30), Ex(60))),
        ru::new((Ex(30), Ex(60)))
    );
}

#[test]
fn start_bound() {
    when_normal();
    when_ref();

    fn when_normal() {
        let target = ts::range_univ();
        let result = target.start_bound();
        assert_eq!(result, In(&START));
    }

    fn when_ref() {
        let value = ts::range_univ();
        let target = value.as_ref();
        let result = target.start_bound();
        assert_eq!(result, In(&START));
    }
}

#[test]
fn end_bound() {
    when_normal();
    when_ref();

    fn when_normal() {
        let target = ts::range_univ();
        let result = target.end_bound();
        assert_eq!(result, Ex(&END));
    }

    fn when_ref() {
        let value = ts::range_univ();
        let target = value.as_ref();
        let result = target.end_bound();
        assert_eq!(result, Ex(&END));
    }
}

#[test]
fn into_iter() {
    let target = r!(=3, ?6);
    let master = 3..6;
    let asis = target.into_iter();
    let tobe = master.into_iter();
    assert!(asis.eq(tobe));
}

#[test]
fn partial_cmp() {
    when_normal();
    when_unmixable();

    fn when_normal() {
        let datas = [
            // Full range vs others.
            (r!(---, ---), r!(---, ---), Some(Ordering::Equal)),
            (r!(---, ---), r!(=30, ---), None),
            (r!(---, ---), r!(---, ?60), None),
            (r!(---, ---), r!(?30, ---), None),
            (r!(---, ---), r!(---, =60), None),
            (r!(---, ---), r!(=30, ?60), None),
            (r!(---, ---), r!(=30, =30), None),
            (r!(---, ---), r!(=30, ?30), None),
            (r!(---, ---), r!(=60, ?30), None),
            // Included start ranges vs others.
            (r!(=30, ---), r!(=20, ---), None),
            (r!(=30, ---), r!(=30, ---), Some(Ordering::Equal)),
            (r!(=30, ---), r!(=40, ---), None),
            (r!(=30, ---), r!(---, ?20), Some(Ordering::Greater)),
            (r!(=30, ---), r!(---, ?30), Some(Ordering::Greater)),
            (r!(=30, ---), r!(---, ?40), None),
            (r!(=30, ---), r!(?20, ---), None),
            (r!(=30, ---), r!(?30, ---), None),
            (r!(=30, ---), r!(?40, ---), None),
            (r!(=30, ---), r!(---, =20), Some(Ordering::Greater)),
            (r!(=30, ---), r!(---, =30), None),
            (r!(=30, ---), r!(---, =40), None),
            (r!(=30, ---), r!(=10, ?20), Some(Ordering::Greater)),
            (r!(=30, ---), r!(=10, ?30), Some(Ordering::Greater)),
            (r!(=30, ---), r!(=10, ?40), None),
            (r!(=30, ---), r!(=20, ?20), Some(Ordering::Greater)),
            (r!(=30, ---), r!(=30, ?30), None),
            (r!(=30, ---), r!(=40, ?40), None),
            (r!(=30, ---), r!(=20, ?10), None),
            // Excluded end ranges vs others.
            (r!(---, ?60), r!(---, ?50), None),
            (r!(---, ?60), r!(---, ?60), Some(Ordering::Equal)),
            (r!(---, ?60), r!(---, ?70), None),
            (r!(---, ?60), r!(?50, ---), None),
            (r!(---, ?60), r!(?60, ---), Some(Ordering::Less)),
            (r!(---, ?60), r!(?70, ---), Some(Ordering::Less)),
            (r!(---, ?60), r!(---, =50), None),
            (r!(---, ?60), r!(---, =60), None),
            (r!(---, ?60), r!(---, =70), None),
            (r!(---, ?60), r!(=50, ?80), None),
            (r!(---, ?60), r!(=59, ?80), None),
            (r!(---, ?60), r!(=60, ?80), Some(Ordering::Less)),
            (r!(---, ?60), r!(=50, ?50), None),
            (r!(---, ?60), r!(=60, ?60), Some(Ordering::Less)),
            (r!(---, ?60), r!(=70, ?70), Some(Ordering::Less)),
            (r!(---, ?60), r!(=80, ?70), None),
            // Excluded start ranges vs others.
            (r!(?30, ---), r!(?20, ---), None),
            (r!(?30, ---), r!(?30, ---), Some(Ordering::Equal)),
            (r!(?30, ---), r!(?40, ---), None),
            (r!(?30, ---), r!(---, =20), Some(Ordering::Greater)),
            (r!(?30, ---), r!(---, =30), Some(Ordering::Greater)),
            (r!(?30, ---), r!(---, =40), None),
            (r!(?30, ---), r!(=10, ?20), Some(Ordering::Greater)),
            (r!(?30, ---), r!(=10, ?30), Some(Ordering::Greater)),
            (r!(?30, ---), r!(=10, ?40), None),
            (r!(?30, ---), r!(=20, ?20), Some(Ordering::Greater)),
            (r!(?30, ---), r!(=30, ?30), Some(Ordering::Greater)),
            (r!(?30, ---), r!(=40, ?40), None),
            (r!(?30, ---), r!(=20, ?10), None),
            // Included end ranges vs others.
            (r!(---, =60), r!(---, =50), None),
            (r!(---, =60), r!(---, =60), Some(Ordering::Equal)),
            (r!(---, =60), r!(---, =70), None),
            (r!(---, =60), r!(=50, ?80), None),
            (r!(---, =60), r!(=60, ?80), None),
            (r!(---, =60), r!(=61, ?80), Some(Ordering::Less)),
            (r!(---, =60), r!(=50, ?50), None),
            (r!(---, =60), r!(=60, ?60), None),
            (r!(---, =60), r!(=61, ?61), Some(Ordering::Less)),
            (r!(---, =60), r!(=80, ?70), None),
            // Normal ranges vs others.
            (r!(=30, ?60), r!(=00, ?20), Some(Ordering::Greater)),
            (r!(=30, ?60), r!(=00, ?30), Some(Ordering::Greater)),
            (r!(=30, ?60), r!(=00, =30), None),
            (r!(=30, ?60), r!(=59, ?90), None),
            (r!(=30, ?60), r!(=60, ?90), Some(Ordering::Less)),
            (r!(=30, ?60), r!(=20, ?20), Some(Ordering::Greater)),
            (r!(=30, ?60), r!(=30, ?30), None),
            (r!(=30, ?60), r!(=60, ?60), Some(Ordering::Less)),
            (r!(=30, ?60), r!(=80, ?70), None),
            // Empty ranges vs empty ranges.
            (r!(=30, ?30), r!(=20, ?20), Some(Ordering::Greater)),
            (r!(=30, ?30), r!(=30, ?30), Some(Ordering::Equal)),
            (r!(=30, ?30), r!(=40, ?40), Some(Ordering::Less)),
            (r!(=30, ?30), r!(=50, ?40), None),
            (r!(=50, ?40), r!(=30, ?30), None),
        ];

        for (target, other, tobe) in datas {
            let asis = target.partial_cmp(&other);
            let rev = other.partial_cmp(&target);
            assert_eq!(asis, tobe);
            assert_eq!(rev, tobe.map(Ordering::reverse));
        }
    }

    fn when_unmixable() {
        for (target, other) in ts::unmixables() {
            let result = target.partial_cmp(&other);
            assert_eq!(result, None);
        }
    }
}

#[test]
fn shl() {
    let (target, rhs) = (ru::new(30..60), 10);
    let asis0 = (&target).shl(rhs);
    let asis1 = Shl::shl(target, rhs);
    let asis2 = Shl::shl(target, &rhs);
    let asis3 = Shl::shl(&target, rhs);
    let asis4 = Shl::shl(&target, &rhs);
    let tobe = ru::new(20..50);
    assert_eq!(asis0, tobe);
    assert_eq!(asis1, tobe);
    assert_eq!(asis2, tobe);
    assert_eq!(asis3, tobe);
    assert_eq!(asis4, tobe);
}

#[test]
fn shr() {
    let (target, rhs) = (ru::new(30..60), 10);
    let asis0 = (&target).shr(rhs);
    let asis1 = Shr::shr(target, rhs);
    let asis2 = Shr::shr(target, &rhs);
    let asis3 = Shr::shr(&target, rhs);
    let asis4 = Shr::shr(&target, &rhs);
    let tobe = ru::new(40..70);
    assert_eq!(asis0, tobe);
    assert_eq!(asis1, tobe);
    assert_eq!(asis2, tobe);
    assert_eq!(asis3, tobe);
    assert_eq!(asis4, tobe);
}

#[test]
fn bitand() {
    let datas = [
        (r!(=30, ?60), r!(=40, ?70), r!(=040, ?60)),
        (r!(=20, ?40), r!(=50, ?70), r!(=MAX, ?00)),
    ];

    for (target, rhs, tobe) in datas {
        let asis1 = target.bitand(rhs);
        let asis2 = target.bitand(&rhs);
        let asis3 = (&target).bitand(rhs);
        let asis4 = (&target).bitand(&rhs);
        assert_eq!(asis1, tobe);
        assert_eq!(asis2, tobe);
        assert_eq!(asis3, tobe);
        assert_eq!(asis4, tobe);
    }
}

#[test]
fn bitor() {
    let datas = [
        (r!(=30, ?60), r!(=40, ?70), r!(=30, ?70)),
        (r!(=30, ?60), r!(=70, ?80), r!(=30, ?60)),
        (r!(=30, ?60), r!(=10, ?20), r!(=30, ?60)),
        (r!(=60, ?30), r!(=30, ?60), r!(=60, ?30)),
        (r!(=30, ?60), r!(=60, ?30), r!(=30, ?60)),
    ];

    for (target, rhs, tobe) in datas {
        let asis1 = target.bitor(rhs);
        let asis2 = target.bitor(&rhs);
        let asis3 = (&target).bitor(rhs);
        let asis4 = (&target).bitor(&rhs);
        assert_eq!(asis1, tobe);
        assert_eq!(asis2, tobe);
        assert_eq!(asis3, tobe);
        assert_eq!(asis4, tobe);
    }
}

#[test]
fn bitxor() {
    let datas = [
        (r!(=30, ?40), r!(=60, ?70), r!(=030, ?70)),
        (r!(=30, ?30), r!(=60, ?60), r!(=MAX, ?00)),
    ];

    for (target, rhs, tobe) in datas {
        let asis1 = target.bitxor(rhs);
        let asis2 = target.bitxor(&rhs);
        let asis3 = (&target).bitxor(rhs);
        let asis4 = (&target).bitxor(&rhs);
        assert_eq!(asis1, tobe);
        assert_eq!(asis2, tobe);
        assert_eq!(asis3, tobe);
        assert_eq!(asis4, tobe);
    }
}

#[test]
fn shl_assign() {
    when_normal();
    when_overflow();

    fn when_normal() {
        let datas = [
            ((r!(---, ---), 1), r!(---, ---)),
            ((r!(?31, ?61), 1), r!(?30, ?60)),
            ((r!(=31, =61), 1), r!(=30, =60)),
        ];

        for ((target, rhs), after) in datas {
            let mut target1 = target;
            let mut target2 = target;
            target1.shl_assign(rhs);
            target2.shl_assign(&rhs);
            assert_eq!(target1, after);
            assert_eq!(target2, after);
        }
    }

    fn when_overflow() {
        if !tu::overflow_checks() {
            return;
        }

        let datas = [
            ((r!(---, =00), 1), r!(---, =0)),
            ((r!(=0 , ---), 1), r!(=0 ,---)),
        ];

        for ((target, rhs), after) in datas {
            let mut target1 = target;
            let mut target2 = target;
            let result1 = test_panic(|| target1.shl_assign(rhs));
            let result2 = test_panic(|| target2.shl_assign(&rhs));
            assert!(result1.is_panic());
            assert!(result2.is_panic());
            assert_eq!(target1, after);
            assert_eq!(target2, after);
        }
    }
}

#[test]
fn shr_assign() {
    when_normal();
    when_overflow();

    fn when_normal() {
        let datas = [
            ((r!(----, ----), 1), r!(---, ---)),
            ((r!(?030, ?060), 1), r!(?31, ?61)),
            ((r!(=030, =060), 1), r!(=31, =61)),
        ];

        for ((target, rhs), after) in datas {
            let mut target1 = target;
            let mut target2 = target;
            target1.shr_assign(rhs);
            target2.shr_assign(&rhs);
            assert_eq!(target1, after);
            assert_eq!(target2, after);
        }
    }

    fn when_overflow() {
        if !tu::overflow_checks() {
            return;
        }

        let datas = [
            ((r!(----, =MAX), 1), r!(----, =MAX)),
            ((r!(=MAX, ----), 1), r!(=MAX, ----)),
        ];

        for ((target, rhs), after) in datas {
            let mut target1 = target;
            let mut target2 = target;
            let result1 = test_panic(|| target1.shr_assign(rhs));
            let result2 = test_panic(|| target2.shr_assign(&rhs));
            assert!(result1.is_panic());
            assert!(result2.is_panic());
            assert_eq!(target1, after);
            assert_eq!(target2, after);
        }
    }
}

#[test]
fn bitand_assign() {
    let datas = [
        (r!(=30, ?60), r!(=40, ?70), r!(=040, ?60)),
        (r!(=20, ?40), r!(=50, ?70), r!(=MAX, ?00)),
    ];

    for (target, rhs, tobe) in datas {
        let mut target1 = target;
        let mut target2 = target;
        target1.bitand_assign(rhs);
        target2.bitand_assign(&rhs);
        assert_eq!(target1, tobe);
        assert_eq!(target2, tobe);
    }
}

#[test]
fn bitor_assign() {
    let datas = [
        (r!(=30, ?60), r!(=40, ?70), r!(=30, ?70)),
        (r!(=30, ?60), r!(=70, ?80), r!(=30, ?60)),
        (r!(=30, ?60), r!(=10, ?20), r!(=30, ?60)),
        (r!(=60, ?30), r!(=30, ?60), r!(=60, ?30)),
        (r!(=30, ?60), r!(=60, ?30), r!(=30, ?60)),
    ];

    for (target, rhs, tobe) in datas {
        let mut target1 = target;
        let mut target2 = target;
        target1.bitor_assign(rhs);
        target2.bitor_assign(&rhs);
        assert_eq!(target1, tobe);
        assert_eq!(target2, tobe);
    }
}

#[test]
fn bitxor_assign() {
    let datas = [
        (r!(=30, ?40), r!(=60, ?70), r!(=030, ?70)),
        (r!(=30, ?30), r!(=60, ?60), r!(=MAX, ?00)),
    ];

    for (target, rhs, tobe) in datas {
        let mut target1 = target;
        let mut target2 = target;
        target1.bitxor_assign(rhs);
        target2.bitxor_assign(&rhs);
        assert_eq!(target1, tobe);
        assert_eq!(target2, tobe);
    }
}

#[test]
#[rustfmt::skip]
fn try_from() {
    // Ok patterns.
    assert_eq!(Range::<usize>::try_from(ru::new(30..60)), Ok(30..60));
    assert_eq!(RangeFrom::<usize>::try_from(ru::new(30..)), Ok(30..));
    assert_eq!(RangeTo::<usize>::try_from(ru::new(..60)), Ok(..60));
    assert_eq!(RangeInclusive::<usize>::try_from(ru::new(30..=60)), Ok(30..=60));
    assert_eq!(RangeToInclusive::<usize>::try_from(ru::new(..=60)), Ok(..=60));
    assert_eq!(RangeFull::try_from(ru::new::<_, usize>(..)), Ok(..));
    // Err patterns.
    assert_eq!(Range::<usize>::try_from(ru::new(..)), Err(()));
    assert_eq!(RangeFrom::<usize>::try_from(ru::new(..)), Err(()));
    assert_eq!(RangeTo::<usize>::try_from(ru::new(..)), Err(()));
    assert_eq!(RangeInclusive::<usize>::try_from(ru::new(..)), Err(()));
    assert_eq!(RangeToInclusive::<usize>::try_from(ru::new(..)), Err(()));
    assert_eq!(RangeFull::try_from(ru::new(30..60)), Err(()));
}

#[test]
fn index() {
    let vec = (0..10).collect::<Vec<_>>();
    let arr = vec.as_slice();
    let datas = [
        r!(---, ---),
        r!(=03, ---),
        r!(---, ?06),
        r!(?03, ---),
        r!(---, =06),
        r!(=03, ?06),
    ];

    for target in datas {
        let master = (target.start_bound().cloned(), target.end_bound().cloned());
        let asis = arr.index(target);
        let tobe = arr.index(master);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn index_mut() {
    let vec1 = &mut (0..10).collect::<Vec<_>>();
    let vec2 = &mut vec1.clone();
    let arr1 = vec1.as_mut_slice();
    let arr2 = vec2.as_mut_slice();
    let datas = [
        r!(---, ---),
        r!(=03, ---),
        r!(---, ?06),
        r!(?03, ---),
        r!(---, =06),
        r!(=03, ?06),
    ];

    for target in datas {
        let master = (target.start_bound().cloned(), target.end_bound().cloned());
        let asis = arr1.index_mut(target);
        let tobe = arr2.index_mut(master);
        assert_eq!(asis, tobe);
    }
}
