// mod env_vector;
// mod stream;
// use crate::stream;
// hooks the syscalls
// reads the right segment from the byte stream at the correct offset 
// using prehooks for syscalls with no side effects
// handles possible side effects where needed
use libafl_qemu::{
    Qemu,
    SyscallHookResult,
};

use libafl_qemu::GuestAddr;
// const SYS_UNAME: i32 = 63;
// const SYS_SYSINFO: i32 = 99;
// const SYS_GETPID: i32 = 39;
// const SYS_GETPPID: i32 = 110;
// const SYS_STAT: i32 = 262; //actually newafstat
// const SYS_GETDENTS: i32 = 78;
// const SYS_GETTIMEOFDAY: i32 = 96;

const SYS_ACCESS: i32 = 21;
const SYS_FSTAT: i32 = 5;
const SYS_OPENAT: i32 = 257;
const SYS_PRLIMIT64: i32 = 302;
const SYS_READ: i32 = 0;
const SYS_PREAD64: i32 = 17;
const SYS_EXIT_GROUP: i32 = 231;
const SYS_EXIT: i32 = 60;

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
        SYS_EXIT | SYS_EXIT_GROUP =>{
            println!("EXIT THE RUN\n\n");
            // let qemu = unsafe{
            //     libafl_qemu::Qemu::get_unchecked()
            // };

            if let Some(cpu) = qemu.current_cpu(){
                cpu.trigger_breakpoint();
            }
            // SyscallHookResult::Run
            SyscallHookResult::Skip(0)
        },
    
        SYS_ACCESS =>  {
            let bytes = crate::stream::get_current_bytes(crate::stream::OFFSET_ACCESS, crate::stream::SIZE_ACCESS);
            let ret = i32::from_le_bytes(bytes.as_slice().try_into().unwrap());

            println!("pre hook ACCESS\n");
            println!("print the ret {} \n", ret);
            SyscallHookResult::Skip(ret as GuestAddr)
        },
        //   SYS_FSTAT =>  {
        //     let bytes = crate::stream::get_current_bytes(crate::stream::OFFSET_FSTAT, crate::stream::SIZE_FSTAT);
        //     qemu.write_mem(_a1, &bytes).unwrap(); // nog toevoegen dat t na post hook alleen de value overwrite die verandert is 
        //     println!("pre hook FSTAT\n");
        //     SyscallHookResult::Skip(0)
        // },
        SYS_PRLIMIT64 =>  {
            
            if _a2 == 0 { // in dit geval wordt er gelezen van de env (en er worden dus niet gezet)
                let bytes = crate::stream::get_current_bytes(crate::stream::OFFSET_PRLIMIT64, crate::stream::SIZE_PRLIMIT64);
                qemu.write_mem(_a3, &bytes).unwrap();
                println!("pre hook PRLIMIT64\n");
                SyscallHookResult::Skip(0)
            } else { // anders worden er waarden gezet dus dan gwn laten gaaaan
                SyscallHookResult::Run
            }
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
            let bytes = crate::stream::get_current_bytes(crate::stream::OFFSET_OPENAT, crate::stream::SIZE_OPENAT);
            let stream_ret = i32::from_le_bytes(bytes.as_slice().try_into().unwrap());
            println!("post hook OPENAT\n");
            // println!("print the ret {} \n", ret);

            if stream_ret == -1 {
                stream_ret as GuestAddr
            } else{
                ret
            }
            // alleen hooken als de return value is aangepast in de stream naar -1       
            // what if it expects a file to exits that doesnt????     
        },
        SYS_READ =>  {
            let nbytes = _a2 as usize;
            let bytes = crate::stream::get_current_bytes(crate::stream::OFFSET_READ, nbytes.min(crate::stream::SIZE_READ));
            qemu.write_mem(_a1, &bytes).unwrap();
            println!("post hook READ\n");
            // println!("print the ret {} \n", ret);
            nbytes as GuestAddr
        },
        SYS_PREAD64 =>  {
            let nbytes = _a2 as usize;
            // let offset = _a3 as usize;

            let bytes = crate::stream::get_current_bytes(crate::stream::OFFSET_PREAD64, nbytes.min(crate::stream::SIZE_PREAD64));
            qemu.write_mem(_a1, &bytes).unwrap();
            println!("post hook PREAD64\n");
            // println!("print the ret {} \n", ret);
            nbytes as GuestAddr
        }
        SYS_FSTAT =>  {
            let bytes = crate::stream::get_current_bytes(crate::stream::OFFSET_FSTAT, crate::stream::SIZE_FSTAT);
            qemu.write_mem(_a1, &bytes).unwrap(); // nog toevoegen dat t na post hook alleen de value overwrite die verandert is 
            println!("pre hook FSTAT\n");
            ret
        },
        _ => ret

    }
}


pub fn init_hooks(qemu: &Qemu){
    qemu.hooks().add_pre_syscall_hook(0u64, pre_hooks);
    qemu.hooks().add_post_syscall_hook(0u64, post_hooks);
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