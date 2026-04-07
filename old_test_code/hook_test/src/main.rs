use libafl_qemu::{
    Qemu,
    //qemu::hooks::SyscallHookResult,
};

use libafl_qemu_sys::GuestAddr;
const SYS_UNAME: i32 = 63;

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
    //println!("in uname hook with num {}\n ", sys_num);
    // println!("sys_num: {} SYS_UNAME: {} equal: {} \n", sys_num, SYS_UNAME, sys_num == SYS_UNAME);

    if sys_num == SYS_UNAME {
        println!("sys_num: {} SYS_UNAME: {} equal: {} \n", sys_num, SYS_UNAME, sys_num == SYS_UNAME);
        println!("UNAME HOOOK!! \n");
        let qemu = unsafe { libafl_qemu::Qemu::get_unchecked() };

        unsafe { qemu.write_mem(_a0, b"hiitest\0").unwrap() };
        println!("Changed!! \n");
    }else{
        // println!("NOOHOOOK!! \n");
    }
    ret
}

fn main(){
    // qemu init 
    //println!("init\n ");
    let args_command: Vec<String> = std::env::args().collect();
    let qemu = Qemu::init(&args_command).expect("NOOO QEMU");
    // adding the hook 
    //println!("init DONE\n ");
    qemu.hooks().add_post_syscall_hook(0u64, uname_hook);
    unsafe { qemu.run(); };

    // running qemu 
}