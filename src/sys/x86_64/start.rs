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
    "mov rdi, [rsp]",       // &argc = rsp
    "mov rsi, [rsp+8]",     // &argv = rsp+8

    "mov rdx, rdi",         // &envp = rsp+8*argc+8
    "mov rax, 8",
    "mul rdx",
    "add rdx, 8",
    "add rdx, rsp",
    "mov rdx, [rdx]",       // &envp = rsp+8*argc+8

    // The stack will be 16-byte aligned at process entry already.
    // So we're good to go!
    "call {start}",
    start = sym crate::start::start
);
