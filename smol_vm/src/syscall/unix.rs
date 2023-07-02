use crate::{registers::Registers, Stack};

/// Sycall interface, "return" value will be in r0
pub fn vm_syscall(register: &mut Registers, stack: &mut Stack) {
    match register.r0 {
        1 => vm_syscall_write(register, stack),
        60 => vm_syscall_exit(register),
        id => panic!("System call with id: '{id}' is not implemented for linux"),
    }
}

fn vm_syscall_write(register: &mut Registers, stack: &mut Stack) {
    let sp = register.sp + register.r2 as u16;

    let fd = register.r1 as libc::c_int;
    let buf = stack.from_sp_mut(sp).as_mut_ptr() as *const libc::c_void;
    let count = register.r3 as libc::size_t;

    // SAFETY: always safe to call.
    let out = unsafe { libc::write(fd, buf, count) };

    register.r0 = out as u8;
}

fn vm_syscall_exit(register: &mut Registers) {
    let status = register.r1 as libc::c_int;

    // SAFETY: always safe to call.
    unsafe {
        libc::exit(status);
    }
}
