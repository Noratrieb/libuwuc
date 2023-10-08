use crate::utils::CStrRef;
use core::fmt::Write;

pub fn assert_failed(
    assertion: CStrRef<'_>,
    file: CStrRef<'_>,
    line: u32,
    _function: Option<CStrRef<'_>>,
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
