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

int main() {
	volatile char a[] = "heyy";
    volatile char b[] = "Linux";

    int x = strcmp((char*)a, (char*)b);

    printf("%d\n", x);


	return 0;
}