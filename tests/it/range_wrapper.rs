use crate::for_test::aliases::*;
use crate::for_test::consts::*;
use crate::for_test::samples as sv;
use rich_range::parts::*;
use rich_range::prelude::*;
use rich_range::*;
use std::cmp::Ordering;
use std::ops::RangeBounds;
use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};

#[test]
fn new() {
    let result = RangeWrapper::new(START..END);
    assert_eq!(result.start_bound(), In(&START));
    assert_eq!(result.end_bound(), Ex(&END));
}

#[test]
fn from_ref() {
    let base = &sv::range();
    let result = RangeWrapper::from_ref(base);
    assert_eq!(&result.0, base);
}

#[test]
fn is_empty() {
    assert!(rw::new(30..30).is_empty());
    assert!(rw::new(60..30).is_empty());
    assert!(!rw::new(30..60).is_empty());
    assert!(!rw::new(30..=30).is_empty());
}

#[test]
fn is_broken() {
    assert!(rw::new(60..30).is_broken());
    assert!(!rw::new(30..60).is_broken());
    assert!(!rw::new(30..30).is_broken());
    assert!(!rw::new(30..=30).is_broken());
}

#[test]
fn is_cursor() {
    assert!(rw::new(30..30).is_cursor());
    assert!(!rw::new(30..60).is_cursor());
    assert!(!rw::new(60..30).is_cursor());
    assert!(!rw::new(30..=30).is_cursor());
}

#[test]
fn is_cursor_fwd() {
    let target1 = rw::new((In(30), Ex(30)));
    let target2 = rw::new((Ex(30), In(30)));
    assert!(target1.is_cursor_fwd());
    assert!(!target2.is_cursor_fwd());
}

#[test]
fn is_cursor_bwd() {
    let target1 = rw::new((Ex(30), In(30)));
    let target2 = rw::new((In(30), Ex(30)));
    assert!(target1.is_cursor_bwd());
    assert!(!target2.is_cursor_bwd());
}

#[test]
fn is_point() {
    assert!(rw::new(30..=30).is_point());
    assert!(!rw::new(30..30).is_point());
    assert!(!rw::new(30..60).is_point());
    assert!(!rw::new(60..30).is_point());
}

#[test]
fn is_wide() {
    assert!(rw::new::<_, usize>(..).is_wide());
    assert!(rw::new(30..).is_wide());
    assert!(!rw::new(30..60).is_wide());
}

#[test]
fn is_full() {
    assert!(rw::new::<_, usize>(..).is_full());
    assert!(!rw::new(30..).is_full());
    assert!(!rw::new(30..60).is_full());
}

#[test]
fn start_edge() {
    let target = rw::new(30..60);
    let result = target.start_edge();
    assert_eq!(result, Edge::new(Side::S, In(&30)));
}

#[test]
fn end_edge() {
    let target = rw::new(30..60);
    let result = target.end_edge();
    assert_eq!(result, Edge::new(Side::E, Ex(&60)));
}

#[test]
fn head() {
    assert_eq!(rw::new(30..).head(), 30);
    assert_eq!(rw::new((Ex(30), Ub)).head(), 31);
    assert_eq!(rw::new(..60).head(), i32::MIN);
}

#[test]
fn tail() {
    assert_eq!(rw::new(30..60).tail(), 59);
    assert_eq!(rw::new(30..=60).tail(), 60);
    assert_eq!(rw::new(30..).tail(), i32::MAX);
}

#[test]
fn prev() {
    assert_eq!(rw::new(30..).prev(), 29);
    assert_eq!(rw::new((Ex(30), Ub)).prev(), 30);
    assert_eq!(rw::new(..60).prev(), i32::MIN);
}

#[test]
fn next() {
    assert_eq!(rw::new(30..60).next(), 60);
    assert_eq!(rw::new(30..=60).next(), 61);
    assert_eq!(rw::new(30..).next(), i32::MAX);
}

#[test]
fn cursor() {
    assert_eq!(rw::new(30..60).cursor(), None);
    assert_eq!(rw::new(30..=30).cursor(), None);
    assert_eq!(rw::new(30..30).cursor(), Some(&30));
}

#[test]
fn point() {
    assert_eq!(rw::new(30..60).point(), None);
    assert_eq!(rw::new(30..30).point(), None);
    assert_eq!(rw::new(30..=30).point(), Some(&30));
}

#[test]
fn len() {
    assert_eq!(rw::new(30..).len(), None);
    assert_eq!(rw::new(30..60).len(), Some(30));
    assert_eq!(rw::new(60..30).len(), Some(0));
    assert_eq!(rw::new(30..=60).len(), Some(31));
}

#[test]
fn size() {
    assert_eq!(rw::new(30..).size(), None);
    assert_eq!(rw::new(30..60).size(), Some(30));
    assert_eq!(rw::new(60..30).size(), Some(0));
    assert_eq!(rw::new(30..=60).size(), Some(31));
}

#[test]
fn width() {
    assert_eq!(rw::new(30.0..).width(), None);
    assert_eq!(rw::new(30.0..60.0).width(), Some(30.0));
    assert_eq!(rw::new(60.0..30.0).width(), Some(0.0));
    assert_eq!(rw::new(30.0..=60.0).width(), Some(30.0));
}

#[test]
fn bounds() {
    let target = rw::new(30..60);
    let result = target.bounds();
    assert_eq!(result.0, In(&30));
    assert_eq!(result.1, Ex(&60));
}

#[test]
fn edges() {
    let target = rw::new(30..60);
    let result = target.edges();
    assert_eq!(result.0, Edge::new(Side::S, In(&30)));
    assert_eq!(result.1, Edge::new(Side::E, Ex(&60)));
}

#[test]
fn as_ref() {
    let r = rw::new(30..60);
    let r = r.as_ref();
    assert_eq!(r.0.start_bound(), In(&30));
}

#[test]
fn cast() {
    let src = rw::new::<_, u16>(30..60);
    assert_eq!(src.cast::<f32>(), rw::new(30.0..60.0));
}

#[test]
fn try_cast() {
    let src1 = rw::new::<_, i16>(30..60);
    let src2 = rw::new::<_, i16>(-30..60);
    assert_eq!(src1.try_cast::<u16>(), Some(rw::new(30..60)));
    assert_eq!(src2.try_cast::<u16>(), None);
}

#[test]
fn to_range() {
    let target = rw::new(30..=60);
    let result = target.to_range();
    assert_eq!(result, 30..61);
}

#[test]
fn iter() {
    let target = rw::new(3..6);
    let result = target.iter();
    assert!(result.eq([3, 4, 5].into_iter()));
}

#[test]
#[rustfmt::skip]
fn flip() {
    let r1 = rw::new::<_, usize>(..).flip();
    let r2 = rw::new(30..).flip();
    let r3 = rw::new(..60).flip();
    let r4 = rw::new(30..60).flip();
    let r5 = rw::new(30..30).flip();
    assert_eq!(r1, (None, None));
    assert_eq!(r2, (Some(ru::new(..30)), None));
    assert_eq!(r3, (Some(ru::new(60..)), None));
    assert_eq!(r4, (Some(ru::new(..30)), Some(ru::new(60..))));
    assert_eq!(r5, (Some(ru::new(..)), None));
}

#[test]
#[rustfmt::skip]
fn flip_adv() {
    let r1 = rw::new::<_, usize>(..).flip_adv(CursorMode::Off);
    let r2 = rw::new(30..).flip_adv(CursorMode::Off);
    let r3 = rw::new(..60).flip_adv(CursorMode::Off);
    let r4 = rw::new(30..60).flip_adv(CursorMode::Off);
    let r5 = rw::new(30..30).flip_adv(CursorMode::Off);
    let r6 = rw::new(30..30).flip_adv(CursorMode::On);
    assert_eq!(r1, (None, None));
    assert_eq!(r2, (Some(ru::new(..30)), None));
    assert_eq!(r3, (Some(ru::new(60..)), None));
    assert_eq!(r4, (Some(ru::new(..30)), Some(ru::new(60..))));
    assert_eq!(r5, (Some(ru::new(..)), None));
    assert_eq!(r6, (Some(ru::new(..30)), Some(ru::new(30..))));
}

#[test]
fn contains() {
    let target = rw::new(30..60);
    assert!(target.contains(&40));
    assert!(!target.contains(&70));
}

#[test]
fn add_start() {
    let target = rw::new(30..60);
    let result = target.add_start(10);
    assert_eq!(result, rw::new(40..60));
}

#[test]
fn add_end() {
    let target = rw::new(30..60);
    let result = target.add_end(10);
    assert_eq!(result, rw::new(30..70));
}

#[test]
fn sub_start() {
    let target = rw::new(30..60);
    let result = target.sub_start(10);
    assert_eq!(result, rw::new(20..60));
}

#[test]
fn sub_end() {
    let target = rw::new(30..60);
    let result = target.sub_end(10);
    assert_eq!(result, rw::new(30..50));
}

#[test]
fn align_start() {
    let target = rw::new(30..60);
    let result = target.align_start(40);
    assert_eq!(result, rw::new(40..70));
}

#[test]
fn calc_start() {
    let target = rw::new(30..60);
    let result = target.calc_start(40);
    assert_eq!(result, rw::new(20..60));
}

#[test]
fn calc_end() {
    let target = rw::new(30..60);
    let result = target.calc_end(40);
    assert_eq!(result, rw::new(30..70));
}

#[test]
fn align_end() {
    let target = rw::new(30..60);
    let result = target.align_end(70);
    assert_eq!(result, rw::new(40..70));
}

#[test]
fn map() {
    let target = rw::new(30..);
    let result = target.map(|x| x * 2);
    assert_eq!(result, rw::new(60..));
}

#[test]
fn try_map() {
    let target = rw::new::<_, u8>(30..);
    let result1 = target.clone().try_map(|x| x.checked_mul(2));
    let result2 = target.clone().try_map(|x| x.checked_mul(10));
    assert_eq!(result1, Some(rw::new(60..)));
    assert_eq!(result2, None);
}

#[test]
fn checked_shl() {
    let target = rw::new::<_, u8>(30..60);
    let result1 = target.checked_shl(10);
    let result2 = target.checked_shl(40);
    assert_eq!(result1, Some(rw::new(20..50)));
    assert_eq!(result2, None);
}

#[test]
fn checked_shr() {
    let target = rw::new::<_, u8>(30..60);
    let result1 = target.checked_shr(10);
    let result2 = target.checked_shr(200);
    assert_eq!(result1, Some(rw::new(40..70)));
    assert_eq!(result2, None);
}

#[test]
fn checked_add_start() {
    let target = rw::new::<_, u8>(30..60);
    let result1 = target.checked_add_start(10);
    let result2 = target.checked_add_start(250);
    assert_eq!(result1, Some(rw::new(40..60)));
    assert_eq!(result2, None);
}

#[test]
fn checked_add_end() {
    let target = rw::new::<_, u8>(30..60);
    let result1 = target.checked_add_end(10);
    let result2 = target.checked_add_end(250);
    assert_eq!(result1, Some(rw::new(30..70)));
    assert_eq!(result2, None);
}

#[test]
fn checked_sub_start() {
    let target = rw::new::<_, u8>(30..60);
    let result1 = target.checked_sub_start(10);
    let result2 = target.checked_sub_start(40);
    assert_eq!(result1, Some(rw::new(20..60)));
    assert_eq!(result2, None);
}

#[test]
fn checked_sub_end() {
    let target = rw::new::<_, u8>(30..60);
    let result1 = target.checked_sub_end(10);
    let result2 = target.checked_sub_end(70);
    assert_eq!(result1, Some(rw::new(30..50)));
    assert_eq!(result2, None);
}

#[test]
fn checked_calc_start() {
    let target = rw::new::<_, u8>(30..60);
    let r1 = target.checked_calc_start(40);
    let r2 = target.checked_calc_start(70);
    assert_eq!(r1, Some(rw::new(20..60)));
    assert_eq!(r2, None);
}

#[test]
fn checked_calc_end() {
    let target = rw::new::<_, u8>(30..60);
    let r1 = target.checked_calc_end(40);
    let r2 = target.checked_calc_end(230);
    assert_eq!(r1, Some(rw::new(30..70)));
    assert_eq!(r2, None);
}

#[test]
fn checked_align_start() {
    let target = rw::new::<_, u8>(30..60);
    let result1 = target.checked_align_start(40);
    let result2 = target.checked_align_start(230);
    assert_eq!(result1, Some(rw::new(40..70)));
    assert_eq!(result2, None);
}

#[test]
fn checked_align_end() {
    let target = rw::new::<_, u8>(30..60);
    let result1 = target.checked_align_end(70);
    let result2 = target.checked_align_end(20);
    assert_eq!(result1, Some(rw::new(40..70)));
    assert_eq!(result2, None);
}

#[test]
fn equiv() {
    assert!(rw::new(30..60).equiv(&(30..60)));
    assert!(!rw::new(30..60).equiv(&(30..65)));
    assert!(rw::new(30..30).equiv(&(0..0)));
}

#[test]
fn intersects() {
    assert!(rw::new(30..60).intersects(&(50..70)));
    assert!(rw::new(30..60).intersects(&(50..50)));
    assert!(!rw::new(30..60).intersects(&(70..80)));
}

#[test]
fn includes() {
    assert!(rw::new(30..60).includes(&(40..50)));
    assert!(!rw::new(30..60).includes(&(70..80)));
    assert!(!rw::new(30..60).includes(&(60..60)));
    assert!(rw::new(30..30).includes(&(30..30)));
    assert!(!rw::new(30..30).includes(&(40..40)));
}

#[test]
fn included() {
    assert!(rw::new(30..60).included(&(20..70)));
    assert!(!rw::new(30..60).included(&(40..70)));
    assert!(rw::new(30..30).included(&(30..30)));
    assert!(!rw::new(30..30).included(&(40..40)));
}

#[test]
fn adjoins() {
    assert!(rw::new(30..=60).adjoins(&(60..70)));
    assert!(!rw::new(30..=60).adjoins(&(70..80)));
    assert!(rw::new(30..60).adjoins(&(20..=30)));
    assert!(!rw::new(30..60).adjoins(&(10..=20)));
}

#[test]
fn adjoins_prev() {
    assert!(rw::new(30..60).adjoins_prev(&(20..=30)));
    assert!(!rw::new(30..60).adjoins_prev(&(10..=20)));
}

#[test]
fn adjoins_next() {
    assert!(rw::new(30..=60).adjoins_next(&(60..70)));
    assert!(!rw::new(30..=60).adjoins_next(&(70..80)));
}

#[test]
fn touches() {
    assert!(rw::new(30..60).touches(&(60..70)));
    assert!(!rw::new(30..60).touches(&(70..80)));
    assert!(rw::new(30..60).touches(&(20..30)));
    assert!(!rw::new(30..60).touches(&(10..20)));
}

#[test]
fn touches_prev() {
    assert!(rw::new(30..60).touches_prev(&(20..30)));
    assert!(!rw::new(30..60).touches_prev(&(10..20)));
}

#[test]
fn touches_next() {
    assert!(rw::new(30..60).touches_next(&(60..70)));
    assert!(!rw::new(30..60).touches_next(&(70..80)));
}

#[test]
fn rel() {
    let target = rw::new(30..60);
    let result = target.rel(&rw::new(20..70), PosStyle::Step);
    assert_eq!(result, RangeRel::During(true));
}

#[test]
fn cut() {
    let r = rw::new(30..60);
    let (fst, snd) = r.cut(&40, CutMode::Standard);
    assert_eq!(fst, Some(ru::new(30..40)));
    assert_eq!(snd, Some(ru::new(40..60)));
}

#[test]
fn interval() {
    let target = rw::new(30..60);
    let r1 = target.interval(&rw::new(50..70));
    let r2 = target.interval(&rw::new(60..80));
    let r3 = target.interval(&rw::new(70..90));
    assert_eq!(r1, None);
    assert_eq!(r2, None);
    assert_eq!(r3, Some(ru::new(60..70)));
}

#[test]
fn interval_adv() {
    let target = rw::new(30..60);
    let r1 = target.interval_adv(&rw::new(50..70), CursorMode::Off);
    let r2 = target.interval_adv(&rw::new(60..80), CursorMode::Off);
    let r3 = target.interval_adv(&rw::new(60..80), CursorMode::On);
    let r4 = target.interval_adv(&rw::new(70..90), CursorMode::Off);
    assert_eq!(r1, None);
    assert_eq!(r2, None);
    assert_eq!(r3, Some(ru::new(60..60)));
    assert_eq!(r4, Some(ru::new(60..70)));
}

#[test]
fn prod() {
    let r = rw::new(30..60).prod(&rw::new(40..70));
    assert_eq!(r, Some(rw::new(40..60)));
}

#[test]
fn enwrap() {
    let r1 = rw::new(20..40).enwrap(&rw::new(30..50));
    let r2 = rw::new(10..20).enwrap(&rw::new(40..60));
    assert_eq!(r1, Some(rw::new(20..50)));
    assert_eq!(r2, Some(rw::new(10..60)));
}

#[test]
fn union() {
    let r1 = rw::new(30..60).union(&rw::new(40..70));
    let r2 = rw::new(30..60).union(&rw::new(70..80));
    assert_eq!(r1, (rw::new(30..70), None));
    assert_eq!(r2, (rw::new(30..60), Some(rw::new(70..80))));
}

#[test]
fn diff() {
    let r1 = rw::new(30..60).diff(&(50..70));
    let r2 = rw::new(30..60).diff(&(40..50));
    let r3 = rw::new(30..60).diff(&(40..40));
    assert_eq!(r1, (Some(ru::new(30..50)), None));
    assert_eq!(r2, (Some(ru::new(30..40)), Some(ru::new(50..60))));
    assert_eq!(r3, (Some(ru::new(30..60)), None));
}

#[test]
fn diff_adv() {
    let r1 = rw::new(30..60).diff_adv(&(50..70), CursorMode::Off);
    let r2 = rw::new(30..60).diff_adv(&(40..50), CursorMode::Off);
    let r3 = rw::new(30..60).diff_adv(&(40..40), CursorMode::Off);
    let r4 = rw::new(30..60).diff_adv(&(40..40), CursorMode::On);
    assert_eq!(r1, (Some(ru::new(30..50)), None));
    assert_eq!(r2, (Some(ru::new(30..40)), Some(ru::new(50..60))));
    assert_eq!(r3, (Some(ru::new(30..60)), None));
    assert_eq!(r4, (Some(ru::new(30..40)), Some(ru::new(40..60))));
}

#[test]
fn start_bound() {
    let target = sv::range_wrapper();
    let result = target.start_bound();
    assert_eq!(result, In(&START));
}

#[test]
fn end_bound() {
    let target = sv::range_wrapper();
    let result = target.end_bound();
    assert_eq!(result, Ex(&END));
}

#[test]
fn into_iter() {
    let target = RangeWrapper::new(3..6);
    let master = 3..6;
    let asis = target.into_iter();
    let tobe = master.into_iter();
    assert!(asis.eq(tobe));
}

#[test]
fn eq() {
    let datas = [
        (sv::range_wrapper(), sv::range_wrapper().add_end(0), true),
        (sv::range_wrapper(), sv::range_wrapper().add_end(1), false),
    ];

    for (target, other, tobe) in datas {
        let asis = target.eq(&other);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn partial_cmp() {
    let datas = [
        (rw::new(30..60), rw::new(40..50), None),
        (rw::new(30..60), rw::new(10..20), Some(Ordering::Greater)),
        (rw::new(30..60), rw::new(70..80), Some(Ordering::Less)),
    ];

    for (target, other, tobe) in datas {
        let asis = target.partial_cmp(&other);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn shl() {
    with_refs_overloads();
    with_bounds_overloads();

    fn with_refs_overloads() {
        let target = || rw::new(30..60);
        let rhs = 10;
        let asis0 = (&target()).shl(rhs);
        let asis1 = Shl::shl(target(), rhs);
        let asis2 = Shl::shl(target(), &rhs);
        let asis3 = Shl::shl(&target(), rhs);
        let asis4 = Shl::shl(&target(), &rhs);
        let tobe = rw::new(20..50);
        assert_eq!(asis0, tobe);
        assert_eq!(asis1, tobe);
        assert_eq!(asis2, tobe);
        assert_eq!(asis3, tobe);
        assert_eq!(asis4, tobe);
    }

    fn with_bounds_overloads() {
        assert_eq!(rw::new(..).shl(10), rw::new(..));
        assert_eq!(rw::new(30..).shl(10), rw::new(20..));
        assert_eq!(rw::new(..60).shl(10), rw::new(..50));
        assert_eq!(rw::new(..=60).shl(10), rw::new(..=50));
        assert_eq!(rw::new(30..60).shl(10), rw::new(20..50));
        assert_eq!(rw::new(30..=60).shl(10), rw::new(20..=50));
        assert_eq!(rw::new((Ex(30), In(60))).shl(10), rw::new((Ex(20), In(50))));
    }
}

#[test]
fn shr() {
    with_refs_overloads();
    with_bounds_overloads();

    fn with_refs_overloads() {
        let target = || rw::new(30..60);
        let rhs = 10;
        let asis0 = (&target()).shr(rhs);
        let asis1 = Shr::shr(target(), rhs);
        let asis2 = Shr::shr(target(), &rhs);
        let asis3 = Shr::shr(&target(), rhs);
        let asis4 = Shr::shr(&target(), &rhs);
        let tobe = rw::new(40..70);
        assert_eq!(asis0, tobe);
        assert_eq!(asis1, tobe);
        assert_eq!(asis2, tobe);
        assert_eq!(asis3, tobe);
        assert_eq!(asis4, tobe);
    }

    fn with_bounds_overloads() {
        assert_eq!(rw::new(..).shr(10), rw::new(..));
        assert_eq!(rw::new(30..).shr(10), rw::new(40..));
        assert_eq!(rw::new(..60).shr(10), rw::new(..70));
        assert_eq!(rw::new(..=60).shr(10), rw::new(..=70));
        assert_eq!(rw::new(30..60).shr(10), rw::new(40..70));
        assert_eq!(rw::new(30..=60).shr(10), rw::new(40..=70));
        assert_eq!(rw::new((Ex(30), In(60))).shr(10), rw::new((Ex(40), In(70))));
    }
}

#[test]
fn bitand() {
    with_refs_overloads();
    with_bounds_overloads();

    fn with_refs_overloads() {
        let target = || rw::new(30..60);
        let other = || rw::new(40..70);
        let asis1 = target().bitand(other());
        let asis2 = target().bitand(&other());
        let asis3 = (&target()).bitand(other());
        let asis4 = (&target()).bitand(&other());
        let tobe = ru::new(target()).bitand(ru::new(other()));
        let tobe = tobe.try_into().ok().unwrap();
        assert_eq!(asis1.0, tobe);
        assert_eq!(asis2.0, tobe);
        assert_eq!(asis3.0, tobe);
        assert_eq!(asis4.0, tobe);
    }

    fn with_bounds_overloads() {
        assert_eq!(rw::new(..).bitand(rw::new(..)), rw::new::<_, usize>(..));
        assert_eq!(rw::new(30..).bitand(rw::new(40..)), rw::new(40..));
        assert_eq!(rw::new(..60).bitand(rw::new(..70)), rw::new(..60));
        assert_eq!(rw::new(..=60).bitand(rw::new(..=70)), rw::new(..=60));
        assert_eq!(rw::new(30..60).bitand(rw::new(40..70)), rw::new(40..60));
        assert_eq!(rw::new(30..=60).bitand(rw::new(40..=70)), rw::new(40..=60));
        assert_eq!(
            rw::new((Ex(30), Ex(60))).bitand(rw::new((Ub, Ub))),
            rw::new((Ex(30), Ex(60)))
        );
    }
}

#[test]
fn bitor() {
    with_refs_overloads();
    with_bounds_overloads();

    fn with_refs_overloads() {
        let target = || rw::new(30..60);
        let other = || rw::new(40..70);
        let asis1 = target().bitor(other());
        let asis2 = target().bitor(&other());
        let asis3 = (&target()).bitor(other());
        let asis4 = (&target()).bitor(&other());
        let tobe = ru::new(target()).bitor(ru::new(other()));
        let tobe = tobe.try_into().ok().unwrap();
        assert_eq!(asis1.0, tobe);
        assert_eq!(asis2.0, tobe);
        assert_eq!(asis3.0, tobe);
        assert_eq!(asis4.0, tobe);
    }

    fn with_bounds_overloads() {
        assert_eq!(rw::new(..).bitor(rw::new(..)), rw::new::<_, usize>(..));
        assert_eq!(rw::new(30..).bitor(rw::new(40..)), rw::new(30..));
        assert_eq!(rw::new(..60).bitor(rw::new(..70)), rw::new(..70));
        assert_eq!(rw::new(..=60).bitor(rw::new(..=70)), rw::new(..=70));
        assert_eq!(rw::new(30..60).bitor(rw::new(40..70)), rw::new(30..70));
        assert_eq!(rw::new(30..=60).bitor(rw::new(40..=70)), rw::new(30..=70));
        assert_eq!(
            rw::new((Ex(30), Ex(60))).bitor(rw::new((Ub, Ub))),
            rw::new((Ub, Ub))
        );
    }
}

#[test]
fn bitxor() {
    with_refs_overloads();
    with_bounds_overloads();

    fn with_refs_overloads() {
        let target = || rw::new(30..40);
        let other = || rw::new(50..60);
        let asis1 = target().bitxor(other());
        let asis2 = target().bitxor(&other());
        let asis3 = (&target()).bitxor(other());
        let asis4 = (&target()).bitxor(&other());
        let tobe = ru::new(target()).bitxor(ru::new(other()));
        let tobe = tobe.try_into().ok().unwrap();
        assert_eq!(asis1.0, tobe);
        assert_eq!(asis2.0, tobe);
        assert_eq!(asis3.0, tobe);
        assert_eq!(asis4.0, tobe);
    }

    fn with_bounds_overloads() {
        assert_eq!(rw::new(..).bitxor(rw::new(..)), rw::new::<_, usize>(..));
        assert_eq!(rw::new(30..).bitxor(rw::new(40..)), rw::new(30..));
        assert_eq!(rw::new(..60).bitxor(rw::new(..70)), rw::new(..70));
        assert_eq!(rw::new(..=60).bitxor(rw::new(..=70)), rw::new(..=70));
        assert_eq!(rw::new(30..60).bitxor(rw::new(40..70)), rw::new(30..70));
        assert_eq!(rw::new(30..=60).bitxor(rw::new(40..=70)), rw::new(30..=70));
        assert_eq!(
            rw::new((Ex(30), Ex(60))).bitxor(rw::new((Ub, Ub))),
            rw::new((Ub, Ub))
        );
    }
}

#[test]
fn shl_assign() {
    with_refs_overloads();
    with_bounds_overloads();

    fn with_refs_overloads() {
        let target = &sv::range_wrapper();
        let mut target1 = target.clone();
        let mut target2 = target.clone();
        let rhs = 10;
        target1.shl_assign(rhs);
        target2.shl_assign(&rhs);
        assert_eq!(target1, target.shl(&rhs));
        assert_eq!(target2, target.shl(&rhs));
    }

    fn with_bounds_overloads() {
        let target1 = &mut rw::new::<_, usize>(..);
        let target2 = &mut rw::new(30..);
        let target3 = &mut rw::new(..60);
        let target4 = &mut rw::new(..=60);
        let target5 = &mut rw::new(30..60);
        let target6 = &mut rw::new(30..=60);
        let target7 = &mut rw::new((Ex(30), Ub));
        let rhs = 10;
        target1.shl_assign(rhs);
        target2.shl_assign(rhs);
        target3.shl_assign(rhs);
        target4.shl_assign(rhs);
        target5.shl_assign(rhs);
        target6.shl_assign(rhs);
        target7.shl_assign(rhs);
        assert_eq!(target1, &rw::new(..));
        assert_eq!(target2, &rw::new(20..));
        assert_eq!(target3, &rw::new(..50));
        assert_eq!(target4, &rw::new(..=50));
        assert_eq!(target5, &rw::new(20..50));
        assert_eq!(target6, &rw::new(20..=50));
        assert_eq!(target7, &rw::new((Ex(20), Ub)));
    }
}

#[test]
fn shr_assign() {
    with_refs_overloads();
    with_bounds_overloads();

    fn with_refs_overloads() {
        let target = &sv::range_wrapper();
        let mut target1 = target.clone();
        let mut target2 = target.clone();
        let rhs = 10;
        target1.shr_assign(rhs);
        target2.shr_assign(&rhs);
        assert_eq!(target1, target.shr(&rhs));
        assert_eq!(target2, target.shr(&rhs));
    }

    fn with_bounds_overloads() {
        let target1 = &mut rw::new::<_, usize>(..);
        let target2 = &mut rw::new(30..);
        let target3 = &mut rw::new(..60);
        let target4 = &mut rw::new(..=60);
        let target5 = &mut rw::new(30..60);
        let target6 = &mut rw::new(30..=60);
        let target7 = &mut rw::new((Ex(30), Ub));
        let rhs = 10;
        target1.shr_assign(rhs);
        target2.shr_assign(rhs);
        target3.shr_assign(rhs);
        target4.shr_assign(rhs);
        target5.shr_assign(rhs);
        target6.shr_assign(rhs);
        target7.shr_assign(rhs);
        assert_eq!(target1, &rw::new(..));
        assert_eq!(target2, &rw::new(40..));
        assert_eq!(target3, &rw::new(..70));
        assert_eq!(target4, &rw::new(..=70));
        assert_eq!(target5, &rw::new(40..70));
        assert_eq!(target6, &rw::new(40..=70));
        assert_eq!(target7, &rw::new((Ex(40), Ub)));
    }
}

#[test]
fn bitand_assign() {
    with_refs_overloads();
    with_bounds_overloads();

    fn with_refs_overloads() {
        let target = &rw::new(30..60);
        let rhs = &rw::new(40..70);
        let mut target1 = target.clone();
        let mut target2 = target.clone();
        target1.bitand_assign(rhs.clone());
        target2.bitand_assign(&rhs.clone());
        assert_eq!(target1, target.bitand(rhs));
        assert_eq!(target2, target.bitand(rhs));
    }

    fn with_bounds_overloads() {
        let target1 = &mut rw::new::<_, usize>(..);
        let target2 = &mut rw::new(30..);
        let target3 = &mut rw::new(..60);
        let target4 = &mut rw::new(..=60);
        let target5 = &mut rw::new(30..60);
        let target6 = &mut rw::new(30..=60);
        let target7 = &mut rw::new((Ex(30), In(60)));
        target1.bitand_assign(rw::new(..));
        target2.bitand_assign(rw::new(40..));
        target3.bitand_assign(rw::new(..70));
        target4.bitand_assign(rw::new(..=70));
        target5.bitand_assign(rw::new(40..70));
        target6.bitand_assign(rw::new(40..=70));
        target7.bitand_assign(rw::new((In(40), Ub)));
        assert_eq!(target1, &rw::new(..));
        assert_eq!(target2, &rw::new(40..));
        assert_eq!(target3, &rw::new(..60));
        assert_eq!(target4, &rw::new(..=60));
        assert_eq!(target5, &rw::new(40..60));
        assert_eq!(target6, &rw::new(40..=60));
        assert_eq!(target7, &rw::new((In(40), In(60))));
    }
}

#[test]
fn bitor_assign() {
    with_refs_overloads();
    with_bounds_overloads();

    fn with_refs_overloads() {
        let target = &rw::new(30..60);
        let rhs = &rw::new(40..70);
        let mut target1 = target.clone();
        let mut target2 = target.clone();
        target1.bitor_assign(rhs.clone());
        target2.bitor_assign(&rhs.clone());
        assert_eq!(target1, target.bitor(rhs));
        assert_eq!(target2, target.bitor(rhs));
    }

    fn with_bounds_overloads() {
        let target1 = &mut rw::new::<_, usize>(..);
        let target2 = &mut rw::new(30..);
        let target3 = &mut rw::new(..60);
        let target4 = &mut rw::new(..=60);
        let target5 = &mut rw::new(30..60);
        let target6 = &mut rw::new(30..=60);
        let target7 = &mut rw::new((Ex(30), In(60)));
        target1.bitor_assign(rw::new(..));
        target2.bitor_assign(rw::new(40..));
        target3.bitor_assign(rw::new(..70));
        target4.bitor_assign(rw::new(..=70));
        target5.bitor_assign(rw::new(40..70));
        target6.bitor_assign(rw::new(40..=70));
        target7.bitor_assign(rw::new((In(40), Ub)));
        assert_eq!(target1, &rw::new(..));
        assert_eq!(target2, &rw::new(30..));
        assert_eq!(target3, &rw::new(..70));
        assert_eq!(target4, &rw::new(..=70));
        assert_eq!(target5, &rw::new(30..70));
        assert_eq!(target6, &rw::new(30..=70));
        assert_eq!(target7, &rw::new((Ex(30), Ub)));
    }
}

#[test]
fn bitxor_assign() {
    with_refs_overloads();
    with_bounds_overloads();

    fn with_refs_overloads() {
        let target = &rw::new(30..40);
        let rhs = &rw::new(50..60);
        let mut target1 = target.clone();
        let mut target2 = target.clone();
        target1.bitxor_assign(rhs.clone());
        target2.bitxor_assign(&rhs.clone());
        assert_eq!(target1, target.bitxor(rhs));
        assert_eq!(target2, target.bitxor(rhs));
    }

    fn with_bounds_overloads() {
        let target1 = &mut rw::new::<_, usize>(..);
        let target2 = &mut rw::new(30..);
        let target3 = &mut rw::new(..60);
        let target4 = &mut rw::new(..=60);
        let target5 = &mut rw::new(30..60);
        let target6 = &mut rw::new(30..=60);
        let target7 = &mut rw::new((Ex(30), In(60)));
        target1.bitxor_assign(rw::new(..));
        target2.bitxor_assign(rw::new(40..));
        target3.bitxor_assign(rw::new(..70));
        target4.bitxor_assign(rw::new(..=70));
        target5.bitxor_assign(rw::new(40..70));
        target6.bitxor_assign(rw::new(40..=70));
        target7.bitxor_assign(rw::new((In(40), Ub)));
        assert_eq!(target1, &rw::new(..));
        assert_eq!(target2, &rw::new(30..));
        assert_eq!(target3, &rw::new(..70));
        assert_eq!(target4, &rw::new(..=70));
        assert_eq!(target5, &rw::new(30..70));
        assert_eq!(target6, &rw::new(30..=70));
        assert_eq!(target7, &rw::new((Ex(30), Ub)));
    }
}
