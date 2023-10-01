use crate::utils::SharedThinCstr;
use core::fmt::Write;

pub fn assert_failed(
    assertion: SharedThinCstr<'_>,
    file: SharedThinCstr<'_>,
    line: u32,
    _function: Option<SharedThinCstr<'_>>,
) -> ! {
    let _ = writeln!(
        crate::io::Printer(crate::io::STDERR),
        "assertion failed: '{:?}' at {:?}:{}",
        assertion,
        file,
        line
    );
    crate::start::abort();
}
