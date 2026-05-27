// mod env_vector;

use libafl_qemu::{
    Qemu,
    //qemu::hooks::SyscallHookResult,
};

use libafl_qemu::GuestAddr;
const SYS_UNAME: i32 = 63;
const SYS_SYSINFO: i32 = 99;
const SYS_GETPID: i32 = 39;
const SYS_GETPPID: i32 = 110;
const SYS_STAT: i32 = 262; //actually newafstat
const SYS_ACCESS: i32 = 21;
const SYS_GETDENTS: i32 = 78;
const SYS_GETTIMEOFDAY: i32 = 96;

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
    
    let qemu = unsafe { libafl_qemu::Qemu::get_unchecked() };
    let env = crate::env_vector::get_current();

    match sys_num{

        SYS_UNAME =>  {
            // println!("UNAME HOOOK!! \n");
            qemu.write_mem(_a0, &env.uname.sysname).unwrap();
            // println!("UNAME injected \n");
            ret
        },
        SYS_GETPID => { 
            // println!("GETPID HOOK!! \n");
            env.pid_ret as GuestAddr
            // ret
        },
        SYS_GETPPID => { 
            // println!("GET(P)PID HOOK!! \n");
            env.ppid_ret as GuestAddr
            // ret
        },
        SYS_SYSINFO => { 
            // println!("SYSNAME HOOK!! \n");
            qemu.write_mem(_a0, &env.sysinfo.uptime.to_le_bytes()).unwrap();
            // env.sysinfo.uptime as GuestAddr
            ret
        },
        SYS_STAT => { 
            // println!("SYSSTAT HOOK!! \n");
            // println!("writing st_dev: {}", env.stat.st_dev);
            qemu.write_mem(_a2, &env.stat.st_dev.to_le_bytes()).unwrap();
            // env.stat.st_dev as GuestAddr
            // println!("write done");
            ret
        },
        SYS_ACCESS => { 
            // println!("SYSACCESS HOOK!! \n");
            env.acces_ret as GuestAddr
            // ret
        },
        // SYS_GETDENTS => { 
        //     println!("GETDENTS HOOK!! \n");
        //     env.getdents_ret as GuestAddr
        //     // ret
        // },
        SYS_GETTIMEOFDAY => { 
            // println!("GETTIMEOFDAY HOOK!! \n");
            // env.gettimeofday_ret.tv_sec as GuestAddr
            qemu.write_mem(_a0, &env.gettimeofday_ret.tv_sec.to_le_bytes()).unwrap();
            // qemu.write_mem(_a0, &env.gettimeofday_ret.tv_usec.to_le_bytes()).unwrap();
            ret
        },
        _ => ret
    }
}

pub fn init_hooks(qemu: &Qemu){
    qemu.hooks().add_post_syscall_hook(0u64, uname_hook);
}
