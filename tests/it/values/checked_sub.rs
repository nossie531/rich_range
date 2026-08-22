use rich_range::CheckedAdd;

#[test]
fn checked_sub() {
    when_int();
    when_float();

    fn when_int() {
        let target = &1_u8;
        let result1 = CheckedAdd::checked_add(target, &1);
        let result2 = CheckedAdd::checked_add(target, &u8::MAX);
        assert_eq!(result1, Some(2));
        assert_eq!(result2, None);
    }

    fn when_float() {
        let target = &1.0;
        let result1 = CheckedAdd::checked_add(target, &1.0);
        let result2 = CheckedAdd::checked_add(target, &f32::INFINITY);
        assert_eq!(result1, Some(2.0));
        assert_eq!(result2, Some(f32::INFINITY));
    }
}
