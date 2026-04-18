use crate::for_test::aliases::*;
use rich_range::conv::*;
use rich_range::prelude::*;

#[test]
fn new_broken() {
    with_ok();
    with_ng();

    fn with_ok() {
        let r1 = <RangeUniv<_> as RangeSrc<_>>::new_broken();
        let r2 = <RvNormal<'_, usize>>::new_broken();
        let r3 = <RwNormal<_> as RangeSrc<_>>::new_broken();
        let r4 = <RwInclusive<_> as RangeSrc<_>>::new_broken();
        let r5 = <RwBounds<_> as RangeSrc<_>>::new_broken();
        assert_eq!(r1, Ok(ru::new((Ex(usize::MAX), Ex(usize::MIN)))));
        assert_eq!(r2, Ok(ru::new((Ex(usize::MAX), Ex(usize::MIN)))));
        assert_eq!(r3, Ok(rw::new(usize::MAX..usize::MIN)));
        assert_eq!(r4, Ok(rw::new(usize::MAX..=usize::MIN)));
        assert_eq!(r5, Ok(rw::new((Ex(usize::MAX), Ex(usize::MIN)))));
    }

    fn with_ng() {
        let r1 = <RwFrom<usize> as RangeSrc<_>>::new_broken();
        let r2 = <RwTo<usize> as RangeSrc<_>>::new_broken();
        let r3 = <RwToInclusive<usize> as RangeSrc<_>>::new_broken();
        let r4 = <RwFull<usize> as RangeSrc<_>>::new_broken();
        assert!(r1.is_err());
        assert!(r2.is_err());
        assert!(r3.is_err());
        assert!(r4.is_err());
    }
}

#[test]
fn from_bounds() {
    with_ok();
    with_ng();

    fn with_ok() {
        let r1 = <RangeUniv<usize> as RangeSrc<_>>::new((In(30), Ex(60)));
        let r2 = <RvNormal<'_, usize> as RangeSrc<_>>::new((In(30), Ex(60)));
        let r3 = <RwNormal<usize> as RangeSrc<_>>::new((In(30), Ex(60)));
        let r4 = <RwFrom<usize> as RangeSrc<_>>::new((In(30), Ub));
        let r5 = <RwTo<usize> as RangeSrc<_>>::new((Ub, Ex(60)));
        let r6 = <RwInclusive<usize> as RangeSrc<_>>::new((In(30), In(60)));
        let r7 = <RwToInclusive<usize> as RangeSrc<_>>::new((Ub, In(60)));
        let r8 = <RwFull<usize> as RangeSrc<_>>::new::<usize>((Ub, Ub));
        let r9 = <RwBounds<usize> as RangeSrc<_>>::new((Ex(30), Ex(60)));
        assert_eq!(r1, Ok(ru::new(30..60)));
        assert_eq!(r2, Ok(ru::new(30..60)));
        assert_eq!(r3, Ok(rw::new(30..60)));
        assert_eq!(r4, Ok(rw::new(30..)));
        assert_eq!(r5, Ok(rw::new(..60)));
        assert_eq!(r6, Ok(rw::new(30..=60)));
        assert_eq!(r7, Ok(rw::new(..=60)));
        assert_eq!(r8, Ok(rw::new(..)));
        assert_eq!(r9, Ok(rw::new((Ex(30), Ex(60)))));
    }

    fn with_ng() {
        let r1 = <RwNormal<usize> as RangeSrc<_>>::new((Ex(30), Ub));
        let r2 = <RwFrom<usize> as RangeSrc<_>>::new((Ex(30), Ub));
        let r3 = <RwTo<usize> as RangeSrc<_>>::new((Ex(30), Ub));
        let r4 = <RwInclusive<usize> as RangeSrc<_>>::new((Ex(30), Ub));
        let r5 = <RwToInclusive<usize> as RangeSrc<_>>::new((Ex(30), Ub));
        let r6 = <RwFull<usize> as RangeSrc<_>>::new((Ex(30), Ub));
        assert!(r1.is_err());
        assert!(r2.is_err());
        assert!(r3.is_err());
        assert!(r4.is_err());
        assert!(r5.is_err());
        assert!(r6.is_err());
    }
}
