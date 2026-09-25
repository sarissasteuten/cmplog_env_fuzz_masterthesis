use libc;

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

pub const SYS_READLINKAT: syscall_data = syscall_data {
    num: 267,
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

pub const SYS_STATX: syscall_data = syscall_data {
    num: 332,
    consume_size: std::mem::size_of::<libc::statx>(),
};

pub const SYS_LSTAT: syscall_data = syscall_data {
    num: 6,
    consume_size: std::mem::size_of::<libc::stat>(),
};

pub const SYS_STAT: syscall_data = syscall_data {
    num: 4,
    consume_size: std::mem::size_of::<libc::stat>(),
};

pub const SYS_NEWLSTAT: syscall_data = syscall_data {
    num: 6,
    consume_size: std::mem::size_of::<libc::stat>(),
};

pub const SYS_ACCESS: syscall_data = syscall_data {
    num: 21,
    consume_size: 4,
};

pub const SYS_FACCESSAT: syscall_data = syscall_data {
    num: 269,
    consume_size: 4,
};

pub const SYS_FACCESSAT2: syscall_data = syscall_data {
    num: 439,
    consume_size: 4,
};

pub const SYS_GETPID: syscall_data = syscall_data {
    num: 39,
    consume_size: 4,
};

pub const SYS_GETPPID: syscall_data = syscall_data {
    num: 110,
    consume_size: 4,
};

pub const SYS_GETUID: syscall_data = syscall_data {
    num: 102,
    consume_size: 4,
};

pub const SYS_GETEUID: syscall_data = syscall_data {
    num: 107,
    consume_size: 4,
};

pub const SYS_GETGID: syscall_data = syscall_data {
    num: 104,
    consume_size: 4,
};

pub const SYS_GETEGID: syscall_data = syscall_data {
    num: 108,
    consume_size: 4,
};

pub const SYS_GETRESUID: syscall_data = syscall_data {
    num: 118,
    consume_size: 12,
};

pub const SYS_GETRESGID: syscall_data = syscall_data {
    num: 120,
    consume_size: 12,
};

pub const SYS_FSTAT: syscall_data = syscall_data {
    num: 5,
    consume_size: 144,
};

pub const SYS_OPEN: syscall_data = syscall_data {
    num: 2,
    consume_size: 4,
};

pub const SYS_OPENAT: syscall_data = syscall_data {
    num: 257,
    consume_size: 4,
};

pub const SYS_OPENAT2: syscall_data = syscall_data {
    num: 437,
    consume_size: 4,
};

pub const SYS_GETCWD: syscall_data = syscall_data {
    num: 79,
    consume_size: 4096,
};

pub const SYS_PTRACE: syscall_data = syscall_data {
    num: 101,
    consume_size: 8,
};

pub const SYS_TIME: syscall_data = syscall_data {
    num: 201,
    consume_size: 8,
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

pub const SYS_CLOSE: syscall_data = syscall_data {
    num: 3,
    consume_size: 0, // not needed
};

pub const SYS_WAIT4: syscall_data = syscall_data {
    num: 61,
    consume_size: 4, // not needed
};

pub const SYS_GETTIMEOFDAY: syscall_data = syscall_data {
    num: 96,
    consume_size: 16, // not needed
};

pub const SYS_CLOCKGETTIME: syscall_data = syscall_data {
    num: 228,
    consume_size: 16, // not needed
};

pub const SYS_NANOSLEEP: syscall_data = syscall_data {
    num: 35,
    consume_size: 16, // not needed
};

pub const SYS_PRCTL: syscall_data = syscall_data {
    num: 157,
    consume_size: 8, // not needed
};

pub const SYS_IOCTL: syscall_data = syscall_data {
    num: 16,
    consume_size: 0, // not needed
};

pub const SYS_SOCKET: syscall_data = syscall_data {
    num: 41,
    consume_size: 4, // not needed
};

pub const SYS_CONNECT: syscall_data = syscall_data {
    num: 42,
    consume_size: 4, // not needed
};

pub const SYS_SENDTO: syscall_data = syscall_data {
    num: 44,
    consume_size: 4, // not needed
};

pub const SYS_RECVFROM: syscall_data = syscall_data {
    num: 45,
    consume_size: 4, // not needed
};

pub const SYS_BIND: syscall_data = syscall_data {
    num: 49,
    consume_size: 4, // not needed
};

pub const SYS_ACCEPT: syscall_data = syscall_data {
    num: 43,
    consume_size: 4, // not needed
};

pub const SYS_ACCEPT4: syscall_data = syscall_data {
    num: 288,
    consume_size: 4, // not needed
};

pub const SYS_LISTEN: syscall_data = syscall_data {
    num: 50,
    consume_size: 4, // not needed
};

pub const SYS_SCHEDGETAFFINITY: syscall_data = syscall_data {
    num: 204,
    consume_size: 128, // not needed
};

pub const SYS_GETRUSAGE: syscall_data = syscall_data {
    num: 98,
    consume_size: 144, // not needed
};

pub const SYS_GETRANDOM: syscall_data = syscall_data {
    num: 318,
    consume_size: 0, // not needed
};

pub const SYS_STATFS: syscall_data = syscall_data {
    num: 137,
    consume_size: std::mem::size_of::<libc::statfs>(),
};

pub const SYS_FSTATFS: syscall_data = syscall_data {
    num: 138,
    consume_size: std::mem::size_of::<libc::statfs>(),
};

pub const SYS_FORK: syscall_data = syscall_data {
    num: 57,
    consume_size: 0,
};

pub const SYS_VFORK: syscall_data = syscall_data {
    num: 56,
    consume_size: 0,
};

pub const SYS_CLONE: syscall_data = syscall_data {
    num: 56,
    consume_size: 0,
};

pub const SYS_CLONE3: syscall_data = syscall_data {
    num: 435,
    consume_size: 0,
};

pub const SYS_CHMOD: syscall_data = syscall_data { // could fake succes so it doesnt modify files but could also cause failure downstream 
    num: 90,
    consume_size: 4,
};

pub const SYS_UNLINK: syscall_data = syscall_data { // could fake succes so it doesnt modify files but could also cause failure downstream 
    num: 87,
    consume_size: 4,
};

pub const SYS_UNLINKAT: syscall_data = syscall_data { // could fake succes so it doesnt modify files but could also cause failure downstream 
    num: 263,
    consume_size: 4,
};

pub const SYS_EXECVE: syscall_data = syscall_data { // so it cant start other process, which is not followed by fuzzer  
    num: 59,
    consume_size: 4,
};

pub const SYS_EXECVEAT: syscall_data = syscall_data { // so it cant start other process, which is not followed by fuzzer  
    num: 322,
    consume_size: 4,
};
pub const SYS_SETSID: syscall_data = syscall_data {
    num: 112,
    consume_size: 4,
};
pub const SYS_CHROOT: syscall_data = syscall_data {
    num: 161,
    consume_size: 4,
};
pub const SYS_CHDIR: syscall_data = syscall_data {
    num: 80,
    consume_size: 4,
};

pub const SYS_SETRLIMIT: syscall_data = syscall_data {
    num: 160,
    consume_size: 4,
};

pub const SYS_KILL: syscall_data = syscall_data {
    num: 62,
    consume_size: 4,
};

pub const SYS_TKILL: syscall_data = syscall_data {
    num: 200,
    consume_size: 4,
};

pub const SYS_TGKILL: syscall_data = syscall_data {
    num: 130,
    consume_size: 4,
};

pub const SYS_SELECT: syscall_data = syscall_data {
    num: 23,
    consume_size: 0,
};

pub const SYS_PSELECT6: syscall_data = syscall_data {
    num: 270,
    consume_size: 0,
};

pub const N_SYSCALLS: usize = 69;

pub const SYSCALLS: [syscall_data; N_SYSCALLS] = [
    SYS_ACCESS,
    SYS_FSTAT,
    SYS_OPENAT,
    SYS_OPEN,
    SYS_OPENAT2,
    SYS_PREAD64,
    SYS_PRLIMIT64,
    SYS_READ,
    SYS_MMAP,
    SYS_UNAME,
    SYS_SYSINFO,
    SYS_READLINK,
    SYS_READLINKAT,
    SYS_GETDENTS64,
    SYS_NEWFSTATAT,
    SYS_LSTAT,
    SYS_GETTIMEOFDAY,
    SYS_GETPID,
    SYS_GETPPID,
    SYS_GETUID,
    SYS_GETEUID,
    SYS_GETGID,
    SYS_GETEGID,
    SYS_GETRESUID,
    SYS_GETRESGID, 
    SYS_STAT,
    SYS_GETCWD,
    SYS_PTRACE,
    SYS_TIME,
    SYS_CLOCKGETTIME,
    SYS_NANOSLEEP,
    SYS_PRCTL,
    SYS_IOCTL,
    SYS_SOCKET,
    SYS_CONNECT,
    SYS_SCHEDGETAFFINITY,
    SYS_GETRUSAGE,
    SYS_GETRANDOM,
    SYS_STATFS,
    SYS_FSTATFS, 
    SYS_FORK,
    SYS_VFORK,
    SYS_CLONE,
    SYS_CLONE3,
    SYS_CHMOD,
    SYS_UNLINK,
    SYS_UNLINKAT,
    SYS_EXECVE,
    SYS_EXECVEAT,
    SYS_STATX,
    SYS_RECVFROM,
    SYS_BIND,
    SYS_LISTEN,
    SYS_SENDTO,
    SYS_NEWLSTAT,
    SYS_FACCESSAT,
    SYS_FACCESSAT2,
    SYS_WAIT4,
    SYS_ACCEPT,
    SYS_ACCEPT4,
    SYS_SETSID,
    SYS_CHROOT,
    SYS_CHDIR,
    SYS_SETRLIMIT,
    SYS_KILL,
    SYS_TKILL,
    SYS_TGKILL,
    SYS_SELECT,
    SYS_PSELECT6
];
