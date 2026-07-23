// hook_test.c
// Test sample to exercise all newly added syscall hooks.
// Each stage prints a marker to stderr so progress is visible in fuzzer logs.
// Compile static for easiest snapshot/hook placement:
//   gcc -static -O0 -o hook_test hook_test.c
// Or dynamic:
//   gcc -O0 -o hook_test hook_test.c

#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/syscall.h>
#include <sys/stat.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <netinet/in.h>
#include <time.h>
#include <errno.h>
#include <linux/openat2.h>

static void marker(const char *name) {
    fprintf(stderr, "-- STAGE %s\n", name);
}

// 127.0.0.1 in network byte order, declared early so main() can use it
static unsigned int inet_addr_fallback(void) {
    return 0x0100007f;
}

int main() {
    marker("START");

    // ---- openat / openat2 ----
    int fd = open("/proc/cpuinfo", O_RDONLY);
    fprintf(stderr, "open(/proc/cpuinfo) fd=%d errno=%d\n", fd, errno);
    if (fd >= 0) close(fd);
    marker("OPENAT_DONE");

    struct open_how how = {0};
    how.flags = O_RDONLY;
    long fd2 = syscall(SYS_openat2, AT_FDCWD, "/proc/meminfo", &how, sizeof(how));
    fprintf(stderr, "openat2(/proc/meminfo) fd=%ld errno=%d\n", fd2, errno);
    if (fd2 >= 0) close((int)fd2);
    marker("OPENAT2_DONE");

    // ---- access / faccessat / faccessat2 ----
    int a1 = access("/etc/passwd", F_OK);
    fprintf(stderr, "access(/etc/passwd) ret=%d errno=%d\n", a1, errno);

    int a2 = syscall(SYS_faccessat, AT_FDCWD, "/etc/shadow", F_OK, 0);
    fprintf(stderr, "faccessat(/etc/shadow) ret=%d errno=%d\n", a2, errno);

#ifdef SYS_faccessat2
    int a3 = syscall(SYS_faccessat2, AT_FDCWD, "/etc/hosts", F_OK, 0);
    fprintf(stderr, "faccessat2(/etc/hosts) ret=%d errno=%d\n", a3, errno);
#endif
    marker("ACCESS_DONE");

    // ---- stat / lstat / newfstatat / statx ----
    struct stat st;
    int s1 = stat("/proc/self/exe", &st);
    fprintf(stderr, "stat(/proc/self/exe) ret=%d size=%ld\n", s1, (long)st.st_size);

    struct stat st2;
    int s2 = lstat("/proc/self/exe", &st2);
    fprintf(stderr, "lstat(/proc/self/exe) ret=%d size=%ld\n", s2, (long)st2.st_size);

    struct stat st3;
    int s3 = fstatat(AT_FDCWD, "/etc/hostname", &st3, 0);
    fprintf(stderr, "fstatat(/etc/hostname) ret=%d size=%ld\n", s3, (long)st3.st_size);

#ifdef SYS_statx
    struct statx stx;
    int s4 = syscall(SYS_statx, AT_FDCWD, "/etc/os-release", 0, STATX_BASIC_STATS, &stx);
    fprintf(stderr, "statx(/etc/os-release) ret=%d size=%llu\n", s4, (unsigned long long)stx.stx_size);
#endif
    marker("STAT_DONE");

    // ---- chmod ----
    int c1 = chmod("/tmp/hook_test_dummy_file", 0755);
    fprintf(stderr, "chmod(dummy) ret=%d errno=%d\n", c1, errno);
    marker("CHMOD_DONE");

    // ---- unlink / unlinkat ----
    int u1 = unlink("/tmp/hook_test_dummy_file");
    fprintf(stderr, "unlink(dummy) ret=%d errno=%d\n", u1, errno);

    int u2 = unlinkat(AT_FDCWD, "/tmp/hook_test_dummy_file2", 0);
    fprintf(stderr, "unlinkat(dummy2) ret=%d errno=%d\n", u2, errno);
    marker("UNLINK_DONE");

    // ---- socket / connect / bind / listen / getsockname ----
    int sock = socket(AF_INET, SOCK_STREAM, 0);
    fprintf(stderr, "socket() fd=%d errno=%d\n", sock, errno);

    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons(31337);
    addr.sin_addr.s_addr = inet_addr_fallback();

    int cret = connect(sock, (struct sockaddr*)&addr, sizeof(addr));
    fprintf(stderr, "connect() ret=%d errno=%d\n", cret, errno);
    marker("CONNECT_DONE");

    int sock2 = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in baddr;
    memset(&baddr, 0, sizeof(baddr));
    baddr.sin_family = AF_INET;
    baddr.sin_port = htons(4444);
    baddr.sin_addr.s_addr = 0; // INADDR_ANY

    int bret = bind(sock2, (struct sockaddr*)&baddr, sizeof(baddr));
    fprintf(stderr, "bind() ret=%d errno=%d\n", bret, errno);

    int lret = listen(sock2, 5);
    fprintf(stderr, "listen() ret=%d errno=%d\n", lret, errno);

    struct sockaddr_in gname;
    socklen_t glen = sizeof(gname);
    int gret = getsockname(sock2, (struct sockaddr*)&gname, &glen);
    fprintf(stderr, "getsockname() ret=%d errno=%d port=%d\n", gret, errno, ntohs(gname.sin_port));
    marker("BIND_LISTEN_DONE");

    // ---- send / recv ----
    char sendbuf[32] = "ping";
    int sret = send(sock, sendbuf, strlen(sendbuf), 0);
    fprintf(stderr, "send() ret=%d errno=%d\n", sret, errno);

    char recvbuf[64] = {0};
    int rret = recv(sock, recvbuf, sizeof(recvbuf) - 1, 0);
    fprintf(stderr, "recv() ret=%d errno=%d first_bytes=%02x%02x%02x%02x\n",
            rret, errno,
            (unsigned char)recvbuf[0], (unsigned char)recvbuf[1],
            (unsigned char)recvbuf[2], (unsigned char)recvbuf[3]);
    marker("SEND_RECV_DONE");

    close(sock);
    close(sock2);

    // ---- nanosleep ----
    struct timespec req = {0, 100000000}; // 100ms
    struct timespec rem = {0, 0};
    int nret = nanosleep(&req, &rem);
    fprintf(stderr, "nanosleep() ret=%d errno=%d rem.tv_sec=%ld rem.tv_nsec=%ld\n",
            nret, errno, (long)rem.tv_sec, (long)rem.tv_nsec);
    marker("NANOSLEEP_DONE");

    // ---- execve / execveat (expect failure if hooked) ----
    char *argv[] = {"/bin/true", NULL};
    char *envp[] = {NULL};
    int eret = execve("/bin/true", argv, envp);
    // execve only returns on failure
    fprintf(stderr, "execve() returned (should only happen on failure) ret=%d errno=%d\n", eret, errno);
    marker("EXECVE_DONE");

    // ---- clone / clone3 (via fork-style wrapper) ----
    pid_t pid = fork();
    if (pid == 0) {
        fprintf(stderr, "CHILD: in child process after fork\n");
        marker("CHILD_DONE");
        _exit(0);
    } else {
        fprintf(stderr, "PARENT: fork() returned pid=%d\n", pid);
        marker("FORK_DONE");
    }

    marker("ALL_DONE");
    return 0;
}