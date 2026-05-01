use crate::for_test::aliases::*;
use rich_range::parts::*;
use rich_range::prelude::*;
use rich_range::*;
use std::cmp::Ordering;
use std::ops::RangeBounds;

#[test]
fn new() {
    let r = RangeView::new(&(30..60));
    assert_eq!(r.0, &(30..60));
}

#[test]
fn to_univ() {
    let r = rv::new(&(30..60)).to_univ();
    assert_eq!(r, ru::new(30..60));
}

#[test]
fn contains() {
    let target = rv::new(&(30..60));
    assert!(target.contains(&40));
    assert!(!target.contains(&70));
}

#[test]
fn is_empty() {
    assert!(rv::new(&(30..30)).is_empty());
    assert!(rv::new(&(60..30)).is_empty());
    assert!(!rv::new(&(30..60)).is_empty());
    assert!(!rv::new(&(30..=30)).is_empty());
}

#[test]
fn is_broken() {
    assert!(rv::new(&(60..30)).is_broken());
    assert!(!rv::new(&(30..60)).is_broken());
    assert!(!rv::new(&(30..30)).is_broken());
    assert!(!rv::new(&(30..=30)).is_broken());
}

#[test]
fn is_cursor() {
    assert!(rv::new(&(30..30)).is_cursor());
    assert!(!rv::new(&(30..60)).is_cursor());
    assert!(!rv::new(&(60..30)).is_cursor());
    assert!(!rv::new(&(30..=30)).is_cursor());
}

#[test]
fn is_cursor_fwd() {
    let target1 = rv::new(&(In(30), Ex(30)));
    let target2 = rv::new(&(Ex(30), In(30)));
    assert!(target1.is_cursor_fwd());
    assert!(!target2.is_cursor_fwd());
}

#[test]
fn is_cursor_bwd() {
    let target1 = rv::new(&(Ex(30), In(30)));
    let target2 = rv::new(&(In(30), Ex(30)));
    assert!(target1.is_cursor_bwd());
    assert!(!target2.is_cursor_bwd());
}

#[test]
fn is_point() {
    assert!(rv::new(&(30..=30)).is_point());
    assert!(!rv::new(&(30..30)).is_point());
    assert!(!rv::new(&(30..60)).is_point());
    assert!(!rv::new(&(60..30)).is_point());
}

#[test]
fn is_wide() {
    assert!(rv::new::<_, usize>(&(..)).is_wide());
    assert!(rv::new(&(30..)).is_wide());
    assert!(!rv::new(&(30..60)).is_wide());
}

#[test]
fn is_full() {
    assert!(rv::new::<_, usize>(&(..)).is_full());
    assert!(!rv::new(&(30..)).is_full());
    assert!(!rv::new(&(30..60)).is_full());
}

#[test]
fn start_edge() {
    let target = rv::new(&(30..60));
    let result = target.start_edge();
    assert_eq!(result, Edge::new(Side::S, In(&30)));
}

#[test]
fn end_edge() {
    let target = rv::new(&(30..60));
    let result = target.end_edge();
    assert_eq!(result, Edge::new(Side::E, Ex(&60)));
}

#[test]
fn head() {
    assert_eq!(rv::new(&(30..)).head(), 30);
    assert_eq!(rv::new(&(Ex(30), Ub)).head(), 31);
    assert_eq!(rv::new(&(..60)).head(), i32::MIN);
}

#[test]
fn tail() {
    assert_eq!(rv::new(&(30..60)).tail(), 59);
    assert_eq!(rv::new(&(30..=60)).tail(), 60);
    assert_eq!(rv::new(&(30..)).tail(), i32::MAX);
}

#[test]
fn prev() {
    assert_eq!(rv::new(&(30..)).prev(), 29);
    assert_eq!(rv::new(&(Ex(30), Ub)).prev(), 30);
    assert_eq!(rv::new(&(..60)).prev(), i32::MIN);
}

#[test]
fn next() {
    assert_eq!(rv::new(&(30..60)).next(), 60);
    assert_eq!(rv::new(&(30..=60)).next(), 61);
    assert_eq!(rv::new(&(30..)).next(), i32::MAX);
}

#[test]
fn cursor() {
    assert_eq!(rv::new(&(30..60)).cursor(), None);
    assert_eq!(rv::new(&(30..=30)).cursor(), None);
    assert_eq!(rv::new(&(30..30)).cursor(), Some(&30));
}

#[test]
fn point() {
    assert_eq!(rv::new(&(30..60)).point(), None);
    assert_eq!(rv::new(&(30..30)).point(), None);
    assert_eq!(rv::new(&(30..=30)).point(), Some(&30));
}

#[test]
fn len() {
    assert_eq!(rv::new(&(30..)).len(), None);
    assert_eq!(rv::new(&(30..60)).len(), Some(30));
    assert_eq!(rv::new(&(60..30)).len(), Some(0));
    assert_eq!(rv::new(&(30..=60)).len(), Some(31));
}

#[test]
fn size() {
    assert_eq!(rv::new(&(30..)).size(), None);
    assert_eq!(rv::new(&(30..60)).size(), Some(30));
    assert_eq!(rv::new(&(60..30)).size(), Some(0));
    assert_eq!(rv::new(&(30..=60)).size(), Some(31));
}

#[test]
fn width() {
    assert_eq!(rv::new(&(30.0..)).width(), None);
    assert_eq!(rv::new(&(30.0..60.0)).width(), Some(30.0));
    assert_eq!(rv::new(&(60.0..30.0)).width(), Some(0.0));
    assert_eq!(rv::new(&(30.0..=60.0)).width(), Some(30.0));
}

#[test]
fn bounds() {
    let target = rv::new(&(30..60));
    let result = target.bounds();
    assert_eq!(result.0, In(&30));
    assert_eq!(result.1, Ex(&60));
}

#[test]
fn edges() {
    let target = rv::new(&(30..60));
    let result = target.edges();
    assert_eq!(result.0, Edge::new(Side::S, In(&30)));
    assert_eq!(result.1, Edge::new(Side::E, Ex(&60)));
}

#[test]
fn as_ref() {
    let target = rv::new(&(30..60));
    let result = target.as_ref();
    assert_eq!(result.start_bound(), In(&30));
    assert_eq!(result.end_bound(), Ex(&60));
}

#[test]
fn cast() {
    let src = rv::new::<_, u16>(&(30..60));
    assert_eq!(src.cast::<f32>(), ru::new(30.0..60.0));
}

#[test]
fn try_cast() {
    let src1 = rv::new::<_, i16>(&(30..60));
    let src2 = rv::new::<_, i16>(&(-30..60));
    assert_eq!(src1.try_cast::<u16>(), Some(ru::new(30..60)));
    assert_eq!(src2.try_cast::<u16>(), None);
}

#[test]
fn to_range() {
    let target = rv::new(&(30..=60));
    let result = target.to_range();
    assert_eq!(result, 30..61);
}

#[test]
fn into_option() {
    let target1 = rv::new(&(30..60));
    let target2 = rv::new(&(60..30));
    let result1 = target1.into_option();
    let result2 = target2.into_option();
    assert_eq!(result1, Some(target1));
    assert_eq!(result2, None);
}

#[test]
fn iter() {
    let target = rv::new(&(3..6));
    let result = target.iter();
    assert!(result.eq([3, 4, 5].into_iter()));
}

#[test]
fn flip() {
    let r1 = rv::new::<_, usize>(&(..)).flip();
    let r2 = rv::new(&(30..)).flip();
    let r3 = rv::new(&(..60)).flip();
    let r4 = rv::new(&(30..60)).flip();
    let r5 = rv::new(&(30..30)).flip();
    assert_eq!(r1, (None, None));
    assert_eq!(r2, (Some(ru::new(..30)), None));
    assert_eq!(r3, (Some(ru::new(60..)), None));
    assert_eq!(r4, (Some(ru::new(..30)), Some(ru::new(60..))));
    assert_eq!(r5, (Some(ru::new(..)), None));
}

#[test]
fn flip_adv() {
    let r1 = rv::new::<_, usize>(&(..)).flip_adv(CursorMode::Off);
    let r2 = rv::new(&(30..)).flip_adv(CursorMode::Off);
    let r3 = rv::new(&(..60)).flip_adv(CursorMode::Off);
    let r4 = rv::new(&(30..60)).flip_adv(CursorMode::Off);
    let r5 = rv::new(&(30..30)).flip_adv(CursorMode::Off);
    let r6 = rv::new(&(30..30)).flip_adv(CursorMode::On);
    assert_eq!(r1, (None, None));
    assert_eq!(r2, (Some(ru::new(..30)), None));
    assert_eq!(r3, (Some(ru::new(60..)), None));
    assert_eq!(r4, (Some(ru::new(..30)), Some(ru::new(60..))));
    assert_eq!(r5, (Some(ru::new(..)), None));
    assert_eq!(r6, (Some(ru::new(..30)), Some(ru::new(30..))));
}

#[test]
fn shift() {
    let target = rv::new(&(30..60));
    let result = target.shift(10, false);
    assert_eq!(result, ru::new(20..50));
}

#[test]
fn shl() {
    let target = rv::new(&(30..60));
    let result = target.shl(10);
    assert_eq!(result, ru::new(20..50));
}

#[test]
fn shr() {
    let target = rv::new(&(30..60));
    let result = target.shr(10);
    assert_eq!(result, ru::new(40..70));
}

#[test]
fn add_start() {
    let target = rv::new(&(30..60));
    let result = target.add_start(10);
    assert_eq!(result, ru::new(40..60));
}

#[test]
fn add_end() {
    let target = rv::new(&(30..60));
    let result = target.add_end(10);
    assert_eq!(result, ru::new(30..70));
}

#[test]
fn sub_start() {
    let target = rv::new(&(30..60));
    let result = target.sub_start(10);
    assert_eq!(result, ru::new(20..60));
}

#[test]
fn sub_end() {
    let target = rv::new(&(30..60));
    let result = target.sub_end(10);
    assert_eq!(result, ru::new(30..50));
}

#[test]
fn calc_start() {
    let target = rv::new(&(30..60));
    let result = target.calc_start(40);
    assert_eq!(result, ru::new(20..60));
}

#[test]
fn calc_end() {
    let target = rv::new(&(30..60));
    let result = target.calc_end(40);
    assert_eq!(result, ru::new(30..70));
}

#[test]
fn align_start() {
    let target = rv::new(&(30..60));
    let result = target.align_start(40);
    assert_eq!(result, ru::new(40..70));
}

#[test]
fn align_end() {
    let target = rv::new(&(30..60));
    let result = target.align_end(70);
    assert_eq!(result, ru::new(40..70));
}

#[test]
fn map() {
    let target = rv::new(&(30..));
    let result = target.map(|x| x * 2);
    assert_eq!(result, ru::new(60..));
}

#[test]
fn try_map() {
    let target = rv::new::<_, u8>(&(30..));
    let result1 = target.clone().try_map(|x| x.checked_mul(2));
    let result2 = target.clone().try_map(|x| x.checked_mul(10));
    assert_eq!(result1, Some(ru::new(60..)));
    assert_eq!(result2, None);
}

#[test]
fn checked_shift() {
    let target = rv::new::<_, u8>(&(30..60));
    let result1 = target.checked_shift(10, false);
    let result2 = target.checked_shift(40, false);
    assert_eq!(result1, Some(ru::new(20..50)));
    assert_eq!(result2, None);
}

#[test]
fn checked_shl() {
    let target = rv::new::<_, u8>(&(30..60));
    let result1 = target.checked_shl(10);
    let result2 = target.checked_shl(40);
    assert_eq!(result1, Some(ru::new(20..50)));
    assert_eq!(result2, None);
}

#[test]
fn checked_shr() {
    let target = rv::new::<_, u8>(&(30..60));
    let result1 = target.checked_shr(10);
    let result2 = target.checked_shr(200);
    assert_eq!(result1, Some(ru::new(40..70)));
    assert_eq!(result2, None);
}

#[test]
fn checked_add_start() {
    let target = rv::new::<_, u8>(&(30..60));
    let result1 = target.checked_add_start(10);
    let result2 = target.checked_add_start(250);
    assert_eq!(result1, Some(ru::new(40..60)));
    assert_eq!(result2, None);
}

#[test]
fn checked_add_end() {
    let target = rv::new::<_, u8>(&(30..60));
    let result1 = target.checked_add_end(10);
    let result2 = target.checked_add_end(250);
    assert_eq!(result1, Some(ru::new(30..70)));
    assert_eq!(result2, None);
}

#[test]
fn checked_sub_start() {
    let target = rv::new::<_, u8>(&(30..60));
    let result1 = target.checked_sub_start(10);
    let result2 = target.checked_sub_start(40);
    assert_eq!(result1, Some(ru::new(20..60)));
    assert_eq!(result2, None);
}

#[test]
fn checked_sub_end() {
    let target = rv::new::<_, u8>(&(30..60));
    let result1 = target.checked_sub_end(10);
    let result2 = target.checked_sub_end(70);
    assert_eq!(result1, Some(ru::new(30..50)));
    assert_eq!(result2, None);
}

#[test]
fn checked_calc_start() {
    let target = rv::new::<_, u8>(&(30..60));
    let r1 = target.checked_calc_start(40);
    let r2 = target.checked_calc_start(70);
    assert_eq!(r1, Some(ru::new(20..60)));
    assert_eq!(r2, None);
}

#[test]
fn checked_calc_end() {
    let target = rv::new::<_, u8>(&(30..60));
    let r1 = target.checked_calc_end(40);
    let r2 = target.checked_calc_end(230);
    assert_eq!(r1, Some(ru::new(30..70)));
    assert_eq!(r2, None);
}

#[test]
fn checked_align_start() {
    let target = rv::new::<_, u8>(&(30..60));
    let result1 = target.checked_align_start(40);
    let result2 = target.checked_align_start(230);
    assert_eq!(result1, Some(ru::new(40..70)));
    assert_eq!(result2, None);
}

#[test]
fn checked_align_end() {
    let target = rv::new::<_, u8>(&(30..60));
    let result1 = target.checked_align_end(70);
    let result2 = target.checked_align_end(20);
    assert_eq!(result1, Some(ru::new(40..70)));
    assert_eq!(result2, None);
}

#[test]
fn equiv() {
    assert!(rv::new(&(30..60)).equiv(&(30..60)));
    assert!(!rv::new(&(30..60)).equiv(&(30..65)));
    assert!(rv::new(&(30..30)).equiv(&(0..0)));
}

#[test]
fn intersects() {
    assert!(rv::new(&(30..60)).intersects(&(50..70)));
    assert!(rv::new(&(30..60)).intersects(&(50..50)));
    assert!(!rv::new(&(30..60)).intersects(&(70..80)));
}

#[test]
fn includes() {
    assert!(rv::new(&(30..60)).includes(&(40..50)));
    assert!(!rv::new(&(30..60)).includes(&(70..80)));
    assert!(!rv::new(&(30..60)).includes(&(60..60)));
    assert!(rv::new(&(30..30)).includes(&(30..30)));
    assert!(!rv::new(&(30..30)).includes(&(40..40)));
}

#[test]
fn included() {
    assert!(rv::new(&(30..60)).included(&(20..70)));
    assert!(!rv::new(&(30..60)).included(&(40..70)));
    assert!(rv::new(&(30..30)).included(&(30..30)));
    assert!(!rv::new(&(30..30)).included(&(40..40)));
}

#[test]
fn adjoins() {
    assert!(rv::new(&(30..=60)).adjoins(&(60..70)));
    assert!(!rv::new(&(30..=60)).adjoins(&(70..80)));
    assert!(rv::new(&(30..60)).adjoins(&(20..=30)));
    assert!(!rv::new(&(30..60)).adjoins(&(10..=20)));
}

#[test]
fn adjoins_prev() {
    assert!(rv::new(&(30..60)).adjoins_prev(&(20..=30)));
    assert!(!rv::new(&(30..60)).adjoins_prev(&(10..=20)));
}

#[test]
fn adjoins_next() {
    assert!(rv::new(&(30..=60)).adjoins_next(&(60..70)));
    assert!(!rv::new(&(30..=60)).adjoins_next(&(70..80)));
}

#[test]
fn touches() {
    assert!(rv::new(&(30..60)).touches(&(60..70)));
    assert!(!rv::new(&(30..60)).touches(&(70..80)));
    assert!(rv::new(&(30..60)).touches(&(20..30)));
    assert!(!rv::new(&(30..60)).touches(&(10..20)));
}

#[test]
fn touches_prev() {
    assert!(rv::new(&(30..60)).touches_prev(&(20..30)));
    assert!(!rv::new(&(30..60)).touches_prev(&(10..20)));
}

#[test]
fn touches_next() {
    assert!(rv::new(&(30..60)).touches_next(&(60..70)));
    assert!(!rv::new(&(30..60)).touches_next(&(70..80)));
}

#[test]
fn rel() {
    let target = rv::new(&(30..60));
    let result = target.rel(&rv::new(&(20..70)), PosStyle::Step);
    assert_eq!(result, RangeRel::During(true));
}

#[test]
fn cut() {
    let r = rv::new(&(30..60));
    let (fst, snd) = r.cut(&40);
    assert_eq!(fst, Some(ru::new(30..40)));
    assert_eq!(snd, Some(ru::new(40..60)));
}

#[test]
fn cut_adv() {
    let r = rv::new(&(30..60));
    let (fst, snd) = r.cut_adv(&40, CutMode::FallbackFw);
    assert_eq!(fst, Some(ru::new(30..40)));
    assert_eq!(snd, Some(ru::new(40..60)));
}

#[test]
fn interval() {
    let target = rv::new(&(30..60));
    let r1 = target.interval(&rv::new(&(50..70)));
    let r2 = target.interval(&rv::new(&(60..80)));
    let r3 = target.interval(&rv::new(&(70..90)));
    assert_eq!(r1, None);
    assert_eq!(r2, None);
    assert_eq!(r3, Some(ru::new(60..70)));
}

#[test]
fn interval_adv() {
    let target = rv::new(&(30..60));
    let r1 = target.interval_adv(&rv::new(&(50..70)), CursorMode::Off);
    let r2 = target.interval_adv(&rv::new(&(60..80)), CursorMode::Off);
    let r3 = target.interval_adv(&rv::new(&(60..80)), CursorMode::On);
    let r4 = target.interval_adv(&rv::new(&(70..90)), CursorMode::Off);
    assert_eq!(r1, None);
    assert_eq!(r2, None);
    assert_eq!(r3, Some(ru::new(60..60)));
    assert_eq!(r4, Some(ru::new(60..70)));
}

#[test]
fn prod() {
    let r = rv::new(&(30..60)).prod(&rv::new(&(40..70)));
    assert_eq!(r, Some(ru::new(40..60)));
}

#[test]
fn enwrap() {
    let r1 = rv::new(&(20..40)).enwrap(&rv::new(&(30..50)));
    let r2 = rv::new(&(10..20)).enwrap(&rv::new(&(40..60)));
    assert_eq!(r1, Some(ru::new(20..50)));
    assert_eq!(r2, Some(ru::new(10..60)));
}

#[test]
fn union() {
    let r1 = rv::new(&(30..60)).union(&rv::new(&(40..70)));
    let r2 = rv::new(&(30..60)).union(&rv::new(&(70..80)));
    assert_eq!(r1, (ru::new(30..70), None));
    assert_eq!(r2, (ru::new(30..60), Some(ru::new(70..80))));
}

#[test]
fn diff() {
    let r1 = rv::new(&(30..60)).diff(&(50..70));
    let r2 = rv::new(&(30..60)).diff(&(40..50));
    let r3 = rv::new(&(30..60)).diff(&(40..40));
    assert_eq!(r1, (Some(ru::new(30..50)), None));
    assert_eq!(r2, (Some(ru::new(30..40)), Some(ru::new(50..60))));
    assert_eq!(r3, (Some(ru::new(30..60)), None));
}

#[test]
fn diff_adv() {
    let r1 = rv::new(&(30..60)).diff_adv(&(50..70), CursorMode::Off);
    let r2 = rv::new(&(30..60)).diff_adv(&(40..50), CursorMode::Off);
    let r3 = rv::new(&(30..60)).diff_adv(&(40..40), CursorMode::Off);
    let r4 = rv::new(&(30..60)).diff_adv(&(40..40), CursorMode::On);
    assert_eq!(r1, (Some(ru::new(30..50)), None));
    assert_eq!(r2, (Some(ru::new(30..40)), Some(ru::new(50..60))));
    assert_eq!(r3, (Some(ru::new(30..60)), None));
    assert_eq!(r4, (Some(ru::new(30..40)), Some(ru::new(40..60))));
}

#[test]
fn into_iter() {
    let target = rv::new(&(3..6));
    let master = 3..6;
    let asis = target.into_iter();
    let tobe = master.into_iter();
    assert!(asis.eq(tobe));
}

#[test]
fn partial_cmp() {
    let datas = [
        (rv::new(&(30..60)), rv::new(&(40..50)), None),
        (
            rv::new(&(30..60)),
            rv::new(&(10..20)),
            Some(Ordering::Greater),
        ),
        (rv::new(&(30..60)), rv::new(&(70..80)), Some(Ordering::Less)),
    ];

    for (target, other, tobe) in datas {
        let asis = target.partial_cmp(&other);
        assert_eq!(asis, tobe);
    }
}
