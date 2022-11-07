//! Rust wrappers around the x86-family I/O instructions.

use core::arch::asm;

/// Read a `u8`-sized value from `port`.
pub unsafe fn inb(port: u16) -> u8 {
    // The registers for the `in` and `out` instructions are always the
    // same: `a` for value, and `d` for the port address.
    let result: u8;
    asm!(
        "inb {0:x}, {1}",
        in(reg) port,
        out(reg_byte) result,
    );
    result
}

/// Write a `u8`-sized `value` to `port`.
pub unsafe fn outb(value: u8, port: u16) {
    asm!(
        "outb {1}, {0:x}",
        in(reg) port,
        in(reg_byte) value,
    );
//    llvm_asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile");
}

/// Read a `u16`-sized value from `port`.
pub unsafe fn inw(port: u16) -> u16 {
    let result: u16;
    asm!(
        "inw {0:x}, {1:x}",
        in(reg) port,
        out(reg) result,
    );
//    llvm_asm!("inw %dx, %ax" : "={ax}"(result) : "{dx}"(port) :: "volatile");
    result
}

/// Write a `u8`-sized `value` to `port`.
pub unsafe fn outw(value: u16, port: u16) {
    asm!(
        "inw {0:x}, {1:x}",
        in(reg) port,
        in(reg) value,
    );
//    llvm_asm!("outw %ax, %dx" :: "{dx}"(port), "{ax}"(value) :: "volatile");
}

/// Read a `u32`-sized value from `port`.
pub unsafe fn inl(port: u16) -> u32 {
    let result: u32;
    asm!(
        "inw {0:x}, {1:e}",
        in(reg) port,
        out(reg) result,
    );
//    llvm_asm!("inl %dx, %eax" : "={eax}"(result) : "{dx}"(port) :: "volatile");
    result
}

/// Write a `u32`-sized `value` to `port`.
pub unsafe fn outl(value: u32, port: u16) {
    asm!(
        "out {1:e}, {0:x}",
        in(reg) port,
        in(reg) value,
    );
//    llvm_asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(value) :: "volatile");
}
