use rich_range::CheckedSub;

#[test]
fn checked_add() {
    with_int();
    with_float();

    fn with_int() {
        let target = &3_u8;
        let result1 = CheckedSub::checked_sub(target, &1);
        let result2 = CheckedSub::checked_sub(target, &u8::MAX);
        assert_eq!(result1, Some(2));
        assert_eq!(result2, None);
    }

    fn with_float() {
        let target = &3.0;
        let result1 = CheckedSub::checked_sub(target, &1.0);
        let result2 = CheckedSub::checked_sub(target, &f32::INFINITY);
        assert_eq!(result1, Some(2.0));
        assert_eq!(result2, Some(f32::NEG_INFINITY));
    }
}
