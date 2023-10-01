core::arch::global_asm!(
    ".global _start",
    "_start:",
    // Hello, World!
    // We have been started. All of this assembly sucks and we want into rustland as quickly as possible.
    // Have a bit of fun and then, run away to libuwuc::start::start.
    // First, mark the top-most stack frame as 0 to prevent sadness for frame-pointer based unwinding.
    "mov rbp, 0",
    // We're off to a good start already.
    // Pass the variables to the start function arguments.
    "mov rdi, rsp",

    // The stack will be 16-byte aligned at process entry already.
    // So we're good to go!
    "call {start}",
    // Our start never returns, but just to be sure...
    "ud2",
    start = sym crate::start::start
);
