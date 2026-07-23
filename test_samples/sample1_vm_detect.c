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

static void fake_payload_stage() {

	uint64_t x = 0x13371337;

	for (uint64_t i = 0; i < 500000; i++) {

		x ^= (i * 0x5bd1e995ULL);

		x = (x << 7) | (x >> (64 - 7));

		x += 0x41424344;
	}

	if (x == 0xdeadbeef)
		abort();
}

static void decoy_stage() {

	volatile uint64_t x = 0;

	for (uint64_t i = 0; i < 100000; i++) {
		x += i;
	}
}

int main() {
	// char *prog_name = argv[0];
	// // fork();

	int fd = open("/proc/self/status", O_RDONLY);
	// fprintf(stderr, "open /proc/self/status fd=%d\n", fd);
	if (fd > 0) {
		char buf[256] = {0};
		read(fd, buf, sizeof(buf)-1);
		// fprintf(stderr, "read: %s\n", buf);
		close(fd);
	}

	struct winsize ws;
	// fprintf(stderr, "pre ioctl \n");
	if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == 0) {
		//  fprintf(stderr, "ioctl success ws_col=%d ws_row=%d\n", ws.ws_col, ws.ws_row);
		if (ws.ws_col == 0 || ws.ws_row == 0) {
			// Geen terminal → sandbox → exit
			// fprintf(stderr, "SANDVOBC\n");
			return 0;
		}
	}

// 	// fprintf(stderr, "ioctl check voorbijjjjj\n");
	struct utsname uts;

	// for(volatile int i = 0; i < 100000; i++)
	// {
	// 	uname(&uts);
	// }
	if (uname(&uts) != 0)
		return 0;
	// wont get thourgh this yet probs

	// for(volatile int i = 0; i < 100000; i++)
	// {
	// 	uname(&uts);
	// }
	// printf(
	// 	"bytes=%02x %02x %02x %02x\n",
	// 	(unsigned char)uts.sysname[0],
	// 	(unsigned char)uts.sysname[1],
	// 	(unsigned char)uts.sysname[2],
	// 	(unsigned char)uts.sysname[3]
	// );

	// fprintf(stderr, "uts.sysname = \"%s\"\n", uts.sysname);

	if (!strstr(uts.sysname, "Linux"))
		return 0;
		
	// printf("hier");
	// fprintf(stderr, "uts.sysname = \"%s\"\n", uts.sysname);
	// fprintf(stderr, " na linux check");
	
	
	// if (!strstr(uts.machine, "x86_64"))
	// 	return 0;
	// fprintf(stderr, " na utsmachine");
	
	// printf("uts.machine = \"%s\"\n", uts.machine);
	// if (*(uint32_t*)uts.sysname != 0x756e694c)
	// return 0;
	
	if (strstr(uts.release, "azure"))
		return 0;

	if (strstr(uts.machine, "i686"))
		return 0;

// 	// printf("-- STAGE 1 UNAME_PROFILE \n");
	// fprintf(stderr, "-- STAGE 1 UNAME_PROFILE\n");
// 	// // fflush(stdout);
// 	// // printf("hier2");
	

	struct sysinfo info;
	// printf("hier3");

	if (sysinfo(&info) != 0)
		return 0;
	// printf("hier4");


	if (info.totalram != (4ULL * 1024 * 1024 * 1024))
		return 0;
	// // printf("hier5");


	if (info.uptime < 600)
		return 0;

// 	// // printf("---- STAGE 2 RESOURCE_PROFILE\n");
	// fprintf(stderr, "---- STAGE 2 RESOURCE_PROFILE\n");
	// // fflush(stdout);

	char exe_path[256];

	// ssize_t n =
	// 	readlink(
	// 		"/proc/self/exe",
	// 		exe_path,
	// 		sizeof(exe_path)-1
	// 	);

	// fprintf(stderr,"hier5");
	ssize_t n = syscall(SYS_readlinkat, AT_FDCWD, "/proc/self/exe", exe_path, sizeof(exe_path)-1);
	// // fprintf(stderr,"hier6");
	// if (n <= 0)
	// 	return 0;

	// exe_path[n] = '\0';

	// printf("exe_path = \"%s\"\n", exe_path);
	// printf("n = %zd\n", n);
	// // printf("hier");
	if (!strstr(exe_path, "qemu"))
		return 0;
	printf("\nhier1");
	

	if (strstr(exe_path, "sandbox"))
		return 0;
	printf("hier2");

	if (strstr(exe_path, "tmp"))
		return 0;
	printf("hier3");

	if (access("/dev/vboxguest", F_OK) == 0)
		return 0;
	printf("hier4");

	fprintf(stderr, "------ STAGE 3 EXECUTION_CONTEXT\n");
	// fflush(stdout);
	
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
	// fflush(stdout);
	
	int cpu_fd = open("/proc/cpuinfo", O_RDONLY);

	if (cpu_fd < 0)
		return 0;

	char cpuinfo[2048] = {0};

	read(cpu_fd, cpuinfo, sizeof(cpuinfo)-1);

	close(cpu_fd);

	if (strstr(cpuinfo, "hypervisor"))
		return 0;

	fprintf(stderr, "---------- STAGE 5 CPU_ENVIRONMENT\n");
	// fflush(stdout);
	
	struct rlimit rl;

	if (prlimit(0, RLIMIT_STACK, NULL, &rl) != 0)
		return 0;

	if (rl.rlim_max < (8 * 1024 * 1024))
		return 0;

	fprintf(stderr, "------------ STAGE 6 RESOURCE_LIMITS\n");
	// fflush(stdout);
	
	struct stat st;

	if (stat("/proc/self/exe", &st) != 0)
		return 0;

	if (st.st_uid == 1337)
		return 0;

	fprintf(stderr, "-------------- STAGE 7 FILE_METADATA\n");
	// fflush(stdout);
	
	if (strstr(exe_path, "analysis")) {

		decoy_stage();

		return 0;
	}

	fprintf(stderr, "target environment accepted\n");
	// fflush(stdout);
	

	fake_payload_stage();

	fprintf(stderr, "payload stage reached\n");
	// fflush(stdout);
	

	return 0;
}