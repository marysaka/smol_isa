#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub mod x86_64_linux;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub use x86_64_linux::vm_syscall;
