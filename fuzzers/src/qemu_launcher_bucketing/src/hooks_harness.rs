// hooks the syscalls
// reads the right segment from the byte stream at the correct offset
// using prehooks for syscalls with no side effects
// handles possible side effects where needed
use crate::class_path;
use crate::stream;
use crate::syscalls;
use crate::metrics;
// use crate::snapshot;
// use libafl_qemu_sys;
use libafl_qemu::{GuestAddr, Qemu, Regs, SyscallHookResult};
use std::cell::RefCell;
use std::collections::HashMap;
// use std::sync::atomic::AtomicBool;
// use std::sync::atomic::Ordering;
// use std::sync::atomic::AtomicU64;

use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::ptr;
use libafl_qemu::modules::SnapshotModule;
use std::sync::atomic::AtomicU64;
pub static PRINTING: AtomicBool = AtomicBool::new(false);

thread_local! {
    pub static FAKED_FD_MAPPING: RefCell<HashMap<i32,i32>> = RefCell::new(HashMap::new());
    pub static SYSCALL_COUNT: RefCell<HashMap<(u64,u64),u32>> = RefCell::new(HashMap::new());

}

pub static HARNESS_PC: AtomicU64 = AtomicU64::new(0);
pub static HARNESS_SP: AtomicU64 = AtomicU64::new(0);
pub static HARNESS_ARGV: AtomicU64 = AtomicU64::new(0);
pub static HARNESS_ARGC: AtomicU64 = AtomicU64::new(0);
pub static HARNESS_READY: AtomicBool = AtomicBool::new(false);
pub static HARNESS_ENTRY: AtomicU64 = AtomicU64::new(0);
pub static HARNESS_FAKE_RET: AtomicU64 = AtomicU64::new(0);

// pub fn set_harness_ret_addr(addr: GuestAddr) {
//     HARNESS_RET_ADDR.store(addr as u64, Ordering::Relaxed);
// }


fn classify_path(qemu: &Qemu, addr: GuestAddr, num: i32) -> class_path::PathRules {
    let mut buf = vec![0u8; 256];
    qemu.read_mem(addr, &mut buf).unwrap();
    let nul_pos = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    let path = std::str::from_utf8(&buf[..nul_pos]).unwrap();
    metrics::behavior_seen(num as i64, path);
    class_path::class_path(path)
}

fn fuzzed_size(num: i32, max: GuestAddr) -> u32 {
    let mut fuzzed_size = crate::stream::consume_bytes(num, 4);
    fuzzed_size.resize(4, 0);
    let fuzzed_size = u32::from_le_bytes(fuzzed_size.as_slice().try_into().unwrap());
    fuzzed_size % (max as u32)
}

fn write_fuzzed_bytes(qemu: &Qemu, addr: GuestAddr, num: i32, size: usize) {
    let mut fuzzed_bytes = crate::stream::consume_bytes(num, size);
    &fuzzed_bytes.resize(size, 0); // this already checks if it is even less then 4
    qemu.write_mem(addr, &fuzzed_bytes[..size]).unwrap();
}

fn write_ret_32(qemu: &Qemu, num: i32, size: usize) -> i32{
    let mut fuzzed_bytes = crate::stream::consume_bytes(num, size);
    &fuzzed_bytes.resize(4, 0); // this already checks if it is even less then 4
    i32::from_le_bytes(fuzzed_bytes.as_slice().try_into().unwrap())
}

fn write_ret_64(qemu: &Qemu, num: i32, size: usize) -> i64{
    let mut fuzzed_bytes = crate::stream::consume_bytes(num, size);
    &fuzzed_bytes.resize(size, 0); // this already checks if it is even less then 4
    i64::from_le_bytes(fuzzed_bytes.as_slice().try_into().unwrap())
}

extern "C" fn pre_hooks(
    _data: u64,
    mut sys_num: i32,
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
    // eprintln!("PRE HOOKS {} \n", sys_num);
    metrics::syscall_seen(sys_num as i64, _a0, _a1, _a2, _a3, _a4, _a5, _a6, _a7);
   
    let pc = qemu.read_reg(Regs::Pc).unwrap();
    let key = (sys_num as u64, pc as u64);
    let count = SYSCALL_COUNT.with(|c| {
        let mut c = c.borrow_mut();
        let counter = c.entry(key).or_insert(0);
        *counter += 1;
        *counter
    });

    if count > 50{
            // eprintln!("In LOOOOP");
        SYSCALL_COUNT.with(|c: &RefCell<HashMap<(u64, u64), u32>>| {
                c.borrow_mut().clear();
            });
        FAKED_FD_MAPPING.with(|f: &RefCell<HashMap<i32,i32>>| {
            f.borrow_mut().clear();
        });
            if let Some(cpu) = qemu.current_cpu() {
                cpu.trigger_breakpoint(); // immediate stop path
            }
            unsafe {
                libafl_qemu_sys::libafl_sync_exit_cpu();
            }
            return SyscallHookResult::Skip(0)
    }
  
    match sys_num {
        n if n == syscalls::SYS_WAIT4.num => {
            if _a1 != 0 {
                let status: i32 = 0; 
                qemu.write_mem(_a1, &status.to_le_bytes()).unwrap();
            }
            SyscallHookResult::Skip(1337)
        }
        n if n == syscalls::SYS_EXIT.num || n == syscalls::SYS_EXIT_GROUP.num => {
            let ready = HARNESS_READY.load(Ordering::Relaxed);
            // eprintln!("[EXIT HOOK] ready={}", ready);
            let entry = HARNESS_ENTRY.load(Ordering::Relaxed) as GuestAddr;
            // eprintln!("exit fired, READY={}, entry={:#x}", ready, entry);
            SYSCALL_COUNT.with(|c: &RefCell<HashMap<(u64, u64), u32>>| {
                c.borrow_mut().clear();
            });
            FAKED_FD_MAPPING.with(|f: &RefCell<HashMap<i32,i32>>| {
                f.borrow_mut().clear();
            });
            
            if !ready {
                if entry != 0 {
                    let sp = qemu.read_reg(Regs::Sp).unwrap() as GuestAddr;
                    let mut new_sp = sp & !0xf; // align
                    new_sp -= 8;               // room for return address
                    // let fake_ret: u64 = 0x0;
                    // let sp = qemu.read_reg(Regs::Sp).unwrap();
                    let fake_ret = HARNESS_FAKE_RET.load(Ordering::Relaxed);
                    qemu.write_mem(new_sp, &fake_ret.to_le_bytes()).unwrap();
                    qemu.write_reg(Regs::Sp, new_sp).unwrap(); // use new_sp!
                    qemu.write_reg(Regs::Pc, entry).unwrap();
                    // qemu.write_reg(Regs::Pc, entry).unwrap();
                    // qemu.write_reg(Regs::Sp, sp & !0xf).unwrap(); // align stack
                    return SyscallHookResult::Skip(0);
                }
                return SyscallHookResult::Run;
            }
            
            let pc = HARNESS_PC.load(Ordering::Relaxed) as GuestAddr;
            let sp = HARNESS_SP.load(Ordering::Relaxed) as GuestAddr;
            let argc = HARNESS_ARGC.load(Ordering::Relaxed) as GuestAddr;
            let argv = HARNESS_ARGV.load(Ordering::Relaxed) as GuestAddr;
            // eprintln!("resetting: argc={:#x} argv={:#x}", argc, argv);
            qemu.write_reg(Regs::Pc, pc).unwrap();
            qemu.write_reg(Regs::Sp, sp).unwrap();
            qemu.write_reg(Regs::Rdi, argc).unwrap();
            qemu.write_reg(Regs::Rsi, argv).unwrap();
            if let Some(cpu) = qemu.current_cpu() {
                cpu.trigger_breakpoint(); // immediate stop path
            }
            return SyscallHookResult::Skip(0);
        }

        n if n == syscalls::SYS_UNAME.num => {
            let pc = qemu.read_reg(Regs::Pc).unwrap();

            write_fuzzed_bytes(
                &qemu,
                _a0,
                syscalls::SYS_UNAME.num,
                syscalls::SYS_UNAME.consume_size,
            );
           
            SyscallHookResult::Skip(0)
        }

        n if n == syscalls::SYS_STAT.num || n == syscalls::SYS_NEWLSTAT.num || n == syscalls::SYS_LSTAT.num => {
            // println!("HOOK UNAME");
            write_fuzzed_bytes(
                &qemu,
                _a1,
                n,
                syscalls::SYS_STAT.consume_size,
            );
            // println!("end HOOK STAT");
            SyscallHookResult::Skip(0)
        }

        n if n == syscalls::SYS_STATX.num => {
            // println!("HOOK UNAME");
            write_fuzzed_bytes(
                &qemu,
                _a4,
                n,
                syscalls::SYS_STATX.consume_size,
            );
            // println!("end HOOK STAT");
            SyscallHookResult::Skip(0)
        }        

        // n if n == syscalls::SYS_LSTAT.num => {
        //     // println!("HOOK UNAME");
        //     write_fuzzed_bytes(
        //         &qemu,
        //         _a1,
        //         syscalls::SYS_LSTAT.num,
        //         syscalls::SYS_STAT.consume_size,
        //     );
        //     // println!("end HOOK STAT");
        //     SyscallHookResult::Skip(0)
        // }

        n if n == syscalls::SYS_SYSINFO.num => {
            // println!("IN HOOK SYSINFO");
            write_fuzzed_bytes(
                &qemu,
                _a0,
                syscalls::SYS_SYSINFO.num,
                syscalls::SYS_SYSINFO.consume_size,
            );
            // println!("IN HOOK SYSINFOEND");
            SyscallHookResult::Skip(0)
        }

        n if n == syscalls::SYS_GETTIMEOFDAY.num => {
            write_fuzzed_bytes(
                &qemu,
                _a0,
                syscalls::SYS_GETTIMEOFDAY.num,
                syscalls::SYS_GETTIMEOFDAY.consume_size,
            );
            SyscallHookResult::Skip(0)
        }

        n if n == syscalls::SYS_CLOCKGETTIME.num => {
            write_fuzzed_bytes(
                &qemu,
                _a1,
                syscalls::SYS_CLOCKGETTIME.num,
                syscalls::SYS_CLOCKGETTIME.consume_size,
            );
            SyscallHookResult::Skip(0)
        }

        n if n == syscalls::SYS_NANOSLEEP.num => {
            // PRINTING.swap(true, Ordering::Relaxed);
            write_fuzzed_bytes(
                &qemu,
                _a1,
                syscalls::SYS_NANOSLEEP.num,
                syscalls::SYS_NANOSLEEP.consume_size,
            );
            let ret = write_ret_32(&qemu, syscalls::SYS_NANOSLEEP.num, 4);
            SyscallHookResult::Skip(ret as GuestAddr)
            // SyscallHookResult::Skip(0)
        }

        n if n == syscalls::SYS_GETRUSAGE.num => {
            write_fuzzed_bytes(
                &qemu,
                _a1,
                syscalls::SYS_GETRUSAGE.num,
                syscalls::SYS_GETRUSAGE.consume_size,
            );
            SyscallHookResult::Skip(0)
        }

        n if n == syscalls::SYS_SCHEDGETAFFINITY.num => {
            // println!("HOOK sched getaffinity");

            write_fuzzed_bytes(
                &qemu,
                _a2,
                syscalls::SYS_SCHEDGETAFFINITY.num,
                syscalls::SYS_SCHEDGETAFFINITY.consume_size,
            );
            SyscallHookResult::Skip(_a1 as GuestAddr)
        }

        n if n == syscalls::SYS_GETCWD.num => {
            write_fuzzed_bytes(
                &qemu,
                _a0,
                syscalls::SYS_GETCWD.num,
                syscalls::SYS_GETCWD.consume_size,
            );
            SyscallHookResult::Skip(0)
        }

        n if n == syscalls::SYS_EXECVE.num || n == syscalls::SYS_EXECVEAT.num => {
            let mut buf = vec![0u8; 256];
            qemu.read_mem(_a0, &mut buf).unwrap();
            let nul_pos = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
            let path = std::str::from_utf8(&buf[..nul_pos]).unwrap();
            metrics::behavior_seen(n as i64, path);
            SyscallHookResult::Skip(-2i64 as GuestAddr) // returns ENOENT, so seems like the doesnt exist so it cant just start a new process that is not followed by the fuzzer. 
        }

        n if n == syscalls::SYS_CHMOD.num => {
            // let ret = write_ret_32(&qemu, n, syscalls::SYS_CHMOD.consume_size);
            let mut buf = vec![0u8; 256];
            qemu.read_mem(_a0, &mut buf).unwrap();
            let nul_pos = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
            let path = std::str::from_utf8(&buf[..nul_pos]).unwrap();
            metrics::behavior_seen(n as i64, &format!("{},mode =  {:o}", path, _a1));
            SyscallHookResult::Skip(0) // faking success so it doesnt actually modify the file system
        }

        n if n == syscalls::SYS_UNLINK.num => {
            // let ret = write_ret_32(&qemu, n, syscalls::SYS_CHMOD.consume_size);
            let mut buf = vec![0u8; 256];
            qemu.read_mem(_a0, &mut buf).unwrap();
            let nul_pos = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
            let path = std::str::from_utf8(&buf[..nul_pos]).unwrap();
            metrics::behavior_seen(n as i64, path);
            SyscallHookResult::Skip(0) // faking success so it doesnt actually modify the file system
        }

        n if n == syscalls::SYS_UNLINKAT.num => {
            // let ret = write_ret_32(&qemu, n, syscalls::SYS_CHMOD.consume_size);
            let mut buf = vec![0u8; 256];
            qemu.read_mem(_a1, &mut buf).unwrap();
            let nul_pos = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
            let path = std::str::from_utf8(&buf[..nul_pos]).unwrap();
            metrics::behavior_seen(n as i64, path);
            SyscallHookResult::Skip(0) // faking success so it doesnt actually modify the file system
        }

        n if n == syscalls::SYS_ACCESS.num || n == syscalls::SYS_FACCESSAT.num || n == syscalls::SYS_FACCESSAT2.num => {
            let ret = write_ret_32(&qemu, n, syscalls::SYS_ACCESS.consume_size);
            SyscallHookResult::Skip(ret as GuestAddr)
        }

        n if n == syscalls::SYS_GETPID.num => {
            let ret = write_ret_32(&qemu, syscalls::SYS_GETPID.num, syscalls::SYS_GETPID.consume_size);
            SyscallHookResult::Skip(ret as GuestAddr)
        }

        n if n == syscalls::SYS_GETPPID.num => {
            let ret = write_ret_32(&qemu, syscalls::SYS_GETPPID.num, syscalls::SYS_GETPPID.consume_size);
            SyscallHookResult::Skip(ret as GuestAddr)
        }

        n if n == syscalls::SYS_GETUID.num => {
            let ret = write_ret_32(&qemu, syscalls::SYS_GETUID.num, syscalls::SYS_GETUID.consume_size);
            SyscallHookResult::Skip(ret as GuestAddr)
        }

        n if n == syscalls::SYS_GETEUID.num => {
            let ret = write_ret_32(&qemu, syscalls::SYS_GETEUID.num, syscalls::SYS_GETEUID.consume_size);
            SyscallHookResult::Skip(ret as GuestAddr)
        }

        n if n == syscalls::SYS_GETGID.num => {
            let ret = write_ret_32(&qemu, syscalls::SYS_GETGID.num, syscalls::SYS_GETGID.consume_size);
            SyscallHookResult::Skip(ret as GuestAddr)
        }

        n if n == syscalls::SYS_GETEGID.num => {
            let ret = write_ret_32(&qemu, syscalls::SYS_GETEGID.num, syscalls::SYS_GETEGID.consume_size);
            SyscallHookResult::Skip(ret as GuestAddr)
        }

        n if n == syscalls::SYS_PTRACE.num => {
            let ret = write_ret_64(&qemu, syscalls::SYS_PTRACE.num, syscalls::SYS_PTRACE.consume_size);
            SyscallHookResult::Skip(ret as GuestAddr)
        }

        n if n == syscalls::SYS_TIME.num => {
            let ret = write_ret_64(&qemu, syscalls::SYS_TIME.num, syscalls::SYS_TIME.consume_size);
            SyscallHookResult::Skip(ret as GuestAddr)
        }

        n if n == syscalls::SYS_PRCTL.num => {
            metrics::behavior_seen(n as i64, &format!("cmd={}", _a0));
            let ret = write_ret_64(&qemu, syscalls::SYS_PRCTL.num, syscalls::SYS_PRCTL.consume_size);
            SyscallHookResult::Skip(ret as GuestAddr)
        }

        n if n == syscalls::SYS_IOCTL.num => {
            let request_num = _a1 as u64; 
            // eprintln!("request num {:#x}", request_num);
            let buf_dir = (request_num >> 30) & 0x3; 
            let buf_size = ((request_num >> 16) & 0x3FFF) as usize;

            
            //because of old style just fuzzing everyhting that has buff
            let size = match request_num {
                0x5413 => 8,   // TIOCGWINSZ 
                0x8912 => 64,  // SIOCGIFCONF
                0x8915 => 40,  // SIOCGIFADDR
                0x8927 => 40,  // SIOCGIFHWADDR
                _ if buf_size > 0 => buf_size,  
                _ => 0,  
            };

            if size>0 && _a2 != 0 {
                // let size = if buf_size > 0 { buf_size } else { 128 };
                write_fuzzed_bytes(&qemu, _a2, syscalls::SYS_IOCTL.num, size);
            }
            let ret = write_ret_32(&qemu, syscalls::SYS_IOCTL.num, syscalls::SYS_IOCTL.consume_size);
            // eprintln!("HOOK IOCTL");
            SyscallHookResult::Skip(ret as GuestAddr)
        }

        n if n == syscalls::SYS_SOCKET.num => {
            let ret = write_ret_32(&qemu, syscalls::SYS_SOCKET.num, syscalls::SYS_SOCKET.consume_size);
            SyscallHookResult::Skip(ret as GuestAddr)
        }

        n if n == syscalls::SYS_CONNECT.num || n == syscalls::SYS_BIND.num => {
            let mut buf = vec![0u8;16];
            let arguments = if qemu.read_mem(_a1, &mut buf).is_ok(){format!("{}.{}.{}.{}:{}", buf[4], buf[5], buf[6], buf[7], u16::from_be_bytes([buf[2],buf[3]]))}
            else{
                "unknown".to_string()
            };
            metrics::behavior_seen(n as i64, &arguments);
            let ret = write_ret_32(&qemu, n, syscalls::SYS_CONNECT.consume_size);
            SyscallHookResult::Skip(ret as GuestAddr)
        }

        n if n == syscalls::SYS_SENDTO.num => {
            let mut buf = vec![0u8;16];
            let arguments = if qemu.read_mem(_a4, &mut buf).is_ok(){format!("{}.{}.{}.{}:{}", buf[4], buf[5], buf[6], buf[7], u16::from_be_bytes([buf[2],buf[3]]))}
            else{
                "unknown".to_string()
            };
            metrics::behavior_seen(n as i64, &arguments);
            let ret = write_ret_32(&qemu, n, syscalls::SYS_CONNECT.consume_size);
            SyscallHookResult::Skip(ret as GuestAddr)
        }

        n if n == syscalls::SYS_LISTEN.num => {
            let ret = write_ret_32(&qemu, n, syscalls::SYS_LISTEN.consume_size);
            SyscallHookResult::Skip(ret as GuestAddr)
        }

        n if n == syscalls::SYS_RECVFROM.num => {
            let size = _a2;
            // let ret = write_ret_32(&qemu, syscalls::SYS_GETRANDOM.num, syscalls::SYS_GETRANDOM.consume_size);
            write_fuzzed_bytes(&qemu, _a1, syscalls::SYS_RECVFROM.num, size as usize);
            SyscallHookResult::Skip(size as GuestAddr)
        }

        n if n == syscalls::SYS_GETRANDOM.num => {
            if _a1 == 0 {
                return SyscallHookResult::Skip(0);
            }

            let size = _a1;
            // let ret = write_ret_32(&qemu, syscalls::SYS_GETRANDOM.num, syscalls::SYS_GETRANDOM.consume_size);
            write_fuzzed_bytes(&qemu, _a0, syscalls::SYS_GETRANDOM.num, size as usize);
            SyscallHookResult::Skip(size as GuestAddr)
        }

        n if n == syscalls::SYS_PRLIMIT64.num => {
            if _a2 == 0 {
                // in dit geval wordt er gelezen van de env (en er worden dus niet gezet)
                write_fuzzed_bytes(
                    &qemu,
                    _a3,
                    syscalls::SYS_PRLIMIT64.num,
                    syscalls::SYS_PRLIMIT64.consume_size,
                );
                SyscallHookResult::Skip(0)
            } else {
                // anders worden er waarden gezet dus dan gwn laten gaaaan
                SyscallHookResult::Run
            }
        }

        n if n == syscalls::SYS_GETDENTS64.num => {
            let fd = _a0 as i32;

            FAKED_FD_MAPPING.with(|fd_map| {
                let fd_map = fd_map.borrow();

                if let Some(real_fd) = fd_map.get(&fd) {
                    // checking if fd is faked
                    //fuzzing size
                    let size = fuzzed_size(syscalls::SYS_GETDENTS64.num, _a2);

                    //value
                    write_fuzzed_bytes(&qemu, _a1, syscalls::SYS_GETDENTS64.num, size as usize);
                    return SyscallHookResult::Skip(size as GuestAddr);
                }
                SyscallHookResult::Run
            })
        }

        n if n == syscalls::SYS_NEWFSTATAT.num => {
            //classing path
            let path_class = classify_path(&qemu, _a1, n);
            //  eprintln!("newfstatat class: {:?} addr: {:#x}", path_class, _a1);
            if path_class == class_path::PathRules::Env_resources {
                //if path is env related
                write_fuzzed_bytes(
                    &qemu,
                    _a2,
                    syscalls::SYS_NEWFSTATAT.num,
                    syscalls::SYS_NEWFSTATAT.consume_size,
                );
                return SyscallHookResult::Skip(0);
            }
            SyscallHookResult::Run
        }

        n if n == syscalls::SYS_READ.num || n == syscalls::SYS_PREAD64.num => {
            let fd = _a0 as i32;

            FAKED_FD_MAPPING.with(|fd_map| {
                let fd_map = fd_map.borrow();
                // eprintln!("READ faked fd={} size={}", fd, _a2);

                if let Some(real_fd) = fd_map.get(&fd) {
                    //checking if fd is faked, in that case fuzzing
                    // println!("HOOK UNAME");
                    write_fuzzed_bytes(&qemu, _a1, syscalls::SYS_READ.num, _a2 as usize);
                    return SyscallHookResult::Skip(_a2 as GuestAddr);
                }
                SyscallHookResult::Run
            })
        }

        n if n == syscalls::SYS_READLINK.num => {
            //classing path
            // println!("hook READLINK");

            let path_class = classify_path(&qemu, _a0, n);

            if path_class == class_path::PathRules::Env_resources {
            // println!("hook READLINK");
                // let size = fuzzed_size(syscalls::SYS_READLINK.num, _a2);
                let size = _a2;
                write_fuzzed_bytes(&qemu, _a1, syscalls::SYS_READLINK.num, size as usize);
                return SyscallHookResult::Skip(size as GuestAddr);
            }
            SyscallHookResult::Run
        }

        n if n == syscalls::SYS_READLINKAT.num => {
            //classing path
            let path_class = classify_path(&qemu, _a1, n);

            if path_class == class_path::PathRules::Env_resources {
                // println!("hook READLINKAT");
                // let size = fuzzed_size(syscalls::SYS_READLINKAT.num, _a3);
                let size = _a3;
                write_fuzzed_bytes(&qemu, _a2, syscalls::SYS_READLINKAT.num, size as usize);
                return SyscallHookResult::Skip(size as GuestAddr);
            }
            SyscallHookResult::Run
        }

        n if n == syscalls::SYS_FSTAT.num => {
            let fd = _a0 as i32;

            FAKED_FD_MAPPING.with(|fd_map| {
                let fd_map = fd_map.borrow();

                if let Some(real_fd) = fd_map.get(&fd) {
                    //checking if fd is faked
                    write_fuzzed_bytes(&qemu, _a1, syscalls::SYS_FSTAT.num, syscalls::SYS_FSTAT.consume_size);
                    return SyscallHookResult::Skip(_a2 as GuestAddr);
                }
                SyscallHookResult::Run
            })
        }

         n if n == syscalls::SYS_FSTATFS.num => {
            let fd = _a0 as i32;

            FAKED_FD_MAPPING.with(|fd_map| {
                let fd_map = fd_map.borrow();

                if let Some(real_fd) = fd_map.get(&fd) {
                    //checking if fd is faked
                    write_fuzzed_bytes(&qemu, _a1, syscalls::SYS_FSTATFS.num, syscalls::SYS_FSTATFS.consume_size);
                    return SyscallHookResult::Skip(_a2 as GuestAddr);
                }
                SyscallHookResult::Run
            })
        }

        n if n == syscalls::SYS_CLOSE.num =>{
            let fd = _a0 as i32;
            FAKED_FD_MAPPING.with(|fd_map| {
                if let Some(real_fd) = fd_map.borrow_mut().remove(&fd) {
                    unsafe{libc::close(real_fd)};
                }
            });
            SyscallHookResult::Skip(0)
        }

        n if n == syscalls::SYS_FORK.num || n == syscalls::SYS_VFORK.num || n == syscalls::SYS_CLONE.num || n == syscalls::SYS_CLONE3.num => {
            // PRINTING.swap(true, Ordering::Relaxed);
            SyscallHookResult::Skip(0) // child gets 0
            // SyscallHookResult::Skip(1337) // child gets 0
        }

        n if n == syscalls::SYS_MMAP.num => {
            //FAILSAFE to chec if faked fd enters MMAP
            let fd = _a4 as i32;

            FAKED_FD_MAPPING.with(|fd_map| {
                let fd_map = fd_map.borrow();

                if let Some(real_fd) = fd_map.get(&fd) {
                    println!("NOOO should not have fake {}\n", real_fd);
                    // return SyscallHookResult::Skip(0);
                }
            });
            SyscallHookResult::Run
        }
        _ => SyscallHookResult::Run,
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

    match sys_num {
        n if n == syscalls::SYS_OPENAT.num || n == syscalls::SYS_OPENAT2.num => {
            //classify path
            
            let path_class = classify_path(&qemu, _a1, n);
            // eprintln!("openat class: {:?}", path_class);
            if path_class == class_path::PathRules::Env_resources {
                // faking fd
                let mut fuzzed_bytes = crate::stream::consume_bytes(
                    n,
                    syscalls::SYS_OPENAT.consume_size,
                );
                fuzzed_bytes.resize(syscalls::SYS_OPENAT.consume_size, 0);
                let mut fuzzed_fd = i32::from_le_bytes(fuzzed_bytes.as_slice().try_into().unwrap());

                if fuzzed_fd >= 0 && fuzzed_fd <= 10 {
                    fuzzed_fd = fuzzed_fd.wrapping_abs() + 0x100000;
                }

                FAKED_FD_MAPPING.with(|fd_map| {
                    fd_map.borrow_mut().insert(fuzzed_fd.wrapping_abs(), ret as i32);
                });
                return fuzzed_fd.wrapping_abs() as GuestAddr;
            }

            ret as GuestAddr
        }

        n if n == syscalls::SYS_OPEN.num => {
            //classify path
            
            let path_class = classify_path(&qemu, _a0, n);
            // eprintln!("open class: {:?}", path_class);
            if path_class == class_path::PathRules::Env_resources {
                // faking fd
                let mut fuzzed_bytes = crate::stream::consume_bytes(
                    syscalls::SYS_OPEN.num,
                    syscalls::SYS_OPEN.consume_size,
                );
                fuzzed_bytes.resize(syscalls::SYS_OPEN.consume_size, 0);
                let mut fuzzed_fd = i32::from_le_bytes(fuzzed_bytes.as_slice().try_into().unwrap());

                if fuzzed_fd >= 0 && fuzzed_fd <= 10 {
                    fuzzed_fd = fuzzed_fd.wrapping_abs() + 0x100000;
                }

                FAKED_FD_MAPPING.with(|fd_map| {
                    fd_map.borrow_mut().insert(fuzzed_fd.abs(), ret as i32);
                });
                return fuzzed_fd.wrapping_abs() as GuestAddr;
            }

            ret as GuestAddr
        }

        n if n == syscalls::SYS_READ.num || n == syscalls::SYS_PREAD64.num => {
            let real_nbytes = (ret as usize).min(stream::BUCKET_SIZE);
            write_fuzzed_bytes(&qemu, _a1, syscalls::SYS_READ.num, real_nbytes);
            ret
        }

        n if n == syscalls::SYS_FSTAT.num => {
            write_fuzzed_bytes(
                &qemu,
                _a1,
                syscalls::SYS_FSTAT.num,
                syscalls::SYS_FSTAT.consume_size,
            );
            ret
        }

        n if n == syscalls::SYS_FSTATFS.num => {
            write_fuzzed_bytes(
                &qemu,
                _a1,
                syscalls::SYS_FSTATFS.num,
                syscalls::SYS_FSTATFS.consume_size,
            );
            ret
        }
        _ => ret,
    }
}

pub fn init_hooks(qemu: &Qemu) {
    
    qemu.hooks().add_pre_syscall_hook(0u64, pre_hooks);
    qemu.hooks().add_post_syscall_hook(0u64, post_hooks);
}
