// mod env_vector;
// mod stream;
use crate::stream;
use crate::class_path;
use libc;
// hooks the syscalls
// reads the right segment from the byte stream at the correct offset 
// using prehooks for syscalls with no side effects
// handles possible side effects where needed
use std::cell::RefCell;
use std::collections::HashMap;
use libafl_qemu::{
    Qemu,
    SyscallHookResult,
    GuestAddr, Regs
};

thread_local!{
    pub static FD_MAPPING: RefCell<HashMap<i32,i32>> = RefCell::new(HashMap::new());
}


pub struct syscall_data {
    pub num: i32, 
    pub consume_size: usize,
}

pub const SYS_ACCESS: syscall_data = syscall_data {
    num: 21, 
    consume_size: 4,
};
pub const SYS_FSTAT: syscall_data = syscall_data {
    num: 5, 
    consume_size: 144,
};
pub const SYS_OPENAT: syscall_data = syscall_data {
    num: 257, 
    consume_size: 4,
};
pub const SYS_PRLIMIT64: syscall_data = syscall_data {
    num: 302, 
    consume_size: 16,
};
pub const SYS_READ: syscall_data = syscall_data {
    num: 0, 
    consume_size: 0, // specified in call
};
pub const SYS_PREAD64: syscall_data = syscall_data {
    num: 17, 
    consume_size: 0, // specified in call
};
pub const SYS_MMAP: syscall_data = syscall_data {
    num: 9, 
    consume_size: 0, // specified in call
};
pub const SYS_EXIT_GROUP: syscall_data = syscall_data {
    num: 231, 
    consume_size: 0, // not needed
};
pub const SYS_EXIT: syscall_data = syscall_data {
    num: 60, 
    consume_size: 0, // not needed
};

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
        n if n == SYS_EXIT.num || n == SYS_EXIT_GROUP.num =>{
            // println!("EXIT THE RUN\n\n");

            if let Some(cpu) = qemu.current_cpu(){
                cpu.trigger_breakpoint();
            }
            SyscallHookResult::Skip(0)
        },
    
        n if n == SYS_ACCESS.num =>  {
            let mut fuzzed_bytes = crate::stream::consume_bytes(SYS_ACCESS.num, SYS_ACCESS.consume_size);

            &fuzzed_bytes.resize(SYS_ACCESS.consume_size,0); // this already checks if it is even less then 4

            let ret = i32::from_le_bytes(fuzzed_bytes.as_slice().try_into().unwrap());

            // println!("pre hook ACCESS\n");
            // println!("print the ret {} \n", ret);
            SyscallHookResult::Skip(ret as GuestAddr)
        },

        n if n == SYS_PRLIMIT64.num =>  {
            
            if _a2 == 0 { // in dit geval wordt er gelezen van de env (en er worden dus niet gezet)
                let mut fuzzed_bytes = crate::stream::consume_bytes(SYS_PRLIMIT64.num, SYS_PRLIMIT64.consume_size);
                // &fuzzed_bytes.resize(SYS_PRLIMIT64.consume_size,0); // this already checks if it is even less then 4

                qemu.write_mem(_a3, &fuzzed_bytes).unwrap();
                // println!("pre hook PRLIMIT64\n");
                SyscallHookResult::Skip(0)
            } else { // anders worden er waarden gezet dus dan gwn laten gaaaan
                SyscallHookResult::Run
            }
        },
        n if n == SYS_READ.num || n == SYS_PREAD64.num =>  {
            // println!("pre hook READ with a0: {}\n", _a0);
            let fd = _a0 as i32;

            // println!("print the ret {} \n", ret);
            // println!("print the fuzzed fd {} \n", fd);
            FD_MAPPING.with(|fd_map| {
                let fd_map = fd_map.borrow();

                if let Some(real_fd) = fd_map.get(&fd){
                    // println!("pre hook READ with faked fd with a0: {}\n", _a0);
                    // println!("print the real fd {} \n", real_fd);
                    let requested_size = _a2 as usize;
                    let mut fuzzed_bytes = crate::stream::consume_bytes(SYS_READ.num, requested_size);
                    fuzzed_bytes.resize(requested_size,0);
                    qemu.write_mem(_a1, &fuzzed_bytes).unwrap();
                    return SyscallHookResult::Skip(requested_size as GuestAddr);
                }
                SyscallHookResult::Run
            })
        },
         n if n == SYS_FSTAT.num =>  {
        //    println!("pre hook FSTAT\n");
            let fd = _a0 as i32;

            // println!("print the ret {} \n", ret);
            // println!("print the fuzzed fd {} \n", fd);
            FD_MAPPING.with(|fd_map| {
                let fd_map = fd_map.borrow();

                if let Some(real_fd) = fd_map.get(&fd){
                    // println!("print the real fd {} \n", real_fd);
                    let requested_size = _a2 as usize;
                    let mut fuzzed_bytes = crate::stream::consume_bytes(SYS_FSTAT.num, SYS_FSTAT.consume_size);
                    fuzzed_bytes.resize(requested_size,0);
                    qemu.write_mem(_a1, &fuzzed_bytes).unwrap();
                    return SyscallHookResult::Skip(requested_size as GuestAddr);
                }
                SyscallHookResult::Run
            })
        },
        n if n == SYS_MMAP.num =>  {
        //    println!("pre hook MMAP\n");
            let fd = _a4 as i32;

            // println!("print the ret {} \n", ret);
            // println!("print the fuzzed fd {} \n", fd);
            FD_MAPPING.with(|fd_map| {
                let fd_map = fd_map.borrow();

                if let Some(real_fd) = fd_map.get(&fd){
                    println!("NOOO should not have fake {}\n", real_fd);
                }
                SyscallHookResult::Run
            })
        },
        _ => {
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
        n if n == SYS_OPENAT.num =>  {
            // println!("post hook OPENAT\n");
            let mut buf = vec![0u8; 256];
            qemu.read_mem(_a1, &mut buf).unwrap();
            let nul_pos = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
            let path = std::str::from_utf8(&buf[..nul_pos]).unwrap();
            let path_class = class_path::class_path(path);



            if path_class == class_path::PathRules::Env_resources{
                // println!("FAKING \n");
                 let mut fuzzed_bytes = crate::stream::consume_bytes(SYS_OPENAT.num, SYS_OPENAT.consume_size);
                fuzzed_bytes.resize(SYS_OPENAT.consume_size,0);
                let mut fuzzed_fd = i32::from_le_bytes(fuzzed_bytes.as_slice().try_into().unwrap());

                if fuzzed_fd >= 0 && fuzzed_fd <= 10 {
                    fuzzed_fd =  fuzzed_fd.abs() + 0x100000;
                }

                FD_MAPPING.with(|fd_map|{
                    fd_map.borrow_mut().insert(fuzzed_fd.abs(), ret as i32);
                });
                // println!("print the ret {} \n", ret);
                // println!("print the fuzzed fd {} \n", fuzzed_fd.abs());

                return fuzzed_fd.abs() as GuestAddr;
            }

            ret as GuestAddr
           
        },
        n if n == SYS_READ.num || n == SYS_PREAD64.num =>  {
            // println!("post hook Pread\n");
            // println!("print the _a0 {} \n", _a0);
            // println!("post hook Pread\n");

            let real_nbytes = (ret as usize).min(stream::BUCKET_SIZE); // the actual returned size not the requested
            let mut fuzzed_bytes = crate::stream::consume_bytes(SYS_READ.num, real_nbytes);
           
            fuzzed_bytes.resize(real_nbytes,0); // this already checks if it is even less then 4
            qemu.write_mem(_a1, &fuzzed_bytes).unwrap();
            // println!("print the ret {} \n", ret);
            
            // nbytes as GuestAddr
            ret 
        },
        n if n == SYS_MMAP.num =>  {
        //    println!("SHOULD I FUZZ MMAP??\n");
           ret
            
        },       
        n if n == SYS_FSTAT.num =>  {
            let mut fuzzed_bytes = crate::stream::consume_bytes(SYS_FSTAT.num, SYS_FSTAT.consume_size);
    
            qemu.write_mem(_a1, &fuzzed_bytes).unwrap();
            
            ret
        },
        _ => ret

    }
}


pub fn init_hooks(qemu: &Qemu){
    qemu.hooks().add_pre_syscall_hook(0u64, pre_hooks);
    qemu.hooks().add_post_syscall_hook(0u64, post_hooks);
}