# Decisions

Design decisions to this project will be documented in this file.

## Give up maximum efficiency about shift assignment operator.

### Context 

If `R` of `RangeWrapper<R, T>` is `core::ops::RangeInclusive`,
we can't implement `<<=` and `>>=` with maximum efficiency.

Because it requires `T` to implements `SubAssign` or `AddAssign`.
And then, `SubAssign` and `AddAssign` requires access to `&mut T`. 
Here, unlike other range types, `RangeInclusive` does not have public
bounds fields. Therefore mutable references cannot be obtained from them.

### Options

- Give up for maximum efficiency
- Exclude `RangeInclusive` from the operation

### Decision

Choose 1st option. Because 2nd option would become an unexpected API
for many users. Additionally, the difference in efficiency is small.
(For simplicity, `RangeUniv<T>` is also implement in the same style.)
