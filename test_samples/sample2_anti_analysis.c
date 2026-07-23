#define _GNU_SOURCE

#include <sys/utsname.h>
#include <sys/sysinfo.h>
#include <sys/resource.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <sys/time.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

static void benign_decoy() {

	volatile uint64_t x = 0;

	for (uint64_t i = 0; i < 10000; i++) {
		x ^= (i * 0x1337);
	}
}

static void payload_stage() {

	volatile uint64_t x = 0x41414141;

	for (uint64_t i = 0; i < 30000000; i++) {

		x ^= (i * 0x5bd1e995ULL);

		x = (x << 9) | (x >> (64 - 9));

		x += 0x13371337;
	}

	if (x == 0xdeadbeef)
		abort();
}

int main() {

   // target linux x86

	struct utsname uts;

	if (uname(&uts) != 0)
		return 0;

	// wont get thourgh this yet probs
	// if (!strstr(uts.sysname, "Linux"))
	// 	return 0;

	// if (!strstr(uts.machine, "x86_64"))
	// 	return 0;

	// if (strstr(uts.release, "azure"))
	// 	return 0;

	// if (strstr(uts.release, "aws"))
	// 	return 0;
		
	// printf("-- STAGE 1 UNAME_PROFILE\n");

	struct sysinfo info;

	if (sysinfo(&info) != 0)
		return 0;

	if (info.totalram < (4ULL * 1024 * 1024 * 1024))
		return 0;

	if (info.uptime < 1200)
		return 0;

	printf("---- STAGE 2 RESOURCE_PROFILE\n");
		
	struct timeval a, b;

	gettimeofday(&a, NULL);

	for (volatile int i = 0; i < 500000; i++);

	gettimeofday(&b, NULL);

	long elapsed =
		(b.tv_sec - a.tv_sec) * 1000000L +
		(b.tv_usec - a.tv_usec);

	if (elapsed > 500000) {

		benign_decoy();

		return 0;
	}

	printf("-------- STAGE 3 TIMING_ANALYSIS\n");
	
	int status_fd =
		open("/proc/self/status", O_RDONLY);

	if (status_fd < 0)
		return 0;

	char status_buf[4096] = {0};

	read(status_fd,
		 status_buf,
		 sizeof(status_buf)-1);

	close(status_fd);

	// wont get thourgh this yet probs
	if (memmem(
			status_buf,
			sizeof(status_buf),
			"Trace\t0",
			1
		) == NULL)
	{
		benign_decoy();

		return 0;
	}
	
	printf("---------- STAGE 4 TRACER_CHECK\n");

	char exe_path[256];

	ssize_t n =
		readlink(
			"/proc/self/exe",
			exe_path,
			sizeof(exe_path)-1
		);

	if (n <= 0)
		return 0;
	
	printf("HERE1");

	exe_path[n] = '\0';
	printf("HERE2");

	if (!strstr(exe_path, "qemu"))
		return 0;

	printf("HERE4");
	if (strstr(exe_path, "sandbox"))
		return 0;

	printf("HERE5");
	if (strstr(exe_path, "sample"))
		return 0;

	printf("HERE6");
	printf("------------ STAGE 5 EXECUTION_CONTEXT\n");

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

	if (memmem(dentbuf, dent_n, "gdb", 3))
		return 0;

	if (memmem(dentbuf, dent_n, "wireshark", 9))
		return 0;

	if (!memmem(dentbuf, dent_n, "strace", 6))
		return 0;

	if (memmem(dentbuf, dent_n, "qemu", 4))
		return 0;

	printf("-------------- STAGE 6 PROCESS_INSPECTION\n");
	
	struct rlimit rl;

	if (prlimit(0, RLIMIT_STACK, NULL, &rl) != 0)
		return 0;

	if (rl.rlim_max < (8 * 1024 * 1024))
		return 0;
		
	printf("---------------- STAGE 7 RESOURCE_LIMITS\n");
	
	printf("target environment accepted\n");

	payload_stage();

	printf("payload stage reached\n");

	return 0;
}