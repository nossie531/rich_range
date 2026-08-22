use rich_range::*;
use test_panic::prelude::*;

const I8_OVF: usize = i8::MAX as usize + 1;
const U8_OVF: usize = u8::MAX as usize + 1;

#[test]
fn forward() {
    let datas = [
        (42_u8, 1, ok(43)),
        (u8::MIN, 1, ok(u8::MIN + 1)),
        (u8::MAX, 1, ng()),
    ];

    for (target, count, tobe) in datas {
        let asis = test_panic(|| Step::forward(target, count));
        assert_eqa!(asis, tobe);
    }
}

#[test]
fn backward() {
    let datas = [
        (42_u8, 1, ok(41)),
        (u8::MIN, 1, ng()),
        (u8::MAX, 1, ok(u8::MAX - 1)),
    ];

    for (target, count, tobe) in datas {
        let asis = test_panic(|| Step::backward(target, count));
        assert_eqa!(asis, tobe);
    }
}

#[test]
fn steps_between() {
    when_normal();
    when_large_type();

    fn when_normal() {
        let datas = [(30, 60, (30, Some(30))), (60, 30, (0, Some(0)))];

        for (start, end, tobe) in datas {
            let asis = Step::steps_between(&start, &end);
            assert_eq!(asis, tobe);
        }
    }

    fn when_large_type() {
        if size_of::<i128>() <= size_of::<usize>() {
            return;
        }

        let result = Step::steps_between(&i128::MIN, &i128::MAX);
        assert_eq!(result, (usize::MAX, None));
    }
}

#[test]
fn forward_checked() {
    when_unsinged();
    when_singed();

    fn when_unsinged() {
        let datas = [
            (0, U8_OVF, None),
            (42_u8, 1, Some(43)),
            (u8::MIN, 1, Some(u8::MIN + 1)),
            (u8::MAX, 1, None),
        ];

        for (target, count, tobe) in datas {
            let asis = Step::forward_checked(target, count);
            assert_eq!(asis, tobe);
        }
    }

    fn when_singed() {
        let datas = [
            (0, U8_OVF, None),
            (-1_i8, I8_OVF, Some((-1 + I8_OVF as isize) as i8)),
            (-42_i8, 1, Some(-41)),
            (i8::MIN, 1, Some(i8::MIN + 1)),
            (i8::MAX, 1, None),
        ];

        for (target, count, tobe) in datas {
            let asis = Step::forward_checked(target, count);
            assert_eq!(asis, tobe);
        }
    }
}

#[test]
fn backward_checked() {
    when_unsinged();
    when_singed();

    fn when_unsinged() {
        let datas = [
            (0, U8_OVF, None),
            (42_u8, 1, Some(41)),
            (u8::MIN, 1, None),
            (u8::MAX, 1, Some(u8::MAX - 1)),
        ];

        for (target, count, tobe) in datas {
            let asis = Step::backward_checked(target, count);
            assert_eq!(asis, tobe);
        }
    }

    fn when_singed() {
        let datas = [
            (0, U8_OVF, None),
            (1_i8, I8_OVF, Some((1 - I8_OVF as isize) as i8)),
            (42_i8, 1, Some(41)),
            (i8::MIN, 1, None),
            (i8::MAX, 1, Some(i8::MAX - 1)),
        ];

        for (target, count, tobe) in datas {
            let asis = Step::backward_checked(target, count);
            assert_eq!(asis, tobe);
        }
    }
}
