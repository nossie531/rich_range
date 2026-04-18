use crate::for_test::aliases::*;
use rich_range::conv::*;
use rich_range::prelude::*;

#[test]
fn into_bounds() {
    let r1 = RangeParts::parts(ru::new(30..60));
    let r2 = RangeParts::parts(rw::new(30..60));
    let r3 = RangeParts::parts(rw::new(30..));
    let r4 = RangeParts::parts(rw::new(..60));
    let r5 = RangeParts::parts(rw::new(30..=60));
    let r6 = RangeParts::parts(rw::new(..=60));
    let r7 = RangeParts::parts(rw::new::<_, usize>(..));
    let r8 = RangeParts::parts(rw::new((Ex(30), Ex(60))));
    assert_eq!(r1, (In(30), Ex(60)));
    assert_eq!(r2, (In(30), Ex(60)));
    assert_eq!(r3, (In(30), Ub));
    assert_eq!(r4, (Ub, Ex(60)));
    assert_eq!(r5, (In(30), In(60)));
    assert_eq!(r6, (Ub, In(60)));
    assert_eq!(r7, (Ub, Ub));
    assert_eq!(r8, (Ex(30), Ex(60)));
}
