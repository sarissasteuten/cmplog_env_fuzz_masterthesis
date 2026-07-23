use dashmap::DashMap;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicU64, Ordering, AtomicBool};
use std::fs::{OpenOptions, File};
use std::io::{Write, BufWriter};
use std::sync::Mutex;

static TIME_START: OnceLock<Instant> = OnceLock::new();
static TIME_END: OnceLock<Instant> = OnceLock::new();

static FOUND_SYSCALLS_TIME: OnceLock<DashMap<i64,u64>> = OnceLock::new(); // found syscalls with their time of whole fuzzing run 
static BEHAVIOR_SYSCALLS: OnceLock<DashMap<String,u64>> = OnceLock::new(); // found syscalls with their time of whole fuzzing run 
// static FOUND_SYSCALLS: OnceLock<DashMap<i64, ()>> = OnceLock::new(); // found syscalls in execution

static LAST_COVERAGE_LOG: OnceLock<Mutex<Instant>> = OnceLock::new();
static LAST_THROUGHPUT_LOG: OnceLock<Mutex<Instant>> = OnceLock::new();

static UNIQUE_SYSCALLS: AtomicU64 = AtomicU64::new(0); 
static CONNECT: AtomicBool = AtomicBool::new(false); 
static EXECVE: AtomicBool = AtomicBool::new(false);
static FORK: AtomicBool = AtomicBool::new(false);

static RESULTS_FILE: OnceLock<Mutex<BufWriter<File>>> = OnceLock::new();
static SYSCALLS_ALL_FILE: OnceLock<Mutex<BufWriter<File>>> = OnceLock::new();
static SYSCALLS_SEEN: OnceLock<Mutex<BufWriter<File>>> = OnceLock::new();
static BEHAVIOR_SEEN: OnceLock<Mutex<BufWriter<File>>> = OnceLock::new();

pub fn get_found_syscalls_time() -> &'static DashMap<i64, u64>
{
    FOUND_SYSCALLS_TIME.get_or_init(DashMap::new)
}

pub fn get_behavior_syscalls() -> &'static DashMap<String, u64>
{
    BEHAVIOR_SYSCALLS.get_or_init(DashMap::new)
}

fn time_spend()-> u64{
 TIME_START.get()
        .map(|t| t.elapsed().as_millis() as u64)
        .unwrap_or(0)
}

pub fn init_time(){
    TIME_START.get_or_init(Instant::now);
}

pub fn init_results_file(){
    init_time();
    // eprintln!("event,value,time");
}

pub fn init_log_files(client_id:u32){
    let dir = format!("syscall_results/client_{}", client_id);
    std::fs::create_dir_all(&dir).unwrap();
    // let syscalls_all = File::create("syscalls_all.log").unwrap();
    // let mut title = BufWriter::new(syscalls_all);
    // writeln!(title, "syscall_num,time,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7").ok();
    // SYSCALLS_ALL_FILE.get_or_init(|| Mutex::new(title));

    let syscalls_seen = File::create(format!("{}/syscalls_seen_{}.log",dir, client_id)).unwrap();
    let mut title2 = BufWriter::new(syscalls_seen);
    writeln!(title2, "syscall_num,time,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7").ok();
    SYSCALLS_SEEN.get_or_init(|| Mutex::new(title2));

    let behavior_seen = File::create(format!("{}/behavior_seen_{}.log",dir, client_id)).unwrap();
    let mut title3 = BufWriter::new(behavior_seen);
    writeln!(title3, "syscall_num,time,arguments").ok();
    BEHAVIOR_SEEN.get_or_init(|| Mutex::new(title3));
}

// pub fn syscall_all(num: i64, arg0: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64, arg6: u64, arg7: u64){
//     let time = time_spend();
    
       
//     // eprintln!("syscall_all,{},{}", num, time);
//     if let Some(file) = SYSCALLS_ALL_FILE.get() { 
//         let mut f = file.lock().unwrap(); 
//         writeln!(f, "{},{},{},{},{},{},{},{},{},{}", num, time, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7).ok(); 
//     }
// }

pub fn syscall_seen(num: i64, arg0: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64, arg6: u64, arg7: u64){
    let time = time_spend();
    let found = get_found_syscalls_time();
    if !found.contains_key(&num){
        found.insert(num, time);
       
        // eprintln!("syscall_log,{},{}", num, time);
        if let Some(file) = SYSCALLS_SEEN.get() { 
            let mut f = file.lock().unwrap(); 
            writeln!(f, "{},{},{},{},{},{},{},{},{},{}", num, time, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7).ok(); 
            f.flush().ok();
        }
    }
}

pub fn behavior_seen(num: i64, arguments: &str){
    let time = time_spend();
    let found = get_behavior_syscalls();
    let key = format!("{}-{}", num, arguments);

    if !found.contains_key(&key){
        found.insert(key, time);
       
        // eprintln!("behavior_log,{},{}", num, time);
        if let Some(file) = BEHAVIOR_SEEN.get() { 
            let mut f = file.lock().unwrap(); 
            writeln!(f, "{},{},{}", num, time, arguments).ok(); 
            f.flush().ok();
        }
    }
}
