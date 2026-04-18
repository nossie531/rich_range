use crate::for_test::aliases::*;
use crate::for_test::consts::*;
use crate::for_test::macros::*;
use rich_range::prelude::*;
use test_panic::prelude::*;

#[test]
fn next() {
    with_normal();
    with_range_to();

    fn with_normal() {
        let target = &mut r!(=10, ?13).iter();
        assert_eq!(target.next(), Some(10));
        assert_eq!(target.next(), Some(11));
        assert_eq!(target.next(), Some(12));
        assert_eq!(target.next(), None);
    }

    fn with_range_to() {
        let target = &mut r!(---, ?30).iter();
        let result = test_panic(|| target.next());
        assert!(result.is_panic());
    }
}

#[test]
fn nth() {
    with_normal();
    with_range_to();
    with_overflow();
    with_overflow2();

    fn with_normal() {
        let datas = [
            (r!(=10, ?10), 0, vec![]),
            (r!(=13, ?10), 0, vec![]),
            (r!(=10, ?13), 0, vec![10, 11, 12]),
            (r!(?10, =13), 0, vec![11, 12, 13]),
            (r!(=10, ?25), 3, vec![13, 17, 21]),
        ];

        for (range, n, tobes) in datas {
            let target = &mut range.iter();
            for tobe in tobes {
                let asis = target.nth(n);
                assert_eq!(asis, Some(tobe));
            }

            assert_eq!(target.nth(n), None);
            assert_eq!(target.nth(n), None);
        }
    }

    fn with_range_to() {
        let target = &mut r!(---, ?30).iter();
        let result = test_panic(|| target.nth(3));
        assert!(result.is_panic());
    }

    fn with_overflow() {
        let target = &mut ru::new(0_u8..).iter();
        target.nth(128);

        let result = test_panic(|| target.nth(128));
        assert!(result.is_panic());
    }

    fn with_overflow2() {
        let target = &mut ru::new((Ex(0_u8), Ub)).iter();
        let result = test_panic(|| target.nth(255));
        assert!(result.is_panic());
    }
}

#[test]
fn size_hint() {
    let datas = [
        (r!(=10, ?030).iter(), (20, Some(20))),
        (r!(=10, ----).iter(), (MAX, None)),
        (r!(---, ?030).iter(), (MAX, None)),
        (r!(=00, =MAX).iter(), (MAX, None)),
    ];

    for (target, tobe) in datas {
        let asis = target.size_hint();
        assert_eq!(asis, tobe);
    }
}

#[test]
fn count() {
    let datas = [
        (r!(=10, ?010).iter(), ok(0)),
        (r!(=10, ?030).iter(), ok(20)),
        (r!(=10, ----).iter(), ng()),
        (r!(---, ?030).iter(), ng()),
        (r!(=00, =MAX).iter(), ng()),
    ];

    for (target, tobe) in datas {
        let asis = test_panic(|| target.count());
        assert_eqa!(asis, tobe);
    }
}

#[test]
fn last() {
    let datas = [
        (r!(=10, ?10).iter(), ok(None)),
        (r!(=10, ?30).iter(), ok(Some(29))),
        (r!(=10, =30).iter(), ok(Some(30))),
        (r!(=10, ---).iter(), ng()),
        (r!(---, ?30).iter(), ng()),
    ];

    for (target, tobe) in datas {
        let asis = test_panic(|| target.last());
        assert_eqa!(asis, tobe);
    }
}

#[test]
fn min() {
    let datas = [
        (r!(=10, ?10).iter(), ok(None)),
        (r!(=10, ?30).iter(), ok(Some(10))),
        (r!(?10, ?30).iter(), ok(Some(11))),
        (r!(=10, ---).iter(), ok(Some(10))),
        (r!(---, ?30).iter(), ng()),
    ];

    for (target, tobe) in datas {
        let asis = test_panic(|| target.min());
        assert_eqa!(asis, tobe);
    }
}

#[test]
fn max() {
    let datas = [
        (r!(=10, ?10).iter(), ok(None)),
        (r!(=10, ?30).iter(), ok(Some(29))),
        (r!(=10, =30).iter(), ok(Some(30))),
        (r!(---, ?30).iter(), ok(Some(29))),
        (r!(=10, ---).iter(), ng()),
    ];

    for (target, tobe) in datas {
        let asis = test_panic(|| target.max());
        assert_eqa!(asis, tobe);
    }
}

#[test]
fn is_sorted() {
    let targets = [
        r!(=10, ?10).iter(),
        r!(=10, ?30).iter(),
        r!(=30, ?10).iter(),
        r!(=10, ---).iter(),
        r!(---, ?30).iter(),
        r!(---, ---).iter(),
    ];

    for target in targets {
        let result = target.is_sorted();
        assert!(result);
    }
}

#[test]
fn next_back() {
    with_normal();
    with_range_from();

    fn with_normal() {
        let target = &mut r!(=10, ?13).iter();
        assert_eq!(target.next_back(), Some(12));
        assert_eq!(target.next_back(), Some(11));
        assert_eq!(target.next_back(), Some(10));
        assert_eq!(target.next_back(), None);
    }

    fn with_range_from() {
        let target = &mut r!(=10, ---).iter();
        let result = test_panic(|| target.next_back());
        assert!(result.is_panic());
    }
}

#[test]
fn nth_back() {
    with_normal();
    with_range_from();
    with_overflow();
    with_overflow2();

    fn with_normal() {
        let datas = [
            (r!(=10, ?10), 0, vec![]),
            (r!(=13, ?10), 0, vec![]),
            (r!(=10, ?13), 0, vec![12, 11, 10]),
            (r!(?10, =13), 0, vec![13, 12, 11]),
            (r!(=10, ?25), 3, vec![21, 17, 13]),
        ];

        for (range, n, tobes) in datas {
            let target = &mut range.iter();
            for tobe in tobes {
                let asis = target.nth_back(n);
                assert_eq!(asis, Some(tobe));
            }

            assert_eq!(target.nth_back(n), None);
            assert_eq!(target.nth_back(n), None);
        }
    }

    fn with_range_from() {
        let target = &mut r!(=10, ---).iter();
        let result = test_panic(|| target.nth_back(3));
        assert!(result.is_panic());
    }

    fn with_overflow() {
        let target = &mut ru::new(..255u8).iter();
        target.nth_back(128);

        let result = test_panic(|| target.nth_back(128));
        assert!(result.is_panic());
    }

    fn with_overflow2() {
        let target = &mut ru::new(..255u8).iter();
        let result = test_panic(|| target.nth_back(255));
        assert!(result.is_panic());
    }
}
