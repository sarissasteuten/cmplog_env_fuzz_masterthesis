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

pub const SYS_GETTIMEOFDAY: syscall_data = syscall_data {
    num: 96,
    consume_size: 16, // not needed
};

pub const N_SYSCALLS: usize = 14;

pub const SYSCALLS: [syscall_data; N_SYSCALLS] = [
    SYS_ACCESS,
    SYS_FSTAT,
    SYS_OPENAT,
    SYS_PREAD64,
    SYS_PRLIMIT64,
    SYS_READ,
    SYS_MMAP,
    SYS_UNAME,
    SYS_SYSINFO,
    SYS_READLINK,
    SYS_GETDENTS64,
    SYS_NEWFSTATAT,
    SYS_LSTAT,
    SYS_GETTIMEOFDAY,
];
