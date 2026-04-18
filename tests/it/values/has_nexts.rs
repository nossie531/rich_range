use rich_range::*;

#[test]
fn next() {
    with_step();
    with_float();

    fn with_step() {
        let datas = [
            (42, Some(43)),
            (i8::MIN, Some(i8::MIN + 1)),
            (i8::MAX, None),
        ];

        for (target, tobe) in datas {
            let asis = HasNexts::next(&target);
            assert_eq!(asis, tobe);
        }
    }

    fn with_float() {
        let datas = [
            (42_f32, Some((42_f32).next_up())),
            (f32::MIN, Some(f32::MIN.next_up())),
            (f32::MAX, Some(f32::MAX.next_up())),
            (f32::NEG_INFINITY, Some(f32::NEG_INFINITY.next_up())),
            (f32::INFINITY, None),
            (f32::NAN, None),
        ];

        for (target, tobe) in datas {
            let asis = HasNexts::next(&target);
            assert_eq!(asis, tobe);
        }
    }
}

#[test]
fn prev() {
    with_step();
    with_float();

    fn with_step() {
        let datas = [
            (42, Some(41)),
            (i8::MIN, None),
            (i8::MAX, Some(i8::MAX - 1)),
        ];

        for (target, tobe) in datas {
            let asis = HasNexts::prev(&target);
            assert_eq!(asis, tobe);
        }
    }

    fn with_float() {
        let datas = [
            (42_f32, Some((42_f32).next_down())),
            (f32::MIN, Some(f32::MIN.next_down())),
            (f32::MAX, Some(f32::MAX.next_down())),
            (f32::NEG_INFINITY, None),
            (f32::INFINITY, Some(f32::INFINITY.next_down())),
            (f32::NAN, None),
        ];

        for (target, tobe) in datas {
            let asis = HasNexts::prev(&target);
            assert_eq!(asis, tobe);
        }
    }
}
