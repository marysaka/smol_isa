use std::arch::asm;

use crate::{registers::Registers, Stack};

/// Sycall interface, "return" value will be in r0
pub fn vm_syscall(register: &mut Registers, stack: &mut Stack) {
    // x86_64 syscall table -> smol is mapping
    // argument | x64 reg | smol reg
    // ---------|---------|----------
    // ID       | rax     | r0
    // 1        | rdi     | r1
    // 2        | rsi     | r2
    // 3        | rdx     | r3
    // 4        | r10     | r4
    // 5        | r8      | r5
    // 6        | r9      | r6
    //
    // note that the ID is the number of the system call
    // the return value of the system call will be in rax (r0)
    unsafe {
        match register.r0 {
            1 => vm_syscall_write(register, stack),
            60 => vm_syscall_exit(register),
            id => panic!("System call with id: '{id}' is not implemented for x86_64 linux"),
        }
    }
}

unsafe fn vm_syscall_write(register: &mut Registers, stack: &mut Stack) {
    let mut out: i64;
    let sp = register.sp + register.r2 as u16;
    let data = stack.from_sp_mut(sp).as_mut_ptr();
    asm!(
        "mov rax, 1",
        "syscall",
        in("rdi") register.r1 as i64,
        in("rsi") data,
        in("rdx") register.r3 as i64,
        lateout("rax") out,
    );
    register.r0 = out as u8;
}

unsafe fn vm_syscall_exit(register: &mut Registers) {
    let mut out: i64;
    asm!(
        "mov rax, 60",
        "syscall",
        in("rdi") register.r1 as i64,
        lateout("rax") out,
    );
    register.r0 = out as u8;
}
