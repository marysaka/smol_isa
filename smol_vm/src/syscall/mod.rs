#[cfg(unix)]
mod unix;

#[cfg(unix)]
use unix as imp;

pub use imp::vm_syscall;
