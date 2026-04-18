use crate::for_test::aliases::*;
use rich_range::prelude::*;

#[test]
fn from() {
    let r = rw::new(30..60);
    assert_eq!(r.0.start, 30);
    assert_eq!(r.0.end, 60);
}

#[test]
fn refr() {
    let r = rw::refr(&(30..60));
    assert_eq!(&r.0.start, &30);
    assert_eq!(&r.0.end, &60);
}

#[test]
fn univ() {
    let r = ru::new(30..60);
    assert_eq!(r.start, In(30));
    assert_eq!(r.end, Ex(60));
}
