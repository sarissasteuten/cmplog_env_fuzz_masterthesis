// execve("/bin/true", ["/bin/true"], 0x7ffedbe1d590 /* 31 vars */) = 0 // not faked // starts the program bin/true
// brk(NULL)                               = 0x5ac55bf2a000 // not faking 
// mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7121ef14c000 // not faking, (can be used to check how much memory is available)lm klopt niet volgensmij -> usually done with sysinfo 
// // has a lot of side effects
// access("/etc/ld.so.preload", R_OK)      = -1 ENOENT (No such file or directory) // could be skipt // checks wheter specific files exist, so yesss hooking 
// openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3 // could be skip , actually not 
// fstat(3, {st_mode=S_IFREG|0644, st_size=25015, ...}) = 0 // fake it 
// mmap(NULL, 25015, PROT_READ, MAP_PRIVATE, 3, 0) = 0x7121ef145000
// close(3)                                = 0
// openat(AT_FDCWD, "/lib/x86_64-linux-gnu/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
// read(3, "\177ELF\2\1\1\3\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0\220\243\2\0\0\0\0\0"..., 832) = 832
// pread64(3, "\6\0\0\0\4\0\0\0@\0\0\0\0\0\0\0@\0\0\0\0\0\0\0@\0\0\0\0\0\0\0"..., 784, 64) = 784
// fstat(3, {st_mode=S_IFREG|0755, st_size=2125328, ...}) = 0
// pread64(3, "\6\0\0\0\4\0\0\0@\0\0\0\0\0\0\0@\0\0\0\0\0\0\0@\0\0\0\0\0\0\0"..., 784, 64) = 784
// mmap(NULL, 2170256, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7121eee00000
// mmap(0x7121eee28000, 1605632, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x28000) = 0x7121eee28000
// mmap(0x7121eefb0000, 323584, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1b0000) = 0x7121eefb0000
// mmap(0x7121eefff000, 24576, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1fe000) = 0x7121eefff000
// mmap(0x7121ef005000, 52624, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x7121ef005000
// close(3)                                = 0
// mmap(NULL, 12288, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7121ef142000
// arch_prctl(ARCH_SET_FS, 0x7121ef142740) = 0
// set_tid_address(0x7121ef142a10)         = 2826
// set_robust_list(0x7121ef142a20, 24)     = 0
// rseq(0x7121ef143060, 0x20, 0, 0x53053053) = 0
// mprotect(0x7121eefff000, 16384, PROT_READ) = 0
// mprotect(0x5ac5320f0000, 4096, PROT_READ) = 0
// mprotect(0x7121ef184000, 8192, PROT_READ) = 0
// prlimit64(0, RLIMIT_STACK, NULL, {rlim_cur=8192*1024, rlim_max=RLIM64_INFINITY}) = 0
// munmap(0x7121ef145000, 25015)           = 0
// exit_group(0)                           = ?
#define _GNU_SOURCE
#include <sys/utsname.h>
#include <sys/sysinfo.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <sys/resource.h>
#include <sys/mman.h>
#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <fcntl.h>

int main() {
	
	brk(NULL);
	void *m1 = mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
	access("/etc/ld.so.preload", R_OK);
	int fd = openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC);
	struct stat st;
	fstat(fd, &st);
	void *m2 = mmap(NULL, 25015, PROT_READ, MAP_PRIVATE, fd, 0);
	close(fd);
	int fd2 = openat(AT_FDCWD, "/lib/x86_64-linux-gnu/libc.so.6", O_RDONLY|O_CLOEXEC);
	char buf[832];
	read(fd2, buf, 832);
	char buf2[784];
	pread(fd2, buf2, 784, 64);
	fstat(fd2, &st);
	pread(fd2, buf2, 784, 64);
	void *m3 = mmap(NULL, 2170256, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, fd2, 0);
	close(fd2);
	void *m4 = mmap(NULL, 12288, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
	struct rlimit rl;
	prlimit(0, RLIMIT_STACK, NULL, &rl);
	munmap(m2, 25015);
	
	return 0;
}