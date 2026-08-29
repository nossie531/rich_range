use crate::for_test::aliases::*;
use crate::for_test::consts::*;
use crate::for_test::macros::*;
use crate::for_test::*;
use rich_range::parts::*;
use rich_range::prelude::*;
use rich_range::*;
use test_panic::prelude::*;

#[test]
fn is_empty() {
    when_normal();
    when_unmixable();

    fn when_normal() {
        let datas = [
            (r!(---, ?60), false),
            (r!(=30, ---), false),
            (r!(?30, ---), false),
            (r!(=30, ?60), false),
            (r!(=30, =30), false),
            (r!(=30, ?30), true),
            (r!(=60, ?30), true),
        ];

        for (target, tobe) in datas {
            let asis = target.is_empty();
            assert_eq!(asis, tobe);
        }
    }

    fn when_unmixable() {
        for target in ts::ranges_unordered() {
            let result = target.is_empty();
            assert!(result);
        }
    }
}

#[test]
fn is_broken() {
    let datas = [
        (r!(---, ?60), false),
        (r!(=30, ---), false),
        (r!(?30, ---), false),
        (r!(=30, ?60), false),
        (r!(=30, =30), false),
        (r!(=30, ?30), false),
        (r!(?30, =30), false),
        (r!(=60, ?30), true),
        (r!(?60, ?60), true),
    ];

    for (target, tobe) in datas {
        let asis = target.is_broken();
        assert_eq!(asis, tobe);
    }
}

#[test]
fn is_cursor() {
    let datas = [
        (r!(=30, ?30), true),
        (r!(?30, =30), true),
        (r!(=30, ?60), false),
        (r!(=60, ?30), false),
    ];

    for (target, tobe) in datas {
        let asis = target.is_cursor();
        assert_eq!(asis, tobe);
    }
}

#[test]
fn is_cursor_fwd() {
    let datas = [
        (r!(---, ?60), false),
        (r!(=30, ---), false),
        (r!(?30, ---), false),
        (r!(=30, ?60), false),
        (r!(=30, =30), false),
        (r!(=30, ?30), true),
        (r!(?30, =30), false),
        (r!(=60, ?30), false),
    ];

    for (target, tobe) in datas {
        let asis = target.is_cursor_fwd();
        assert_eq!(asis, tobe);
    }
}

#[test]
fn is_cursor_bwd() {
    let datas = [
        (r!(---, ?60), false),
        (r!(=30, ---), false),
        (r!(?30, ---), false),
        (r!(=30, ?60), false),
        (r!(=30, =30), false),
        (r!(=30, ?30), false),
        (r!(?30, =30), true),
        (r!(=60, ?30), false),
    ];

    for (target, tobe) in datas {
        let asis = target.is_cursor_bwd();
        assert_eq!(asis, tobe);
    }
}

#[test]
fn is_point() {
    let datas = [
        (r!(=30, ?60), false),
        (r!(=60, ?30), false),
        (r!(=30, ?30), false),
        (r!(?30, ?30), false),
        (r!(=30, =30), true),
    ];

    for (target, tobe) in datas {
        let asis = target.is_point();
        assert_eq!(asis, tobe);
    }
}

#[test]
fn is_wide() {
    let datas = [
        (r!(---, ---), true),
        (r!(---, ?60), true),
        (r!(=30, ---), true),
        (r!(=30, ?60), false),
    ];

    for (target, tobe) in datas {
        let asis = target.is_wide();
        assert_eq!(asis, tobe);
    }
}

#[test]
fn is_full() {
    let datas = [
        (r!(---, ---), true),
        (r!(---, ?60), false),
        (r!(=30, ---), false),
        (r!(=30, ?60), false),
    ];

    for (target, tobe) in datas {
        let asis = target.is_full();
        assert_eq!(asis, tobe);
    }
}

#[test]
fn start_edge() {
    let target = ru::new(30..60);
    let result = RichRangeBounds::start_edge(&target);
    assert_eq!(result, Edge::new(Side::S, In(&30)));
}

#[test]
fn end_edge() {
    let target = ru::new(30..60);
    let result = RichRangeBounds::end_edge(&target);
    assert_eq!(result, Edge::new(Side::E, Ex(&60)));
}

#[test]
fn head() {
    let datas = [
        (r!(----, ?60), ok(0)),
        (r!(=30_, ?60), ok(30)),
        (r!(?30_, ?60), ok(31)),
        (r!(=MAX, ?60), ok(MAX)),
        (r!(?MAX, ?60), ng()),
    ];

    for (target, tobe) in datas {
        let asis = test_panic(|| target.head());
        assert_eqa!(asis, tobe);
    }
}

#[test]
fn tail() {
    let datas = [
        (r!(=30, ----), ok(MAX)),
        (r!(=30, ?60_), ok(59)),
        (r!(?30, =60_), ok(60)),
        (r!(?30, ?MAX), ok(MAX - 1)),
        (r!(=00, ?00_), ng()),
    ];

    for (target, tobe) in datas {
        let asis = test_panic(|| target.tail());
        assert_eqa!(asis, tobe);
    }
}

#[test]
fn prev() {
    let datas = [
        (r!(---, ?60), ok(0)),
        (r!(=30, ?60), ok(29)),
        (r!(?30, ?60), ok(30)),
        (r!(?00, ?60), ok(0)),
        (r!(=00, ?60), ng()),
    ];

    for (target, tobe) in datas {
        let asis = test_panic(|| target.prev());
        assert_eqa!(asis, tobe);
    }
}

#[test]
fn next() {
    let datas = [
        (r!(=30, ----), ok(MAX)),
        (r!(=30, ?60_), ok(60)),
        (r!(?30, =60_), ok(61)),
        (r!(?30, ?MAX), ok(MAX)),
        (r!(?30, =MAX), ng()),
    ];

    for (target, tobe) in datas {
        let asis = test_panic(|| target.next());
        assert_eqa!(asis, tobe);
    }
}

#[test]
fn cursor() {
    let datas = [
        (r!(---, ?60), None),
        (r!(=30, ---), None),
        (r!(?30, ---), None),
        (r!(=30, ?60), None),
        (r!(=30, =30), None),
        (r!(=30, ?30), Some(&30)),
        (r!(?30, =30), Some(&30)),
        (r!(=60, ?30), None),
    ];

    for (target, tobe) in datas {
        let asis = target.cursor();
        assert_eq!(asis, tobe);
    }
}

#[test]
fn point() {
    let datas = [
        (r!(=30, ?60), None),
        (r!(=30, ?30), None),
        (r!(=30, =30), Some(&30)),
    ];

    for (target, tobe) in datas {
        let asis = target.point();
        assert_eq!(asis, tobe);
    }
}

#[test]
fn len() {
    when_normals();
    when_isize_limits();
    when_large_type();

    fn when_normals() {
        let datas = [
            (r!(---, ?60), None),
            (r!(=30, ---), None),
            (r!(=30, ?60), Some(30)),
            (r!(=30, =60), Some(31)),
            (r!(?30, ?60), Some(29)),
            (r!(?30, =60), Some(30)),
            (r!(=30, =30), Some(1)),
            (r!(=30, ?30), Some(0)),
            (r!(?30, ?31), Some(0)),
            (r!(=60, ?30), Some(0)),
            (r!(=60, =30), Some(0)),
            (r!(?60, ?30), Some(0)),
            (r!(?60, =30), Some(0)),
        ];

        for (target, tobe) in datas {
            let asis = target.len();
            assert_eq!(asis, tobe);
        }
    }

    fn when_isize_limits() {
        let target = ru::new(isize::MIN..=isize::MAX);
        let result = target.len();
        assert_eq!(result, None);
    }

    fn when_large_type() {
        if size_of::<i128>() <= size_of::<usize>() {
            return;
        }

        let target = ru::new(isize::MIN as i128..=isize::MAX as i128 + 1);
        let result = target.len();
        assert_eq!(result, None);
    }
}

#[test]
fn size() {
    when_normals();
    when_signed_limits_half();
    when_signed_limits();

    fn when_normals() {
        let datas = [
            (r!(---, ?60), None),
            (r!(---, =60), None),
            (r!(=30, ---), None),
            (r!(?30, ---), None),
            (r!(=30, ?60), Some(30)),
            (r!(=30, =60), Some(31)),
            (r!(?30, ?60), Some(29)),
            (r!(?30, =60), Some(30)),
            (r!(=30, =30), Some(1)),
            (r!(=30, ?30), Some(0)),
            (r!(?30, ?31), Some(0)),
            (r!(=60, ?30), Some(0)),
            (r!(=60, =30), Some(0)),
            (r!(?60, ?30), Some(0)),
            (r!(?60, =30), Some(0)),
        ];

        for (target, tobe) in datas {
            let asis = target.size();
            assert_eq!(asis, tobe);
        }
    }

    fn when_signed_limits_half() {
        let target = ru::new((i8::MIN / 2)..(i8::MAX / 2));
        let result = target.size();
        assert_eq!(result, Some((i8::MAX / 2) - (i8::MIN / 2)));
    }

    fn when_signed_limits() {
        if !tu::overflow_checks() {
            return;
        }

        let target = ru::new(i8::MIN..=i8::MAX);
        let result = test_panic(|| target.size());
        assert!(result.is_panic());
    }
}

#[test]
fn width() {
    when_normals();
    when_signed_limits_half();
    when_signed_limits();

    fn when_normals() {
        let datas = [
            (r!(---, ?60), None),
            (r!(---, =60), None),
            (r!(=30, ---), None),
            (r!(?30, ---), None),
            (r!(=30, ?60), Some(30)),
            (r!(=30, =60), Some(30)),
            (r!(?30, ?60), Some(30)),
            (r!(=30, =30), Some(0)),
            (r!(=30, ?30), Some(0)),
            (r!(?30, ?31), Some(1)),
            (r!(=60, ?30), Some(0)),
        ];

        for (target, tobe) in datas {
            let asis = target.width();
            assert_eq!(asis, tobe);
        }
    }

    fn when_signed_limits_half() {
        let target = ru::new((i8::MIN / 2)..(i8::MAX / 2));
        let result = target.width();
        assert_eq!(result, Some((i8::MAX / 2) - (i8::MIN / 2)));
    }

    fn when_signed_limits() {
        if !tu::overflow_checks() {
            return;
        }

        let target = ru::new(i8::MIN..=i8::MAX);
        let result = test_panic(|| target.width());
        assert!(result.is_panic());
    }
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
    let target = ts::range_univ();
    let result = target.as_ref();
    assert_eq!(result, ru::new(&START..&END));
}

#[test]
fn cast() {
    let datas = [
        (r!(<u8>, ---, ---), r!(<u16>, ---, ---)),
        (r!(<u8>, =30, ---), r!(<u16>, =30, ---)),
        (r!(<u8>, ---, ?60), r!(<u16>, ---, ?60)),
        (r!(<u8>, ?30, ---), r!(<u16>, ?30, ---)),
        (r!(<u8>, ---, =60), r!(<u16>, ---, =60)),
    ];

    for (target, tobe) in datas {
        let asis = target.cast();
        assert_eq!(asis, tobe);
    }
}

#[test]
fn try_cast() {
    let datas = [
        (r!(----, ---), ro!(<u8>, ---, ---)),
        (r!(=030, ---), ro!(<u8>, =30, ---)),
        (r!(----, ?60), ro!(<u8>, ---, ?60)),
        (r!(?030, ---), ro!(<u8>, ?30, ---)),
        (r!(----, =60), ro!(<u8>, ---, =60)),
        (r!(=260, =60), ro!(<u8>, --------)),
    ];

    for (target, tobe) in datas {
        let asis = target.try_cast();
        assert_eq!(asis, tobe);
    }
}

#[test]
fn to_range() {
    let datas = [
        (r!(---, ?60), ok(0..60)),
        (r!(=30, ?60), ok(30..60)),
        (r!(?30, ?60), ok(31..60)),
        (r!(=30, ---), ok(30..MAX)),
        (r!(=30, =60), ok(30..61)),
        (r!(?MAX, ?60), ng()),
        (r!(=30, =MAX), ng()),
    ];

    for (target, tobe) in datas {
        let asis = test_panic(|| target.to_range());
        assert_eqa!(asis, tobe);
    }
}

#[test]
fn iter() {
    when_normal();
    when_inclusive_limits();

    fn when_normal() {
        let target = r!(=3, ?6);
        let master = 3..6;
        let asis = target.iter();
        let tobe = master.into_iter();
        assert!(asis.eq(tobe));
    }

    fn when_inclusive_limits() {
        let target = ru::new(0_u8..=255);
        let master = 0_u8..=255;
        let asis = target.iter();
        let tobe = master.into_iter();
        assert!(asis.eq(tobe));
    }
}

#[test]
fn flip_xxx() {
    when_normal();
    when_cursor_mode_off();
    when_cursor_mode_on();

    fn when_normal() {
        let datas = [
            (r!(---, ---), (ro!(--------), ro!(--------))),
            (r!(=30, ---), (ro!(---, ?30), ro!(--------))),
            (r!(---, ?60), (ro!(=60, ---), ro!(--------))),
            (r!(?30, ---), (ro!(---, =30), ro!(--------))),
            (r!(---, =60), (ro!(?60, ---), ro!(--------))),
            (r!(=30, ?60), (ro!(---, ?30), ro!(=60, ---))),
            (r!(?30, =60), (ro!(---, =30), ro!(?60, ---))),
            (r!(=60, ?30), (ro!(---, ---), ro!(--------))),
        ];

        for (target, tobe) in datas {
            let asis1 = target.flip();
            let asis2 = target.flip_adv(CursorMode::On);
            assert_eq!(asis1, tobe);
            assert_eq!(asis2, tobe);
        }
    }

    fn when_cursor_mode_off() {
        let datas = [
            (r!(=30, ?30), (ro!(--, --), ro!(--))),
            (r!(?30, =30), (ro!(--, --), ro!(--))),
        ];

        for (target, tobe) in datas {
            let asis1 = target.flip();
            let asis2 = target.flip_adv(CursorMode::Off);
            assert_eq!(asis1, tobe);
            assert_eq!(asis2, tobe);
        }
    }

    fn when_cursor_mode_on() {
        let datas = [
            (r!(=30, ?30), (ro!(--, ?30), ro!(=30, --))),
            (r!(?30, =30), (ro!(--, =30), ro!(?30, --))),
        ];

        for (target, tobe) in datas {
            let asis = target.flip_adv(CursorMode::On);
            assert_eq!(asis, tobe);
        }
    }
}

#[test]
fn shift() {
    when_normal();
    when_overflow();

    fn when_normal() {
        let datas = [
            (r!(?30, ?60), 10, false, r!(?20, ?50)),
            (r!(?30, ?60), 10, true, r!(?40, ?70)),
        ];

        for (target, value, positive, tobe) in datas {
            let asis = target.shift(value, positive);
            assert_eq!(asis, tobe);
        }
    }

    fn when_overflow() {
        if !tu::overflow_checks() {
            return;
        }

        let datas = [
            (r!(<u8>, ---, =000), 10, false),
            (r!(<u8>, ---, =250), 10, true),
        ];

        for (target, value, positive) in datas {
            let result = test_panic(|| target.shift(value, positive));
            assert!(result.is_panic());
        }
    }
}

#[test]
fn shl() {
    when_normal();
    when_overflow();

    fn when_normal() {
        let datas = [
            (r!(---, ---), 10, r!(---, ---)),
            (r!(?30, ?60), 10, r!(?20, ?50)),
            (r!(=30, =60), 10, r!(=20, =50)),
        ];

        for (target, rhs, tobe) in datas {
            let asis = target.shl(rhs);
            assert_eq!(asis, tobe);
        }
    }

    fn when_overflow() {
        if !tu::overflow_checks() {
            return;
        }

        let datas = [(r!(---, =00), 10), (r!(=00, ---), 10)];

        for (target, rhs) in datas {
            let result = test_panic(|| target.shl(rhs));
            assert!(result.is_panic());
            dbg!(result.is_panic());
        }
    }
}

#[test]
fn shr() {
    when_normal();
    when_overflow();

    fn when_normal() {
        let datas = [
            (r!(---, ---), 10, r!(---, ---)),
            (r!(?30, ?60), 10, r!(?40, ?70)),
            (r!(=30, =60), 10, r!(=40, =70)),
        ];

        for (target, rhs, tobe) in datas {
            let asis = target.shr(rhs);
            assert_eq!(asis, tobe);
        }
    }

    fn when_overflow() {
        if !tu::overflow_checks() {
            return;
        }

        let datas = [(r!(<u8>, ----, =250), 10), (r!(<u8>, =250, ----), 10)];

        for (target, rhs) in datas {
            let result = test_panic(|| target.shr(rhs));
            assert!(result.is_panic());
        }
    }
}

#[test]
fn add_start() {
    when_normal();
    when_overflow();

    fn when_normal() {
        let target = r!(=30, ?60);
        let result = target.add_start(10);
        assert_eq!(result, r!(=40, ?60));
    }

    fn when_overflow() {
        if !tu::overflow_checks() {
            return;
        }

        let target = r!(<u8>, =30, ?60);
        let result = test_panic(|| target.add_start(230));
        assert!(result.is_panic());
    }
}

#[test]
fn add_end() {
    when_normal();
    when_overflow();

    fn when_normal() {
        let target = r!(=30, ?60);
        let result = target.add_end(10);
        assert_eq!(result, r!(=30, ?70));
    }

    fn when_overflow() {
        if !tu::overflow_checks() {
            return;
        }

        let target = r!(<u8>, =30, ?60);
        let result = test_panic(|| target.add_end(200));
        assert!(result.is_panic());
    }
}

#[test]
fn sub_start() {
    when_normal();
    when_overflow();

    fn when_normal() {
        let target = r!(=30, ?60);
        let result = target.sub_start(10);
        assert_eq!(result, r!(=20, ?60));
    }

    fn when_overflow() {
        if !tu::overflow_checks() {
            return;
        }

        let target = r!(<u8>, =30, ?60);
        let result = test_panic(|| target.sub_start(70));
        assert!(result.is_panic());
    }
}

#[test]
fn sub_end() {
    when_normal();
    when_overflow();

    fn when_normal() {
        let target = r!(=30, ?60);
        let result = target.sub_end(10);
        assert_eq!(result, r!(=30, ?50));
    }

    fn when_overflow() {
        if !tu::overflow_checks() {
            return;
        }

        let target = r!(<u8>, =30, ?60);
        let result = test_panic(|| target.sub_end(70));
        assert!(result.is_panic());
    }
}

#[test]
fn calc_start() {
    when_normal();
    when_overflow();

    fn when_normal() {
        let datas = [
            (r!(---, =60), 40, ng()),
            (r!(=30, ---), 40, ng()),
            (r!(=30, ?60), 40, ok(r!(=20, ?60))),
            (r!(?30, ?60), 40, ok(r!(?20, ?60))),
            (r!(=30, =60), 40, ok(r!(=20, =60))),
        ];

        for (target, value, tobe) in datas {
            let asis = test_panic(|| target.calc_start(value));
            assert_eqa!(asis, tobe);
        }
    }

    fn when_overflow() {
        if !tu::overflow_checks() {
            return;
        }

        let target = r!(<u8>, =30, ?60);
        let result = test_panic(|| target.calc_start(70));
        assert!(result.is_panic());
    }
}

#[test]
fn calc_end() {
    when_normal();
    when_overflow();

    fn when_normal() {
        let datas = [
            (r!(---, =60), 040, ng()),
            (r!(=30, ---), 040, ng()),
            (r!(=30, ?60), 040, ok(r!(=30, ?70))),
            (r!(?30, ?60), 040, ok(r!(?30, ?70))),
            (r!(=30, =60), 040, ok(r!(=30, =70))),
        ];

        for (target, value, tobe) in datas {
            let asis = test_panic(|| target.calc_end(value));
            assert_eqa!(asis, tobe);
        }
    }

    fn when_overflow() {
        if !tu::overflow_checks() {
            return;
        }

        let target = r!(<u8>, =30, ?60);
        let result = test_panic(|| target.calc_end(230));
        assert!(result.is_panic());
    }
}

#[test]
fn align_start() {
    when_normal();
    when_signed();
    when_unordered();
    when_buggy_range();

    fn when_normal() {
        let datas = [
            // Backword and forward.
            (r!(<u8>, =30, ?60), 20, ok(r!(<u8>, =20, ?50))),
            (r!(<u8>, =30, ?60), 30, ok(r!(<u8>, =30, ?60))),
            (r!(<u8>, =30, ?60), 40, ok(r!(<u8>, =40, ?70))),
            // Bounds variations.
            (r!(<u8>, ---, ---), 40, ng()),
            (r!(<u8>, ---, ?60), 40, ng()),
            (r!(<u8>, =30, ---), 40, ok(r!(<u8>, =40, ---))),
            (r!(<u8>, ?30, =60), 40, ok(r!(<u8>, ?40, =70))),
            // Empty bounds.
            (r!(<u8>, =30, ?30), 40, ok(r!(<u8>, =40, ?40))),
            (r!(<u8>, ?30, ?30), 40, ok(r!(<u8>, ?40, ?40))),
            (r!(<u8>, =60, ?30), 50, ok(r!(<u8>, =50, ?20))),
            (r!(<u8>, =60, ?30), 60, ok(r!(<u8>, =60, ?30))),
            (r!(<u8>, =60, ?30), 70, ok(r!(<u8>, =70, ?40))),
            // Result overflows.
            (r!(<u8>, =30, ?60), 230, ng()),
            (r!(<u8>, =60, ?30), 20, ng()),
        ];

        for (target, value, tobe) in datas {
            let asis = test_panic(|| target.align_start(value));
            assert_eqa!(asis, tobe);
        }
    }

    fn when_signed() {
        let datas = [
            // Small offset and small width.
            (r!(<i8>, =20, ?30), 10, ok(r!(<i8>, =10, ?20))),
            (r!(<i8>, =20, ?30), 30, ok(r!(<i8>, =30, ?40))),
            // Small offset and big width.
            (r!(<i8>, =-110, ?0110), -100, ok(r!(<i8>, =-100, ?0120))),
            (r!(<i8>, =-110, ?0110), -120, ok(r!(<i8>, =-120, ?0100))),
            (r!(<i8>, =0110, ?-110), 0100, ok(r!(<i8>, =0100, ?-120))),
            (r!(<i8>, =0110, ?-110), 0120, ok(r!(<i8>, =0120, ?-100))),
            // Big offset and small width.
            (r!(<i8>, =-120, ?-110), 0110, ok(r!(<i8>, =0110, ?0120))),
            (r!(<i8>, =0110, ?0120), -110, ok(r!(<i8>, =-110, ?-100))),
            (r!(<i8>, =-110, ?-120), 0110, ok(r!(<i8>, =0110, ?0100))),
            (r!(<i8>, =0120, ?0110), -110, ok(r!(<i8>, =-110, ?-120))),
            // Big offset and big width.
            (r!(<i8>, =-110, ?0010), 0020, ng()),
            (r!(<i8>, =-110, ?0110), 0100, ng()),
            (r!(<i8>, =0110, ?-110), -100, ng()),
        ];

        for (target, value, tobe) in datas {
            let asis = test_panic(|| target.align_start(value));
            assert_eqa!(asis, tobe);
        }
    }

    fn when_unordered() {
        let nan = f32::NAN;
        let datas = [
            (r!(<f32>, =nan, ?60.), 40., ng()),
            (r!(<f32>, =30., ?nan), 40., ng()),
            (r!(<f32>, =30., ?nan), nan, ng()),
            (r!(<f32>, =30., ?60.), 40., ok(r!(<f32>, =40., ?70.))),
        ];

        for (target, value, tobe) in datas {
            let asis = test_panic(|| target.align_start(value));
            assert_eqa!(asis, tobe);
        }
    }

    fn when_buggy_range() {
        let datas = [
            // Various bounds.
            (BuggyRange::<i8>::new(In(30), Ex(60)), 40),
            (BuggyRange::<i8>::new(In(30), Ub), 40),
            (BuggyRange::<i8>::new(Ub, Ex(60)), 40),
            // Small offset and big width.
            (BuggyRange::<i8>::new(In(-110), Ex(100)), -100),
            // Big offset and small width.
            (BuggyRange::<i8>::new(In(-120), Ex(-110)), 110),
        ];

        for (range, value) in datas {
            let result = test_panic(|| range.align_start(value));
            assert!(result.is_panic());
        }
    }
}

#[test]
fn align_end() {
    when_normal();
    when_signed();
    when_unordered();
    when_buggy_range();

    fn when_normal() {
        let datas = [
            // Backword and forward.
            (r!(<u8>, =30, ?60), 50, ok(r!(<u8>, =20, ?50))),
            (r!(<u8>, =30, ?60), 60, ok(r!(<u8>, =30, ?60))),
            (r!(<u8>, =30, ?60), 70, ok(r!(<u8>, =40, ?70))),
            // Bounds variations.
            (r!(<u8>, ---, ---), 70, ng()),
            (r!(<u8>, =30, ---), 70, ng()),
            (r!(<u8>, ---, ?60), 70, ok(r!(<u8>, ---, ?70))),
            (r!(<u8>, ?30, =60), 70, ok(r!(<u8>, ?40, =70))),
            // Empty bounds.
            (r!(<u8>, =30, ?30), 50, ok(r!(<u8>, =50, ?50))),
            (r!(<u8>, ?30, ?30), 50, ok(r!(<u8>, ?50, ?50))),
            (r!(<u8>, =60, ?30), 40, ok(r!(<u8>, =70, ?40))),
            (r!(<u8>, =60, ?30), 50, ok(r!(<u8>, =80, ?50))),
            (r!(<u8>, =60, ?30), 60, ok(r!(<u8>, =90, ?60))),
            // Result overflows.
            (r!(<u8>, =30, ?60), 20, ng()),
            (r!(<u8>, =60, ?30), 230, ng()),
        ];

        for (target, value, tobe) in datas {
            let asis = test_panic(|| target.align_end(value));
            assert_eqa!(asis, tobe);
        }
    }

    fn when_signed() {
        let datas = [
            // Small offset and small width.
            (r!(<i8>, =20, ?30), 20, ok(r!(<i8>, =10, ?20))),
            (r!(<i8>, =20, ?30), 40, ok(r!(<i8>, =30, ?40))),
            // Small offset and big width.
            (r!(<i8>, =-110, ?0110), 0100, ok(r!(<i8>, =-120, ?0100))),
            (r!(<i8>, =-110, ?0110), 0120, ok(r!(<i8>, =-100, ?0120))),
            (r!(<i8>, =0110, ?-110), -100, ok(r!(<i8>, =0120, ?-100))),
            (r!(<i8>, =0110, ?-110), -120, ok(r!(<i8>, =0100, ?-120))),
            // Big offset and small width.
            (r!(<i8>, =-120, ?-110), 0110, ok(r!(<i8>, =0100, ?0110))),
            (r!(<i8>, =0110, ?0120), -110, ok(r!(<i8>, =-120, ?-110))),
            (r!(<i8>, =-110, ?-120), 0110, ok(r!(<i8>, =0120, ?0110))),
            (r!(<i8>, =0120, ?0110), -110, ok(r!(<i8>, =-100, ?-110))),
            // Big offset and big width.
            (r!(<i8>, =-010, ?0110), -020, ng()),
            (r!(<i8>, =-110, ?0110), -100, ng()),
            (r!(<i8>, =0110, ?-110), 0100, ng()),
        ];

        for (target, value, tobe) in datas {
            let asis = test_panic(|| target.align_end(value));
            assert_eqa!(asis, tobe);
        }
    }

    fn when_unordered() {
        let nan = f32::NAN;
        let datas = [
            (r!(<f32>, =nan, ?60.), 50., ng()),
            (r!(<f32>, =30., ?nan), 50., ng()),
            (r!(<f32>, =30., ?nan), nan, ng()),
            (r!(<f32>, =30., ?60.), 50., ok(r!(<f32>, =20., ?50.))),
        ];

        for (target, value, tobe) in datas {
            let asis = test_panic(|| target.align_end(value));
            assert_eqa!(asis, tobe);
        }
    }

    fn when_buggy_range() {
        let datas = [
            // Various bounds.
            (BuggyRange::<i8>::new(In(30), Ex(60)), 40),
            (BuggyRange::<i8>::new(In(30), Ub), 40),
            (BuggyRange::<i8>::new(Ub, Ex(60)), 40),
            // Small offset and big width.
            (BuggyRange::<i8>::new(In(-110), Ex(110)), 100),
            // Big offset and small width.
            (BuggyRange::<i8>::new(In(-120), Ex(-110)), 110),
        ];

        for (range, value) in datas {
            let result = test_panic(|| range.align_end(value));
            assert!(result.is_panic());
        }
    }
}

#[test]
fn map() {
    let datas = [
        (r!(---, ---), r!(---, ---)),
        (r!(=30, ---), r!(=20, ---)),
        (r!(---, ?60), r!(---, ?50)),
        (r!(?30, ---), r!(?20, ---)),
        (r!(---, =60), r!(---, =50)),
    ];

    for (target, tobe) in datas {
        let asis = target.map(|x| x - 10);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn try_map() {
    let ca10 = (|x: u8| x.checked_add(10)) as fn(_) -> _;
    let cs10 = (|x: u8| x.checked_sub(10)) as fn(_) -> _;
    let datas = [
        (r!(<u8>, ---, ----), ca10, ro!(<u8>, ---, ---)),
        (r!(<u8>, =30, ----), ca10, ro!(<u8>, =40, ---)),
        (r!(<u8>, ---, ?060), ca10, ro!(<u8>, ---, ?70)),
        (r!(<u8>, ?30, ----), ca10, ro!(<u8>, ?40, ---)),
        (r!(<u8>, ---, =060), ca10, ro!(<u8>, ---, =70)),
        (r!(<u8>, =30, =250), ca10, ro!(<u8>, --------)),
        (r!(<u8>, =30, ?250), ca10, ro!(<u8>, --------)),
        (r!(<u8>, =00, =060), cs10, ro!(<u8>, --------)),
    ];

    for (target, f, tobe) in datas {
        let asis = target.try_map(f);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn checked_shift() {
    let datas = [
        (r!(<u8>, =30, ?60), 10, false, ro!(<u8>, =20, ?50)),
        (r!(<u8>, =30, ?60), 100, true, ro!(<u8>, =130, ?160)),
        (r!(<u8>, =30, ?60), 40, false, ro!(<u8>, --------)),
        (r!(<u8>, =30, ?60), 200, true, ro!(<u8>, ----------)),
    ];

    for (target, rhs, positive, tobe) in datas {
        let asis = target.checked_shift(rhs, positive);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn checked_shl() {
    let datas = [
        (r!(<u8>, =30, ?60), 10, ro!(<u8>, =20, ?50)),
        (r!(<u8>, =30, ?60), 40, ro!(<u8>, --------)),
        (r!(<u8>, =60, ?30), 40, ro!(<u8>, --------)),
    ];

    for (target, rhs, tobe) in datas {
        let asis = target.checked_shl(rhs);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn checked_shr() {
    let datas = [
        (r!(<u8>, =30, ?60), 100, ro!(<u8>, =130, ?160)),
        (r!(<u8>, =30, ?60), 200, ro!(<u8>, ----------)),
        (r!(<u8>, =60, ?30), 200, ro!(<u8>, ----------)),
    ];

    for (target, rhs, tobe) in datas {
        let asis = target.checked_shr(rhs);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn checked_add_start() {
    let datas = [
        (r!(<u8>, =30, ?60), 010, ro!(<u8>, =40, ?60)),
        (r!(<u8>, =30, ?60), 230, ro!(<u8>, --------)),
    ];

    for (target, rhs, tobe) in datas {
        let asis = target.checked_add_start(rhs);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn checked_add_end() {
    let datas = [
        (r!(<u8>, =30, ?60), 010, ro!(<u8>, =30, ?70)),
        (r!(<u8>, =30, ?60), 200, ro!(<u8>, --------)),
    ];

    for (target, rhs, tobe) in datas {
        let asis = target.checked_add_end(rhs);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn checked_sub_start() {
    let datas = [
        (r!(<u8>, =30, ?60), 10, ro!(<u8>, =20, ?60)),
        (r!(<u8>, =30, ?60), 40, ro!(<u8>, --------)),
    ];

    for (target, rhs, tobe) in datas {
        let asis = target.checked_sub_start(rhs);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn checked_sub_end() {
    let datas = [
        (r!(<u8>, =30, ?60), 10, ro!(<u8>, =30, ?50)),
        (r!(<u8>, =30, ?60), 70, ro!(<u8>, --------)),
    ];

    for (target, rhs, tobe) in datas {
        let asis = target.checked_sub_end(rhs);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn checked_calc_start() {
    let datas = [
        (r!(<u8>, ---, =60), 40, ng()),
        (r!(<u8>, =30, ---), 40, ng()),
        (r!(<u8>, =30, ?60), 40, ok(ro!(<u8>, =20, ?60))),
        (r!(<u8>, ?30, ?60), 40, ok(ro!(<u8>, ?20, ?60))),
        (r!(<u8>, =30, =60), 40, ok(ro!(<u8>, =20, =60))),
        (r!(<u8>, =30, =60), 70, ok(ro!(<u8>, --------))),
    ];

    for (target, value, tobe) in datas {
        let asis = test_panic(|| target.checked_calc_start(value));
        assert_eqa!(asis, tobe);
    }
}

#[test]
fn checked_calc_end() {
    let datas = [
        (r!(<u8>, ---, =60), 040, ng()),
        (r!(<u8>, =30, ---), 040, ng()),
        (r!(<u8>, =30, ?60), 040, ok(ro!(<u8>, =30, ?70))),
        (r!(<u8>, ?30, ?60), 040, ok(ro!(<u8>, ?30, ?70))),
        (r!(<u8>, =30, =60), 040, ok(ro!(<u8>, =30, =70))),
        (r!(<u8>, =30, =60), 230, ok(ro!(<u8>, --------))),
    ];

    for (target, value, tobe) in datas {
        let asis = test_panic(|| target.checked_calc_end(value));
        assert_eqa!(asis, tobe);
    }
}

#[test]
fn checked_align_start() {
    let datas = [
        (r!(<u8>, =30, ?60), 040, ro!(<u8>, =40, ?70)),
        (r!(<u8>, =30, ?60), 230, ro!(<u8>, --------)),
    ];

    for (target, rhs, tobe) in datas {
        let asis = target.checked_align_start(rhs);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn checked_align_end() {
    let datas = [
        (r!(<u8>, =30, ?60), 70, ro!(<u8>, =40, ?70)),
        (r!(<u8>, =30, ?60), 20, ro!(<u8>, --------)),
    ];

    for (target, rhs, tobe) in datas {
        let asis = target.checked_align_end(rhs);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn equiv() {
    let datas = [
        (r!(=30, ?60), r!(=30, ?60), true),
        (r!(=30, ?30), r!(=60, ?60), true),
        (r!(=30, ?60), r!(=30, ?59), false),
        (r!(=30, ?60), r!(=31, ?60), false),
        (r!(=30, ?60), r!(=30, =60), false),
        (r!(=30, ?60), r!(?30, ?60), false),
    ];

    for (x, y, tobe) in datas {
        let asis = x.equiv(&y);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn intersects() {
    when_normal();
    when_unmixable();

    fn when_normal() {
        let datas = [
            // Full ranges vs others.
            (r!(---, ---), r!(---, ---), true),
            (r!(---, ---), r!(=30, ---), true),
            (r!(---, ---), r!(---, ?60), true),
            (r!(---, ---), r!(?30, ---), true),
            (r!(---, ---), r!(---, =60), true),
            (r!(---, ---), r!(=30, ?60), true),
            (r!(---, ---), r!(=30, ?30), true),
            // Included start ranges vs others.
            (r!(=30, ---), r!(=20, ---), true),
            (r!(=30, ---), r!(=30, ---), true),
            (r!(=30, ---), r!(=40, ---), true),
            (r!(=30, ---), r!(---, ?20), false),
            (r!(=30, ---), r!(---, ?30), false),
            (r!(=30, ---), r!(---, ?40), true),
            (r!(=30, ---), r!(?20, ---), true),
            (r!(=30, ---), r!(?30, ---), true),
            (r!(=30, ---), r!(?40, ---), true),
            (r!(=30, ---), r!(---, =20), false),
            (r!(=30, ---), r!(---, =30), true),
            (r!(=30, ---), r!(---, =40), true),
            (r!(=30, ---), r!(=20, ?60), true),
            (r!(=30, ---), r!(=30, ?60), true),
            (r!(=30, ---), r!(=20, ?20), false),
            (r!(=30, ---), r!(=30, ?30), true),
            (r!(=30, ---), r!(=60, ?60), true),
            // Excluded end ranges vs others.
            (r!(---, ?60), r!(---, ?50), true),
            (r!(---, ?60), r!(---, ?60), true),
            (r!(---, ?60), r!(---, ?70), true),
            (r!(---, ?60), r!(?50, ---), true),
            (r!(---, ?60), r!(?60, ---), false),
            (r!(---, ?60), r!(?70, ---), false),
            (r!(---, ?60), r!(---, =50), true),
            (r!(---, ?60), r!(---, =60), true),
            (r!(---, ?60), r!(---, =70), true),
            (r!(---, ?60), r!(=30, ?50), true),
            (r!(---, ?60), r!(=30, ?60), true),
            (r!(---, ?60), r!(=50, ?70), true),
            (r!(---, ?60), r!(=60, ?70), false),
            (r!(---, ?60), r!(=50, ?50), true),
            (r!(---, ?60), r!(=60, ?60), false),
            (r!(---, ?60), r!(=70, ?70), false),
            // Excluded start ranges vs others.
            (r!(?30, ---), r!(?20, ---), true),
            (r!(?30, ---), r!(?30, ---), true),
            (r!(?30, ---), r!(?40, ---), true),
            (r!(?30, ---), r!(---, =20), false),
            (r!(?30, ---), r!(---, =30), false),
            (r!(?30, ---), r!(---, =40), true),
            (r!(?30, ---), r!(=20, ?60), true),
            (r!(?30, ---), r!(=30, ?60), true),
            (r!(?30, ---), r!(=40, ?60), true),
            (r!(?30, ---), r!(=20, ?20), false),
            (r!(?30, ---), r!(=30, ?30), false),
            (r!(?30, ---), r!(=40, ?40), true),
            // Included end ranges vs others.
            (r!(---, =60), r!(---, =50), true),
            (r!(---, =60), r!(---, =60), true),
            (r!(---, =60), r!(---, =70), true),
            (r!(---, =60), r!(=30, ?50), true),
            (r!(---, =60), r!(=30, ?60), true),
            (r!(---, =60), r!(=30, ?70), true),
            (r!(---, =60), r!(=60, ?70), true),
            (r!(---, =60), r!(=70, ?80), false),
            (r!(---, ?60), r!(=50, ?50), true),
            (r!(---, ?60), r!(=60, ?60), false),
            (r!(---, ?70), r!(=70, ?70), false),
            // Normal ranges vs others.
            (r!(=30, ?60), r!(=00, ?30), false),
            (r!(=30, ?60), r!(=40, ?50), true),
            (r!(=30, ?60), r!(=60, ?90), false),
            (r!(=30, ?60), r!(=40, ?40), true),
            (r!(=30, ?60), r!(=20, ?20), false),
            (r!(=30, ?60), r!(=30, ?30), true),
            (r!(=30, ?60), r!(=60, ?60), false),
            (r!(=30, ?60), r!(=70, ?70), false),
            // Empty ranges vs empty ranges.
            (r!(=30, ?30), r!(=30, ?30), true),
            (r!(=30, ?30), r!(?30, =30), true),
            (r!(=30, ?30), r!(=60, ?60), false),
            (r!(=30, ?30), r!(=40, ?20), false),
        ];

        for (x, y, tobe) in datas {
            let asis1 = x.intersects(&y);
            let asis2 = y.intersects(&x);
            assert_eq!(asis1, tobe);
            assert_eq!(asis2, tobe);
        }
    }

    fn when_unmixable() {
        for (x, y) in ts::range_pairs_unmixable() {
            let result1 = x.intersects(&y);
            let result2 = y.intersects(&x);
            assert!(!result1);
            assert!(!result2);
        }
    }
}

#[test]
fn include_xxx() {
    when_normal();
    when_unmixable();

    fn when_normal() {
        let datas = [
            // Full range vs others.
            (r!(---, ---), r!(---, ---), true),
            (r!(---, ---), r!(=30, ---), true),
            (r!(---, ---), r!(---, ?60), true),
            (r!(---, ---), r!(?30, ---), true),
            (r!(---, ---), r!(---, =60), true),
            (r!(---, ---), r!(=30, ?60), true),
            (r!(---, ---), r!(=00, ?00), true),
            // Included start range vs others.
            (r!(=30, ---), r!(---, ---), false),
            (r!(=30, ---), r!(=20, ---), false),
            (r!(=30, ---), r!(=30, ---), true),
            (r!(=30, ---), r!(=40, ---), true),
            (r!(=30, ---), r!(---, ?20), false),
            (r!(=30, ---), r!(---, ?30), false),
            (r!(=30, ---), r!(---, ?40), false),
            (r!(=30, ---), r!(?20, ---), false),
            (r!(=30, ---), r!(?30, ---), true),
            (r!(=30, ---), r!(?40, ---), true),
            (r!(=30, ---), r!(---, =20), false),
            (r!(=30, ---), r!(---, =30), false),
            (r!(=30, ---), r!(---, =40), false),
            (r!(=30, ---), r!(=20, ?40), false),
            (r!(=30, ---), r!(=30, ?40), true),
            (r!(=30, ---), r!(=20, =20), false),
            (r!(=30, ---), r!(=30, =30), true),
            (r!(=30, ---), r!(=20, ?20), false),
            (r!(=30, ---), r!(=30, ?30), true),
            (r!(=30, ---), r!(=40, ?40), true),
            (r!(=30, ---), r!(=60, ?30), false),
            // Excluded end range vs others.
            (r!(---, ?60), r!(---, ---), false),
            (r!(---, ?60), r!(=50, ---), false),
            (r!(---, ?60), r!(=60, ---), false),
            (r!(---, ?60), r!(=70, ---), false),
            (r!(---, ?60), r!(---, ?50), true),
            (r!(---, ?60), r!(---, ?60), true),
            (r!(---, ?60), r!(---, ?70), false),
            (r!(---, ?60), r!(?50, ---), false),
            (r!(---, ?60), r!(?60, ---), false),
            (r!(---, ?60), r!(?70, ---), false),
            (r!(---, ?60), r!(---, =50), true),
            (r!(---, ?60), r!(---, =60), false),
            (r!(---, ?60), r!(---, =70), false),
            (r!(---, ?60), r!(=30, ?50), true),
            (r!(---, ?60), r!(=30, ?60), true),
            (r!(---, ?60), r!(=30, ?70), false),
            (r!(---, ?60), r!(=50, =50), true),
            (r!(---, ?60), r!(=60, =60), false),
            (r!(---, ?60), r!(=50, ?50), true),
            (r!(---, ?60), r!(=60, ?60), false),
            (r!(---, ?60), r!(=60, ?30), false),
            // Excluded start range vs others.
            (r!(?30, ---), r!(---, ---), false),
            (r!(?30, ---), r!(=20, ---), false),
            (r!(?30, ---), r!(=30, ---), false),
            (r!(?30, ---), r!(=40, ---), true),
            (r!(?30, ---), r!(---, ?20), false),
            (r!(?30, ---), r!(---, ?30), false),
            (r!(?30, ---), r!(---, ?40), false),
            (r!(?30, ---), r!(?20, ---), false),
            (r!(?30, ---), r!(?30, ---), true),
            (r!(?30, ---), r!(?40, ---), true),
            (r!(?30, ---), r!(---, =20), false),
            (r!(?30, ---), r!(---, =30), false),
            (r!(?30, ---), r!(---, =40), false),
            (r!(?30, ---), r!(=20, ?40), false),
            (r!(?30, ---), r!(=30, ?40), false),
            (r!(?30, ---), r!(=31, ?40), true),
            (r!(?30, ---), r!(=30, =30), false),
            (r!(?30, ---), r!(=31, =31), true),
            (r!(?30, ---), r!(=30, ?30), false),
            (r!(?30, ---), r!(=40, ?40), true),
            (r!(?30, ---), r!(=60, ?30), false),
            // Included end range vs others.
            (r!(---, =60), r!(---, ---), false),
            (r!(---, =60), r!(=50, ---), false),
            (r!(---, =60), r!(=70, ---), false),
            (r!(---, =60), r!(---, ?50), true),
            (r!(---, =60), r!(---, ?60), true),
            (r!(---, =60), r!(---, ?70), false),
            (r!(---, =60), r!(?50, ---), false),
            (r!(---, =60), r!(?70, ---), false),
            (r!(---, =60), r!(---, =50), true),
            (r!(---, =60), r!(---, =60), true),
            (r!(---, =60), r!(---, =70), false),
            (r!(---, =60), r!(=30, ?50), true),
            (r!(---, =60), r!(=30, ?60), true),
            (r!(---, =60), r!(=30, ?70), false),
            (r!(---, =60), r!(=60, =60), true),
            (r!(---, =60), r!(=61, =61), false),
            (r!(---, =60), r!(=60, ?60), true),
            (r!(---, =60), r!(=61, ?61), false),
            (r!(---, =60), r!(=60, ?30), false),
            // Normal range vs others.
            (r!(=30, ?60), r!(---, ---), false),
            (r!(=30, ?60), r!(=20, ---), false),
            (r!(=30, ?60), r!(=50, ---), false),
            (r!(=30, ?60), r!(=70, ---), false),
            (r!(=30, ?60), r!(---, ?20), false),
            (r!(=30, ?60), r!(---, ?50), false),
            (r!(=30, ?60), r!(---, ?70), false),
            (r!(=30, ?60), r!(?20, ---), false),
            (r!(=30, ?60), r!(?50, ---), false),
            (r!(=30, ?60), r!(?70, ---), false),
            (r!(=30, ?60), r!(---, =20), false),
            (r!(=30, ?60), r!(---, =50), false),
            (r!(=30, ?60), r!(---, =70), false),
            (r!(=30, ?60), r!(=40, ?50), true),
            (r!(=30, ?60), r!(=20, ?20), false),
            (r!(=30, ?60), r!(=30, ?30), true),
            (r!(=30, ?60), r!(=60, ?60), false),
            (r!(=30, ?60), r!(=60, ?30), false),
            // Empty ranges vs empty ranges.
            (r!(=30, ?30), r!(=20, ?20), false),
            (r!(=30, ?30), r!(=30, ?30), true),
            (r!(=30, ?30), r!(=40, ?40), false),
            (r!(=30, ?30), r!(=40, ?20), false),
        ];

        for (x, y, tobe) in datas {
            let asis1 = x.includes(&y);
            let asis2 = y.included(&x);
            assert_eq!(asis1, tobe);
            assert_eq!(asis2, tobe);
        }
    }

    fn when_unmixable() {
        for (x, y) in ts::range_pairs_unmixable() {
            let result1 = x.includes(&y);
            let result2 = y.included(&x);
            assert!(!result1);
            assert!(!result2);
        }
    }
}

#[test]
fn adjoins() {
    assert!(RichRangeBounds::adjoins(&ru::new(30..=60), &(60..70)));
    assert!(!RichRangeBounds::adjoins(&ru::new(30..=60), &(70..80)));
    assert!(RichRangeBounds::adjoins(&ru::new(30..60), &(20..=30)));
    assert!(!RichRangeBounds::adjoins(&ru::new(30..60), &(10..=20)));
}

#[test]
fn adjoins_xxx() {
    when_normal();
    when_unmixable();

    fn when_normal() {
        let datas = [
            // Unbound end vs others.
            (r!(---, ---), r!(---, ---), false),
            (r!(---, ---), r!(=30, ---), false),
            (r!(---, ---), r!(?30, ---), false),
            // Excluded end vs others.
            (r!(---, ?30), r!(---, ---), false),
            (r!(---, ?30), r!(=20, ---), false),
            (r!(---, ?30), r!(=30, ---), false),
            (r!(---, ?30), r!(=40, ---), false),
            (r!(---, ?30), r!(?20, ---), false),
            (r!(---, ?30), r!(?30, ---), false),
            (r!(---, ?30), r!(?40, ---), false),
            (r!(---, ?30), r!(=30, ?30), false),
            (r!(---, ?30), r!(=30, ?20), false),
            // Included end vs others.
            (r!(---, =30), r!(---, ---), false),
            (r!(---, =30), r!(=20, ---), false),
            (r!(---, =30), r!(=30, ---), true),
            (r!(---, =30), r!(=40, ---), false),
            (r!(---, =30), r!(?20, ---), false),
            (r!(---, =30), r!(?30, ---), false),
            (r!(---, =30), r!(?40, ---), false),
            (r!(---, =30), r!(?30, =30), false),
            (r!(---, =30), r!(?30, =20), false),
            (r!(---, =30), r!(?30, ?30), false),
            // Empty ranges vs empty ranges.
            (r!(=30, ?30), r!(=30, ?30), false),
            (r!(=30, ?30), r!(=40, ?40), false),
            (r!(=30, ?30), r!(?30, =30), false),
            (r!(=30, ?30), r!(=30, ?20), false),
            (r!(=30, ?20), r!(=30, ?20), false),
        ];

        for (x, y, tobe) in datas {
            let asis1 = x.adjoins_next(&y);
            let asis2 = y.adjoins_prev(&x);
            assert_eq!(asis1, tobe);
            assert_eq!(asis2, tobe);
        }
    }

    fn when_unmixable() {
        for (x, y) in ts::range_pairs_unmixable() {
            let result1 = x.adjoins_next(&y);
            let result2 = x.adjoins_prev(&y);
            assert!(!result1);
            assert!(!result2);
        }
    }
}

#[test]
fn touches() {
    assert!(RichRangeBounds::touches(&ru::new(30..60), &(60..70)));
    assert!(!RichRangeBounds::touches(&ru::new(30..60), &(70..80)));
    assert!(RichRangeBounds::touches(&ru::new(30..60), &(20..30)));
    assert!(!RichRangeBounds::touches(&ru::new(30..60), &(10..20)));
}

#[test]
fn touches_xxx() {
    when_normal();
    when_unmixable();

    fn when_normal() {
        let datas = [
            // Unbound end vs others.
            (r!(---, ---), r!(---, ---), false),
            (r!(---, ---), r!(=30, ---), false),
            (r!(---, ---), r!(?30, ---), false),
            // Excluded end vs others.
            (r!(---, ?30), r!(---, ---), false),
            (r!(---, ?30), r!(=20, ---), false),
            (r!(---, ?30), r!(=30, ---), true),
            (r!(---, ?30), r!(=40, ---), false),
            (r!(---, ?30), r!(?20, ---), false),
            (r!(---, ?30), r!(?30, ---), false),
            (r!(---, ?30), r!(?40, ---), false),
            (r!(---, ?30), r!(=30, ?30), true),
            (r!(---, ?30), r!(=30, ?20), false),
            // Included end vs others.
            (r!(---, =30), r!(---, ---), false),
            (r!(---, =30), r!(=20, ---), false),
            (r!(---, =30), r!(=30, ---), false),
            (r!(---, =30), r!(=40, ---), false),
            (r!(---, =30), r!(?20, ---), false),
            (r!(---, =30), r!(?30, ---), true),
            (r!(---, =30), r!(?40, ---), false),
            (r!(---, =30), r!(?30, =30), true),
            (r!(---, =30), r!(?30, =20), false),
            (r!(---, =30), r!(?30, ?30), false),
            // Empty ranges vs empty ranges.
            (r!(=30, ?30), r!(=30, ?30), true),
            (r!(=30, ?30), r!(=40, ?40), false),
            (r!(=30, ?30), r!(?30, =30), false),
            (r!(=30, ?30), r!(=30, ?20), false),
            (r!(=30, ?20), r!(=30, ?20), false),
        ];

        for (x, y, tobe) in datas {
            let asis1 = x.touches_next(&y);
            let asis2 = y.touches_prev(&x);
            assert_eq!(asis1, tobe);
            assert_eq!(asis2, tobe);
        }
    }

    fn when_unmixable() {
        for (x, y) in ts::range_pairs_unmixable() {
            let result1 = x.touches_next(&y);
            let result2 = x.touches_prev(&y);
            assert!(!result1);
            assert!(!result2);
        }
    }
}

#[test]
#[rustfmt::skip]
fn rel() {
    when_normal();
    when_unmixable();

    fn when_normal() {
        let datas = [
            // Undefineds.
            (r!(=30, ?60), r!(=80, ?70), PosStyle::Step, RangeRel::Undefined),
            (r!(=60, ?30), r!(=60, ?30), PosStyle::Step, RangeRel::Undefined),
            // Equals.
            (r!(=30, ?60), r!(=30, ?60), PosStyle::Step, RangeRel::Equal),
            (r!(?30, =60), r!(?30, =60), PosStyle::Step, RangeRel::Equal),
            (r!(=30, ---), r!(=30, ---), PosStyle::Step, RangeRel::Equal),
            (r!(---, ?60), r!(---, ?60), PosStyle::Step, RangeRel::Equal),
            // Meets.
            (r!(=30, ?60), r!(=60, ?70), PosStyle::Step, RangeRel::Meets(true)),
            (r!(=30, =60), r!(?60, ?70), PosStyle::Step, RangeRel::Meets(true)),
            (r!(=30, =60), r!(=60, ?70), PosStyle::Real, RangeRel::Meets(true)),
            (r!(=30, ?60), r!(=20, ?30), PosStyle::Step, RangeRel::Meets(false)),
            (r!(?30, ?60), r!(=20, =30), PosStyle::Step, RangeRel::Meets(false)),
            (r!(=30, ?60), r!(=20, =30), PosStyle::Real, RangeRel::Meets(false)),
            // Starts.
            (r!(=30, ?60), r!(=30, ?70), PosStyle::Step, RangeRel::Starts(true)),
            (r!(?30, ?60), r!(?30, ?70), PosStyle::Step, RangeRel::Starts(true)),
            (r!(---, ?60), r!(---, ?70), PosStyle::Step, RangeRel::Starts(true)),
            (r!(=30, ?60), r!(=30, ?40), PosStyle::Step, RangeRel::Starts(false)),
            (r!(?30, ?60), r!(?30, ?40), PosStyle::Step, RangeRel::Starts(false)),
            (r!(---, ?60), r!(---, ?40), PosStyle::Step, RangeRel::Starts(false)),
            // Finishes
            (r!(=30, ?60), r!(=20, ?60), PosStyle::Step, RangeRel::Finishes(true)),
            (r!(=30, =60), r!(=20, =60), PosStyle::Step, RangeRel::Finishes(true)),
            (r!(=30, ---), r!(=20, ---), PosStyle::Step, RangeRel::Finishes(true)),
            (r!(=30, ?60), r!(=50, ?60), PosStyle::Step, RangeRel::Finishes(false)),
            (r!(=30, =60), r!(=50, =60), PosStyle::Step, RangeRel::Finishes(false)),
            (r!(=30, ---), r!(=50, ---), PosStyle::Step, RangeRel::Finishes(false)),
            // Before.
            (r!(=30, ?60), r!(=70, ?80), PosStyle::Step, RangeRel::Before(true)),
            (r!(=30, ?60), r!(?60, ?80), PosStyle::Step, RangeRel::Before(true)),
            (r!(=30, ?60), r!(=10, ?20), PosStyle::Step, RangeRel::Before(false)),
            (r!(?30, ?60), r!(=10, ?30), PosStyle::Step, RangeRel::Before(false)),
            (r!(=30, ?60), r!(=60, ?70), PosStyle::Real, RangeRel::Before(true)),
            (r!(=30, =60), r!(?60, ?70), PosStyle::Real, RangeRel::Before(true)),
            (r!(=30, ?60), r!(=20, ?30), PosStyle::Real, RangeRel::Before(false)),
            (r!(?30, ?60), r!(=20, =30), PosStyle::Real, RangeRel::Before(false)),
            // Overlaps.
            (r!(=30, ?60), r!(=50, ?70), PosStyle::Step, RangeRel::Overlaps(true)),
            (r!(=30, =60), r!(=60, ?70), PosStyle::Step, RangeRel::Overlaps(true)),
            (r!(=30, ?60), r!(=20, ?40), PosStyle::Step, RangeRel::Overlaps(false)),
            (r!(=30, ?60), r!(=20, =30), PosStyle::Step, RangeRel::Overlaps(false)),
            // During.
            (r!(=30, ?60), r!(=20, ?70), PosStyle::Step, RangeRel::During(true)),
            (r!(?30, ?60), r!(=30, =60), PosStyle::Step, RangeRel::During(true)),
            (r!(=30, ?60), r!(=40, ?50), PosStyle::Step, RangeRel::During(false)),
            (r!(=30, =60), r!(?40, ?60), PosStyle::Step, RangeRel::During(false)),
        ];

        for (x, y, ps, tobe) in datas {
            let asis1 = x.rel(&y, ps);
            let asis2 = y.rel(&x, ps).inverse();
            assert_eq!(asis1, tobe);
            assert_eq!(asis2, tobe);
        }
    }

    fn when_unmixable() {
        for (x, y) in ts::range_pairs_unmixable() {
            let result = x.rel(&y, PosStyle::Step);
            assert_eq!(result, RangeRel::Undefined);
        }
    }
}

#[test]
fn cut_xxx() {
    when_normal();
    when_cut_mode();

    fn when_normal() {
        let datas = [
            // Broken empty.
            (r!(=60, ?30), 40, (ro!(--------), ro!(--------))),
            // Cut at a position inside the range.
            (r!(=30, ?60), 40, (ro!(=30, ?40), ro!(=40, ?60))),
            // Cut at a position outside the range.
            (r!(=30, ?60), 20, (ro!(--------), ro!(=30, ?60))),
            (r!(=30, ?60), 70, (ro!(=30, ?60), ro!(--------))),
            // Cut at the edge of range.
            (r!(=30, ?60), 30, (ro!(--------), ro!(=30, ?60))),
            (r!(=30, ?60), 60, (ro!(=30, ?60), ro!(--------))),
        ];

        for (target, pos, tobe) in datas {
            let asis1 = target.cut(&pos);
            let asis2 = target.cut_adv(&pos, CutMode::FallbackFw);
            assert_eq!(asis1, tobe);
            assert_eq!(asis2, tobe);
        }
    }

    fn when_cut_mode() {
        #[rustfmt::skip]
        let datas = [
            // Cut at a position inside the range.
            (r!(---, ---), 40, CutMode::FallbackFw, (ro!(---, ?40), ro!(=40, ---))),
            (r!(---, ---), 40, CutMode::FallbackBw, (ro!(---, =40), ro!(?40, ---))),
            (r!(---, ---), 40, CutMode::FallbackIn, (ro!(---, =40), ro!(=40, ---))),
            (r!(---, ---), 40, CutMode::FallbackEx, (ro!(---, ?40), ro!(?40, ---))),
            (r!(---, ---), 40, CutMode::AlwaysFw,   (ro!(---, ?40), ro!(=40, ---))),
            (r!(---, ---), 40, CutMode::AlwaysBw,   (ro!(---, =40), ro!(?40, ---))),
            (r!(---, ---), 40, CutMode::AlwaysIn,   (ro!(---, =40), ro!(=40, ---))),
            (r!(---, ---), 40, CutMode::AlwaysEx,   (ro!(---, ?40), ro!(?40, ---))),
            (r!(=30, ?60), 40, CutMode::FallbackFw, (ro!(=30, ?40), ro!(=40, ?60))),
            (r!(=30, ?60), 40, CutMode::FallbackBw, (ro!(=30, ?40), ro!(=40, ?60))),
            (r!(=30, ?60), 40, CutMode::FallbackIn, (ro!(=30, ?40), ro!(=40, ?60))),
            (r!(=30, ?60), 40, CutMode::FallbackEx, (ro!(=30, ?40), ro!(=40, ?60))),
            (r!(=30, ?60), 40, CutMode::AlwaysFw,   (ro!(=30, ?40), ro!(=40, ?60))),
            (r!(=30, ?60), 40, CutMode::AlwaysBw,   (ro!(=30, =40), ro!(?40, ?60))),
            (r!(=30, ?60), 40, CutMode::AlwaysIn,   (ro!(=30, =40), ro!(=40, ?60))),
            (r!(=30, ?60), 40, CutMode::AlwaysEx,   (ro!(=30, ?40), ro!(?40, ?60))),
            // Cut at the edge of range.
            (r!(=30, ?60), 30, CutMode::FallbackFw, (ro!(--------), ro!(=30, ?60))),
            (r!(=30, ?60), 30, CutMode::FallbackBw, (ro!(--------), ro!(=30, ?60))),
            (r!(=30, ?60), 30, CutMode::FallbackIn, (ro!(--------), ro!(=30, ?60))),
            (r!(=30, ?60), 30, CutMode::FallbackBw, (ro!(--------), ro!(=30, ?60))),
            (r!(=30, ?60), 30, CutMode::AlwaysFw,   (ro!(--------), ro!(=30, ?60))),
            (r!(=30, ?60), 30, CutMode::AlwaysBw,   (ro!(--------), ro!(=30, ?60))),
            (r!(=30, ?60), 30, CutMode::AlwaysIn,   (ro!(--------), ro!(=30, ?60))),
            (r!(=30, ?60), 30, CutMode::AlwaysEx,   (ro!(--------), ro!(=30, ?60))),
            (r!(=30, ?60), 60, CutMode::FallbackFw, (ro!(=30, ?60), ro!(--------))),
            (r!(=30, ?60), 60, CutMode::FallbackBw, (ro!(=30, ?60), ro!(--------))),
            (r!(=30, ?60), 60, CutMode::FallbackIn, (ro!(=30, ?60), ro!(--------))),
            (r!(=30, ?60), 60, CutMode::FallbackEx, (ro!(=30, ?60), ro!(--------))),
            (r!(=30, ?60), 60, CutMode::AlwaysFw,   (ro!(=30, ?60), ro!(--------))),
            (r!(=30, ?60), 60, CutMode::AlwaysBw,   (ro!(=30, ?60), ro!(--------))),
            (r!(=30, ?60), 60, CutMode::AlwaysIn,   (ro!(=30, ?60), ro!(--------))),
            (r!(=30, ?60), 60, CutMode::AlwaysEx,   (ro!(=30, ?60), ro!(--------))),
        ];

        for (target, pos, mode, tobe) in datas {
            let asis2 = target.cut_adv(&pos, mode);
            assert_eq!(asis2, tobe);
        }
    }
}

#[test]
fn interval_xxx() {
    let datas = [
        // With full range.
        (r!(---, ---), r!(---, ---), ro!(--------)),
        (r!(---, ---), r!(=30, ---), ro!(--------)),
        (r!(---, ---), r!(---, ?60), ro!(--------)),
        (r!(---, ---), r!(=30, ?60), ro!(--------)),
        (r!(---, ---), r!(=60, ?30), ro!(--------)),
        // Half range vs normal range.
        (r!(=30, ---), r!(=00, ?20), ro!(=20, ?30)),
        (r!(=30, ---), r!(=10, ?30), ro!(=30, ?30)),
        (r!(=30, ---), r!(=20, ?40), ro!(--------)),
        (r!(?30, ---), r!(=00, ?20), ro!(=20, =30)),
        (r!(?30, ---), r!(=10, ?30), ro!(=30, =30)),
        (r!(?30, ---), r!(=20, ?40), ro!(--------)),
        (r!(---, ?60), r!(=50, ?70), ro!(--------)),
        (r!(---, ?60), r!(=60, ?80), ro!(=60, ?60)),
        (r!(---, ?60), r!(=70, ?90), ro!(=60, ?70)),
        (r!(---, =60), r!(=50, ?70), ro!(--------)),
        (r!(---, =60), r!(=60, ?80), ro!(--------)),
        (r!(---, =60), r!(=70, ?90), ro!(?60, ?70)),
        // Normal range vs normal range.
        (r!(=30, ?60), r!(=00, ?20), ro!(=20, ?30)),
        (r!(=30, ?60), r!(=10, ?30), ro!(=30, ?30)),
        (r!(=30, ?60), r!(=20, ?40), ro!(--------)),
        (r!(=30, ?60), r!(=50, ?70), ro!(--------)),
        (r!(=30, ?60), r!(=60, ?80), ro!(=60, ?60)),
        (r!(=30, ?60), r!(=70, ?90), ro!(=60, ?70)),
        // Normal range vs subtle range.
        (r!(=30, ?60), r!(?00, =20), ro!(?20, ?30)),
        (r!(=30, ?60), r!(?10, =30), ro!(--------)),
        (r!(=30, ?60), r!(?20, =40), ro!(--------)),
        (r!(=30, ?60), r!(?50, =70), ro!(--------)),
        (r!(=30, ?60), r!(?60, =80), ro!(=60, =60)),
        (r!(=30, ?60), r!(?70, =90), ro!(=60, =70)),
        // Subtle range vs subtle range.
        (r!(?30, =60), r!(?00, =20), ro!(?20, =30)),
        (r!(?30, =60), r!(?10, =30), ro!(?30, =30)),
        (r!(?30, =60), r!(?20, =40), ro!(--------)),
        (r!(?30, =60), r!(?50, =70), ro!(--------)),
        (r!(?30, =60), r!(?60, =80), ro!(?60, =60)),
        (r!(?30, =60), r!(?70, =90), ro!(?60, =70)),
    ];

    for (target, other, tobe) in datas {
        let asis1 = target.interval(&other);
        let asis2 = target.interval_adv(&other, CursorMode::Off);
        let asis3 = target.interval_adv(&other, CursorMode::On);
        let asis1_rev = other.interval(&target);
        let asis2_rev = other.interval_adv(&target, CursorMode::Off);
        let asis3_rev = other.interval_adv(&target, CursorMode::On);
        assert_eq!(asis1, tobe.map(|x| x.into_option()).flatten());
        assert_eq!(asis2, tobe.map(|x| x.into_option()).flatten());
        assert_eq!(asis3, tobe);
        assert_eq!(asis1, asis1_rev);
        assert_eq!(asis2, asis2_rev);
        assert_eq!(asis3, asis3_rev);
    }
}

#[test]
fn prod() {
    let datas = [
        // Full ranges vs others.
        (r!(---, ---), r!(---, ---), ro!(---, ---)),
        (r!(---, ---), r!(=30, ---), ro!(=30, ---)),
        (r!(---, ---), r!(---, ?60), ro!(---, ?60)),
        (r!(---, ---), r!(?30, ---), ro!(?30, ---)),
        (r!(---, ---), r!(---, =60), ro!(---, =60)),
        (r!(---, ---), r!(=30, ?60), ro!(=30, ?60)),
        (r!(---, ---), r!(=30, ?30), ro!(=30, ?30)),
        // Included start ranges vs others.
        (r!(=30, ---), r!(=20, ---), ro!(=30, ---)),
        (r!(=30, ---), r!(=30, ---), ro!(=30, ---)),
        (r!(=30, ---), r!(=40, ---), ro!(=40, ---)),
        (r!(=30, ---), r!(---, ?20), ro!(--------)),
        (r!(=30, ---), r!(---, ?30), ro!(--------)),
        (r!(=30, ---), r!(---, ?40), ro!(=30, ?40)),
        (r!(=30, ---), r!(?20, ---), ro!(=30, ---)),
        (r!(=30, ---), r!(?30, ---), ro!(?30, ---)),
        (r!(=30, ---), r!(?40, ---), ro!(?40, ---)),
        (r!(=30, ---), r!(---, =20), ro!(--------)),
        (r!(=30, ---), r!(---, =30), ro!(=30, =30)),
        (r!(=30, ---), r!(---, =40), ro!(=30, =40)),
        (r!(=30, ---), r!(=20, ?60), ro!(=30, ?60)),
        (r!(=30, ---), r!(=30, ?60), ro!(=30, ?60)),
        (r!(=30, ---), r!(=20, ?20), ro!(--------)),
        (r!(=30, ---), r!(=30, ?30), ro!(=30, ?30)),
        (r!(=30, ---), r!(=60, ?60), ro!(=60, ?60)),
        // Excluded end ranges vs others.
        (r!(---, ?60), r!(---, ?50), ro!(---, ?50)),
        (r!(---, ?60), r!(---, ?60), ro!(---, ?60)),
        (r!(---, ?60), r!(---, ?70), ro!(---, ?60)),
        (r!(---, ?60), r!(?50, ---), ro!(?50, ?60)),
        (r!(---, ?60), r!(?60, ---), ro!(--------)),
        (r!(---, ?60), r!(?70, ---), ro!(--------)),
        (r!(---, ?60), r!(---, =50), ro!(---, =50)),
        (r!(---, ?60), r!(---, =60), ro!(---, ?60)),
        (r!(---, ?60), r!(---, =70), ro!(---, ?60)),
        (r!(---, ?60), r!(=30, ?50), ro!(=30, ?50)),
        (r!(---, ?60), r!(=30, ?60), ro!(=30, ?60)),
        (r!(---, ?60), r!(=40, ?70), ro!(=40, ?60)),
        (r!(---, ?60), r!(=60, ?70), ro!(--------)),
        (r!(---, ?60), r!(=50, ?50), ro!(=50, ?50)),
        (r!(---, ?60), r!(=60, ?60), ro!(--------)),
        (r!(---, ?60), r!(=70, ?70), ro!(--------)),
        // Excluded start ranges vs others.
        (r!(?30, ---), r!(?20, ---), ro!(?30, ---)),
        (r!(?30, ---), r!(?30, ---), ro!(?30, ---)),
        (r!(?30, ---), r!(?40, ---), ro!(?40, ---)),
        (r!(?30, ---), r!(---, =20), ro!(--------)),
        (r!(?30, ---), r!(---, =30), ro!(--------)),
        (r!(?30, ---), r!(---, =40), ro!(?30, =40)),
        (r!(?30, ---), r!(=20, ?60), ro!(?30, ?60)),
        (r!(?30, ---), r!(=30, ?60), ro!(?30, ?60)),
        (r!(?30, ---), r!(=40, ?60), ro!(=40, ?60)),
        (r!(?30, ---), r!(=20, ?20), ro!(--------)),
        (r!(?30, ---), r!(=30, ?30), ro!(--------)),
        (r!(?30, ---), r!(=40, ?40), ro!(=40, ?40)),
        // Included end ranges vs others.
        (r!(---, =60), r!(---, =50), ro!(---, =50)),
        (r!(---, =60), r!(---, =60), ro!(---, =60)),
        (r!(---, =60), r!(---, =70), ro!(---, =60)),
        (r!(---, =60), r!(=30, ?50), ro!(=30, ?50)),
        (r!(---, =60), r!(=30, ?60), ro!(=30, ?60)),
        (r!(---, =60), r!(=30, ?70), ro!(=30, =60)),
        (r!(---, =60), r!(=60, ?70), ro!(=60, =60)),
        (r!(---, =60), r!(=70, ?80), ro!(--------)),
        (r!(---, ?60), r!(=50, ?50), ro!(=50, ?50)),
        (r!(---, ?60), r!(=60, ?60), ro!(--------)),
        (r!(---, ?70), r!(=70, ?70), ro!(--------)),
        // Normal ranges vs others.
        (r!(=30, ?60), r!(=00, ?30), ro!(--------)),
        (r!(=30, ?60), r!(=40, ?50), ro!(=40, ?50)),
        (r!(=30, ?60), r!(=00, ?70), ro!(=30, ?60)),
        (r!(=30, =60), r!(?30, ?60), ro!(?30, ?60)),
        (r!(=30, ?60), r!(=20, ?20), ro!(--------)),
        (r!(=30, ?60), r!(=30, ?30), ro!(=30, ?30)),
        (r!(=30, ?60), r!(=60, ?60), ro!(--------)),
        (r!(=30, ?60), r!(=70, ?70), ro!(--------)),
        // Empty ranges vs empty ranges.
        (r!(=30, ?30), r!(=30, ?30), ro!(=30, ?30)),
        (r!(=30, ?30), r!(?30, =30), ro!(=30, ?30)),
        (r!(?30, =30), r!(?30, =30), ro!(?30, =30)),
        (r!(=30, ?30), r!(=40, ?40), ro!(--------)),
    ];

    for (x, y, tobe) in datas {
        let asis_1 = x.prod(&y);
        let asis_2 = y.prod(&x);
        assert_eq!(asis_1, tobe);
        assert_eq!(asis_2, tobe);
    }
}

#[test]
fn enwrap() {
    let datas = [
        // Full ranges vs others.
        (r!(---, ---), r!(---, ---), ro!(---, ---)),
        (r!(---, ---), r!(=30, ---), ro!(---, ---)),
        (r!(---, ---), r!(---, ?60), ro!(---, ---)),
        (r!(---, ---), r!(?30, ---), ro!(---, ---)),
        (r!(---, ---), r!(---, =60), ro!(---, ---)),
        (r!(---, ---), r!(=30, ?60), ro!(---, ---)),
        // Included start ranges vs others.
        (r!(=30, ---), r!(=20, ---), ro!(=20, ---)),
        (r!(=30, ---), r!(=30, ---), ro!(=30, ---)),
        (r!(=30, ---), r!(=40, ---), ro!(=30, ---)),
        (r!(=30, ---), r!(---, ?20), ro!(---, ---)),
        (r!(=30, ---), r!(---, ?40), ro!(---, ---)),
        (r!(=30, ---), r!(?20, ---), ro!(?20, ---)),
        (r!(=30, ---), r!(?30, ---), ro!(=30, ---)),
        (r!(=30, ---), r!(?40, ---), ro!(=30, ---)),
        (r!(=30, ---), r!(---, =20), ro!(---, ---)),
        (r!(=30, ---), r!(---, =40), ro!(---, ---)),
        (r!(=30, ---), r!(=10, ?20), ro!(=10, ---)),
        (r!(=30, ---), r!(=30, ?40), ro!(=30, ---)),
        (r!(=30, ---), r!(=40, ?60), ro!(=30, ---)),
        // // Excluded end ranges vs others.
        (r!(---, ?60), r!(---, ?50), ro!(---, ?60)),
        (r!(---, ?60), r!(---, ?70), ro!(---, ?70)),
        (r!(---, ?60), r!(?50, ---), ro!(---, ---)),
        (r!(---, ?60), r!(?70, ---), ro!(---, ---)),
        (r!(---, ?60), r!(---, =50), ro!(---, ?60)),
        (r!(---, ?60), r!(---, =60), ro!(---, =60)),
        (r!(---, ?60), r!(---, =70), ro!(---, =70)),
        (r!(---, ?60), r!(=30, ?50), ro!(---, ?60)),
        (r!(---, ?60), r!(=30, ?60), ro!(---, ?60)),
        (r!(---, ?60), r!(=40, ?70), ro!(---, ?70)),
        (r!(---, ?60), r!(=60, ?70), ro!(---, ?70)),
        // // Excluded start ranges vs others.
        (r!(?30, ---), r!(?20, ---), ro!(?20, ---)),
        (r!(?30, ---), r!(?30, ---), ro!(?30, ---)),
        (r!(?30, ---), r!(?40, ---), ro!(?30, ---)),
        (r!(?30, ---), r!(---, =20), ro!(---, ---)),
        (r!(?30, ---), r!(---, =40), ro!(---, ---)),
        (r!(?30, ---), r!(=10, ?20), ro!(=10, ---)),
        (r!(?30, ---), r!(=20, ?30), ro!(=20, ---)),
        (r!(?30, ---), r!(=30, ?40), ro!(=30, ---)),
        (r!(?30, ---), r!(=40, ?50), ro!(?30, ---)),
        // Included end ranges vs others.
        (r!(---, =60), r!(---, =50), ro!(---, =60)),
        (r!(---, =60), r!(---, =60), ro!(---, =60)),
        (r!(---, =60), r!(---, =70), ro!(---, =70)),
        (r!(---, =60), r!(=30, ?50), ro!(---, =60)),
        (r!(---, =60), r!(=30, ?60), ro!(---, =60)),
        (r!(---, =60), r!(=30, ?70), ro!(---, ?70)),
        (r!(---, =60), r!(=70, ?80), ro!(---, ?80)),
        // Normal ranges vs others.
        (r!(=30, ?60), r!(=10, ?20), ro!(=10, ?60)),
        (r!(=30, ?60), r!(=40, ?50), ro!(=30, ?60)),
        (r!(=30, ?60), r!(=20, ?70), ro!(=20, ?70)),
        (r!(=30, =60), r!(?30, ?60), ro!(=30, =60)),
        // Empty ranges vs others.
        (r!(=20, ?20), r!(=30, ?60), ro!(=30, ?60)),
        (r!(=30, ?30), r!(=30, ?60), ro!(=30, ?60)),
        (r!(=40, ?40), r!(=30, ?60), ro!(=30, ?60)),
        (r!(=60, ?30), r!(=30, ?60), ro!(=30, ?60)),
        // Empty ranges vs empty ranges.
        (r!(=30, ?30), r!(=30, ?30), ro!(--------)),
        (r!(=30, ?30), r!(=40, ?40), ro!(--------)),
        (r!(=30, ?30), r!(=60, ?30), ro!(--------)),
        (r!(=60, ?30), r!(=70, ?40), ro!(--------)),
    ];

    for (x, y, tobe) in datas {
        let asis_1 = x.enwrap(&y);
        let asis_2 = y.enwrap(&x);
        assert_eq!(asis_1, tobe);
        assert_eq!(asis_2, tobe);
    }
}

#[test]
fn union() {
    let datas = [
        // Full ranges vs others
        (r!(---, ---), r!(---, ---), (r!(---, ---), ro!(--------))),
        (r!(---, ---), r!(=30, ---), (r!(---, ---), ro!(--------))),
        (r!(---, ---), r!(---, ?60), (r!(---, ---), ro!(--------))),
        (r!(---, ---), r!(?30, ---), (r!(---, ---), ro!(--------))),
        (r!(---, ---), r!(---, =60), (r!(---, ---), ro!(--------))),
        (r!(---, ---), r!(=00, ?00), (r!(---, ---), ro!(--------))),
        (r!(---, ---), r!(=30, ?60), (r!(---, ---), ro!(--------))),
        // Included start ranges vs others.
        (r!(=30, ---), r!(=20, ---), (r!(=20, ---), ro!(--------))),
        (r!(=30, ---), r!(---, ?20), (r!(---, ?20), ro!(=30, ---))),
        (r!(=30, ---), r!(---, ?60), (r!(---, ---), ro!(--------))),
        (r!(=30, ---), r!(?20, ---), (r!(?20, ---), ro!(--------))),
        (r!(=30, ---), r!(?30, ---), (r!(=30, ---), ro!(--------))),
        (r!(=30, ---), r!(?40, ---), (r!(=30, ---), ro!(--------))),
        (r!(=30, ---), r!(---, =20), (r!(---, =20), ro!(=30, ---))),
        (r!(=30, ---), r!(---, =29), (r!(---, =29), ro!(=30, ---))),
        (r!(=30, ---), r!(---, =30), (r!(---, ---), ro!(--------))),
        (r!(=30, ---), r!(=00, ?20), (r!(=00, ?20), ro!(=30, ---))),
        (r!(=30, ---), r!(=20, ?30), (r!(=20, ---), ro!(--------))),
        (r!(=30, ---), r!(=40, ?60), (r!(=30, ---), ro!(--------))),
        (r!(=30, ---), r!(=20, ?20), (r!(=20, ?20), ro!(=30, ---))),
        (r!(=30, ---), r!(=30, ?30), (r!(=30, ---), ro!(--------))),
        (r!(=30, ---), r!(=40, ?40), (r!(=30, ---), ro!(--------))),
        (r!(=30, ---), r!(=60, ?30), (r!(=30, ---), ro!(=60, ?30))),
        // Excluded end ranges vs others.
        (r!(---, ?60), r!(---, ?50), (r!(---, ?60), ro!(--------))),
        (r!(---, ?60), r!(---, ?60), (r!(---, ?60), ro!(--------))),
        (r!(---, ?60), r!(---, ?70), (r!(---, ?70), ro!(--------))),
        (r!(---, ?60), r!(?50, ---), (r!(---, ---), ro!(--------))),
        (r!(---, ?60), r!(?60, ---), (r!(---, ?60), ro!(?60, ---))),
        (r!(---, ?60), r!(---, =50), (r!(---, ?60), ro!(--------))),
        (r!(---, ?60), r!(---, =60), (r!(---, =60), ro!(--------))),
        (r!(---, ?60), r!(---, =70), (r!(---, =70), ro!(--------))),
        (r!(---, ?60), r!(=20, ?50), (r!(---, ?60), ro!(--------))),
        (r!(---, ?60), r!(=40, ?60), (r!(---, ?60), ro!(--------))),
        (r!(---, ?60), r!(=40, ?70), (r!(---, ?70), ro!(--------))),
        (r!(---, ?60), r!(=60, ?80), (r!(---, ?80), ro!(--------))),
        (r!(---, ?60), r!(=70, ?80), (r!(---, ?60), ro!(=70, ?80))),
        (r!(---, ?60), r!(=50, ?50), (r!(---, ?60), ro!(--------))),
        // Excluded start ranges vs others.
        (r!(?30, ---), r!(?20, ---), (r!(?20, ---), ro!(--------))),
        (r!(?30, ---), r!(?30, ---), (r!(?30, ---), ro!(--------))),
        (r!(?30, ---), r!(?40, ---), (r!(?30, ---), ro!(--------))),
        (r!(?30, ---), r!(---, =20), (r!(---, =20), ro!(?30, ---))),
        (r!(?30, ---), r!(---, =29), (r!(---, =29), ro!(?30, ---))),
        (r!(?30, ---), r!(---, =30), (r!(---, ---), ro!(--------))),
        (r!(?30, ---), r!(=00, ?00), (r!(=00, ?00), ro!(?30, ---))),
        (r!(?30, ---), r!(=00, ?20), (r!(=00, ?20), ro!(?30, ---))),
        (r!(?30, ---), r!(=20, ?60), (r!(=20, ---), ro!(--------))),
        (r!(?30, ---), r!(=40, ?60), (r!(?30, ---), ro!(--------))),
        // Included end ranges vs others.
        (r!(---, =60), r!(---, =50), (r!(---, =60), ro!(--------))),
        (r!(---, =60), r!(---, =60), (r!(---, =60), ro!(--------))),
        (r!(---, =60), r!(---, =70), (r!(---, =70), ro!(--------))),
        (r!(---, =60), r!(=20, ?50), (r!(---, =60), ro!(--------))),
        (r!(---, =60), r!(=40, ?60), (r!(---, =60), ro!(--------))),
        (r!(---, =60), r!(=40, ?70), (r!(---, ?70), ro!(--------))),
        (r!(---, =60), r!(=60, ?70), (r!(---, ?70), ro!(--------))),
        (r!(---, =60), r!(=70, ?80), (r!(---, =60), ro!(=70, ?80))),
        // Normal ranges vs others.
        (r!(=30, ?60), r!(=00, ?00), (r!(=00, ?00), ro!(=30, ?60))),
        (r!(=30, ?60), r!(=00, ?20), (r!(=00, ?20), ro!(=30, ?60))),
        (r!(=30, ?60), r!(=00, ?29), (r!(=00, ?29), ro!(=30, ?60))),
        (r!(=30, ?60), r!(=00, ?30), (r!(=00, ?60), ro!(--------))),
        (r!(=30, ?60), r!(=20, ?40), (r!(=20, ?60), ro!(--------))),
        (r!(=30, ?60), r!(=40, ?50), (r!(=30, ?60), ro!(--------))),
        (r!(=30, ?60), r!(=40, ?60), (r!(=30, ?60), ro!(--------))),
        (r!(=30, ?60), r!(=50, ?70), (r!(=30, ?70), ro!(--------))),
        (r!(=30, ?60), r!(=70, ?80), (r!(=30, ?60), ro!(=70, ?80))),
        (r!(=30, ?60), r!(=00, ?80), (r!(=00, ?80), ro!(--------))),
        // Empty ranges vs others.
        (r!(=20, ?20), r!(=30, ?60), (r!(=20, ?20), ro!(=30, ?60))),
        // Empty ranges vs empty ranges.
        (r!(=30, ?30), r!(=30, ?30), (r!(=30, ?30), ro!(--------))),
        (r!(=30, ?30), r!(?30, =30), (r!(=30, ?30), ro!(--------))),
        (r!(?30, =30), r!(=30, ?30), (r!(=30, ?30), ro!(--------))),
        (r!(?30, =30), r!(?30, =30), (r!(?30, =30), ro!(--------))),
    ];

    for (x, y, tobe) in datas {
        let asis = x.union(&y);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn diff_xxx() {
    when_normal();
    when_cursor_mode_off();
    when_cursor_mode_on();

    fn when_normal() {
        let datas = [
            // Full range vs others.
            (r!(---, ---), r!(---, ---), (ro!(--------), ro!(--------))),
            (r!(---, ---), r!(=30, ---), (ro!(---, ?30), ro!(--------))),
            (r!(---, ---), r!(---, ?60), (ro!(--------), ro!(=60, ---))),
            (r!(---, ---), r!(?30, ---), (ro!(---, =30), ro!(--------))),
            (r!(---, ---), r!(---, =60), (ro!(--------), ro!(?60, ---))),
            (r!(---, ---), r!(=30, ?60), (ro!(---, ?30), ro!(=60, ---))),
            // Included start ranges vs others.
            (r!(=30, ---), r!(=20, ---), (ro!(--------), ro!(--------))),
            (r!(=30, ---), r!(=30, ---), (ro!(--------), ro!(--------))),
            (r!(=30, ---), r!(=40, ---), (ro!(=30, ?40), ro!(--------))),
            (r!(=30, ---), r!(---, ?20), (ro!(--------), ro!(=30, ---))),
            (r!(=30, ---), r!(---, ?30), (ro!(--------), ro!(=30, ---))),
            (r!(=30, ---), r!(---, ?40), (ro!(--------), ro!(=40, ---))),
            (r!(=30, ---), r!(?20, ---), (ro!(--------), ro!(--------))),
            (r!(=30, ---), r!(?30, ---), (ro!(=30, =30), ro!(--------))),
            (r!(=30, ---), r!(?40, ---), (ro!(=30, =40), ro!(--------))),
            (r!(=30, ---), r!(---, =20), (ro!(--------), ro!(=30, ---))),
            (r!(=30, ---), r!(---, =30), (ro!(--------), ro!(?30, ---))),
            (r!(=30, ---), r!(---, =40), (ro!(--------), ro!(?40, ---))),
            (r!(=30, ---), r!(=00, ?20), (ro!(--------), ro!(=30, ---))),
            (r!(=30, ---), r!(=00, ?30), (ro!(--------), ro!(=30, ---))),
            (r!(=30, ---), r!(=00, ?40), (ro!(--------), ro!(=40, ---))),
            (r!(=30, ---), r!(=30, ?50), (ro!(--------), ro!(=50, ---))),
            (r!(=30, ---), r!(=40, ?60), (ro!(=30, ?40), ro!(=60, ---))),
            // Excluded end ranges vs others.
            (r!(---, ?60), r!(---, ?50), (ro!(--------), ro!(=50, ?60))),
            (r!(---, ?60), r!(---, ?60), (ro!(--------), ro!(--------))),
            (r!(---, ?60), r!(---, ?70), (ro!(--------), ro!(--------))),
            (r!(---, ?60), r!(?50, ---), (ro!(---, =50), ro!(--------))),
            (r!(---, ?60), r!(?60, ---), (ro!(---, ?60), ro!(--------))),
            (r!(---, ?60), r!(?70, ---), (ro!(---, ?60), ro!(--------))),
            (r!(---, ?60), r!(---, =50), (ro!(--------), ro!(?50, ?60))),
            (r!(---, ?60), r!(---, =60), (ro!(--------), ro!(--------))),
            (r!(---, ?60), r!(---, =70), (ro!(--------), ro!(--------))),
            (r!(---, ?60), r!(=30, ?50), (ro!(---, ?30), ro!(=50, ?60))),
            (r!(---, ?60), r!(=30, ?60), (ro!(---, ?30), ro!(--------))),
            (r!(---, ?60), r!(=30, ?70), (ro!(---, ?30), ro!(--------))),
            (r!(---, ?60), r!(=60, ?70), (ro!(---, ?60), ro!(--------))),
            // Excluded start ranges vs others.
            (r!(?30, ---), r!(?20, ---), (ro!(--------), ro!(--------))),
            (r!(?30, ---), r!(?30, ---), (ro!(--------), ro!(--------))),
            (r!(?30, ---), r!(?40, ---), (ro!(?30, =40), ro!(--------))),
            (r!(?30, ---), r!(---, =20), (ro!(--------), ro!(?30, ---))),
            (r!(?30, ---), r!(---, =30), (ro!(--------), ro!(?30, ---))),
            (r!(?30, ---), r!(---, =40), (ro!(--------), ro!(?40, ---))),
            (r!(?30, ---), r!(=00, ?20), (ro!(--------), ro!(?30, ---))),
            (r!(?30, ---), r!(=00, ?30), (ro!(--------), ro!(?30, ---))),
            (r!(?30, ---), r!(=00, ?40), (ro!(--------), ro!(=40, ---))),
            (r!(?30, ---), r!(=30, ?40), (ro!(--------), ro!(=40, ---))),
            (r!(?30, ---), r!(=40, ?60), (ro!(?30, ?40), ro!(=60, ---))),
            // Included end ranges vs others.
            (r!(---, =60), r!(---, =50), (ro!(--------), ro!(?50, =60))),
            (r!(---, =60), r!(---, =60), (ro!(--------), ro!(--------))),
            (r!(---, =60), r!(---, =70), (ro!(--------), ro!(--------))),
            (r!(---, =60), r!(=30, ?50), (ro!(---, ?30), ro!(=50, =60))),
            (r!(---, =60), r!(=30, ?60), (ro!(---, ?30), ro!(=60, =60))),
            (r!(---, =60), r!(=30, ?70), (ro!(---, ?30), ro!(--------))),
            (r!(---, =60), r!(=60, ?70), (ro!(---, ?60), ro!(--------))),
            // Normal ranges vs others.
            (r!(=30, ?60), r!(=00, ?30), (ro!(--------), ro!(=30, ?60))),
            (r!(=30, ?60), r!(=20, ?40), (ro!(--------), ro!(=40, ?60))),
            (r!(=30, ?60), r!(=30, ?40), (ro!(--------), ro!(=40, ?60))),
            (r!(=30, ?60), r!(=40, ?50), (ro!(=30, ?40), ro!(=50, ?60))),
            (r!(=30, ?60), r!(=50, ?60), (ro!(=30, ?50), ro!(--------))),
            (r!(=30, ?60), r!(=50, ?70), (ro!(=30, ?50), ro!(--------))),
            (r!(=30, ?60), r!(=60, ?70), (ro!(=30, ?60), ro!(--------))),
            (r!(=30, ?60), r!(=00, ?70), (ro!(--------), ro!(--------))),
            // Empty ranges vs others.
            (r!(=30, ?30), r!(=20, ?50), (ro!(--------), ro!(--------))),
            (r!(=30, ?30), r!(=30, ?50), (ro!(--------), ro!(--------))),
            (r!(=30, ?30), r!(=40, ?50), (ro!(=30, ?30), ro!(--------))),
        ];

        for (x, y, tobe) in datas {
            let asis1 = x.diff(&y);
            let asis2 = x.diff_adv(&y, CursorMode::On);
            assert_eq!(asis1, tobe);
            assert_eq!(asis2, tobe);
        }
    }

    fn when_cursor_mode_off() {
        let datas = [
            // Full ranges vs cursors.
            (r!(---, ---), r!(=30, ?30), (ro!(---, ---), ro!(--------))),
            // Included start ranges vs cursors.
            (r!(=30, ---), r!(=20, ?20), (ro!(--------), ro!(=30, ---))),
            (r!(=30, ---), r!(=30, ?30), (ro!(--------), ro!(=30, ---))),
            (r!(=30, ---), r!(=40, ?40), (ro!(=30, ---), ro!(--------))),
            // Excluded end ranges vs cursors.
            (r!(---, ?60), r!(=50, ?50), (ro!(---, ?60), ro!(--------))),
            (r!(---, ?60), r!(=60, ?60), (ro!(---, ?60), ro!(--------))),
            (r!(---, ?60), r!(=70, ?70), (ro!(---, ?60), ro!(--------))),
            // Excluded start ranges vs cursors.
            (r!(?30, ---), r!(=20, ?20), (ro!(--------), ro!(?30, ---))),
            (r!(?30, ---), r!(=30, ?30), (ro!(--------), ro!(?30, ---))),
            (r!(?30, ---), r!(=40, ?40), (ro!(?30, ---), ro!(--------))),
            // Included end ranges vs cursors.
            (r!(---, =60), r!(=50, ?50), (ro!(---, =60), ro!(--------))),
            (r!(---, =60), r!(=60, ?60), (ro!(---, =60), ro!(--------))),
            (r!(---, =60), r!(=70, ?70), (ro!(---, =60), ro!(--------))),
            // Empty ranges vs cursors.
            (r!(=30, ?30), r!(=60, ?60), (ro!(=30, ?30), ro!(--------))),
            (r!(=30, ?30), r!(=30, ?30), (ro!(--------), ro!(--------))),
            (r!(=30, ?30), r!(?30, =30), (ro!(--------), ro!(--------))),
        ];

        for (x, y, tobe) in datas {
            let asis1 = x.diff(&y);
            let asis2 = x.diff_adv(&y, CursorMode::Off);
            assert_eq!(asis1, tobe);
            assert_eq!(asis2, tobe);
        }
    }

    fn when_cursor_mode_on() {
        let datas = [
            // Full ranges vs cursors.
            (r!(---, ---), r!(=30, ?30), (ro!(---, ?30), ro!(=30, ---))),
            (r!(---, ---), r!(?30, =30), (ro!(---, =30), ro!(?30, ---))),
            // Included start ranges vs cursors.
            (r!(=30, ---), r!(=20, ?20), (ro!(--------), ro!(=30, ---))),
            (r!(=30, ---), r!(?20, =20), (ro!(--------), ro!(=30, ---))),
            (r!(=30, ---), r!(=30, ?30), (ro!(--------), ro!(=30, ---))),
            (r!(=30, ---), r!(?30, =30), (ro!(--------), ro!(=30, ---))),
            (r!(=30, ---), r!(=40, ?40), (ro!(=30, ?40), ro!(=40, ---))),
            (r!(=30, ---), r!(?40, =40), (ro!(=30, =40), ro!(?40, ---))),
            // Excluded end ranges vs cursors.
            (r!(---, ?60), r!(=50, ?50), (ro!(---, ?50), ro!(=50, ?60))),
            (r!(---, ?60), r!(?50, =50), (ro!(---, =50), ro!(?50, ?60))),
            (r!(---, ?60), r!(=60, ?60), (ro!(---, ?60), ro!(--------))),
            (r!(---, ?60), r!(?60, =60), (ro!(---, ?60), ro!(--------))),
            (r!(---, ?60), r!(=70, ?70), (ro!(---, ?60), ro!(--------))),
            (r!(---, ?60), r!(?70, =70), (ro!(---, ?60), ro!(--------))),
            // Excluded start ranges vs cursors.
            (r!(?30, ---), r!(=20, ?20), (ro!(--------), ro!(?30, ---))),
            (r!(?30, ---), r!(?20, =20), (ro!(--------), ro!(?30, ---))),
            (r!(?30, ---), r!(=30, ?30), (ro!(--------), ro!(?30, ---))),
            (r!(?30, ---), r!(?30, =30), (ro!(--------), ro!(?30, ---))),
            (r!(?30, ---), r!(=40, ?40), (ro!(?30, ?40), ro!(=40, ---))),
            (r!(?30, ---), r!(?40, =40), (ro!(?30, =40), ro!(?40, ---))),
            // Included end ranges vs cursors.
            (r!(---, =60), r!(=50, ?50), (ro!(---, ?50), ro!(=50, =60))),
            (r!(---, =60), r!(?50, =50), (ro!(---, =50), ro!(?50, =60))),
            (r!(---, =60), r!(=60, ?60), (ro!(---, =60), ro!(--------))),
            (r!(---, =60), r!(?60, =60), (ro!(---, =60), ro!(--------))),
            (r!(---, =60), r!(=70, ?70), (ro!(---, =60), ro!(--------))),
            (r!(---, =60), r!(?70, =70), (ro!(---, =60), ro!(--------))),
            // Empty ranges vs cursors.
            (r!(=30, ?30), r!(=60, ?60), (ro!(=30, ?30), ro!(--------))),
            (r!(=30, ?30), r!(?60, =60), (ro!(=30, ?30), ro!(--------))),
            (r!(=30, ?30), r!(=30, ?30), (ro!(--------), ro!(--------))),
            (r!(=30, ?30), r!(?30, =30), (ro!(--------), ro!(--------))),
        ];

        for (x, y, tobe) in datas {
            let asis = x.diff_adv(&y, CursorMode::On);
            assert_eq!(asis, tobe);
        }
    }
}
