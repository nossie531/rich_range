use crate::for_test::aliases::*;
use rich_range::parts::*;

#[test]
fn new() {
    let side = Side::S;
    let bound = In(42);
    let result = Edge::new(Side::S, In(42));
    assert_eq!(result.side(), side);
    assert_eq!(result.bound(), bound);
}

#[test]
fn is_unbounded() {
    let t1 = Edge::<i32>::new(Side::S, Ub);
    let t2 = Edge::new(Side::S, In(42));
    assert!(t1.is_unbounded());
    assert!(!t2.is_unbounded());
}

#[test]
fn is_included() {
    let t1 = Edge::new(Side::S, In(42));
    let t2 = Edge::new(Side::S, Ex(42));
    assert!(t1.is_included());
    assert!(!t2.is_included());
}

#[test]
fn is_excluded() {
    let t1 = Edge::new(Side::S, Ex(42));
    let t2 = Edge::new(Side::S, In(42));
    assert!(t1.is_excluded());
    assert!(!t2.is_excluded());
}

#[test]
fn side() {
    let edge = Edge::new(Side::S, In(42));
    assert_eq!(edge.side(), Side::S);
}

#[test]
fn bound() {
    let edge = Edge::new(Side::S, In(42));
    assert_eq!(edge.bound(), In(42));
}

#[test]
fn pos() {
    let t1 = Edge::new(Side::S, In(42));
    let t2 = Edge::new(Side::S, Ex(42));
    let t3 = Edge::<i32>::new(Side::S, Ub);
    assert_eq!(t1.pos(), Some(42));
    assert_eq!(t2.pos(), Some(42));
    assert_eq!(t3.pos(), None);
}

#[test]
fn as_ref() {
    let edge = Edge::new(Side::S, In(42));
    let result = edge.as_ref();
    assert_eq!(result.bound(), In(&42));
}

#[test]
fn with_bound() {
    let edge = Edge::new(Side::S, In(42));
    let result = edge.with_bound(Ex(43));
    assert_eq!(result, Edge::new(Side::S, Ex(43)));
}

#[test]
fn with_included() {
    let t1 = Edge::new(Side::S, In(42));
    let t2 = Edge::<i32>::new(Side::S, Ub);
    assert_eq!(t1.with_included(false), Edge::new(Side::S, Ex(42)));
    assert_eq!(t2.with_included(false), Edge::new(Side::S, Ub));
}

#[test]
fn with_pos() {
    let t1 = Edge::new(Side::S, In(42));
    let t2 = Edge::new(Side::S, Ex(42));
    let t3 = Edge::new(Side::S, Ub);
    assert_eq!(t1.with_pos(43), Edge::new(Side::S, In(43)));
    assert_eq!(t2.with_pos(43), Edge::new(Side::S, Ex(43)));
    assert_eq!(t3.with_pos(43), Edge::new(Side::S, Ub));
}

#[test]
fn map() {
    let edge = Edge::new(Side::S, In(42));
    let result = edge.map(|x| x + 3);
    assert_eq!(result, Edge::new(Side::S, In(45)));
}

#[test]
fn try_map() {
    let edge = Edge::new(Side::S, In(42_u8));
    let r1 = edge.try_map(|x| x.checked_add(200));
    let r2 = edge.try_map(|x| x.checked_add(220));
    assert_eq!(r1, Some(Edge::new(Side::S, In(242))));
    assert_eq!(r2, None);
}

#[test]
fn cloned() {
    let edge = Edge::new(Side::S, In(&42));
    let result = edge.cloned();
    assert_eq!(result, Edge::new(Side::S, In(42)));
}

#[test]
fn xxx_cmp() {
    when_normal();
    when_unordered();

    fn when_normal() {
        let values = [
            (0, Edge::new(Side::S, Ub)),
            (1, Edge::new(Side::E, Ex(30))),
            (2, Edge::new(Side::S, In(30))),
            (2, Edge::new(Side::E, In(30))),
            (3, Edge::new(Side::S, In(40))),
            (3, Edge::new(Side::E, In(40))),
            (4, Edge::new(Side::S, Ex(40))),
            (5, Edge::new(Side::E, Ub)),
        ];

        for pair in values.windows(2) {
            let (k1, v1) = pair[0];
            let (k2, v2) = pair[1];
            let asis_total = v1.cmp(&v2);
            let asis_partial = v1.partial_cmp(&v2);
            let tobe = k1.cmp(&k2);
            assert_eq!(asis_total, tobe);
            assert_eq!(asis_partial, Some(tobe));
        }
    }

    fn when_unordered() {
        let v1 = Edge::new(Side::S, In(30.0));
        let v2 = Edge::new(Side::S, In(f32::NAN));
        let asis1 = v1.partial_cmp(&v2);
        let asis2 = v2.partial_cmp(&v1);
        assert_eq!(asis1, None);
        assert_eq!(asis2, None);
    }
}
