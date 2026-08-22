use std::panic::{self, PanicHookInfo};
use std::sync::LazyLock;

/// Returns `true` if profile setting `overflow_checks` is enabled.
///
/// This is substitute for nightly only feature `cfg_overflow_checks`.
///
/// https://dev-doc.rust-lang.org/nightly/unstable-book/language-features/cfg-overflow-checks.html
pub(crate) fn overflow_checks() -> bool {
    static RET: LazyLock<bool> = LazyLock::new(|| {
        #[allow(arithmetic_overflow)]
        let calc_overflow = || u8::MAX + 1;
        let default_panic_hook = panic::take_hook();
        let empty_panic_hook = Box::new(|_: &PanicHookInfo| {});

        panic::set_hook(empty_panic_hook);
        let overflow_result = panic::catch_unwind(calc_overflow);
        panic::set_hook(default_panic_hook);

        overflow_result.is_err()
    });

    *LazyLock::force(&RET)
}
