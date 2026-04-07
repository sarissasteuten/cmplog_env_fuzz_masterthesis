use libafl_qemu::{
    Qemu,
    //qemu::hooks::SyscallHookResult,
};

use libafl_qemu_sys::GuestAddr;
const SYS_uname: i32 = 63;

extern "C" fn uname_hook(
    _data: u64,
    ret: GuestAddr,
    sys_num: i32,
    _a0: GuestAddr,
    _a1: GuestAddr,
    _a2: GuestAddr,
    _a3: GuestAddr,
    _a4: GuestAddr,
    _a5: GuestAddr,
    _a6: GuestAddr,
    _a7: GuestAddr,
) -> GuestAddr {
    if sys_num == SYS_uname {
        println!("UNAME HOOOK!! ");
    }
    ret
}

fn main(){
    // qemu init 
    let args_command: Vec<String> = std::env::args().collect();
    let qemu = Qemu::init(&args_command).expect("NOOO QEMU");
    // adding the hook 
    qemu.hooks().add_post_syscall_hook(0u64, uname_hook);

    // running qemu 
}