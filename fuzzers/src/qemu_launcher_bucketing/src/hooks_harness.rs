// hooks the syscalls
// reads the right segment from the byte stream at the correct offset 
// using prehooks for syscalls with no side effects
// handles possible side effects where needed
use crate::stream;
use crate::class_path;
use libc;
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

pub const SYS_UNAME: syscall_data = syscall_data {
    num: 63, 
    consume_size: std::mem::size_of::<libc::utsname>(),
};

pub const SYS_SYSINFO: syscall_data = syscall_data {
    num: 99, 
    consume_size: std::mem::size_of::<libc::sysinfo>(),
};
pub const SYS_READLINK: syscall_data = syscall_data {
    num: 89, 
    consume_size: 0,
};
pub const SYS_GETDENTS64: syscall_data = syscall_data {
    num: 217, 
    consume_size: 0,
};
pub const SYS_NEWFSTATAT: syscall_data = syscall_data {
    num: 262, 
    consume_size: std::mem::size_of::<libc::stat>(),
};
pub const SYS_LSTAT: syscall_data = syscall_data {
    num: 6, 
    consume_size: std::mem::size_of::<libc::stat>(),
};
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

fn classify_path(qemu: &Qemu, addr: GuestAddr,) -> class_path::PathRules {
                let mut buf = vec![0u8; 256];
                qemu.read_mem(addr, &mut buf).unwrap();
                let nul_pos = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
                let path = std::str::from_utf8(&buf[..nul_pos]).unwrap();
                class_path::class_path(path)
} 

fn fuzzed_size( num: i32, max: GuestAddr,) -> u32 {
    let mut fuzzed_size = crate::stream::consume_bytes(num, 4);
    fuzzed_size.resize(4,0);
    let fuzzed_size = u32::from_le_bytes(fuzzed_size.as_slice().try_into().unwrap());
    fuzzed_size % (max as u32)
}

fn write_fuzzed_bytes(qemu: &Qemu, addr: GuestAddr, num: i32, size: usize,){
    let mut fuzzed_bytes = crate::stream::consume_bytes(num, size);
    &fuzzed_bytes.resize(size,0); // this already checks if it is even less then 4
    qemu.write_mem(addr, &fuzzed_bytes[..size]).unwrap();
}

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
            // handling exit
            if let Some(cpu) = qemu.current_cpu(){
                cpu.trigger_breakpoint();
            }
            SyscallHookResult::Skip(0)
        },

        n if n == SYS_UNAME.num =>  {
            write_fuzzed_bytes(&qemu, _a0, SYS_UNAME.num, SYS_UNAME.consume_size);
            SyscallHookResult::Skip(0)
        },

        n if n == SYS_SYSINFO.num =>  {
            write_fuzzed_bytes(&qemu, _a0, SYS_SYSINFO.num, SYS_SYSINFO.consume_size);
            SyscallHookResult::Skip(0)
        },
    
        n if n == SYS_ACCESS.num =>  {
            let mut fuzzed_bytes = crate::stream::consume_bytes(SYS_ACCESS.num, SYS_ACCESS.consume_size);
            &fuzzed_bytes.resize(SYS_ACCESS.consume_size,0); // this already checks if it is even less then 4
            let ret = i32::from_le_bytes(fuzzed_bytes.as_slice().try_into().unwrap());
            SyscallHookResult::Skip(ret as GuestAddr)
        },

        n if n == SYS_PRLIMIT64.num =>  {
            
            if _a2 == 0 { // in dit geval wordt er gelezen van de env (en er worden dus niet gezet)
                write_fuzzed_bytes(&qemu, _a3, SYS_PRLIMIT64.num, SYS_PRLIMIT64.consume_size);
                SyscallHookResult::Skip(0)
            } else { // anders worden er waarden gezet dus dan gwn laten gaaaan
                SyscallHookResult::Run
            }
        },

        n if n == SYS_GETDENTS64.num =>  {
            let fd = _a0 as i32;

            FD_MAPPING.with(|fd_map| {
                let fd_map = fd_map.borrow();

                if let Some(real_fd) = fd_map.get(&fd){ // checking if fd is faked
                    //fuzzing size
                    let size = fuzzed_size(SYS_GETDENTS64.num, _a2);
                                     
                    //value
                    write_fuzzed_bytes(&qemu, _a1, SYS_GETDENTS64.num, size as usize);
                    return SyscallHookResult::Skip(size as GuestAddr);
                }
                SyscallHookResult::Run
            })
        },

        n if n == SYS_NEWFSTATAT.num =>  {
            //classing path
            let path_class = classify_path(&qemu, _a1);

            if path_class == class_path::PathRules::Env_resources{ //if path is env related
                write_fuzzed_bytes(&qemu, _a2, SYS_NEWFSTATAT.num, SYS_NEWFSTATAT.consume_size);
                return SyscallHookResult::Skip(0);
            }
            SyscallHookResult::Run
        },
        
        n if n == SYS_READ.num || n == SYS_PREAD64.num =>  {
            let fd = _a0 as i32;

            FD_MAPPING.with(|fd_map| {
                let fd_map = fd_map.borrow();

                if let Some(real_fd) = fd_map.get(&fd){ //checking if fd is faked, in that case fuzzing 
                    write_fuzzed_bytes(&qemu, _a1, SYS_READ.num, _a2 as usize);
                    return SyscallHookResult::Skip(_a2 as GuestAddr);
                }
                SyscallHookResult::Run
            })
        },

        n if n == SYS_READLINK.num =>  {
            //classing path
            let path_class = classify_path(&qemu, _a0);
            
            if path_class == class_path::PathRules::Env_resources{//if path is env related
                // fuzzing size
                let size = fuzzed_size(SYS_READLINK.num, _a2);
                                     
                //value
                write_fuzzed_bytes(&qemu, _a1, SYS_READLINK.num, size as usize);
                return SyscallHookResult::Skip(size as GuestAddr);
            }
            SyscallHookResult::Run
        },

         n if n == SYS_FSTAT.num =>  {
            let fd = _a0 as i32;

            FD_MAPPING.with(|fd_map| {
                let fd_map = fd_map.borrow();

                if let Some(real_fd) = fd_map.get(&fd){ //checking if fd is faked
                    write_fuzzed_bytes(&qemu, _a1, SYS_READ.num, _a2 as usize);
                    return SyscallHookResult::Skip(_a2 as GuestAddr);
                }
                SyscallHookResult::Run
            })
        },
        n if n == SYS_MMAP.num =>  {
            //FAILSAFE to chec if faked fd enters MMAP
            let fd = _a4 as i32;

            FD_MAPPING.with(|fd_map| {
                let fd_map = fd_map.borrow();

                if let Some(real_fd) = fd_map.get(&fd){
                    println!("NOOO should not have fake {}\n", real_fd);
                }
                return SyscallHookResult::Skip(0);
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
            //classify path
            let path_class = classify_path(&qemu, _a1);
    
            if path_class == class_path::PathRules::Env_resources{
                // faking fd
                let mut fuzzed_bytes = crate::stream::consume_bytes(SYS_OPENAT.num, SYS_OPENAT.consume_size);
                fuzzed_bytes.resize(SYS_OPENAT.consume_size,0);
                let mut fuzzed_fd = i32::from_le_bytes(fuzzed_bytes.as_slice().try_into().unwrap());

                if fuzzed_fd >= 0 && fuzzed_fd <= 10 {
                    fuzzed_fd =  fuzzed_fd.abs() + 0x100000;
                }

                FD_MAPPING.with(|fd_map|{
                    fd_map.borrow_mut().insert(fuzzed_fd.abs(), ret as i32);
                });
                return fuzzed_fd.abs() as GuestAddr;
            }

            ret as GuestAddr
           
        },
        n if n == SYS_READ.num || n == SYS_PREAD64.num =>  {
            let real_nbytes = (ret as usize).min(stream::BUCKET_SIZE);
            write_fuzzed_bytes(&qemu, _a1, SYS_READ.num, real_nbytes); 
            ret 
        },    
        n if n == SYS_FSTAT.num =>  {
            write_fuzzed_bytes(&qemu, _a1, SYS_FSTAT.num, SYS_FSTAT.consume_size);
            ret
        },
        _ => ret

    }
}


pub fn init_hooks(qemu: &Qemu){
    qemu.hooks().add_pre_syscall_hook(0u64, pre_hooks);
    qemu.hooks().add_post_syscall_hook(0u64, post_hooks);
}