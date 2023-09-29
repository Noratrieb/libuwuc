#[macro_export]
macro_rules! trap {
    () => {
        ::core::arch::asm!("ud2");
    };
}
pub(crate) use trap;