// mod env_vector;
// mod stream;
use crate::stream;
// hooks the syscalls
// reads the right segment from the byte stream at the correct offset 
// using prehooks for syscalls with no side effects
// handles possible side effects where needed
use libafl_qemu::{
    Qemu,
    SyscallHookResult,
};

use libafl_qemu::GuestAddr;
const SYS_UNAME: i32 = 63;
const SYS_SYSINFO: i32 = 99;
const SYS_GETPID: i32 = 39;
const SYS_GETPPID: i32 = 110;
const SYS_STAT: i32 = 262; //actually newafstat
const SYS_GETDENTS: i32 = 78;
const SYS_GETTIMEOFDAY: i32 = 96;

const SYS_ACCESS: i32 = 21;
const SYS_FSTAT: i32 = 5;
const SYS_OPENAT: i32 = 257;
const SYS_PRLIMIT64: i32 = 302;
const SYS_READ: i32 = 0;
const SYS_PREAD64: i32 = 17;

extern "C" fn pre_hooks(
    _data: u64,
    sys_num: i32,
    _a0: GuestAddr,
    _a1: GuestAddr,
    _a2: GuestAddr,
    _a3: GuestAddr,
    _a4: GuestAddr,
    _a5: GuestAddr,
    _a6: GuestAddr,
    _a7: GuestAddr,
) -> SyscallHookResult {
    
    let qemu = unsafe { libafl_qemu::Qemu::get_unchecked() };
    
    match sys_num{
         SYS_ACCESS =>  {
            let bytes = crate::stream::get_current_bytes(crate::stream::OFFSET_ACCESS, crate::stream::SIZE_ACCESS);
            let ret = i32::from_le_bytes(bytes.as_slice().try_into().unwrap());

            println!("pre hook ACCESS\n");
            println!("print the ret {} \n", ret);
            SyscallHookResult::Skip(ret as GuestAddr)
        },
        SYS_FSTAT =>  {
            // let bytes = crate::stream::get_current_bytes(crate::stream::OFFSET_FSTAT, crate::stream::SIZE_FSTAT);
            // let ret = i32::from_le_bytes(bytes.as_slice().try_into().unwrap());
            let ret = 0; // place holder

            println!("pre hook FSTAT\n");
            println!("print the ret {} \n", ret);
            SyscallHookResult::Skip(ret as GuestAddr)
        },
        SYS_PRLIMIT64 =>  {
            // let bytes = crate::stream::get_current_bytes(crate::stream::OFFSET_PRLIMIT64, crate::stream::SIZE_PRLIMIT64);
            // let ret = i32::from_le_bytes(bytes.as_slice().try_into().unwrap());
            let ret = 0; // place holder

            println!("pre hook PRLIMIT64\n");
            println!("print the ret {} \n", ret);
            SyscallHookResult::Skip(ret as GuestAddr)
        },
        SYS_PREAD64 =>  {
            // let bytes = crate::stream::get_current_bytes(crate::stream::OFFSET_PREAD64, crate::stream::SIZE_PREAD64);
            // let ret = i32::from_le_bytes(bytes.as_slice().try_into().unwrap());
            let ret = 0; // place holder

            println!("pre hook PREAD64\n");
            println!("print the ret {} \n", ret);
            SyscallHookResult::Skip(ret as GuestAddr)
        },
        _ => {
            // println!("pre hook num {}\n", sys_num);
             SyscallHookResult::Run
        }

    }
}

extern "C" fn post_hooks(
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
    
    let qemu = unsafe { libafl_qemu::Qemu::get_unchecked() };
    // let env = crate::env_vector::get_current();

    match sys_num{
       SYS_OPENAT =>  {
            // let bytes = crate::stream::get_current_bytes(crate::stream::OFFSET_OPENAT, crate::stream::SIZE_OPENAT);
            // let ret = i32::from_le_bytes(bytes.as_slice().try_into().unwrap());

            println!("post hook OPENAT\n");
            println!("print the ret {} \n", ret);
            ret
        },
        SYS_READ =>  {
            // let bytes = crate::stream::get_current_bytes(crate::stream::OFFSET_READ, crate::stream::SIZE_READ);
            // let ret = i32::from_le_bytes(bytes.as_slice().try_into().unwrap());

            println!("post hook READ\n");
            println!("print the ret {} \n", ret);
            ret
        },
        _ => ret

    }
}


pub fn init_hooks(qemu: &Qemu){
    qemu.hooks().add_pre_syscall_hook(0u64, pre_hooks);
    // qemu.hooks().add_post_syscall_hook(0u64, post_hooks);
}

// extern "C" fn hooks(
//     _data: u64,
//     ret: GuestAddr,
//     sys_num: i32,
//     _a0: GuestAddr,
//     _a1: GuestAddr,
//     _a2: GuestAddr,
//     _a3: GuestAddr,
//     _a4: GuestAddr,
//     _a5: GuestAddr,
//     _a6: GuestAddr,
//     _a7: GuestAddr,
// ) -> GuestAddr {
    
//     let qemu = unsafe { libafl_qemu::Qemu::get_unchecked() };
//     let env = crate::env_vector::get_current();

//     match sys_num{

//         SYS_UNAME =>  {
//             // println!("UNAME HOOOK!! \n");
//             qemu.write_mem(_a0, &env.uname.sysname).unwrap();
//             // println!("UNAME injected \n");
//             ret
//         },
//         SYS_GETPID => { 
//             // println!("GETPID HOOK!! \n");
//             env.pid_ret as GuestAddr
//             // ret
//         },
//         SYS_GETPPID => { 
//             // println!("GET(P)PID HOOK!! \n");
//             env.ppid_ret as GuestAddr
//             // ret
//         },
//         SYS_SYSINFO => { 
//             // println!("SYSNAME HOOK!! \n");
//             qemu.write_mem(_a0, &env.sysinfo.uptime.to_le_bytes()).unwrap();
//             // env.sysinfo.uptime as GuestAddr
//             ret
//         },
//         SYS_STAT => { 
//             // println!("SYSSTAT HOOK!! \n");
//             // println!("writing st_dev: {}", env.stat.st_dev);
//             qemu.write_mem(_a2, &env.stat.st_dev.to_le_bytes()).unwrap();
//             // env.stat.st_dev as GuestAddr
//             // println!("write done");
//             ret
//         },
//         SYS_ACCESS => { 
//             // println!("SYSACCESS HOOK!! \n");
//             env.acces_ret as GuestAddr
//             // ret
//         },
//         // SYS_GETDENTS => { 
//         //     println!("GETDENTS HOOK!! \n");
//         //     env.getdents_ret as GuestAddr
//         //     // ret
//         // },
//         SYS_GETTIMEOFDAY => { 
//             // println!("GETTIMEOFDAY HOOK!! \n");
//             // env.gettimeofday_ret.tv_sec as GuestAddr
//             qemu.write_mem(_a0, &env.gettimeofday_ret.tv_sec.to_le_bytes()).unwrap();
//             // qemu.write_mem(_a0, &env.gettimeofday_ret.tv_usec.to_le_bytes()).unwrap();
//             ret
//         },
//         _ => ret
//     }
// }