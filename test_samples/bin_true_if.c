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
#include <stdlib.h>
// #include <string.h>
#include <sys/syscall.h>
#include <dirent.h>
#include <stdint.h>

int main() {

	struct utsname uts;
	uname(&uts);
	// printf("uts machine = %s\n",uts.sysname);

	// if (strcmp(uts.sysname, "LL") == 0) {
	// if (memcmp(uts.sysname, "Hi", 2) == 0){
	// 	// printf("UNAME CHECK FAILED\n");
	// 	printf("UNAME\n");
	// 	// __builtin_trap();
	// 	// abort();
	// 	return 1;
	// }else 
	if (*(uint16_t*)uts.sysname == 0x6948){
		// printf("sys1 UNAME\n");
		// __builtin_trap();
		// abort();
		return 1;

	}else{
		// printf("UNAME CHECK PASSED\n");
		// __builtin_trap();
		// return 1;
	}

	
	brk(NULL);
	void *m1 = mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
	if (access("/etc/ld.so.preload", R_OK) == 0)
	{
		// printf("sys2 ACCESS: file exists\n");
		abort();
		
	}
	int fd = openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC);
	struct stat st;
	fstat(fd, &st);
	// printf("stsize = %ld\n",st.st_size);

	if (st.st_size == 0x44444444)
	{
		// printf("sys3 FSTAT VALUE FOUND == 0x41414141\n");
		__builtin_trap();
	} 
	void *m2 = mmap(NULL, 25015, PROT_READ, MAP_PRIVATE, fd, 0);
	close(fd);
	int fd2 = openat(AT_FDCWD, "/lib/x86_64-linux-gnu/libc.so.6", O_RDONLY|O_CLOEXEC);
	char buf[832];
	read(fd2, buf, 832);
	char buf2[784];
	pread64(fd2, buf2, 784, 64);
	fstat(fd2, &st);
	pread(fd2, buf2, 784, 64);
	void *m3 = mmap(NULL, 2170256, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, fd2, 0);
	close(fd2);
	void *m4 = mmap(NULL, 12288, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
	struct rlimit rl;
	prlimit(0, RLIMIT_STACK, NULL, &rl);
	if (rl.rlim_max == 0x41414141)
	{
		// printf("sys5 MAXX PRLIMIT\n");
		__builtin_trap();
	} 

	int fd6 = open("/proc/cpuinfo", O_RDONLY);
	if (fd6 >= 0) {

		char buf[2048] = {0};
		read(fd6, buf, sizeof(buf)-1);
		if (*(uint32_t*)buf == 0x69696969) {
			// printf("sys10 HYPERVISOR FOUND\n");
			return 1;
		}
	}

	munmap(m2, 25015);
	struct sysinfo info;
	sysinfo(&info);

	if (info.totalram < (2ULL * 1024 * 1024 * 1024)) {
		// printf("sys6 SYSINFO\n");
		return 1;
	}else{
		// printf("HIGH RAM DETECTED\n");
	}

	char buf3[256];

	ssize_t n = readlink("/proc/self/exe", buf3, sizeof(buf3) - 1);

	if (n > 0) {
		buf3[n] = '\0';

		// printf("HIERERERER\n");
		// printf("buf3 = %s\n", buf3);
		if (strstr(buf3, "qemu")) {
			// printf("QEMU DETECTED\n");
			return 1;
		}
		if (n == 0x41) {
			// printf("sys7 Readlink size\n");
			return 1;
		}
	}

	int fd4 = open("/proc", O_RDONLY | O_DIRECTORY);
	char buf4[4096];
	int n4 = syscall(SYS_getdents64, fd4, buf4, sizeof(buf4));

	if (n4 > 0) {

		if (memmem(buf4, n4, "qemu", 4) != NULL) {
			// printf("QEMU PROCESS FOUND\n");
			return 1;
		}
	}
	if (n4 == 0x44) {
		// printf("sys8 GETDENTS64 HIT\n");
		return 1;
	}

	struct stat st2;

	// printf("STAT callinf\n");
	if (stat("/proc/self/exe", &st2) == 0) {

		if (st2.st_size == 0x41444144) {
			// printf("sys9 STAT HIT\n");
			return 1;
		}
	}

	// ADDED 
	
	return 0;
}