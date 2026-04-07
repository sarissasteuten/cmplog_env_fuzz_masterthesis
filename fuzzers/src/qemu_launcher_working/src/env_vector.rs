use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVector {
    pub uname: Uname,
    pub sysinfo: SysInfo,
    pub pid_ret: i32,
    pub ppid_ret: i32,
    pub stat: Stat,
    pub acces_ret: i32,
    pub getdents_ret: i32,
    pub gettimeofday_ret: TimeVal,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uname {
    pub nodename: Vec<u8>,
    pub sysname: Vec<u8>,
    // pub release: Vec<u8>,
    // pub version: Vec<u8>,
    // pub machine: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysInfo {
    pub uptime: i64,        
    // pub loads: [u64; 3],
    // pub totalram: u64,  
    // pub freeram: u64,  
    // pub sharedram: u64,
    // pub bufferram: u64,
    // pub totalswap: u64,
    // pub freeswap: u64,
    // pub procs: u16,
    // pub totalhigh: u64,
    // pub freehigh: u64,
    // pub mem_unit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stat {
    pub st_dev: u64,
    // pub st_ino: u64;
    // pub st_mode: u32;
    // pub st_nlink: u64;
    // pub st_uid: u32;
    // pub st_gid: u32;
    // pub st_rdev: u64
    // pub st_size: i64;
    // pub st_blksize: i64;
    // pub st_blocks: i64;
    // // struct timespec  st_atim;
    // // struct timespec  st_mtim;
    // // struct timespec  st_ctim;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeVal {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

impl EnvVector{
    pub fn default_vec() -> Self{
        let mut nodename = vec![0u8;65];
        let mut sysname = vec![0u8;65];
        nodename[..6].copy_from_slice(b"ubuntu");
        sysname[..7].copy_from_slice(b"hiitest");

        let uname = Uname { nodename, sysname };
        
        let sysinfo = SysInfo {
            uptime: 172834,
        };

        let pid_ret = 5201;
        let ppid_ret = 3018;
        let stat = Stat { st_dev: 2049};
        let acces_ret = 0;
        let getdents_ret = 0;
        let gettimeofday_ret = TimeVal{
            tv_sec: 1700006734,
            tv_usec: 643901,
        };

        Self { uname, sysinfo, pid_ret, ppid_ret, stat, acces_ret, getdents_ret, gettimeofday_ret}
    }
}

use std::cell::RefCell;
thread_local!{
    pub static CURRENT_ENV_VEC: RefCell<EnvVector> = RefCell::new(EnvVector::default_vec());
}

pub fn set_current(env: EnvVector) {
    CURRENT_ENV_VEC.with(|vec| *vec.borrow_mut() = env);
}

pub fn get_current() -> EnvVector {
    CURRENT_ENV_VEC.with(|vec| vec.borrow().clone())
}

pub fn set_seed(path: &str){
    // println!("\nSETSEED\n");
    // println!("Writing seed to: {}", path);
    let seed = EnvVector::default_vec();
    let bytes_seed = bincode::serialize(&seed).unwrap();
    std::fs::write(path, bytes_seed).unwrap();
}