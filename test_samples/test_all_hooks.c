#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/sysinfo.h>
#include <sys/utsname.h>
#include <sys/time.h>
#include <sys/resource.h>
#include <sys/ptrace.h>
#include <sys/prctl.h>
#include <sys/ioctl.h>
#include <sys/socket.h>
#include <sys/vfs.h>
#include <sys/mman.h>
#include <sched.h>
#include <time.h>
#include <fcntl.h>
#include <dirent.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <errno.h>
#include <linux/random.h>
#include <sys/syscall.h>

// Helper: print bytes as hex
void print_hex(const char *label, const void *data, size_t len) {
	const unsigned char *bytes = (const unsigned char *)data;
	printf("  %s (%zu bytes): ", label, len);
	for (size_t i = 0; i < len && i < 64; i++) {
		printf("%02x ", bytes[i]);
	}
	if (len > 64) printf("... (truncated)");
	printf("\n");
}

int main() {
	// printf("=== SYSCALL HOOK TEST ===\n\n");

	// // ==========================================
	// // TIER 1: Direct Environment Info
	// // ==========================================
	// printf("--- TIER 1: Direct Environment Info ---\n\n");

	// // 1. uname (63)
	// {
	//     printf("[1] uname (syscall 63)\n");
	//     struct utsname uts;
	//     memset(&uts, 0, sizeof(uts));
	//     int ret = uname(&uts);
	//     printf("  return: %d\n", ret);
	//     printf("  sysname:    \"%s\"\n", uts.sysname);
	//     printf("  nodename:   \"%s\"\n", uts.nodename);
	//     printf("  release:    \"%s\"\n", uts.release);
	//     printf("  version:    \"%s\"\n", uts.version);
	//     printf("  machine:    \"%s\"\n", uts.machine);
	//     print_hex("raw bytes", &uts, sizeof(uts));
	//     printf("\n");
	// }

	// // 2. sysinfo (99)
	// {
	//     printf("[2] sysinfo (syscall 99)\n");
	//     struct sysinfo si;
	//     memset(&si, 0, sizeof(si));
	//     int ret = sysinfo(&si);
	//     printf("  return: %d\n", ret);
	//     printf("  uptime:   %ld seconds\n", si.uptime);
	//     printf("  totalram: %lu bytes\n", si.totalram);
	//     printf("  freeram:  %lu bytes\n", si.freeram);
	//     printf("  procs:    %d\n", si.procs);
	//     print_hex("raw bytes", &si, sizeof(si));
	//     printf("\n");
	// }

	// // 3. getpid (39)
	// {
	//     printf("[3] getpid (syscall 39)\n");
	//     pid_t pid = getpid();
	//     printf("  pid: %d\n", pid);
	//     print_hex("raw bytes", &pid, sizeof(pid));
	//     printf("\n");
	// }

	// // 4. getppid (110)
	// {
	//     printf("[4] getppid (syscall 110)\n");
	//     pid_t ppid = getppid();
	//     printf("  ppid: %d\n", ppid);
	//     print_hex("raw bytes", &ppid, sizeof(ppid));
	//     printf("\n");
	// }

	// // 5. getuid (102)
	// {
	//     printf("[5] getuid (syscall 102)\n");
	//     uid_t uid = getuid();
	//     printf("  uid: %d\n", uid);
	//     print_hex("raw bytes", &uid, sizeof(uid));
	//     printf("\n");
	// }

	// // 6. geteuid (107)
	// {
	//     printf("[6] geteuid (syscall 107)\n");
	//     uid_t euid = geteuid();
	//     printf("  euid: %d\n", euid);
	//     print_hex("raw bytes", &euid, sizeof(euid));
	//     printf("\n");
	// }

	// // 7. getgid (104)
	// {
	//     printf("[7] getgid (syscall 104)\n");
	//     gid_t gid = getgid();
	//     printf("  gid: %d\n", gid);
	//     print_hex("raw bytes", &gid, sizeof(gid));
	//     printf("\n");
	// }

	// // 8. getegid (108)
	// {
	//     printf("[8] getegid (syscall 108)\n");
	//     gid_t egid = getegid();
	//     printf("  egid: %d\n", egid);
	//     print_hex("raw bytes", &egid, sizeof(egid));
	//     printf("\n");
	// }

	// // 9. getresuid (118)
	// {
	//     printf("[9] getresuid (syscall 118)\n");
	//     uid_t ruid, euid, suid;
	//     int ret = getresuid(&ruid, &euid, &suid);
	//     printf("  return: %d\n", ret);
	//     printf("  ruid: %d, euid: %d, suid: %d\n", ruid, euid, suid);
	//     printf("\n");
	// }

	// // 10. getresgid (120)
	// {
	//     printf("[10] getresgid (syscall 120)\n");
	//     gid_t rgid, egid, sgid;
	//     int ret = getresgid(&rgid, &egid, &sgid);
	//     printf("  return: %d\n", ret);
	//     printf("  rgid: %d, egid: %d, sgid: %d\n", rgid, egid, sgid);
	//     printf("\n");
	// }

	// // 11. getcwd (79)
	// {
	//     printf("[11] getcwd (syscall 79)\n");
	//     char cwd[4096];
	//     memset(cwd, 0, sizeof(cwd));
	//     char *ret = getcwd(cwd, sizeof(cwd));
	//     printf("  return: %s\n", ret ? "success" : "NULL");
	//     printf("  cwd: \"%s\"\n", cwd);
	//     print_hex("raw bytes", cwd, strlen(cwd) + 1);
	//     printf("\n");
	// }

	// // 12. ptrace (101)
	// {
	//     printf("[12] ptrace (syscall 101)\n");
	//     long ret = ptrace(PTRACE_TRACEME, 0, NULL, NULL);
	//     printf("  PTRACE_TRACEME return: %ld (errno: %d = %s)\n",
	//            ret, errno, strerror(errno));
	//     printf("\n");
	// }

	// // ==========================================
	// // TIER 1b: Time-Related
	// // ==========================================
	// printf("--- TIER 1b: Time-Related ---\n\n");

	// // 13. time (201)
	// {
	//     printf("[13] time (syscall 201)\n");
	//     time_t t = time(NULL);
	//     printf("  time: %ld\n", (long)t);
	//     print_hex("raw bytes", &t, sizeof(t));
	//     printf("\n");
	// }

	// // 14. gettimeofday (96)
	// {
	//     printf("[14] gettimeofday (syscall 96)\n");
	//     struct timeval tv;
	//     memset(&tv, 0, sizeof(tv));
	//     int ret = gettimeofday(&tv, NULL);
	//     printf("  return: %d\n", ret);
	//     printf("  tv_sec: %ld, tv_usec: %ld\n", (long)tv.tv_sec, (long)tv.tv_usec);
	//     print_hex("raw bytes", &tv, sizeof(tv));
	//     printf("\n");
	// }

	// // 15. clock_gettime (228)
	// {
	//     printf("[15] clock_gettime (syscall 228)\n");
	//     struct timespec ts;
	//     memset(&ts, 0, sizeof(ts));
	//     int ret = clock_gettime(CLOCK_REALTIME, &ts);
	//     printf("  return: %d\n", ret);
	//     printf("  tv_sec: %ld, tv_nsec: %ld\n", (long)ts.tv_sec, ts.tv_nsec);
	//     print_hex("raw bytes", &ts, sizeof(ts));
	//     printf("\n");
	// }

	// // 16. nanosleep (35)
	// {
	//     printf("[16] nanosleep (syscall 35)\n");
	//     struct timespec req = {.tv_sec = 0, .tv_nsec = 1000000}; // 1ms
	//     struct timespec rem;
	//     memset(&rem, 0, sizeof(rem));
	//     int ret = nanosleep(&req, &rem);
	//     printf("  return: %d\n", ret);
	//     printf("  requested: %ld.%09ld\n", (long)req.tv_sec, req.tv_nsec);
	//     printf("  remaining: %ld.%09ld\n", (long)rem.tv_sec, rem.tv_nsec);
	//     printf("\n");
	// }

	// // ==========================================
	// // TIER 2: File-Based
	// // ==========================================
	// printf("--- TIER 2: File-Based ---\n\n");

	// // 17. open (2) - open a known env file
	// {
	//     printf("[17] open (syscall 2)\n");
	//     int fd = open("/proc/cpuinfo", O_RDONLY);
	//     printf("  open(\"/proc/cpuinfo\"): fd=%d (errno: %d)\n", fd, errno);
	//     if (fd >= 0) close(fd);
	//     printf("\n");
	// }

	// // 18. openat (257) - open a known env file
	// {
	//     printf("[18] openat (syscall 257)\n");
	//     int fd = openat(AT_FDCWD, "/proc/self/maps", O_RDONLY);
	//     printf("  openat(\"/proc/self/maps\"): fd=%d (errno: %d)\n", fd, errno);
	//     if (fd >= 0) close(fd);
	//     printf("\n");
	// }

	// // 19. read (0) - read from an env file
	// {
	//     printf("[19] read (syscall 0)\n");
	//     int fd = open("/proc/cpuinfo", O_RDONLY);
	//     if (fd >= 0) {
	//         char buf[256];
	//         memset(buf, 0, sizeof(buf));
	//         ssize_t n = read(fd, buf, sizeof(buf) - 1);
	//         printf("  read %zd bytes from /proc/cpuinfo\n", n);
	//         printf("  first 80 chars: \"%.80s\"\n", buf);
	//         print_hex("raw bytes", buf, n > 64 ? 64 : (n > 0 ? n : 0));
	//         close(fd);
	//     } else {
	//         printf("  could not open /proc/cpuinfo\n");
	//     }
	//     printf("\n");
	// }

	// // 20. pread64 (17)
	// {
	//     printf("[20] pread64 (syscall 17)\n");
	//     int fd = open("/proc/self/maps", O_RDONLY);
	//     if (fd >= 0) {
	//         char buf[256];
	//         memset(buf, 0, sizeof(buf));
	//         ssize_t n = pread(fd, buf, sizeof(buf) - 1, 0);
	//         printf("  pread64 %zd bytes from /proc/self/maps\n", n);
	//         printf("  first 80 chars: \"%.80s\"\n", buf);
	//         close(fd);
	//     } else {
	//         printf("  could not open /proc/self/maps\n");
	//     }
	//     printf("\n");
	// }

	// // 21. stat (4)
	// {
	//     printf("[21] stat (syscall 4)\n");
	//     struct stat st;
	//     memset(&st, 0, sizeof(st));
	//     int ret = stat("/proc/cpuinfo", &st);
	//     printf("  stat(\"/proc/cpuinfo\"): return=%d\n", ret);
	//     printf("  st_size: %ld, st_mode: %o, st_uid: %d\n",
	//            (long)st.st_size, st.st_mode, st.st_uid);
	//     print_hex("raw bytes", &st, sizeof(st));
	//     printf("\n");
	// }

	// // 22. fstat (5)
	// {
	//     printf("[22] fstat (syscall 5)\n");
	//     int fd = open("/etc/hostname", O_RDONLY);
	//     if (fd >= 0) {
	//         struct stat st;
	//         memset(&st, 0, sizeof(st));
	//         int ret = fstat(fd, &st);
	//         printf("  fstat(fd for /etc/hostname): return=%d\n", ret);
	//         printf("  st_size: %ld, st_mode: %o\n", (long)st.st_size, st.st_mode);
	//         print_hex("raw bytes", &st, sizeof(st));
	//         close(fd);
	//     } else {
	//         printf("  could not open /etc/hostname\n");
	//     }
	//     printf("\n");
	// }

	// // 23. lstat (6)
	// {
	//     printf("[23] lstat (syscall 6)\n");
	//     struct stat st;
	//     memset(&st, 0, sizeof(st));
	//     int ret = lstat("/proc/self/exe", &st);
	//     printf("  lstat(\"/proc/self/exe\"): return=%d\n", ret);
	//     printf("  st_size: %ld, st_mode: %o\n", (long)st.st_size, st.st_mode);
	//     printf("\n");
	// }

	// // 24. newfstatat (262) - via syscall directly
	// {
	//     printf("[24] newfstatat (syscall 262)\n");
	//     struct stat st;
	//     memset(&st, 0, sizeof(st));
	//     int ret = syscall(SYS_newfstatat, AT_FDCWD, "/proc/version", &st, 0);
	//     printf("  newfstatat(\"/proc/version\"): return=%d\n", ret);
	//     printf("  st_size: %ld, st_mode: %o\n", (long)st.st_size, st.st_mode);
	//     printf("\n");
	// }

	// // 25. access (21)
	// {
	//     printf("[25] access (syscall 21)\n");
	//     int ret1 = access("/usr/bin/gdb", F_OK);
	//     int ret2 = access("/usr/bin/strace", F_OK);
	//     int ret3 = access("/proc/self/status", F_OK);
	//     printf("  access(\"/usr/bin/gdb\"):          %d (errno: %d)\n", ret1, errno);
	//     printf("  access(\"/usr/bin/strace\"):        %d (errno: %d)\n", ret2, errno);
	//     printf("  access(\"/proc/self/status\"):      %d (errno: %d)\n", ret3, errno);
	//     printf("\n");
	// }

	// // 26. readlink (89)
	// {
	//     printf("[26] readlink (syscall 89)\n");
	//     char buf[256];
	//     memset(buf, 0, sizeof(buf));
	//     ssize_t n = readlink("/proc/self/exe", buf, sizeof(buf) - 1);
	//     printf("  readlink(\"/proc/self/exe\"): %zd bytes\n", n);
	//     if (n > 0) {
	//         buf[n] = '\0';
	//         printf("  target: \"%s\"\n", buf);
	//     }
	//     print_hex("raw bytes", buf, n > 0 ? n : 0);
	//     printf("\n");
	// }

	// // 27. getdents64 (217) - read /proc/ directory
	// {
	//     printf("[27] getdents64 (syscall 217)\n");
	//     int fd = open("/proc", O_RDONLY | O_DIRECTORY);
	//     if (fd >= 0) {
	//         char buf[1024];
	//         long nread = syscall(SYS_getdents64, fd, buf, sizeof(buf));
	//         printf("  getdents64(/proc): %ld bytes read\n", nread);
	//         print_hex("raw bytes (first 64)", buf, nread > 64 ? 64 : (nread > 0 ? nread : 0));
	//         close(fd);
	//     } else {
	//         printf("  could not open /proc\n");
	//     }
	//     printf("\n");
	// }

	// // 28. statfs (137)
	// {
	//     printf("[28] statfs (syscall 137)\n");
	//     struct statfs sfs;
	//     memset(&sfs, 0, sizeof(sfs));
	//     int ret = statfs("/", &sfs);
	//     printf("  statfs(\"/\"): return=%d\n", ret);
	//     printf("  f_type: 0x%lx, f_bsize: %ld, f_blocks: %ld\n",
	//            (unsigned long)sfs.f_type, (long)sfs.f_bsize, (long)sfs.f_blocks);
	//     print_hex("raw bytes", &sfs, sizeof(sfs));
	//     printf("\n");
	// }

	// // 29. fstatfs (138)
	// {
	//     printf("[29] fstatfs (syscall 138)\n");
	//     int fd = open("/", O_RDONLY);
	//     if (fd >= 0) {
	//         struct statfs sfs;
	//         memset(&sfs, 0, sizeof(sfs));
	//         int ret = fstatfs(fd, &sfs);
	//         printf("  fstatfs(fd for /): return=%d\n", ret);
	//         printf("  f_type: 0x%lx, f_bsize: %ld\n",
	//                (unsigned long)sfs.f_type, (long)sfs.f_bsize);
	//         close(fd);
	//     }
	//     printf("\n");
	// }

	// // ==========================================
	// // TIER 2b: Memory
	// // ==========================================
	// printf("--- TIER 2b: Memory ---\n\n");

	// // 30. mmap (9)
	// {
	//     printf("[30] mmap (syscall 9)\n");
	//     void *p = mmap(NULL, 4096, PROT_READ | PROT_WRITE,
	//                    MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
	//     printf("  mmap(4096 bytes): %p (errno: %d)\n", p, errno);
	//     if (p != MAP_FAILED) {
	//         print_hex("first 32 bytes of mapped region", p, 32);
	//         munmap(p, 4096);
	//     }
	//     printf("\n");
	// }

	// // 31. prlimit64 (302)
	// {
	//     printf("[31] prlimit64 (syscall 302)\n");
	//     struct rlimit rl;
	//     memset(&rl, 0, sizeof(rl));
	//     int ret = prlimit(getpid(), RLIMIT_NOFILE, NULL, &rl);
	//     printf("  prlimit64(RLIMIT_NOFILE): return=%d\n", ret);
	//     printf("  rlim_cur: %lu, rlim_max: %lu\n",
	//            (unsigned long)rl.rlim_cur, (unsigned long)rl.rlim_max);
	//     print_hex("raw bytes", &rl, sizeof(rl));
	//     printf("\n");
	// }

	// // ==========================================
	// // TIER 3: Nice to Have
	// // ==========================================
	// printf("--- TIER 3: Nice to Have ---\n\n");

	// // 32. prctl (157)
	// {
	//     printf("[32] prctl (syscall 157)\n");
	//     char name[16];
	//     memset(name, 0, sizeof(name));
	//     int ret = prctl(PR_GET_NAME, (unsigned long)name);
	//     printf("  prctl(PR_GET_NAME): return=%d, name=\"%s\"\n", ret, name);

	//     int dumpable = prctl(PR_GET_DUMPABLE);
	//     printf("  prctl(PR_GET_DUMPABLE): %d\n", dumpable);
	//     printf("\n");
	// }

	// // 33. ioctl (16) - query terminal size
	// {
	//     printf("[33] ioctl (syscall 16)\n");
	//     struct winsize ws;
	//     memset(&ws, 0, sizeof(ws));
	//     int ret = ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws);
	//     printf("  ioctl(TIOCGWINSZ): return=%d\n", ret);
	//     printf("  rows: %d, cols: %d\n", ws.ws_row, ws.ws_col);
	//     printf("\n");
	// }

	// // 34. socket (41) + connect (42)
	// {
	//     printf("[34] socket (syscall 41) + connect (syscall 42)\n");
	//     int sockfd = socket(AF_INET, SOCK_STREAM, 0);
	//     printf("  socket(): fd=%d (errno: %d)\n", sockfd, errno);
	//     if (sockfd >= 0) {
	//         struct sockaddr_in addr;
	//         memset(&addr, 0, sizeof(addr));
	//         addr.sin_family = AF_INET;
	//         addr.sin_port = htons(80);
	//         inet_pton(AF_INET, "8.8.8.8", &addr.sin_addr);
	//         int ret = connect(sockfd, (struct sockaddr *)&addr, sizeof(addr));
	//         printf("  connect(8.8.8.8:80): return=%d (errno: %d = %s)\n",
	//                ret, errno, strerror(errno));
	//         close(sockfd);
	//     }
	//     printf("\n");
	// }

	// // 35. sched_getaffinity (204)
	// {
	//     printf("[35] sched_getaffinity (syscall 204)\n");
	//     cpu_set_t mask;
	//     CPU_ZERO(&mask);
	//     int ret = sched_getaffinity(0, sizeof(mask), &mask);
	//     printf("  return: %d\n", ret);
	//     int count = CPU_COUNT(&mask);
	//     printf("  CPU count: %d\n", count);
	//     print_hex("raw mask bytes", &mask, 16);
	//     printf("\n");
	// }

	// // 36. getrusage (98)
	// {
	//     printf("[36] getrusage (syscall 98)\n");
	//     struct rusage ru;
	//     memset(&ru, 0, sizeof(ru));
	//     int ret = getrusage(RUSAGE_SELF, &ru);
	//     printf("  return: %d\n", ret);
	//     printf("  user time: %ld.%06ld\n",
	//            (long)ru.ru_utime.tv_sec, (long)ru.ru_utime.tv_usec);
	//     printf("  system time: %ld.%06ld\n",
	//            (long)ru.ru_stime.tv_sec, (long)ru.ru_stime.tv_usec);
	//     printf("  max RSS: %ld KB\n", ru.ru_maxrss);
	//     print_hex("raw bytes", &ru, sizeof(ru));
	//     printf("\n");
	// }

	// // 37. getrandom (318)
	// {
	//     printf("[37] getrandom (syscall 318)\n");
	//     unsigned char buf[32];
	//     memset(buf, 0, sizeof(buf));
	//     ssize_t ret = syscall(SYS_getrandom, buf, sizeof(buf), 0);
	//     printf("  getrandom(32 bytes): return=%zd\n", ret);
	//     print_hex("random bytes", buf, ret > 0 ? ret : 0);
	//     printf("\n");
	// }

	// ==========================================
	// COMBINED ENVIRONMENT CHECKS (like real malware)
	// ==========================================
	// printf("--- COMBINED: Realistic Malware Checks ---\n\n");

	// Check 1: Is this Linux? -> okay
	{
	// printf("[COMBINED 1] OS Check via uname\n");
	struct utsname uts;
	uname(&uts);
	// if (uname(&uts) != 0)
	// return 0;
	if (strstr(uts.sysname, "Linux")) {
		printf("  PASS: Running on Linux (\"%s\")\n", uts.sysname);
		fflush(stdout);
	} else {
		// printf("  FAIL\n", uts.sysname);
		fflush(stdout);
		// return 0;
	}
	printf("\n");
	}

	// Check 2: Is uptime suspicious? -> ok
	{
		printf("[COMBINED 2] Uptime Check via sysinfo\n");
		struct sysinfo si;
		sysinfo(&si);
		if (si.uptime < 300) {
			printf("  SUSPICIOUS: Uptime only %ld seconds - looks like fresh VM\n", si.uptime);
		} else {
			printf("  OK: Uptime %ld seconds (%ld hours)\n", si.uptime, si.uptime / 3600);
		}
		printf("\n");
	}

	// Check 3: Running as root? -> ok
	{
		printf("[COMBINED 3] Privilege Check\n");
		uid_t uid = getuid();
		uid_t euid = geteuid();
		printf("  uid=%d, euid=%d\n", uid, euid);
		if (euid == 0) {
			printf("  Running as root\n");
		} else {
			printf("  Running as normal user\n");
		}
		printf("\n");
	}

	// Check 4: VM detection via DMI --> okay after a while
	{
		printf("[COMBINED 4] VM Detection via /sys/class/dmi\n");
		int fd = open("/sys/class/dmi/id/product_name", O_RDONLY);
		if (fd >= 0) {
			char buf[256];
			memset(buf, 0, sizeof(buf));
			ssize_t n = read(fd, buf, sizeof(buf) - 1);
			if (n > 0) {
				// Remove trailing newline
				if (buf[n-1] == '\n') buf[n-1] = '\0';
				printf("  Product name: \"%s\"\n", buf);
				if (strstr(buf, "VirtualBox") || strstr(buf, "VMware") ||
					strstr(buf, "QEMU") || strstr(buf, "KVM")) {
					printf("  DETECTED: Virtual machine!\n");
				} else {
					// printf("  Looks like a real machine\n");
				}
			}
			close(fd);
		} else {
			printf("  Could not read DMI info (errno: %d)\n", errno);
		}
		printf("\n");
	}

	// Check 5: Debugger detection via ptrace --> okay
	{
		printf("[COMBINED 5] Debugger Detection via ptrace\n");
		long ret = ptrace(PTRACE_TRACEME, 0, NULL, NULL);
		if (ret == -1) {
			printf("  DETECTED: Being traced! (ptrace returned -1)\n");
		} else {
			printf("  OK: Not being traced\n");
		}
		printf("\n");
	}

	// Check 6: CPU count via sched_getaffinity
	{
		printf("[COMBINED 6] CPU Count Check\n");
		cpu_set_t mask;
		CPU_ZERO(&mask);
		sched_getaffinity(0, sizeof(mask), &mask);
		// syscall(SYS_sched_getaffinity, 0, sizeof(mask), &mask);
		// printf(" mask: \"%s\"\n", &mask);
		int cpus = CPU_COUNT(&mask);
		printf("  CPUs: %d\n", cpus);
		if (cpus <= 2) {
			printf("  SUSPICIOUS: Only %d CPUs - might be a VM\n", cpus);
		} else {
			printf("  OK:6 %d CPUs\n", cpus);
		}
		printf("\n");
	}

	// // Check 7: Process check via /proc readdir
	// {
	//     printf("[COMBINED 7] Process Enumeration via /proc\n");
	//     int process_count = 0;
	//     DIR *dir = opendir("/proc");
	//     if (dir) {
	//         struct dirent *entry;
	//         while ((entry = readdir(dir)) != NULL) {
	//             // Count numeric directories (PIDs)
	//             if (entry->d_name[0] >= '0' && entry->d_name[0] <= '9') {
	//                 process_count++;
	//             }
	//         }
	//         closedir(dir);
	//     }
	//     printf("  Running processes: %d\n", process_count);
	//     if (process_count < 20) {
	//         printf("  SUSPICIOUS: Very few processes - sandbox?\n");
	//     } else {
	//         printf("  OK: Normal process count\n");
	//     }
	//     printf("\n");
	// }

	// Check 8: Timing check --> okay
	{
		printf("[COMBINED 8] Timing Check\n");
		struct timespec start, end;
		clock_gettime(CLOCK_MONOTONIC, &start);

		// Do some work
		volatile int x = 0;
		for (int i = 0; i < 1000000; i++) x += i;

		clock_gettime(CLOCK_MONOTONIC, &end);
		long elapsed_ns = (end.tv_sec - start.tv_sec) * 1000000000L +
						  (end.tv_nsec - start.tv_nsec);
		printf("  Loop took %ld ns\n", elapsed_ns);
		if (elapsed_ns > 1000000000L) { // > 1 second for a simple loop
			printf("  SUSPICIOUS: Too slow - being instrumented?\n");
		} else {
			printf("  OK: Normal timing\n");
		}
		printf("\n");
	}

	printf("=== ALL TESTS COMPLETE ===\n");
	return 0;
}