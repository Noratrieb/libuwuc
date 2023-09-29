// libcore seems to require this symbol, even though it's unused.
#[no_mangle]
fn rust_eh_personality() {
    unsafe {
        crate::sys::helpers::trap!();
    }
}
