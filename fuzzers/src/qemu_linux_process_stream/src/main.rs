//! A systemmode linux kernel example
mod hooks_harness;
mod stream;
#[cfg(target_os = "linux")]
mod fuzzer;

#[cfg(target_os = "linux")]
pub fn main() {
    fuzzer::fuzz();
}

#[cfg(not(target_os = "linux"))]
pub fn main() {
    panic!("qemu and libafl_qemu is only supported on linux!");
}
