#define _GNU_SOURCE

#include <sys/utsname.h>
#include <sys/sysinfo.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <sys/resource.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>
#include <dirent.h>

#include <sys/ioctl.h>
#include <net/if.h>
#include <sys/socket.h>
#include <termios.h>
#include <unistd.h>

#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>

static void time_wasting() {

	uint64_t x = 0x13371337;

	for (uint64_t i = 0; i < 500000; i++) {

		x ^= (i * 0x5bd1e995ULL);

		x = (x << 7) | (x >> (64 - 7));

		x += 0x41424344;
	}

	if (x == 0xdeadbeef)
		abort();
}

static void run_distraction() {

	volatile uint64_t x = 0;

	for (uint64_t i = 0; i < 100000; i++) {
		x += i;
	}
}

int main() {

	struct utsname uts;
	
	if (uname(&uts) != 0)
		return 0;

	if (!strstr(uts.sysname, "Linux"))
		return 0;
		

	if (!strstr(uts.machine, "x86_64"))
		return 0;
	
	if (strstr(uts.release, "azure"))
		return 0;

	if (strstr(uts.machine, "i686"))
		return 0;


	fprintf(stderr, "-- STAGE 1 UNAME_PROFILE\n");

	struct sysinfo info;

	if (sysinfo(&info) != 0)
		return 0;
	
	if (info.totalram != (4ULL * 1024 * 1024 * 1024)) // lastig i guess
		return 0;

	if (info.uptime < 600)
		return 0;

	fprintf(stderr, "---- STAGE 2 RESOURCE_PROFILE\n");

	char exe_path[256];
	ssize_t n = syscall(SYS_readlinkat, AT_FDCWD, "/proc/self/exe", exe_path, sizeof(exe_path)-1);
	
	exe_path[n] = '\0';

	if (!strstr(exe_path, "qemu"))
		return 0;
	
	if (strstr(exe_path, "sandbox"))
		return 0;

	if (strstr(exe_path, "tmp"))
		return 0;

	if (access("/dev/vboxguest", F_OK) == 0)
		return 0;
	
	fprintf(stderr, "------ STAGE 3 EXECUTION_CONTEXT\n");
	
	int proc_fd = open("/proc", O_RDONLY | O_DIRECTORY);

	if (proc_fd < 0)
		return 0;

	char dentbuf[4096];

	int dent_n = syscall(
		SYS_getdents64,
		proc_fd,
		dentbuf,
		sizeof(dentbuf)
	);

	close(proc_fd);

	if (dent_n <= 0)
		return 0;

	if (memmem(dentbuf, dent_n, "wireshark", 9))
		return 0;

	if (memmem(dentbuf, dent_n, "gdb", 3))
		return 0;

	if (memmem(dentbuf, dent_n, "strace", 6))
		return 0;

	if (memmem(dentbuf, dent_n, "qemu", 4))
		return 0;

	fprintf(stderr, "-------- STAGE 4 PROCESS_INSPECTION\n");

	int cpu_fd = open("/proc/cpuinfo", O_RDONLY);

	if (cpu_fd < 0)
		return 0;

	char cpuinfo[2048] = {0};

	read(cpu_fd, cpuinfo, sizeof(cpuinfo)-1);

	close(cpu_fd);

	if (strstr(cpuinfo, "hypervisor"))
		return 0;

	fprintf(stderr, "---------- STAGE 5 CPU_ENVIRONMENT\n");
	
	struct rlimit rl;

	if (prlimit(0, RLIMIT_STACK, NULL, &rl) != 0)
		return 0;

	if (rl.rlim_max < (8 * 1024 * 1024))
		return 0;

	fprintf(stderr, "------------ STAGE 6 RESOURCE_LIMITS\n");

	struct stat st;

	if (stat("/proc/self/exe", &st) != 0)
		return 0;

	if (st.st_uid == 1337)
		return 0;

	fprintf(stderr, "-------------- STAGE 7 FILE_METADATA\n");
	
	if (strstr(exe_path, "analysis")) {

		run_distraction();

		return 0;
	}

	fprintf(stderr, "env passed\n");

	time_wasting();

	fprintf(stderr, "after time wasting\n");
	

	return 0;
}