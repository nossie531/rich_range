# rich_range

Range calculation helper.

_The author of this crate is not good at English._  
_Forgive me if the document is hard to read._

## Examples

```rust
use rich_range::prelude::*;

let r = rw::new(30..60) & rw::new(40..70);
assert_eq!(r.0, 40..60);

let r = ru::new(30..) & ru::new(..60);
assert_eq!(r, ru::new(30..60));
```

## Highlights

This crate is designed with the following focus.

- Multiple range operations with sufficient variety.
- Abstraction and normalization of bound variants.
- Interoperability with Rust standard ranges in short code.
- Similar usage style to Rust [new range API] (nightly as of 2025).

[new range API]: https://rust-lang.github.io/rfcs/3550-new-range.html

## Core items

- [`RichRangeBounds<T>`][RichRangeBounds] -
  Rich version of [`RangeBounds<T>`][RangeBounds]
- [`RangeUniv<T>`][RangeUniv] -
  For universal bound types (from [`ru::new`][shorthands::ru::new])
- [`RangeView<'a, R, T>`][RangeView] -
  For all range type abstraction (from [`rv::new`][shorthands::rv::new])
- [`RangeWrapper<R, T>`][RangeWrapper] -
  For type keeping calculation (from [`rw::new`][shorthands::rw::new])

## Cheat sheet

Following is a rough list of items.

- **Value(s) -> Range**

  [`RangeUniv`][RangeUniv] ⋯
  [`new`][RangeUniv::new],
  [`new_broken`][RangeUniv::new_broken],
  [`new_cursor`][RangeUniv::new_cursor],
  [`new_point`][RangeUniv::new_point]

- **Range -> Boolean**

  [`is_empty`][RichRangeBounds::is_empty],
  [`is_broken`][RichRangeBounds::is_broken],
  [`is_cursor`][RichRangeBounds::is_cursor],
  [`is_cursor_fwd`][RichRangeBounds::is_cursor_fwd],
  [`is_cursor_bwd`][RichRangeBounds::is_cursor_bwd],
  [`is_point`][RichRangeBounds::is_point],
  [`is_wide`][RichRangeBounds::is_wide],
  [`is_full`][RichRangeBounds::is_full]

- **Range -> Value**

  [`head`][RichRangeBounds::head],
  [`tail`][RichRangeBounds::tail],
  [`prev`][RichRangeBounds::prev],
  [`next`][RichRangeBounds::next],
  [`cursor`][RichRangeBounds::cursor],
  [`point`][RichRangeBounds::point],
  [`len`][RichRangeBounds::len],
  [`size`][RichRangeBounds::size],
  [`width`][RichRangeBounds::width],
  [`start_edge`][RichRangeBounds::start_edge],
  [`end_edge`][RichRangeBounds::end_edge],
  [`bounds`][RichRangeBounds::bounds],
  [`edges`][RichRangeBounds::edges],
  [`iter`][RichRangeBounds::iter]

  [`RangeUniv`][RangeUniv] ⋯
  [`start`][RangeUniv::start],
  [`end`][RangeUniv::end]

- **Range -> Range(s)**

  [`as_ref`][RichRangeBounds::as_ref],
  [`cast`][RichRangeBounds::cast],
  [`try_cast`][RichRangeBounds::try_cast],
  [`to_range`][RichRangeBounds::to_range],
  [`into_option`][RichRangeBounds::into_option],
  [`flip`][RichRangeBounds::flip],
  [`flip_adv`][RichRangeBounds::flip_adv]

  [`RangeUniv`][RangeUniv] ⋯
  [`from`][RangeUniv::from],
  [`RangeXXX<T>::try_from`][RangeUniv#method.try_from]

  [`RangeView`][RangeView] ⋯
  [`new`][RangeView::new],
  [`to_univ`][RangeView::to_univ]

  [`RangeWrapper`][RangeWrapper] ⋯
  [`new`][RangeWrapper::new],
  [`from_ref`][RangeWrapper::from_ref]

- **Range + Value -> Boolean**

  [`contains`][RangeBounds::contains]

- **Range + Value -> Range(s)**

  [`add_start`][RichRangeBounds::add_start],
  [`add_end`][RichRangeBounds::add_end],
  [`sub_start`][RichRangeBounds::sub_start],
  [`sub_end`][RichRangeBounds::sub_end],
  [`calc_start`][RichRangeBounds::calc_start],
  [`calc_end`][RichRangeBounds::calc_end],
  [`align_start`][RichRangeBounds::align_start],
  [`align_end`][RichRangeBounds::align_end],
  [`map`][RichRangeBounds::map],
  [`try_map`][RichRangeBounds::try_map],
  [`checked_shl`][RichRangeBounds::checked_shl],
  [`checked_shr`][RichRangeBounds::checked_shr],
  [`checked_add_start`][RichRangeBounds::checked_add_start],
  [`checked_add_end`][RichRangeBounds::checked_add_end],
  [`checked_sub_start`][RichRangeBounds::checked_sub_start],
  [`checked_sub_end`][RichRangeBounds::checked_sub_end],
  [`checked_calc_start`][RichRangeBounds::checked_calc_start],
  [`checked_calc_end`][RichRangeBounds::checked_calc_end],
  [`checked_align_start`][RichRangeBounds::checked_align_start],
  [`checked_align_end`][RichRangeBounds::checked_align_end],
  [`cut`][RichRangeBounds::cut]

  [`RangeUniv`][RangeUniv] ⋯
  [`<<`][RangeUniv#main_shl],
  [`>>`][RangeUniv#main_shr],
  [`<<=`][RangeUniv#main_shl_assign],
  [`>>=`][RangeUniv#main_shr_assign],
  [`with_start_bound`][RangeUniv::with_start_bound],
  [`with_end_bound`][RangeUniv::with_end_bound]

  [`RangeWrapper`][RangeWrapper] ⋯
  [`<<`][RangeWrapper#main_shl],
  [`>>`][RangeWrapper#main_shr],
  [`<<=`][RangeWrapper#main_shl_assign],
  [`>>=`][RangeWrapper#main_shr_assign]

- **Range + Range -> Boolean**

  [`equiv`][RichRangeBounds::equiv],
  [`intersects`][RichRangeBounds::intersects],
  [`includes`][RichRangeBounds::includes],
  [`included`][RichRangeBounds::included],
  [`adjoins`][RichRangeBounds::adjoins],
  [`adjoins_prev`][RichRangeBounds::adjoins_prev],
  [`adjoins_next`][RichRangeBounds::adjoins_next],
  [`touches`][RichRangeBounds::touches],
  [`touches_prev`][RichRangeBounds::touches_prev],
  [`touches_next`][RichRangeBounds::touches_next]

  [`RangeUniv`][RangeUniv] ⋯
  [`<`][RangeUniv::partial_cmp],
  [`>`][RangeUniv::partial_cmp],
  [`<=`][RangeUniv::partial_cmp],
  [`>=`][RangeUniv::partial_cmp],
  [`==`][RangeUniv::eq],
  [`!=`][RangeUniv::eq]

  [`RangeWrapper`][RangeWrapper] ⋯
  [`<`][RangeWrapper::partial_cmp],
  [`>`][RangeWrapper::partial_cmp],
  [`<=`][RangeWrapper::partial_cmp],
  [`>=`][RangeWrapper::partial_cmp],
  [`==`][RangeWrapper::eq],
  [`!=`][RangeWrapper::eq]

- **Range + Range -> Value**

  [`rel`][RichRangeBounds::rel]

- **Range + Range -> Range(s)**

  [`interval`][RichRangeBounds::interval],
  [`interval_adv`][RichRangeBounds::interval_adv],
  [`prod`][RichRangeBounds::prod],
  [`enwrap`][RichRangeBounds::enwrap],
  [`union`][RichRangeBounds::union],
  [`diff`][RichRangeBounds::diff],
  [`diff_adv`][RichRangeBounds::diff_adv]

  [`RangeUniv`][RangeUniv] ⋯
  [`&`][RangeUniv#main_bitand],
  [`|`][RangeUniv#main_bitor],
  [`^`][RangeUniv#main_bitxor],
  [`&=`][RangeUniv#main_bitand_assign],
  [`|=`][RangeUniv#main_bitor_assign],
  [`^=`][RangeUniv#main_bitxor_assign]

  [`RangeWrapper`][RangeWrapper] ⋯
  [`&`][RangeWrapper#main_bitand],
  [`|`][RangeWrapper#main_bitor],
  [`^`][RangeWrapper#main_bitxor],
  [`&=`][RangeWrapper#main_bitand_assign],
  [`|=`][RangeWrapper#main_bitor_assign],
  [`^=`][RangeWrapper#main_bitxor_assign]

- **Indexing**

  [`RangeUniv`][RangeUniv] ⋯
  [`[T]::index`][RangeUniv#method.index],
  [`[T]::index_mut`][RangeUniv#method.index_mut]

## History

See [CHANGELOG](CHANGELOG.md).

<!-- links -->

[!copy_guard]: https://docs.rs/rich_range/0.1.0/
[shorthands::ru::new]: https://docs.rs/rich_range/0.1.0/rich_range/shorthands/ru/fn.new.html
[shorthands::rv::new]: https://docs.rs/rich_range/0.1.0/rich_range/shorthands/rv/fn.new.html
[shorthands::rw::new]: https://docs.rs/rich_range/0.1.0/rich_range/shorthands/rw/fn.new.html
[RangeUniv]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html
[RangeUniv::start]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#structfield.start
[RangeUniv::end]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#structfield.end
[RangeUniv::new]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.new
[RangeUniv::new_broken]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.new_broken
[RangeUniv::new_cursor]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.new_cursor
[RangeUniv::new_point]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.new_point
[RangeUniv::from]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.from
[RangeUniv#method.try_from]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.try_from
[RangeUniv::with_start_bound]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.with_start_bound
[RangeUniv::with_end_bound]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.with_end_bound
[RangeUniv#main_shl]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#main_shl
[RangeUniv#main_shr]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#main_shr
[RangeUniv#main_shl_assign]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#main_shl_assign
[RangeUniv#main_shr_assign]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#main_shr_assign
[RangeUniv::partial_cmp]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.partial_cmp
[RangeUniv::eq]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.eq
[RangeUniv#main_bitand]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#main_bitand
[RangeUniv#main_bitor]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#main_bitor
[RangeUniv#main_bitxor]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#main_bitxor
[RangeUniv#main_bitand_assign]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#main_bitand_assign
[RangeUniv#main_bitor_assign]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#main_bitor_assign
[RangeUniv#main_bitxor_assign]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#main_bitxor_assign
[RangeUniv#method.index]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.index,
[RangeUniv#method.index_mut]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.index_mut
[RangeView]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeView.html
[RangeView::new]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeView.html#method.new
[RangeView::to_univ]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeView.html#method.to_univ
[RangeWrapper]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeWrapper.html
[RangeWrapper::new]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeWrapper.html#method.RangeWrapper.html#method.new
[RangeWrapper::from_ref]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeWrapper.html#method.RangeWrapper.html#method.from_ref
[RangeWrapper#main_shl]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeWrapper.html#main_shl
[RangeWrapper#main_shr]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeWrapper.html#main_shr
[RangeWrapper#main_shl_assign]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeWrapper.html#main_shl_assign
[RangeWrapper#main_shr_assign]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeWrapper.html#main_shr_assign
[RangeWrapper::partial_cmp]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeWrapper.html#method.partial_cmp
[RangeWrapper::eq]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeWrapper.html#method.eq
[RangeWrapper#main_bitand]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeWrapper.html#main_bitand
[RangeWrapper#main_bitor]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeWrapper.html#main_bitor
[RangeWrapper#main_bitxor]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeWrapper.html#main_bitxor
[RangeWrapper#main_bitand_assign]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeWrapper.html#main_bitand_assign
[RangeWrapper#main_bitor_assign]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeWrapper.html#main_bitor_assign
[RangeWrapper#main_bitxor_assign]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeWrapper.html#main_bitxor_assign
[RangeBounds]: https://doc.rust-lang.org/std/ops/trait.RangeBounds.html
[RangeBounds::contains]: https://doc.rust-lang.org/std/ops/trait.RangeBounds.html#method.contains
[RichRangeBounds]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeGounds.html
[RichRangeBounds::is_empty]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.is_empty
[RichRangeBounds::is_broken]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.is_broken
[RichRangeBounds::is_cursor]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.is_cursor
[RichRangeBounds::is_cursor_fwd]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.is_cursor_fwd
[RichRangeBounds::is_cursor_bwd]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.is_cursor_bwd
[RichRangeBounds::is_point]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.is_point
[RichRangeBounds::is_wide]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.is_wide
[RichRangeBounds::is_full]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.is_full
[RichRangeBounds::head]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.head
[RichRangeBounds::tail]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.tail
[RichRangeBounds::prev]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.prev
[RichRangeBounds::next]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.next
[RichRangeBounds::cursor]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.cursor
[RichRangeBounds::point]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.point
[RichRangeBounds::len]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.len
[RichRangeBounds::size]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.size
[RichRangeBounds::width]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.width
[RichRangeBounds::start_edge]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.start_edge
[RichRangeBounds::end_edge]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.end_edge
[RichRangeBounds::bounds]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.bounds
[RichRangeBounds::edges]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.edges
[RichRangeBounds::iter]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.iter
[RichRangeBounds::add_start]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.add_start
[RichRangeBounds::add_end]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.add_end
[RichRangeBounds::sub_start]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.sub_start
[RichRangeBounds::sub_end]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.sub_end
[RichRangeBounds::calc_start]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.calc_start
[RichRangeBounds::calc_end]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.calc_end
[RichRangeBounds::align_start]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.align_start
[RichRangeBounds::align_end]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.align_end
[RichRangeBounds::map]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.map
[RichRangeBounds::try_map]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.try_map
[RichRangeBounds::checked_shl]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.checked_shl
[RichRangeBounds::checked_shr]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.checked_shr
[RichRangeBounds::checked_add_start]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.checked_add_start
[RichRangeBounds::checked_add_end]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.checked_add_end
[RichRangeBounds::checked_sub_start]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.checked_sub_start
[RichRangeBounds::checked_sub_end]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.checked_sub_end
[RichRangeBounds::checked_calc_start]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.checked_calc_start
[RichRangeBounds::checked_calc_end]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.checked_calc_end
[RichRangeBounds::checked_align_start]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.checked_align_start
[RichRangeBounds::checked_align_end]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.checked_align_end
[RichRangeBounds::cut]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.cut
[RichRangeBounds::as_ref]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.as_ref
[RichRangeBounds::cast]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.cast
[RichRangeBounds::try_cast]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.try_cast
[RichRangeBounds::to_range]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.to_range
[RichRangeBounds::into_option]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.into_option
[RichRangeBounds::flip]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.flip
[RichRangeBounds::flip_adv]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.flip_adv
[RichRangeBounds::equiv]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.equiv
[RichRangeBounds::intersects]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.intersects
[RichRangeBounds::includes]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.includes
[RichRangeBounds::included]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.included
[RichRangeBounds::adjoins]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.adjoins
[RichRangeBounds::adjoins_prev]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.adjoins_prev
[RichRangeBounds::adjoins_next]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.adjoins_next
[RichRangeBounds::touches]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.touches
[RichRangeBounds::touches_prev]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.touches_prev
[RichRangeBounds::touches_next]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.touches_next
[RichRangeBounds::rel]: https://docs.rs/rich_range/0.1.0/rich_range/trait.RichRangeBounds.html#method.rel
[RichRangeBounds::interval]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.interval
[RichRangeBounds::interval_adv]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.interval_adv
[RichRangeBounds::prod]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.prod
[RichRangeBounds::enwrap]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.enwrap
[RichRangeBounds::union]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.union
[RichRangeBounds::diff]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.diff
[RichRangeBounds::diff_adv]: https://docs.rs/rich_range/0.1.0/rich_range/struct.RangeUniv.html#method.diff_adv
